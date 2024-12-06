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
    // Create a new logger instance with custom configuration
    let logger = Logger::new()
        .level(Level::Debug)
        .formatter(TextFormatter::new()
            .timestamp_format("%Y-%m-%d %H:%M:%S")
            .colors(true)
            .full_timestamp(true)
            .build())
        .build();

    // Use the custom logger
    logger.with_fields(Fields::new())
        .with_field("component", "auth")
        .info("Authentication successful");
}
```

### Error Handling

Loggix provides convenient error handling through the `with_error` function:

```rust
use std::fs::File;
use loggix::with_error;

fn main() {
    let result = File::open("non_existent.txt");
    if let Err(error) = result {
        with_error(&error).error("Failed to open file");
    }
}
```

### Custom Time Fields

You can attach custom timestamps to your log entries:

```rust
use loggix::with_time;
use chrono::Utc;

fn main() {
    let event_time = Utc::now();
    with_time(event_time).info("Event occurred at specific time");
}
```

### Multiple Fields at Once

For logging multiple fields efficiently:

```rust
use loggix::{Logger, Fields};

fn main() {
    let fields = vec![
        ("user", "john"),
        ("action", "login"),
        ("ip", "192.168.1.1"),
    ];

    Logger::new()
        .build()
        .with_fields(Fields::new())
        .with_fields_map(fields)
        .info("User login activity");
}
```

### Level String Parsing

Parse log levels from strings:

```rust
use loggix::Level;

fn main() {
    if let Some(level) = Level::from_str("INFO") {
        println!("Parsed level: {}", level);
    }
}
```

### JSON Formatting

For machine-readable logs:

```rust
use loggix::{Logger, JSONFormatter};

fn main() {
    let logger = Logger::new()
        .formatter(JSONFormatter::new().pretty(true).build())
        .build();

    logger.with_fields(Fields::new())
        .with_field("user", "john")
        .with_field("action", "login")
        .info("User logged in");
}
```

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

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by [Logrus](https://github.com/sirupsen/logrus)
- Built with ❤️ using Rust
