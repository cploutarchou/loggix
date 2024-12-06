# Loggix 🦀

[![Crates.io](https://img.shields.io/crates/v/loggix.svg)](https://crates.io/crates/loggix)
[![Documentation](https://docs.rs/loggix/badge.svg)](https://docs.rs/loggix)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/d/loggix.svg)](https://crates.io/crates/loggix)

A powerful structured logger for Rust, inspired by [Logrus](https://github.com/sirupsen/logrus). Loggix combines structured logging with Rust's safety and performance guarantees.

## Features

- 🎯 Seven log levels: Trace, Debug, Info, Warning, Error, Fatal, and Panic
- 🔍 Structured logging with fields
- 🎨 Beautiful terminal output with colors (when TTY is attached)
- 📊 JSON formatter for machine processing
- 🪝 Extensible hook system
- 🔒 Thread-safe by default
- 🌍 Global and local logger instances
- 📝 Customizable formatters
- 🎮 Full control over output (any type implementing `std::io::Write`)

## Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
loggix = "1.0.2"
```

### Basic Logging

```rust
use loggix::{info, debug, warn, error};

fn main() {
    debug!("Debug message");
    info!("Info message");
    warn!("Warning message");
    error!("Error message");
}
```

### Structured Logging

```rust
use loggix::with_fields;

fn main() {
    // Log with structured fields
    with_fields!(
        "user_id" => "12345",
        "action" => "login",
        "ip" => "192.168.1.1"
    )
    .info("User login successful")
    .unwrap();
}
```

### JSON Output

```rust
use loggix::{Logger, JSONFormatter, with_fields};

fn main() {
    let logger = Logger::new()
        .formatter(JSONFormatter::new().pretty(true))
        .build();

    with_fields!(
        "transaction_id" => "tx-9876",
        "amount" => 150.50,
        "currency" => "USD"
    )
    .info("Payment processed")
    .unwrap();
}
```

Output:
```json
{
  "timestamp": "2024-12-06T20:30:21.103Z",
  "level": "info",
  "message": "Payment processed",
  "transaction_id": "tx-9876",
  "amount": 150.50,
  "currency": "USD"
}
```

### Error Handling

```rust
use loggix::with_error;
use std::fs::File;

fn main() {
    let result = File::open("non_existent.txt");
    if let Err(error) = result {
        with_error(&error)
            .error("Failed to open file")
            .unwrap();
    }
}
```

### Custom Logger Instance

```rust
use loggix::{Logger, Level, TextFormatter};

fn main() {
    let logger = Logger::new()
        .level(Level::Debug)
        .formatter(TextFormatter::new()
            .timestamp_format("%Y-%m-%d %H:%M:%S")
            .colors(true)
            .build())
        .build();

    logger.with_fields(Default::default())
        .with_field("component", "auth")
        .info("Authentication successful")
        .unwrap();
}
```

## Advanced Usage

### Custom Formatters

Implement the `Formatter` trait to create your own log format:

```rust
use loggix::{Formatter, Entry};
use std::error::Error;

struct MyFormatter;

impl Formatter for MyFormatter {
    fn format(&self, entry: &Entry) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut output = Vec::new();
        // Format the log entry however you want
        write!(&mut output, "MY-LOG: {} - {}", entry.level, entry.message)?;
        Ok(output)
    }
}
```

### Custom Hooks

Implement the `Hook` trait to process log entries:

```rust
use loggix::{Hook, Entry, Level};

struct MetricsHook;

impl Hook for MetricsHook {
    fn fire(&self, entry: &Entry) -> Result<(), Box<dyn Error>> {
        // Send metrics to your metrics system
        if entry.level == Level::Error {
            // Record error metrics
        }
        Ok(())
    }
    
    fn levels(&self) -> Vec<Level> {
        vec![Level::Error, Level::Fatal]
    }
}
```

## Examples

Check out the [examples](examples) directory for more detailed examples:

- [Basic Logging](examples/basic_logging.rs)
- [Structured Logging](examples/structured_logging.rs)
- [JSON Logging](examples/json_logging.rs)
- [Error Handling](examples/error_handling.rs)

## Performance

Loggix is designed for high performance while maintaining flexibility. Here are some key performance characteristics:

### Benchmark Results

```
Basic logging:           813.57 ns/iter
Structured logging:      1.34 µs/iter   (with 2 fields)
Multiple fields:         2.23 µs/iter   (with 4 fields)
```

Key performance features:
- Zero-allocation logging paths for common use cases
- Efficient field storage using pre-allocated hashmaps
- Lock-free architecture where possible
- Linear scaling with number of fields
- Thread-safe by default with minimal overhead

### Running Benchmarks

Run the benchmarks yourself with:
```bash
cargo bench
```

The benchmarks use [Criterion.rs](https://github.com/bheisler/criterion.rs) for statistical analysis and reliable measurements.

## Thread Safety

Loggix is designed to be thread-safe by default. All logging operations are atomic and can be safely used across multiple threads. The library uses `Arc` and `Mutex` internally to protect shared state.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## Roadmap

Here are the planned features and enhancements for Loggix:

### Data Store Integration
- 🗄️ Database Support
  - PostgreSQL integration for persistent logging
  - MongoDB support for document-based logging
  - ClickHouse for high-performance analytics
  - TimescaleDB for time-series data

### Message Queue & Streaming
- 🚀 Apache Kafka Integration
  - Real-time log streaming
  - Multi-topic support
  - Partitioning strategies
- 🌊 Redis Streams Support
- 🔄 RabbitMQ Integration

### Search & Analytics
- 🔍 Elasticsearch Integration
  - Full-text search capabilities
  - Log aggregation and analysis
  - Custom mapping templates
- 📊 OpenSearch Support

### Advanced Features
- 💹 Trading Systems Support
  - High-frequency trading logs
  - Order execution tracking
  - Market data logging
- 🔐 Enhanced Security
  - Log encryption at rest
  - Audit trail capabilities
  - GDPR compliance features
- 🌐 Distributed Systems
  - Distributed tracing integration
  - OpenTelemetry support
  - Correlation ID tracking

### Performance & Scaling
- 🚄 High-Performance Mode
  - Zero-copy logging
  - Lock-free implementation
  - Memory-mapped files
- 🎯 Load Balancing
  - Dynamic log routing
  - Automatic failover
  - Horizontal scaling

### Monitoring & Alerting
- 📡 Real-time Monitoring
  - Custom metrics export
  - Prometheus integration
  - Health check endpoints
- ⚡ Alert System
  - Configurable thresholds
  - Multiple notification channels
  - Alert aggregation

### Additional Features
- 🔄 Log Rotation
  - Size-based rotation
  - Time-based rotation
  - Compression support
- 🎨 Advanced Formatting
  - Custom template engine
  - Multiple output formats
  - Dynamic field masking
- 🧪 Testing Tools
  - Mock logger implementation
  - Assertion helpers
  - Performance benchmarks

These features are in various stages of planning and development. Contributions and feedback are welcome!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
