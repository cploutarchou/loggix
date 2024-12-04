use std::{collections::HashMap, fmt, io::{self, Write}, sync::{Arc, Mutex}};
use chrono::{DateTime, Utc};
use colored::Colorize;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use lazy_static::lazy_static;

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
            }.to_string()
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
        output.write_all(b"\n")?;
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
    
    pub fn log(&self, level: Level, msg: &str, fields: Fields) -> Result<(), Box<dyn std::error::Error>> {
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
        self.output.lock().unwrap().write_all(&formatted)?;
        
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
    
    pub fn trace<M: Into<String>>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        self.logger.log(Level::Trace, &msg.into(), self.fields.clone())
    }
    
    pub fn debug<M: Into<String>>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        self.logger.log(Level::Debug, &msg.into(), self.fields.clone())
    }
    
    pub fn info<M: Into<String>>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        self.logger.log(Level::Info, &msg.into(), self.fields.clone())
    }
    
    pub fn warn<M: Into<String>>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        self.logger.log(Level::Warn, &msg.into(), self.fields.clone())
    }
    
    pub fn error<M: Into<String>>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        self.logger.log(Level::Error, &msg.into(), self.fields.clone())
    }
    
    pub fn fatal<M: Into<String>>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        self.logger.log(Level::Fatal, &msg.into(), self.fields.clone())
    }
    
    pub fn panic<M: Into<String>>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        self.logger.log(Level::Panic, &msg.into(), self.fields.clone())
    }
}

// Global logger
lazy_static! {
    static ref GLOBAL_LOGGER: Arc<Logger> = Logger::new().build();
}

// Global functions
pub fn set_level(level: Level) {
    let logger = GLOBAL_LOGGER.clone();
    if let Some(mut logger) = Arc::get_mut(&mut logger.clone()) {
        logger.level = level;
    }
}

pub fn with_fields<'a>(fields: Fields) -> EntryBuilder<'a> {
    GLOBAL_LOGGER.with_fields(fields)
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
    
    #[derive(Default)]
    pub struct TestWriter {
        pub buffer: Mutex<Vec<u8>>,
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
                buffer: Mutex::new(self.buffer.lock().unwrap().clone()),
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
            let logger = Logger::new()
                .output(Box::new(writer.clone()))
                .build();
            (logger, writer)
        }
    }
    
    #[test]
    fn test_basic_logging() {
        let (logger, writer) = TestLogger::new();
        logger.log(Level::Info, "test message", Fields::new()).unwrap();
        
        let output = String::from_utf8(writer.buffer.lock().unwrap().clone()).unwrap();
        assert!(output.contains("test message"));
        assert!(output.contains("INFO"));
    }
    
    #[test]
    fn test_json_formatter() {
        let writer = TestWriter::default();
        let logger = Logger::new()
            .formatter(JSONFormatter::default())
            .output(Box::new(writer.clone()))
            .build();
            
        logger.log(Level::Info, "test message", Fields::new()).unwrap();
        
        let output = String::from_utf8(writer.buffer.lock().unwrap().clone()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        
        assert_eq!(parsed["level"], "info");
        assert_eq!(parsed["message"], "test message");
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
        let hook = TestHook { called: called.clone() };
        
        let logger = Logger::new()
            .add_hook(hook)
            .build();
            
        logger.log(Level::Info, "test message", Fields::new()).unwrap();
        
        assert!(*called.lock().unwrap());
    }
}
