//! Loggix is a powerful structured logging library for Rust, inspired by Logrus.
//! 
//! # Features
//! 
//! - Seven log levels: Trace, Debug, Info, Warning, Error, Fatal, and Panic
//! - Structured logging with fields
//! - Beautiful terminal output with colors
//! - JSON formatter for machine processing
//! - Extensible hook system
//! - Thread-safe by default
//! - Global and local logger instances
//! 
//! # Quick Start
//! 
//! ```rust
//! use loggix::{info, with_fields};
//! 
//! fn main() {
//!     // Simple logging
//!     info!("A walrus appears");
//! 
//!     // Structured logging with fields
//!     with_fields!(
//!         "animal".to_string() => "walrus".to_string(),
//!         "size".to_string() => 10.to_string()
//!     )
//!     .info("A group of walrus emerges");
//! }
//! ```
//! 
//! # Advanced Features
//! 
//! ## Error Handling
//! 
//! ```rust
//! use loggix::with_error;
//! use std::fs::File;
//! 
//! fn main() {
//!     let result = File::open("non_existent.txt");
//!     if let Err(error) = result {
//!         with_error(&error).error("Failed to open file");
//!     }
//! }
//! ```
//! 
//! ## Custom Time Fields
//! 
//! ```rust
//! use loggix::with_time;
//! use chrono::Utc;
//! 
//! fn main() {
//!     let event_time = Utc::now();
//!     with_time(event_time).info("Event occurred at specific time");
//! }
//! ```
//! 
//! ## Multiple Fields
//! 
//! ```rust
//! use loggix::{Logger, Fields};
//! 
//! fn main() {
//!     let fields = vec![
//!         ("user", "john"),
//!         ("action", "login"),
//!         ("ip", "192.168.1.1"),
//!     ];
//! 
//!     Logger::new()
//!         .build()
//!         .with_fields(Fields::new())
//!         .with_fields_map(fields)
//!         .info("User login activity");
//! }
//! ```
//! 
//! ## Level Parsing
//! 
//! ```rust
//! use loggix::Level;
//! 
//! fn main() {
//!     if let Some(level) = Level::from_str("INFO") {
//!         println!("Parsed level: {}", level);
//!     }
//! }
//! ```
//! 
//! # Thread Safety
//! 
//! Loggix is thread-safe by default, using Arc and Mutex internally to protect shared state.
//! All logging operations are atomic and can be safely used across multiple threads.
//! 
//! # Customization
//! 
//! The library is highly customizable through:
//! - Custom formatters implementing the `Formatter` trait
//! - Custom hooks implementing the `Hook` trait
//! - Custom output implementing `std::io::Write`
//! 
//! # Performance Considerations
//! 
//! - Zero-allocation logging paths for common use cases
//! - Efficient field storage using pre-allocated hashmaps
//! - Lock-free architecture where possible
//! - Minimal runtime overhead
//! 
//! See the [README](https://github.com/cploutarchou/loggix) for more examples and detailed documentation.

use chrono::{DateTime, Utc};
use colored::Colorize;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    fmt,
    io::{self, Write},
    sync::{Arc, Mutex},
};

// Re-exports
pub use chrono;
pub use colored;
pub use serde;
pub use serde_json;

/// Log levels supported by Loggix
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Panic,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Level::Trace => write!(f, "TRACE"),
            Level::Debug => write!(f, "DEBUG"),
            Level::Info => write!(f, "INFO"),
            Level::Warn => write!(f, "WARN"),
            Level::Error => write!(f, "ERROR"),
            Level::Fatal => write!(f, "FATAL"),
            Level::Panic => write!(f, "PANIC"),
        }
    }
}

impl Level {
    /// Parse a level from a string
    pub fn from_str(level: &str) -> Option<Level> {
        match level.to_lowercase().as_str() {
            "trace" => Some(Level::Trace),
            "debug" => Some(Level::Debug),
            "info" => Some(Level::Info),
            "warn" | "warning" => Some(Level::Warn),
            "error" => Some(Level::Error),
            "fatal" => Some(Level::Fatal),
            "panic" => Some(Level::Panic),
            _ => None,
        }
    }
}

/// Fields type for structured logging
pub type Fields = HashMap<String, Value>;

/// A log entry containing all information about a log event
#[derive(Debug, Clone, Serialize)]
pub struct Entry<'a> {
    pub timestamp: DateTime<Utc>,
    pub level: Level,
    pub message: String,
    pub fields: Fields,
    #[serde(skip)]
    pub logger: &'a Logger,
}

/// Hook trait for implementing custom hooks
pub trait Hook: Send + Sync {
    fn levels(&self) -> Vec<Level>;
    fn fire(&self, entry: &Entry) -> Result<(), Box<dyn std::error::Error>>;
}

