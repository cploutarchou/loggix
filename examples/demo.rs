use loggix::{Logger, with_fields};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger with colors enabled
    let logger = Logger::new().build();

    // Basic logging
    logger.with_fields(Default::default())
        .info("Starting application...")?;

    // Structured logging with fields
    with_fields!(
        "user" => "john_doe",
        "role" => "admin"
    )
    .info("User logged in successfully")?;

    // Warning with additional context
    with_fields!(
        "component" => "auth",
        "attempt" => 3,
        "ip" => "192.168.1.100"
    )
    .warn("Multiple login attempts detected")?;

    // Error with structured data
    with_fields!(
        "operation" => "database_query",
        "table" => "users",
        "duration_ms" => 1500
    )
    .error("Query timeout exceeded")?;

    // Debug with complex data
    with_fields!(
        "request_id" => "abc-123",
        "path" => "/api/v1/users",
        "method" => "POST",
        "status" => 201
    )
    .debug("API request processed")?;

    Ok(())
}
