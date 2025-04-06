use loggix::{KafkaHook, Level, Logger};
use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication},
    client::DefaultClientContext,
    consumer::{BaseConsumer, Consumer, StreamConsumer},
    ClientConfig, Message,
};
use std::{fs, path::Path, time::Duration};
use tokio;
use serde::Deserialize;
use chrono;
use serde_json;
use serde_yaml;

#[derive(Debug, Deserialize)]
struct KafkaConfig {
    bootstrap_servers: String,
    group_id: String,
    auto_offset_reset: String,
    socket_timeout_ms: u32,
    session_timeout_ms: u32,
    replication_factor: i32,
    partitions: i32,
}

#[derive(Debug, Deserialize)]
struct Config {
    kafka: KafkaConfig,
}

#[derive(Debug, Deserialize)]
struct LogEntry {
    message: String,
    level: Level,
    #[allow(dead_code)]
    timestamp: chrono::DateTime<chrono::Utc>,
    #[allow(dead_code)]
    fields: std::collections::HashMap<String, serde_json::Value>,
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = Path::new("config.yaml");
    let config_str = fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&config_str)?;
    Ok(config)
}

fn get_kafka_config() -> KafkaConfig {
    load_config()
        .map(|c| c.kafka)
        .unwrap_or_else(|_| KafkaConfig {
            bootstrap_servers: "localhost:9092".to_string(),
            group_id: "test_group".to_string(),
            auto_offset_reset: "earliest".to_string(),
            socket_timeout_ms: 3000,
            session_timeout_ms: 6000,
            replication_factor: 1,
            partitions: 1,
        })
}

async fn ensure_topic_exists(topic: &str, config: &KafkaConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking if topic {} exists...", topic);
    
    // Create admin client to create topic
    let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set("bootstrap.servers", &config.bootstrap_servers)
        .create()?;

    // Always try to create the topic
    println!("Creating topic {}...", topic);
    let topics = vec![NewTopic::new(
        topic,
        config.partitions,
        TopicReplication::Fixed(config.replication_factor),
    )];

    // Ignore error if topic already exists
    match admin_client.create_topics(&topics, &AdminOptions::new()).await {
        Ok(_) => println!("Topic {} created successfully", topic),
        Err(e) => println!("Topic creation returned: {}", e),
    }

    // Wait for topic to be fully created/propagated
    println!("Waiting for topic {} to be fully propagated...", topic);
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Verify topic exists by trying to get metadata
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", &config.bootstrap_servers)
        .create()?;

    let metadata = consumer.fetch_metadata(Some(topic), Duration::from_secs(5))?;
    if !metadata.topics().iter().any(|t| t.name() == topic) {
        return Err("Topic was not created/found successfully".into());
    }

    println!("Topic {} verified", topic);
    Ok(())
}

fn create_test_consumer(topic: &str, config: &KafkaConfig) -> StreamConsumer {
    println!("Creating consumer for topic {}...", topic);
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &config.group_id)
        .set("bootstrap.servers", &config.bootstrap_servers)
        .set("auto.offset.reset", &config.auto_offset_reset)
        .set("socket.timeout.ms", &config.socket_timeout_ms.to_string())
        .set("session.timeout.ms", &config.session_timeout_ms.to_string())
        .create()
        .expect("Consumer creation failed");

    println!("Subscribing to topic {}...", topic);
    consumer
        .subscribe(&[topic])
        .expect("Topic subscription failed");

    consumer
}

#[tokio::test]
async fn test_kafka_hook_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = get_kafka_config();
    println!("Using Kafka config: {:?}", config);

    // Create a unique topic for this test
    let test_topic = format!("test_logs_{}", chrono::Utc::now().timestamp());
    println!("Using test topic: {}", test_topic);

    // Ensure topic exists
    ensure_topic_exists(&test_topic, &config).await?;

    // Create the Kafka hook with a key field
    println!("Creating Kafka hook...");
    let kafka_hook = KafkaHook::new(&config.bootstrap_servers, test_topic.clone())
        .expect("Failed to create Kafka hook")
        .with_key_field("correlation_id".to_string());

    // Create a logger with the Kafka hook
    println!("Creating logger with Kafka hook...");
    let logger = Logger::new().add_hook(kafka_hook).build();

    // Create a consumer before sending messages
    let consumer = create_test_consumer(&test_topic, &config);

    // Send a test log message with a correlation ID
    let test_message = "Test log message";
    let correlation_id = "test-123";
    println!("Sending test message: {} with correlation_id: {}", test_message, correlation_id);
    let mut fields = std::collections::HashMap::new();
    fields.insert("test_key".to_string(), serde_json::Value::String("test_value".to_string()));
    fields.insert("correlation_id".to_string(), serde_json::Value::String(correlation_id.to_string()));
    logger.log_async(Level::Info, test_message, fields).await?;

    // Wait a bit for the message to be delivered
    println!("Waiting for message to be delivered...");
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Try to consume the message with a timeout
    println!("Attempting to consume message...");
    let message = consumer.recv().await
        .map_err(|e| {
            println!("Failed to receive message: {:?}", e);
            e
        })?;

    // Verify the message content
    println!("Verifying message content...");
    let payload = message
        .payload_view::<str>()
        .expect("Failed to get payload")
        .expect("Empty payload");

    let entry: LogEntry = serde_json::from_str(payload).expect("Failed to parse JSON");
    assert_eq!(entry.message, test_message);
    assert_eq!(entry.level, Level::Info);

    // Verify the message key
    println!("Verifying message key...");
    let key = message
        .key_view::<str>()
        .expect("Failed to get key")
        .expect("Empty key");
    assert_eq!(key, correlation_id);

    println!("Test completed successfully");
    Ok(())
}
