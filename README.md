# Loggix: A Modern Structured Logger for Rust

A fully-featured structured logger for Rust, inspired by [Logrus](https://github.com/sirupsen/logrus). Loggix combines the power of structured logging with Rust's safety and performance.

## Features

- Log levels: Trace, Debug, Info, Warn, Error, Fatal, Panic
- Structured logging with fields
- Hooks for actions on log entries
- Formatters:
  - Text formatter with colors (default)
  - JSON formatter
- Thread-safe
- Global and local logger instances
- Customizable output (io::Write)

## Usage

### Basic Logging

```rust
use loggix::{info, debug, warn, error, set_level, Level};

fn main() {
    // Set global log level
    set_level(Level::Debug);

    // Log messages
    info("Application started").unwrap();
    debug("Debug information").unwrap();
    warn("Warning message").unwrap();
    error("Error occurred").unwrap();
}
```

### Structured Logging with Fields

```rust
use loggix::{Logger, with_field, with_fields};
use std::collections::HashMap;

fn main() {
    // Using with_field
    with_field("key", "value")
        .with_field("number", 42)
        .info("Message with fields")
        .unwrap();

    // Using with_fields
    let mut fields = HashMap::new();
    fields.insert("key".to_string(), serde_json::json!("value"));
    fields.insert("number".to_string(), serde_json::json!(42));
    
    with_fields(fields)
        .info("Message with multiple fields")
        .unwrap();
}
```

### Custom Logger Instance

```rust
use loggix::{Logger, Level, JSONFormatter};

fn main() {
    let logger = Logger::new()
        .with_level(Level::Info)
        .with_formatter(JSONFormatter::default());

    logger.info("Using JSON formatter").unwrap();
}
```

### Custom Hooks

```rust
use loggix::{Logger, Hook, Entry};
use std::error::Error;

struct MetricsHook;

impl Hook for MetricsHook {
    fn fire(&self, entry: &Entry) -> Result<(), Box<dyn Error>> {
        // Send log entry to metrics system
        println!("Metrics: {}", entry.level);
        Ok(())
    }
}

fn main() {
    let mut logger = Logger::new();
    logger.add_hook(MetricsHook);
    logger.info("This will trigger the metrics hook").unwrap();
}
```

### Custom Output

```rust
use loggix::Logger;
use std::fs::File;

fn main() {
    let mut logger = Logger::new();
    let file = File::create("app.log").unwrap();
    logger.set_output(file);
    logger.info("This goes to app.log").unwrap();
}
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
loggix = "0.1.0"
```

## Why Loggix?

Loggix combines the best features of structured logging with Rust's safety and performance:

- **Structured Logging**: Log entries are more than just strings; they're structured data that can be easily parsed and analyzed.
- **Type Safety**: Leverages Rust's type system to prevent common logging mistakes.
- **Performance**: Built with performance in mind, using efficient data structures and minimal allocations.
- **Flexibility**: Easily extensible with custom formatters and hooks.
- **Thread Safety**: Safe to use in concurrent applications.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License
