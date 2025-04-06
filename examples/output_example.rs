use loggix::{with_fields, Logger};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger with colors enabled
    let logger = Logger::new().build();

    // Basic logging
    logger
        .with_fields(Default::default())
        .info("Starting application...")?;

    // Debug with request details
    with_fields!(
        "request_id" => "abc-123",
        "method" => "GET",
        "path" => "/api/v1/users"
    )
    .debug("Processing API request")?;

    // Info with user context
    with_fields!(
        "user" => "john_doe",
        "role" => "admin",
        "session" => "5f2c"
    )
    .info("User authenticated")?;

    // Warning about performance
    with_fields!(
        "latency_ms" => 1500,
        "threshold_ms" => 1000,
        "endpoint" => "/api/search"
    )
    .warn("High latency detected")?;

    // Error with details
    with_fields!(
        "operation" => "db_query",
        "table" => "users",
        "error_code" => "ERR_TIMEOUT"
    )
    .error("Database operation failed")?;

    Ok(())
}
