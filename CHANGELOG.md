# Changelog

All notable changes to Loggix will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-12-06

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

[1.0.0]: https://github.com/cploutarchou/loggix/releases/tag/v1.0.0
