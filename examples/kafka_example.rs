use loggix::{KafkaHook, Logger};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a Kafka hook
    let kafka_hook = KafkaHook::new(
        "localhost:9092",   // Replace with your Kafka broker(s)
        "logs".to_string(), // The Kafka topic to send logs to
    )?;

    // Create a logger with the Kafka hook
    let logger = Logger::new().add_hook(kafka_hook).build();

    // Use the logger's log method directly
    logger.log(
        loggix::Level::Info,
        "This log message will be sent to Kafka!",
        std::collections::HashMap::new(),
    )?;

    Ok(())
}
