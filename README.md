# Valkey GLIDE
Valkey General Language Independent Driver for the Enterprise (GLIDE), is an open-source Valkey client library. Valkey GLIDE is one of the official client libraries for Valkey, and it supports all Valkey commands. Valkey GLIDE supports Valkey 7.2 and above, and Redis open-source 6.2, 7.0 and 7.2. Application programmers use Valkey GLIDE to safely and reliably connect their applications to Valkey- and Redis OSS- compatible services. Valkey GLIDE is designed for reliability, optimized performance, and high-availability, for Valkey and Redis OSS based applications. It is sponsored and supported by AWS and GCP, and is pre-configured with best practices learned from over a decade of operating Redis OSS-compatible services used by hundreds of thousands of customers. To help ensure consistency in application development and operations, Valkey GLIDE is implemented using a core driver framework, written in Rust, with language specific extensions. This design ensures consistency in features across languages and reduces overall complexity.

## Supported Engine Versions
Valkey GLIDE is API-compatible with the following engine versions:

| Engine Type           |  6.2  |  7.0  |  7.1  |  7.2  |  8.0  |
|-----------------------|-------|-------|-------|-------|-------|
| Valkey                |   -   |   -   |   -   |   V   |   V   |
| Redis                 |   V   |   V   |   V   |   V   |   -   |

## Current Status and upcoming releases
In the current release, Valkey GLIDE is available for Python, Java and Node.js. Support for Go is currently in **public preview** and support for C# is **under active development**, with plans to include more programming languages in the future.

#### v1.2 (Dec. 2024)
- Vector Similarity Search and JSON modules support
- Availability zone routing for Read from Replica

#### v1.3 (Feb. 2025)
- Public preview for GO support

## Getting Started

**Documentation** 
GLIDE's [documentation site](https://valkey.io/valkey-glide/) currently offers documentation for the Python and Node wrappers. 

**SDKs**
- [Java](./java/README.md)
- [Python](./python/README.md)
- [Node](./node/README.md)
- [Go](./go/README.md)

**General Concepts:** 
- [Custom Command](https://github.com/valkey-io/valkey-glide/wiki/General-Concepts#custom-command)
- [Connection Management](https://github.com/valkey-io/valkey-glide/wiki/General-Concepts#connection-management)
- [Multi-Slot Command Handling](https://github.com/valkey-io/valkey-glide/wiki/General-Concepts#multi-slot-command-handling)
- [Inflight Request Limit](https://github.com/valkey-io/valkey-glide/wiki/General-Concepts#inflight-request-limit)
- [PubSub Support](https://github.com/valkey-io/valkey-glide/wiki/General-Concepts#pubsub-support)
- [Cluster Scan](https://github.com/valkey-io/valkey-glide/wiki/General-Concepts#cluster-scan)
- [Dynamic Password Management](https://github.com/valkey-io/valkey-glide/wiki/General-Concepts#dynamic-password-management)
- [Modules API](https://github.com/valkey-io/valkey-glide/wiki/General-Concepts#modules-api)

**Migration Guides**
- [Jedis](https://github.com/valkey-io/valkey-glide/wiki/Migration-Guide-Jedis)

**Community**
- [Contributors meeting](https://github.com/valkey-io/valkey-glide/wiki/Contributors-meeting)

Looking for more? Check out the [Valkey Glide Wiki](https://github.com/valkey-io/valkey-glide/wiki).

## Getting Help
If you have any questions, feature requests, encounter issues, or need assistance with this project, please don't hesitate to open a GitHub issue. Our community and contributors are here to help you. Before creating an issue, we recommend checking the [existing issues](https://github.com/valkey-io/valkey-glide/issues) to see if your question or problem has already been addressed. If not, feel free to create a new issue, and we'll do our best to assist you. Please provide as much detail as possible in your issue description, including:

1. A clear and concise title
2. Detailed description of the problem or question
3. Reproducible test case or step-by-step instructions
4. Valkey GLIDE version in use
5. Operating system details
6. Server version
7. Cluster or standalone setup information, including topology, number of shards, number of replicas, and data types used
8. Relevant modifications you've made
9. Any unusual aspects of your environment or deployment
10. Log files

## Contributing

GitHub is a platform for collaborative coding. If you're interested in writing code, we encourage you to contribute by submitting pull requests from forked copies of this repository. Additionally, please consider creating GitHub issues for reporting bugs and suggesting new features. Feel free to comment on issues that interest. For more info see [Contributing](./CONTRIBUTING.md).

## License
* [Apache License 2.0](./LICENSE)
