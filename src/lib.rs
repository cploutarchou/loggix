use std::fmt;
use std::sync::{Arc, Mutex};
use std::ops::Deref;
use chrono::Local;
use colored::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

/// Log levels similar to Logrus
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Panic,
}

/// A trait for implementing hooks
pub trait Hook: Send + Sync {
    fn fire(&self, entry: &Entry) -> Result<(), Box<dyn Error>>;
}

/// Fields type alias for structured logging
pub type Fields = HashMap<String, Value>;

/// Logger struct with hooks and formatter support
pub struct Logger {
    level: Level,
    formatter: Arc<dyn Formatter>,
    hooks: Vec<Arc<dyn Hook>>,
    out: Arc<Mutex<Box<dyn std::io::Write + Send>>>,
}

impl fmt::Debug for Logger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Logger")
            .field("level", &self.level)
            .field("hooks_count", &self.hooks.len())
            .finish()
    }
}

/// Logging entry struct with fields support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub level: Level,
    pub message: String,
    pub time: chrono::DateTime<Local>,
    pub fields: Fields,
    #[serde(skip)]
    logger: Option<Arc<Logger>>,
}

impl Entry {
    pub fn new(logger: Arc<Logger>) -> Self {
        Entry {
            level: Level::Info,
            message: String::new(),
            time: Local::now(),
            fields: Fields::new(),
            logger: Some(logger),
        }
    }

    pub fn with_field<T: Serialize>(mut self, key: &str, value: T) -> Self {
        if let Ok(value) = serde_json::to_value(value) {
            self.fields.insert(key.to_string(), value);
        }
        self
    }

    pub fn with_fields(mut self, fields: Fields) -> Self {
        self.fields.extend(fields);
        self
    }

    pub fn log(&self) -> Result<(), Box<dyn Error>> {
        if let Some(logger) = &self.logger {
            logger.process_entry(self)?;
        }
        Ok(())
    }
}

/// Trait for log formatting
pub trait Formatter: Send + Sync {
    fn format(&self, entry: &Entry) -> Result<String, Box<dyn Error>>;
}

/// Text formatter with customizable timestamp format
#[derive(Clone)]
pub struct TextFormatter {
    timestamp_format: String,
    full_timestamp: bool,
}

impl fmt::Debug for TextFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextFormatter")
            .field("timestamp_format", &self.timestamp_format)
            .field("full_timestamp", &self.full_timestamp)
            .finish()
    }
}

impl Default for TextFormatter {
    fn default() -> Self {
        TextFormatter {
            timestamp_format: "%Y-%m-%d %H:%M:%S".to_string(),
            full_timestamp: true,
        }
    }
}

impl Formatter for TextFormatter {
    fn format(&self, entry: &Entry) -> Result<String, Box<dyn Error>> {
        let level_str = entry.level.to_string();
        let level_color = match entry.level {
            Level::Trace => "bright black",
            Level::Debug => "blue",
            Level::Info => "green",
            Level::Warn => "yellow",
            Level::Error => "red",
            Level::Fatal => "magenta",
            Level::Panic => "red bold",
        };

        let time_str = if self.full_timestamp {
            entry.time.format(&self.timestamp_format).to_string()
        } else {
            entry.time.format("%H:%M:%S").to_string()
        };

        let fields_str = if !entry.fields.is_empty() {
            let fields: Vec<String> = entry.fields
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            format!(" {}", fields.join(" "))
        } else {
            String::new()
        };

        Ok(format!(
            "{} [{}] {}{}\n",
            time_str.bright_black(),
            level_str.color(level_color),
            entry.message,
            fields_str
        ))
    }
}

/// JSON formatter
#[derive(Clone)]
pub struct JSONFormatter {
    timestamp_format: String,
}

impl fmt::Debug for JSONFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JSONFormatter")
            .field("timestamp_format", &self.timestamp_format)
            .finish()
    }
}

impl Default for JSONFormatter {
    fn default() -> Self {
        JSONFormatter {
            timestamp_format: "%Y-%m-%dT%H:%M:%S%:z".to_string(),
        }
    }
}

impl Formatter for JSONFormatter {
    fn format(&self, entry: &Entry) -> Result<String, Box<dyn Error>> {
        let mut output = serde_json::Map::new();
        output.insert("level".to_string(), Value::String(entry.level.to_string()));
        output.insert("msg".to_string(), Value::String(entry.message.clone()));
        output.insert("time".to_string(), Value::String(entry.time.format(&self.timestamp_format).to_string()));
        
        for (key, value) in &entry.fields {
            output.insert(key.clone(), value.clone());
        }

        Ok(serde_json::to_string(&output)? + "\n")
    }
}

// Implement Send and Sync for Logger
unsafe impl Send for Logger {}
unsafe impl Sync for Logger {}

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

impl Logger {
    pub fn new() -> Self {
        Logger {
            level: Level::Info,
            formatter: Arc::new(TextFormatter::default()),
            hooks: Vec::new(),
            out: Arc::new(Mutex::new(Box::new(std::io::stdout()))),
        }
    }

