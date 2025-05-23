// Copyright Valkey GLIDE Project Contributors - SPDX Identifier: Apache-2.0

package api

import (
	"github.com/valkey-io/valkey-glide/go/api/config"
	"github.com/valkey-io/valkey-glide/go/api/options"
)

// GenericClusterCommands supports commands for the "Generic Commands" group for cluster client.
//
// See [valkey.io] for details.
//
// [valkey.io]: https://valkey.io/commands/#generic
type GenericClusterCommands interface {
	CustomCommand(args []string) (ClusterValue[interface{}], error)

	CustomCommandWithRoute(args []string, route config.Route) (ClusterValue[interface{}], error)

	Scan(cursor options.ClusterScanCursor) (options.ClusterScanCursor, []string, error)

	ScanWithOptions(
		cursor options.ClusterScanCursor,
		opts options.ClusterScanOptions,
	) (options.ClusterScanCursor, []string, error)

	RandomKey() (Result[string], error)

	RandomKeyWithRoute(opts options.RouteOption) (Result[string], error)
}
