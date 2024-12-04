# Loggix 🦀

A powerful structured logger for Rust, inspired by [Logrus](https://github.com/sirupsen/logrus). Loggix combines structured logging with Rust's safety and performance guarantees.

![Example Output](https://raw.githubusercontent.com/cploutarchou/loggix/main/examples/output.png)

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
loggix = "0.1.0"
```

Basic usage:
```rust
use loggix::{info, debug, warn, error, with_fields};

fn main() {
    // Simple logging
    info!("A walrus appears");
    
    // Structured logging with fields
    with_fields!({
        "animal" => "walrus",
        "size" => 10
    })
    .info("A group of walrus emerges from the ocean");
    
    // Warning with more fields
    with_fields!({
        "omg" => true,
        "number" => 122
    })
    .warn("The group's number increased tremendously!");
    
    // Fatal will exit the program
    with_fields!({
        "omg" => true,
        "number" => 100
    })
    .fatal("The ice breaks!");
}
```

Output with default text formatter (with colors when TTY is attached):
```
[2023-08-10T15:04:05Z INFO] A walrus appears
[2023-08-10T15:04:05Z INFO] A group of walrus emerges from the ocean animal=walrus size=10
[2023-08-10T15:04:05Z WARN] The group's number increased tremendously! omg=true number=122
[2023-08-10T15:04:05Z FATAL] The ice breaks! omg=true number=100
```

With JSON formatter:
```rust
use loggix::{Logger, JSONFormatter};

fn main() {
    let logger = Logger::new()
        .formatter(JSONFormatter::default())
        .build();
        
    logger.with_fields({
        "animal" => "walrus",
        "size" => 10
    })
    .info("A group of walrus emerges from the ocean");
}
```

Output:
```json
{"timestamp":"2023-08-10T15:04:05Z","level":"info","msg":"A group of walrus emerges from the ocean","animal":"walrus","size":10}
```

## Advanced Usage

### Custom Logger Instance

```rust
use loggix::{Logger, Level, TextFormatter};
use std::fs::File;

fn main() {
    // Create a new logger instance
    let logger = Logger::new()
        .level(Level::Debug)
        .formatter(TextFormatter::new()
            .timestamp_format("%Y-%m-%d %H:%M:%S")
            .colors(false)
            .build())
        .output(File::create("app.log").unwrap())
        .build();
    
    // Use the logger
    logger.debug("Debug message");
    logger.info("Info message");
    
    // Create a context logger
    let context_logger = logger.with_fields({
        "request_id" => "123",
        "user_id" => "456"
    });
    
    context_logger.info("Request processed");
    context_logger.warn("Slow response detected");
}
```

### Hooks

```rust
use loggix::{Logger, Hook, Entry, Level};

struct MetricsHook;

impl Hook for MetricsHook {
    fn fire(&self, entry: &Entry) -> Result<(), Box<dyn std::error::Error>> {
        // Send metrics to your metrics system
        println!("Metrics: {} - {}", entry.level, entry.message);
        Ok(())
    }
    
    fn levels(&self) -> Vec<Level> {
        vec![Level::Error, Level::Fatal, Level::Panic]
    }
}

fn main() {
    let logger = Logger::new()
        .add_hook(MetricsHook)
        .build();
        
    logger.error("This will trigger the metrics hook");
}
```

## Thread Safety

Loggix is thread-safe by default, protected by a mutex for concurrent writes. The mutex is held when calling hooks and writing logs.

## Testing

Loggix provides testing utilities to assert log messages:

```rust
use loggix::test::{TestLogger, capture_logs};

#[test]
fn test_logging() {
    let (logger, logs) = TestLogger::new();
    
    logger.info("Test message");
    
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].level, Level::Info);
    assert_eq!(logs[0].message, "Test message");
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License
