# Loggix

A high-performance, async-first logging framework for Rust with Kafka integration.

## Features

- Flexible logging levels (TRACE, DEBUG, INFO, WARN, ERROR, FATAL, PANIC)
- Structured logging with key-value pairs
- Customizable formatters (text and JSON included)
- Asynchronous logging support
  - `log_async` method for async contexts
  - Non-blocking operations
  - Tokio runtime integration
- Hook system for log processing and forwarding
  - Sync and async hook support
  - Multiple hooks per logger
  - Level-based filtering
- Kafka integration with support for:
  - Asynchronous message delivery
  - Custom message keys via fields
  - Topic configuration
  - Error handling and retries
  - Automatic topic creation
  - Message key routing
- Thread-safe logging
- Zero-allocation logging paths
- Configurable output destinations
- Colorized console output (optional)
- Comprehensive benchmarks
- Full test coverage

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
loggix = "1.0"
```

## Quick Start

### Basic Logging

```rust
use loggix::{Logger, Level, Fields};

// Create a logger
let logger = Logger::new().build();

// Log a message
let mut fields = Fields::new();
fields.insert("user_id".to_string(), "123".into());
logger.log(Level::Info, "User logged in", fields).unwrap();

// Async logging
let mut fields = Fields::new();
fields.insert("order_id".to_string(), "456".into());
logger.log_async(Level::Info, "Order processed", fields).await.unwrap();
```

### Structured Logging with JSON

```rust
use loggix::{Logger, JSONFormatter, Level, Fields};
use serde_json::Value;

let logger = Logger::new()
    .formatter(JSONFormatter::new())
    .build();

let mut fields = Fields::new();
fields.insert("transaction_id".to_string(), Value::String("tx-123".to_string()));
fields.insert("amount".to_string(), Value::Number(100.into()));
logger.log(Level::Info, "Payment processed", fields).unwrap();
```

## Kafka Integration

### Setting up Kafka

1. Start the Kafka environment:
```bash
docker-compose up -d
```

2. Create a logger with Kafka hook:
```rust
use loggix::{Logger, KafkaHook, Level, Fields};
use serde_json::Value;

// Create a Kafka hook with message key support
let kafka_hook = KafkaHook::new("localhost:9092", "logs")
    .unwrap()
    .with_key_field("correlation_id".to_string());

// Create a logger with the Kafka hook
let logger = Logger::new()
    .add_hook(kafka_hook)
    .build();

// Log a message with a correlation ID for message routing
let mut fields = Fields::new();
fields.insert("correlation_id".to_string(), Value::String("abc-123".to_string()));
fields.insert("user_id".to_string(), Value::String("456".to_string()));
logger.log_async(Level::Info, "User action", fields).await.unwrap();
```

### Message Key Support

The Kafka hook supports setting a field as the message key:

```rust
// Set up hook with a key field
let kafka_hook = KafkaHook::new("localhost:9092", "logs")
    .unwrap()
    .with_key_field("tenant_id".to_string());

// Any log message with the tenant_id field will use it as the Kafka message key
let mut fields = Fields::new();
fields.insert("tenant_id".to_string(), Value::String("tenant-1".to_string()));
logger.log_async(Level::Info, "Tenant action", fields).await.unwrap();
```

This enables:
- Message routing based on keys
- Message partitioning
- Message deduplication
- Message ordering within partitions

### Async Support

Both the logger and hooks support async operations:

```rust
// Async logging with hooks
logger.log_async(Level::Info, "Async message", fields).await?;

// Hooks automatically use async operations when available
impl Hook for MyHook {
    fn fire_async<'a>(&'a self, entry: &'a Entry) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        // Async implementation
    }
}
```

## Examples

See the `examples/` directory for more examples:
- Basic logging
- Custom formatters
- Kafka integration
- Async logging
- Structured logging
- Error handling

## Benchmarks

Run the benchmarks:
```bash
cargo bench
```

Example benchmark results:
- Sync logging: ~1800ns/iter
- Async logging: ~2000ns/iter
- Kafka logging: ~10000ns/iter (network latency not included)

## Configuration

### YAML Configuration

Create a `config.yaml` file:
```yaml
kafka:
  bootstrap_servers: "localhost:9092"
  group_id: "logger_group"
  auto_offset_reset: "earliest"
  socket_timeout_ms: 3000
  session_timeout_ms: 6000
  replication_factor: 1
  partitions: 1
```

## Performance Tips

1. Use `log_async` in async contexts
2. Reuse field collections when possible
3. Consider message key strategy for Kafka partitioning
4. Use appropriate log levels to minimize processing
5. Configure appropriate batch sizes for Kafka

## Roadmap

- [ ] ElasticSearch integration
- [ ] Log rotation
- [ ] Log compression
- [ ] Sampling and filtering
- [ ] OpenTelemetry integration
- [ ] Prometheus metrics
- [ ] Log aggregation
- [ ] Log analytics

## Changelog

For a detailed list of changes between versions, please see our [CHANGELOG](CHANGELOG.md).

## License

This project is licensed under the MIT License - see the LICENSE file for details.