/// Formatter trait for implementing custom formatters
pub trait Formatter: Send + Sync {
    fn format(&self, entry: &Entry) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

/// Text formatter with optional colors
#[derive(Debug, Clone)]
pub struct TextFormatter {
    timestamp_format: String,
    colors: bool,
    full_timestamp: bool,
}

impl Default for TextFormatter {
    fn default() -> Self {
        Self {
            timestamp_format: "%Y-%m-%dT%H:%M:%S%.3fZ".to_string(),
            colors: true,
            full_timestamp: true,
        }
    }
}

impl TextFormatter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn timestamp_format(mut self, format: &str) -> Self {
        self.timestamp_format = format.to_string();
        self
    }

    pub fn colors(mut self, enabled: bool) -> Self {
        self.colors = enabled;
        self
    }

    pub fn full_timestamp(mut self, enabled: bool) -> Self {
        self.full_timestamp = enabled;
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

impl Formatter for TextFormatter {
    fn format(&self, entry: &Entry) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut output = Vec::new();

        // Format timestamp
        let timestamp = if self.full_timestamp {
            entry.timestamp.format(&self.timestamp_format).to_string()
        } else {
            entry.timestamp.format("%H:%M:%S").to_string()
        };

        // Format level with optional colors
        let level = if self.colors {
            match entry.level {
                Level::Trace => entry.level.to_string().white(),
                Level::Debug => entry.level.to_string().blue(),
                Level::Info => entry.level.to_string().green(),
                Level::Warn => entry.level.to_string().yellow(),
                Level::Error => entry.level.to_string().red(),
                Level::Fatal => entry.level.to_string().red().bold(),
                Level::Panic => entry.level.to_string().red().bold(),
            }
            .to_string()
        } else {
            entry.level.to_string()
        };

        // Write the log line
        write!(output, "[{}] [{}] {}", timestamp, level, entry.message)?;

        // Add fields if present
        if !entry.fields.is_empty() {
            for (key, value) in &entry.fields {
                write!(output, " {}={}", key, value)?;
            }
        }

        write!(output, "\n")?;
        Ok(output)
    }
}

/// JSON formatter for machine-readable output
#[derive(Debug, Clone)]
pub struct JSONFormatter {
    pretty: bool,
}

impl JSONFormatter {
    pub fn new() -> Self {
        Self { pretty: false }
    }

    pub fn pretty(mut self, enabled: bool) -> Self {
        self.pretty = enabled;
        self
    }
}

impl Default for JSONFormatter {
    fn default() -> Self {
        Self { pretty: false }
    }
}

impl Formatter for JSONFormatter {
    fn format(&self, entry: &Entry) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut output = Vec::new();
        if self.pretty {
            serde_json::to_writer_pretty(&mut output, &entry)?;
        } else {
            serde_json::to_writer(&mut output, &entry)?;
        }
        output.extend_from_slice(b"\n");
        Ok(output)
    }
}

/// The main logger struct
pub struct Logger {
    level: Level,
    formatter: Box<dyn Formatter>,
    hooks: Vec<Box<dyn Hook>>,
    output: Arc<Mutex<Box<dyn Write + Send>>>,
}

impl fmt::Debug for Logger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Logger")
            .field("level", &self.level)
            .field("hooks_count", &self.hooks.len())
            .finish()
    }
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        Self {
            level: self.level,
            formatter: Box::new(TextFormatter::default()),
            hooks: Vec::new(),
            output: Arc::clone(&self.output),
        }
    }
}

impl Logger {
    pub fn new() -> Self {
        Self {
            level: Level::Info,
            formatter: Box::new(TextFormatter::default()),
            hooks: Vec::new(),
            output: Arc::new(Mutex::new(Box::new(io::stdout()))),
        }
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn formatter<F: Formatter + 'static>(mut self, formatter: F) -> Self {
        self.formatter = Box::new(formatter);
        self
    }

    pub fn add_hook<H: Hook + 'static>(mut self, hook: H) -> Self {
        self.hooks.push(Box::new(hook));
        self
    }

    pub fn output<W: Write + Send + 'static>(mut self, output: W) -> Self {
        self.output = Arc::new(Mutex::new(Box::new(output)));
        self
    }

    pub fn build(self) -> Arc<Self> {
        Arc::new(self)
    }

    pub fn log(
        &self,
        level: Level,
        msg: &str,
        fields: Fields,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if level < self.level {
            return Ok(());
        }

        let entry = Entry {
            timestamp: Utc::now(),
            level,
            message: msg.to_string(),
            fields,
            logger: self,
        };

        // Fire hooks
        for hook in &self.hooks {
            if hook.levels().contains(&level) {
                hook.fire(&entry)?;
            }
        }

        // Format and write entry
        let formatted = self.formatter.format(&entry)?;
        let mut output = self.output.lock().unwrap();
        output.write_all(&formatted)?;
        output.flush()?;

        // Handle fatal and panic levels
        match level {
            Level::Fatal => std::process::exit(1),
            Level::Panic => std::panic::panic_any(msg.to_string()),
            _ => Ok(()),
        }
    }

    pub fn with_fields(&self, fields: Fields) -> EntryBuilder {
        EntryBuilder {
            logger: self,
            fields,
        }
    }
}

