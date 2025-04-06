use loggix::{Entry, Formatter, Level, Logger};
use serde_json::Value;
use std::error::Error;
use std::io::Write;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
struct TestWriter {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl TestWriter {
    fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get_output(&self) -> Vec<u8> {
        self.buffer.lock().unwrap().clone()
    }
}

impl Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct TestFormatter {
    prefix: String,
}

impl TestFormatter {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }
}

impl Formatter for TestFormatter {
    fn format(&self, entry: &Entry) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut output = Vec::new();
        write!(
            output,
            "{} [{}] {}: {}\n",
            self.prefix,
            entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
            entry.level,
            entry.message
        )?;
        Ok(output)
    }
}

#[test]
fn test_custom_formatter() {
    // Create a test writer to capture output
    let writer = TestWriter::new();
    let writer_clone = writer.clone();

    // Create and set custom formatter
    let formatter = TestFormatter::new("TEST");
    let logger = Logger::new().formatter(formatter).output(writer).build();

    // Log a test message
    let test_message = "Custom formatted message";
    logger
        .log(Level::Info, test_message, std::collections::HashMap::new())
        .unwrap();

    // Get the output
    let output = String::from_utf8(writer_clone.get_output()).unwrap();

    // Verify the format
    assert!(output.starts_with("TEST ["));
    assert!(output.contains(&format!("] {}: {}", Level::Info, test_message)));
}

#[test]
fn test_custom_formatter_with_fields() {
    let writer = TestWriter::new();
    let writer_clone = writer.clone();

    let formatter = TestFormatter::new("TEST");
    let logger = Logger::new().formatter(formatter).output(writer).build();

    // Log with fields
    let mut fields = std::collections::HashMap::new();
    fields.insert("key1".to_string(), Value::String("value1".to_string()));
    fields.insert("key2".to_string(), Value::String("value2".to_string()));

    logger
        .log(Level::Info, "Message with fields", fields)
        .unwrap();

    let output = String::from_utf8(writer_clone.get_output()).unwrap();
    assert!(output.starts_with("TEST ["));
    assert!(output.contains("] INFO: Message with fields"));
}

#[test]
fn test_formatter_error_handling() {
    // Create a formatter that always fails
    #[derive(Debug, Clone)]
    struct FailingFormatter;

    impl Formatter for FailingFormatter {
        fn format(&self, _entry: &Entry) -> Result<Vec<u8>, Box<dyn Error>> {
            Err("Formatter error".into())
        }
    }

    let logger = Logger::new()
        .formatter(FailingFormatter)
        .output(TestWriter::new())
        .build();

    // The log should not panic even if formatter fails
    let result = logger.log(
        Level::Info,
        "This should handle formatter error gracefully",
        std::collections::HashMap::new(),
    );
    assert!(result.is_err());
}
