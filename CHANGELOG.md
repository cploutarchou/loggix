# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.3] - 2025-04-06

### Added
- Apache Kafka Integration
  - Real-time log streaming support via KafkaHook
  - JSON-formatted log messages
  - Configurable Kafka topics and brokers
  - Asynchronous message delivery
  - Example code for Kafka integration
  - Message key support via `with_key_field`
- Custom Formatter Support
  - Public Formatter trait for implementing custom formatters
  - Comprehensive example with CSV formatter
  - Documentation for creating custom formatters
  - Full access to log entry fields for formatting
- Comprehensive benchmarking suite for sync/async logging
- YAML configuration support
- New examples:
  - Kafka integration
  - Custom formatters
  - Structured logging
  - Performance tips and best practices documentation

### Changed
- Enhanced README with detailed feature documentation
- Improved code examples with proper type information
- Optimized async logging performance
- Updated benchmarks with actual performance numbers

### Fixed
- Field types in benchmarks
- Documentation inconsistencies
- Error handling in Kafka integration

## [1.0.2] - 2025-04-05

### Added
- Kafka integration with async support
- Hook system for log processing
- JSON formatter
- Structured logging with fields
- Colorized console output

## [1.0.1] - 2025-04-04

### Added
- Initial release
- Basic logging functionality
- Text formatter
- Multiple log levels
- Thread-safe logging

[1.0.3]: https://github.com/cploutarchou/loggix/compare/v1.0.2...v1.0.3
[1.0.2]: https://github.com/cploutarchou/loggix/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/cploutarchou/loggix/releases/tag/v1.0.1