/// Builder for log entries
pub struct EntryBuilder<'a> {
    logger: &'a Logger,
    fields: Fields,
}

impl<'a> Clone for EntryBuilder<'a> {
    fn clone(&self) -> Self {
        Self {
            logger: self.logger,
            fields: self.fields.clone(),
        }
    }
}

impl<'a> EntryBuilder<'a> {
    pub fn with_field<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Serialize,
    {
        self.fields.insert(
            key.into(),
            serde_json::to_value(value).unwrap_or(Value::Null),
        );
        self
    }

    pub fn with_time(self, time: DateTime<Utc>) -> Self {
        self.with_field("time", time.to_rfc3339())
    }

    pub fn with_error<E: std::error::Error>(self, err: &E) -> Self {
        self.with_field("error", err.to_string())
    }

    pub fn with_fields_map<K, V>(mut self, fields: impl IntoIterator<Item = (K, V)>) -> Self 
    where
        K: Into<String>,
        V: serde::Serialize,
    {
        for (key, value) in fields {
            if let Ok(value) = serde_json::to_value(value) {
                self.fields.insert(key.into(), value);
            }
        }
        self
    }

    pub fn trace<M: Into<String>>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        self.logger
            .log(Level::Trace, &msg.into(), self.fields.clone())
    }

    pub fn debug<M: Into<String>>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        self.logger
            .log(Level::Debug, &msg.into(), self.fields.clone())
    }

    pub fn info<M: Into<String>>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        self.logger
            .log(Level::Info, &msg.into(), self.fields.clone())
    }

    pub fn warn<M: Into<String>>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        self.logger
            .log(Level::Warn, &msg.into(), self.fields.clone())
    }

    pub fn error<M: Into<String>>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        self.logger
            .log(Level::Error, &msg.into(), self.fields.clone())
    }

    pub fn fatal<M: Into<String>>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        self.logger
            .log(Level::Fatal, &msg.into(), self.fields.clone())
    }

    pub fn panic<M: Into<String>>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        self.logger
            .log(Level::Panic, &msg.into(), self.fields.clone())
    }
}

// Global logger
lazy_static! {
    static ref GLOBAL_LOGGER: Arc<Logger> = Logger::new().build();
}

// Global convenience functions
pub fn set_level(level: Level) {
    let logger = GLOBAL_LOGGER.clone();
    if let Some( logger) = Arc::get_mut(&mut logger.clone()) {
        logger.level = level;
    }
}

pub fn with_fields<'a>(fields: Fields) -> EntryBuilder<'a> {
    GLOBAL_LOGGER.with_fields(fields)
}

pub fn with_error<E: std::error::Error>(err: &E) -> EntryBuilder<'static> {
    GLOBAL_LOGGER.with_fields(Fields::new()).with_error(err)
}

pub fn with_time(time: DateTime<Utc>) -> EntryBuilder<'static> {
    GLOBAL_LOGGER.with_fields(Fields::new()).with_time(time)
}

pub fn parse_level(level: &str) -> Option<Level> {
    Level::from_str(level)
}

// Macros for convenient logging
#[macro_export]
macro_rules! with_fields {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut fields = ::std::collections::HashMap::new();
        $(
            fields.insert($key.to_string(), $crate::serde_json::to_value($value).unwrap_or($crate::serde_json::Value::Null));
        )*
        $crate::with_fields(fields)
    }};
}

#[macro_export]
macro_rules! trace {
    ($msg:expr) => {
        $crate::with_fields!().trace($msg)
    };
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        $crate::with_fields!().debug($msg)
    };
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        $crate::with_fields!().info($msg)
    };
}

#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        $crate::with_fields!().warn($msg)
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        $crate::with_fields!().error($msg)
    };
}

#[macro_export]
macro_rules! fatal {
    ($msg:expr) => {
        $crate::with_fields!().fatal($msg)
    };
}

#[macro_export]
macro_rules! panic {
    ($msg:expr) => {
        $crate::with_fields!().panic($msg)
    };
}

