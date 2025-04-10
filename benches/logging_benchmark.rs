use criterion::{black_box, criterion_group, criterion_main, Criterion};
use loggix::{Fields, Level, Logger, TextFormatter};
use serde_json::Value;
use std::io::Write;

struct NullWriter;

impl Write for NullWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn bench_sync_logging(c: &mut Criterion) {
    let logger = Logger::new()
        .formatter(TextFormatter::default().colors(false))
        .output(Box::new(NullWriter))
        .build();

    let mut fields = Fields::new();
    fields.insert("key1".to_string(), Value::String("value1".to_string()));
    fields.insert("key2".to_string(), Value::String("value2".to_string()));

    c.bench_function("sync_logging", |b| {
        b.iter(|| black_box(logger.log(Level::Info, "Test log message", fields.clone())))
    });
}

fn bench_async_logging(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let logger = Logger::new()
        .formatter(TextFormatter::default().colors(false))
        .output(Box::new(NullWriter))
        .build();

    let mut fields = Fields::new();
    fields.insert("key1".to_string(), Value::String("value1".to_string()));
    fields.insert("key2".to_string(), Value::String("value2".to_string()));

    c.bench_function("async_logging", |b| {
        b.iter(|| {
            rt.block_on(async {
                black_box(
                    logger
                        .log_async(Level::Info, "Test log message", fields.clone())
                        .await,
                )
            })
        })
    });
}

criterion_group!(benches, bench_sync_logging, bench_async_logging);
criterion_main!(benches);
