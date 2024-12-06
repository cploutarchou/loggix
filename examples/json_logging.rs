use loggix::{Logger, JSONFormatter, Fields};

fn main() {
    // Create a logger with JSON formatter
    let logger = Logger::new()
        .formatter(JSONFormatter::new().pretty(true))
        .build();

    // Log structured data that will be formatted as JSON
    logger
        .with_fields(Fields::new())
        .with_field("transaction_id", "tx-9876")
        .with_field("amount", 150.50)
        .with_field("currency", "USD")
        .with_field("status", "completed")
        .info("Payment processed")
        .unwrap();
}