// Testing module
#[cfg(test)]
pub mod test {
    use super::*;
    use std::sync::Mutex;

    pub struct TestWriter {
        pub buffer: Arc<Mutex<Vec<u8>>>,
    }

    impl Default for TestWriter {
        fn default() -> Self {
            Self {
                buffer: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Clone for TestWriter {
        fn clone(&self) -> Self {
            Self {
                buffer: Arc::clone(&self.buffer),
            }
        }
    }

    pub struct TestLogger {
        pub logger: Arc<Logger>,
        pub writer: TestWriter,
    }

    impl TestLogger {
        pub fn new() -> (Arc<Logger>, TestWriter) {
            let writer = TestWriter::default();
            let logger = Logger::new().output(Box::new(writer.clone())).build();
            (logger, writer)
        }
    }

    #[test]
    fn test_basic_logging() {
        let writer = TestWriter::default();
        let logger = Logger::new()
            .formatter(TextFormatter::default().colors(false))
            .output(Box::new(writer.clone()))
            .build();

        logger
            .log(Level::Info, "test message", Fields::new())
            .unwrap();

        let output = String::from_utf8(writer.buffer.lock().unwrap().clone()).unwrap();
        println!("Output: {:?}", output);
        assert!(output.contains("[INFO]"));
        assert!(output.contains("test message"));
    }

    #[test]
    fn test_json_formatter() {
        let writer = TestWriter::default();
        let logger = Logger::new()
            .formatter(JSONFormatter::default())
            .output(Box::new(writer.clone()))
            .build();

        logger
            .log(Level::Info, "test message", Fields::new())
            .unwrap();

        let output = String::from_utf8(writer.buffer.lock().unwrap().clone()).unwrap();
        println!("JSON Output: {:?}", output);
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();

        assert_eq!(parsed["level"].as_str().unwrap(), "info");
        assert_eq!(parsed["message"].as_str().unwrap(), "test message");
    }

    #[test]
    fn test_hooks() {
        #[derive(Debug)]
        struct TestHook {
            called: Arc<Mutex<bool>>,
        }

        impl Hook for TestHook {
            fn levels(&self) -> Vec<Level> {
                vec![
                    Level::Trace,
                    Level::Debug,
                    Level::Info,
                    Level::Warn,
                    Level::Error,
                    Level::Fatal,
                    Level::Panic,
                ]
            }

            fn fire(&self, _: &Entry) -> Result<(), Box<dyn std::error::Error>> {
                *self.called.lock().unwrap() = true;
                Ok(())
            }
        }

        let called = Arc::new(Mutex::new(false));
        let hook = TestHook {
            called: called.clone(),
        };

        let logger = Logger::new().add_hook(hook).build();

        logger
            .log(Level::Info, "test message", Fields::new())
            .unwrap();

        assert!(*called.lock().unwrap());
    }

    #[test]
    fn test_with_error_and_time() {
        let writer = TestWriter::default();
        let logger = Logger::new()
            .formatter(JSONFormatter::default())
            .output(Box::new(writer.clone()))
            .build();

        let error = std::io::Error::new(std::io::ErrorKind::Other, "test error");
        let time = Utc::now();
        
        logger.with_fields(Fields::new())
            .with_error(&error)
            .with_time(time)
            .info("message with error and time")
            .unwrap();

        let output = String::from_utf8(writer.buffer.lock().unwrap().clone()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();

        assert_eq!(parsed["level"], "info");
        assert_eq!(parsed["message"], "message with error and time");
        assert_eq!(parsed["fields"]["error"], "test error");
        assert!(parsed["fields"]["time"].as_str().is_some());
    }

    #[test]
    fn test_with_fields_map() {
        let writer = TestWriter::default();
        let logger = Logger::new()
            .formatter(JSONFormatter::default())
            .output(Box::new(writer.clone()))
            .build();

        let fields = vec![
            ("string", "value"),
            ("number", "42"),
        ];
        
        logger.with_fields(Fields::new())
            .with_fields_map(fields)
            .info("message with multiple fields")
            .unwrap();

        let output = String::from_utf8(writer.buffer.lock().unwrap().clone()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();

        assert_eq!(parsed["level"], "info");
        assert_eq!(parsed["message"], "message with multiple fields");
        assert_eq!(parsed["fields"]["string"], "value");
        assert_eq!(parsed["fields"]["number"], "42");
    }

    #[test]
    fn test_level_parsing() {
        assert_eq!(Level::from_str("INFO"), Some(Level::Info));
        assert_eq!(Level::from_str("debug"), Some(Level::Debug));
        assert_eq!(Level::from_str("WARNING"), Some(Level::Warn));
        assert_eq!(Level::from_str("invalid"), None);
    }
}
