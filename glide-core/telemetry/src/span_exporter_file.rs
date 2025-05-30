use chrono::{DateTime, Utc};
use core::fmt;
use futures_util::future::BoxFuture;
use opentelemetry::trace::TraceError;
use opentelemetry_sdk::export::{self, trace::ExportResult};
use serde_json::{Map, Value};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic;

use opentelemetry_sdk::resource::Resource;

/// An OpenTelemetry exporter that writes Spans to a file on export.
pub struct SpanExporterFile {
    resource: Resource,
    is_shutdown: atomic::AtomicBool,
    path: PathBuf,
}

impl fmt::Debug for SpanExporterFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SpanExporterFile")
    }
}

impl SpanExporterFile {
    /// Creates a new SpanExporterFile that writes Spans to a file on export.
    ///
    /// # Arguments
    /// * `path` - The path where metrics will be written. This can be either a file or directory path.
    ///
    /// # Behavior
    /// - If the path points to a directory:
    ///   - The directory must exist
    ///   - Metrics will be written to a file named "signals.json" within that directory
    /// - If the path points to a file:
    ///   - If the file exists, new metrics will be appended to it (existing data is preserved)
    ///   - If the file doesn't exist, it will be created
    ///   - The parent directory must exist
    ///
    /// # Errors
    /// Returns a TraceError if:
    /// - The parent directory doesn't exist
    /// - The path points to a directory that doesn't exist
    /// - The user doesn't have write permissions for the target location
    pub fn new(path: PathBuf) -> Result<Self, TraceError> {
        // TODO: Check if the file exists and has write permissions - https://github.com/valkey-io/valkey-glide/issues/3720
        Ok(Self {
            resource: Resource::default(),
            is_shutdown: atomic::AtomicBool::new(false),
            path,
        })
    }
}

macro_rules! file_writeln {
    ($file:expr, $content:expr) => {{
        if let Err(e) = writeln!($file, "{}", $content) {
            return Box::pin(std::future::ready(Err(TraceError::from(format!(
                "File write error. {e}",
            )))));
        }
    }};
}

impl opentelemetry_sdk::export::trace::SpanExporter for SpanExporterFile {
    /// Write Spans to JSON file
    fn export(&mut self, batch: Vec<export::trace::SpanData>) -> BoxFuture<'static, ExportResult> {
        if self.is_shutdown.load(atomic::Ordering::SeqCst) {
            return Box::pin(std::future::ready(Err(TraceError::from(
                "Exporter is shutdown",
            ))));
        }

        // TODO: Move the writes to Tokio task - https://github.com/valkey-io/valkey-glide/issues/3720
        let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
        else {
            return Box::pin(std::future::ready(Err(TraceError::from(format!(
                "Unable to open exporter file: {} for append.",
                self.path.display()
            )))));
        };

        let spans = to_jsons(batch);

        for span in &spans {
            if let Ok(s) = serde_json::to_string(&span) {
                file_writeln!(file, s);
            }
        }
        Box::pin(std::future::ready(Ok(())))
    }

    fn shutdown(&mut self) {
        self.is_shutdown.store(true, atomic::Ordering::SeqCst);
    }

    fn set_resource(&mut self, res: &opentelemetry_sdk::Resource) {
        self.resource = res.clone();
    }
}

fn to_jsons(batch: Vec<export::trace::SpanData>) -> Vec<Value> {
    let mut spans = Vec::<Value>::new();
    for span in &batch {
        let mut map = Map::new();
        map.insert(
            "scope".to_owned(),
            Value::String(span.instrumentation_scope.name().to_string()),
        );
        if let Some(version) = &span.instrumentation_scope.version() {
            map.insert("version".to_owned(), Value::String(version.to_string()));
        }
        if let Some(schema_url) = &span.instrumentation_scope.schema_url() {
            map.insert(
                "schema_url".to_owned(),
                Value::String(schema_url.to_string()),
            );
        }

        let mut scope_attributes = Vec::<Value>::new();
        for kv in span.instrumentation_scope.attributes() {
            let mut attr = Map::new();
            attr.insert(kv.key.to_string(), Value::String(kv.value.to_string()));
            scope_attributes.push(Value::Object(attr));
        }
        map.insert(
            "scope_attributes".to_owned(),
            Value::Array(scope_attributes),
        );
        map.insert("name".to_owned(), Value::String(span.name.to_string()));
        map.insert(
            "span_id".to_owned(),
            Value::String(span.span_context.span_id().to_string()),
        );
        map.insert(
            "parent_span_id".to_owned(),
            Value::String(span.parent_span_id.to_string()),
        );
        map.insert(
            "trace_id".to_owned(),
            Value::String(span.span_context.trace_id().to_string()),
        );
        map.insert(
            "kind".to_owned(),
            Value::String(format!("{:?}", span.span_kind)),
        );

        let datetime: DateTime<Utc> = span.start_time.into();
        map.insert(
            "start_time".to_owned(),
            Value::String(datetime.timestamp_micros().to_string()),
        );

        let datetime: DateTime<Utc> = span.end_time.into();
        map.insert(
            "end_time".to_owned(),
            Value::String(datetime.timestamp_micros().to_string()),
        );

        map.insert(
            "status".to_owned(),
            Value::String(format!("{:?}", span.status)),
        );

        // Add the span attributes
        let mut span_attributes = Vec::<Value>::new();
        for kv in span.attributes.iter() {
            let mut attr = Map::new();
            attr.insert(kv.key.to_string(), Value::String(kv.value.to_string()));
            span_attributes.push(Value::Object(attr));
        }
        map.insert("span_attributes".to_owned(), Value::Array(span_attributes));

        // Add span events
        let mut events = Vec::<Value>::new();
        for event in span.events.iter() {
            let mut evt = Map::new();
            evt.insert("name".to_owned(), Value::String(event.name.to_string()));
            let datetime: DateTime<Utc> = event.timestamp.into();
            evt.insert(
                "timestamp".to_owned(),
                Value::String(datetime.format("%Y-%m-%d %H:%M:%S%.6f").to_string()),
            );

            let mut event_attributes = Vec::<Value>::new();
            for kv in event.attributes.iter() {
                let mut attr = Map::new();
                attr.insert(kv.key.to_string(), Value::String(kv.value.to_string()));
                event_attributes.push(Value::Object(attr));
            }
            evt.insert(
                "event_attributes".to_owned(),
                Value::Array(event_attributes),
            );
            events.push(Value::Object(evt));
        }
        map.insert("events".to_owned(), Value::Array(events));

        let mut links = Vec::<Value>::new();
        for link in span.links.iter() {
            let mut lk = Map::new();
            lk.insert(
                "trace_id".to_owned(),
                Value::String(link.span_context.trace_id().to_string()),
            );
            lk.insert(
                "span_id".to_owned(),
                Value::String(link.span_context.span_id().to_string()),
            );
            links.push(Value::Object(lk));
        }
        map.insert("links".to_owned(), Value::Array(links));
        spans.push(Value::Object(map));
    }
    spans
}