    pub fn with_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn with_formatter<F: Formatter + 'static>(mut self, formatter: F) -> Self {
        self.formatter = Arc::new(formatter);
        self
    }

    pub fn add_hook<H: Hook + 'static>(&mut self, hook: H) {
        self.hooks.push(Arc::new(hook));
    }

    pub fn set_output<W: std::io::Write + Send + 'static>(&mut self, output: W) {
        self.out = Arc::new(Mutex::new(Box::new(output)));
    }

    fn process_entry(&self, entry: &Entry) -> Result<(), Box<dyn Error>> {
        if entry.level < self.level {
            return Ok(());
        }

        // Fire all hooks
        for hook in &self.hooks {
            hook.fire(entry)?;
        }

        // Format and write entry
        let formatted = self.formatter.format(entry)?;
        self.out.lock().unwrap().write_all(formatted.as_bytes())?;
        self.out.lock().unwrap().flush()?;

        // Handle panic and fatal levels
        match entry.level {
            Level::Panic => panic!("{}", entry.message),
            Level::Fatal => std::process::exit(1),
            _ => Ok(()),
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::new(Arc::new(self.clone()))
    }

    pub fn log(&self, level: Level, msg: &str) -> Result<(), Box<dyn Error>> {
        self.entry()
            .with_field("level", level)
            .with_field("msg", msg)
            .log()
    }

    pub fn trace(&self, msg: &str) -> Result<(), Box<dyn Error>> {
        self.log(Level::Trace, msg)
    }

    pub fn debug(&self, msg: &str) -> Result<(), Box<dyn Error>> {
        self.log(Level::Debug, msg)
    }

    pub fn info(&self, msg: &str) -> Result<(), Box<dyn Error>> {
        self.log(Level::Info, msg)
    }

    pub fn warn(&self, msg: &str) -> Result<(), Box<dyn Error>> {
        self.log(Level::Warn, msg)
    }

    pub fn error(&self, msg: &str) -> Result<(), Box<dyn Error>> {
        self.log(Level::Error, msg)
    }

    pub fn fatal(&self, msg: &str) -> ! {
        self.log(Level::Fatal, msg).unwrap();
        std::process::exit(1)
    }

    pub fn panic(&self, msg: &str) -> ! {
        self.log(Level::Panic, msg).unwrap();
        panic!("{}", msg)
    }
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        Logger {
            level: self.level,
            formatter: Arc::clone(&self.formatter),
            hooks: self.hooks.clone(),
            out: Arc::clone(&self.out),
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger::new()
    }
}

// Global logger setup
lazy_static::lazy_static! {
    static ref GLOBAL_LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}

// Global functions
pub fn set_level(level: Level) {
    GLOBAL_LOGGER.lock().unwrap().level = level;
}

pub fn add_hook<H: Hook + 'static>(hook: H) {
    GLOBAL_LOGGER.lock().unwrap().add_hook(hook);
}

pub fn set_formatter<F: Formatter + 'static>(formatter: F) {
    GLOBAL_LOGGER.lock().unwrap().formatter = Arc::new(formatter);
}

pub fn with_field<T: Serialize>(key: &str, value: T) -> Entry {
    let logger = GLOBAL_LOGGER.lock().unwrap();
    logger.entry().with_field(key, value)
}

pub fn with_fields(fields: Fields) -> Entry {
    let logger = GLOBAL_LOGGER.lock().unwrap();
    logger.entry().with_fields(fields)
}

pub fn trace(msg: &str) -> Result<(), Box<dyn Error>> {
    GLOBAL_LOGGER.lock().unwrap().trace(msg)
}

pub fn debug(msg: &str) -> Result<(), Box<dyn Error>> {
    GLOBAL_LOGGER.lock().unwrap().debug(msg)
}

pub fn info(msg: &str) -> Result<(), Box<dyn Error>> {
    GLOBAL_LOGGER.lock().unwrap().info(msg)
}

pub fn warn(msg: &str) -> Result<(), Box<dyn Error>> {
    GLOBAL_LOGGER.lock().unwrap().warn(msg)
}

pub fn error(msg: &str) -> Result<(), Box<dyn Error>> {
    GLOBAL_LOGGER.lock().unwrap().error(msg)
}

pub fn fatal(msg: &str) -> ! {
    GLOBAL_LOGGER.lock().unwrap().fatal(msg)
}

pub fn panic(msg: &str) -> ! {
    GLOBAL_LOGGER.lock().unwrap().panic(msg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_log_levels() {
        let logger = Logger::new().with_level(Level::Debug);
        logger.debug("Debug message").unwrap();
        logger.info("Info message").unwrap();
    }

    #[test]
    fn test_fields() {
        let logger = Logger::new();
        logger.entry()
            .with_field("key1", "value1")
            .with_field("key2", 42)
            .with_field("key3", true)
            .log()
            .unwrap();
    }

    #[test]
    fn test_json_formatter() {
        let mut logger = Logger::new();
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = Arc::clone(&buffer);
        
        struct VecWriter(Arc<Mutex<Vec<u8>>>);
        impl Write for VecWriter {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                self.0.lock().unwrap().extend_from_slice(buf);
                Ok(buf.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        
        logger.set_output(VecWriter(buffer_clone));
        logger = logger.with_formatter(JSONFormatter::default());
        
        logger.info("test message").unwrap();
        
        let content = String::from_utf8(buffer.lock().unwrap().clone()).unwrap();
        assert!(content.contains("test message"));
    }

    #[test]
    fn test_hooks() {
        struct TestHook;
        impl Hook for TestHook {
            fn fire(&self, entry: &Entry) -> Result<(), Box<dyn Error>> {
                println!("Hook fired for level: {}", entry.level);
                Ok(())
            }
        }

        let mut logger = Logger::new();
        logger.add_hook(TestHook);
        logger.info("Test hook message").unwrap();
    }
}
