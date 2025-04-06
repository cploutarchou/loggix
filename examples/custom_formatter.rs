use loggix::{Entry, Formatter, Level, Logger};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;

/// A custom formatter that outputs logs in a CSV format
#[derive(Debug, Clone)]
struct CSVFormatter {
    include_headers: bool,
    separator: char,
}

impl CSVFormatter {
    pub fn new() -> Self {
        Self {
            include_headers: true,
            separator: ',',
        }
    }

    pub fn separator(mut self, sep: char) -> Self {
        self.separator = sep;
        self
    }

    pub fn include_headers(mut self, include: bool) -> Self {
        self.include_headers = include;
        self
    }
}

impl Default for CSVFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for CSVFormatter {
    fn format(&self, entry: &Entry) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut output = Vec::new();

        // Add headers if enabled
        if self.include_headers {
            write!(
                output,
                "timestamp{}level{}message{}fields\n",
                self.separator, self.separator, self.separator
            )?;
        }

        // Format the entry data
        let fields_str = entry
            .fields
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(";");

        write!(
            output,
            "{}{}{}{}{}{}{}\n",
            entry.timestamp.to_rfc3339(),
            self.separator,
            entry.level,
            self.separator,
            entry.message,
            self.separator,
            fields_str
        )?;

        Ok(output)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create a custom CSV formatter
    let formatter = CSVFormatter::new().separator(',').include_headers(true);

    // Create a logger with the custom formatter
    let logger = Logger::new().formatter(formatter).build();

    // Log some messages with fields
    let mut fields = HashMap::new();
    fields.insert("user_id".to_string(), Value::String("123".to_string()));
    fields.insert("ip".to_string(), Value::String("192.168.1.1".to_string()));
    logger.log(Level::Info, "User logged in", fields)?;

    let mut fields = HashMap::new();
    fields.insert("query_time_ms".to_string(), Value::String("50".to_string()));
    fields.insert("rows_affected".to_string(), Value::String("10".to_string()));
    logger.log(Level::Info, "Database query completed", fields)?;

    Ok(())
}
