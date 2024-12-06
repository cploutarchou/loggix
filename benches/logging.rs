use criterion::{black_box, criterion_group, criterion_main, Criterion};
use loggix::{Logger, Fields};
use std::io::{self, Write};

// A no-op writer for benchmarking
struct NoopWriter;

impl Write for NoopWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn bench_basic_logging(c: &mut Criterion) {
    let logger = Logger::new()
        .output(NoopWriter)
        .build();
    
    c.bench_function("basic_log", |b| {
        b.iter(|| {
            let _ = black_box(
                logger.with_fields(Fields::new())
                    .info("A basic log message")
            );
        })
    });
}

fn bench_structured_logging(c: &mut Criterion) {
    let logger = Logger::new()
        .output(NoopWriter)
        .build();
    
    c.bench_function("structured_log", |b| {
        b.iter(|| {
            let _ = black_box(
                logger.with_fields(Fields::new())
                    .with_field("key1", "value1")
                    .with_field("key2", 42)
                    .info("A structured log message")
            );
        })
    });
}

fn bench_multiple_fields(c: &mut Criterion) {
    let logger = Logger::new()
        .output(NoopWriter)
        .build();
        
    let fields = vec![
        ("user", "john"),
        ("action", "login"),
        ("ip", "192.168.1.1"),
        ("timestamp", "2023-08-10T15:04:05Z"),
    ];
    
    c.bench_function("multiple_fields_log", |b| {
        b.iter(|| {
            let _ = black_box(
                logger.with_fields(Fields::new())
                    .with_fields_map(fields.clone())
                    .info("Multiple fields log message")
            );
        })
    });
}

criterion_group!(benches, bench_basic_logging, bench_structured_logging, bench_multiple_fields);
criterion_main!(benches);
