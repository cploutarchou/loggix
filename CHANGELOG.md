# Changelog

All notable changes to Loggix will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.1/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Apache Kafka Integration
  - Real-time log streaming support via KafkaHook
  - JSON-formatted log messages
  - Configurable Kafka topics and brokers
  - Asynchronous message delivery
  - Example code for Kafka integration
- Custom Formatter Support
  - Public Formatter trait for implementing custom formatters
  - Comprehensive example with CSV formatter
  - Documentation for creating custom formatters
  - Full access to log entry fields for formatting

## [1.0.3] - 2025-04-06

### Added
- Added support for custom message keys in Kafka hook
- Added comprehensive benchmarks for sync and async logging
- Added YAML configuration support
- Added more examples including Kafka integration
- Added performance tips and best practices
- Added structured logging examples with proper types

### Changed
- Improved README with detailed feature documentation
- Updated code examples with proper type information
- Enhanced error handling in Kafka integration
- Optimized async logging performance

### Fixed
- Fixed field types in benchmarks
- Fixed documentation inconsistencies

## [1.0.2] - 2025-04-05

### Added
- Added Kafka integration
- Added hook system for log processing
- Added JSON formatter
- Added structured logging with fields
- Added colorized console output

## [1.0.1] - 2024-12-06

### Added
- Seven log levels: Trace, Debug, Info, Warning, Error, Fatal, and Panic
- Structured logging with fields support
- Beautiful terminal output with colors for TTY
- JSON formatter for machine processing
- Extensible hook system
- Thread-safe logging capabilities
- Global and local logger instances
- Customizable formatters
- Support for any output implementing `std::io::Write`
- Basic logging macros: `debug!`, `info!`, `warn!`, `error!`
- Structured logging with `with_fields!` macro
- Comprehensive documentation and examples
- MIT License
- Benchmarking suite for performance testing
- Example code for basic and advanced usage

### Changed
- Initial stable release, establishing public API

[1.0.1]: https://github.com/cploutarchou/loggix/releases/tag/v1.0.1
