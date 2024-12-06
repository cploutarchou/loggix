use loggix::with_error;
use std::fs::File;
use std::io::Error;

fn main() {
    // Example 1: Logging a file error
    let result = File::open("non_existent.txt");
    if let Err(error) = result {
        with_error(&error)
            .error("Failed to open file")
            .unwrap();
    }

    // Example 2: Custom error
    let custom_error = Error::new(
        std::io::ErrorKind::Other,
        "Database connection failed"
    );
    with_error(&custom_error)
        .error("Database error occurred")
        .unwrap();
}
