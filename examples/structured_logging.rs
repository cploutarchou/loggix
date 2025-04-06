use loggix::{with_fields, Logger};

fn main() {
    // Initialize a custom logger
    let logger = Logger::new().build();

    // Log with structured fields
    with_fields!(
        "user_id" => "12345",
        "action" => "login",
        "ip" => "192.168.1.1"
    )
    .info("User login successful")
    .unwrap();

    // Add fields using with_field
    logger
        .with_fields(Default::default())
        .with_field("request_id", "abc-123")
        .with_field("path", "/api/v1/users")
        .with_field("method", "GET")
        .info("API request received")
        .unwrap();
}
