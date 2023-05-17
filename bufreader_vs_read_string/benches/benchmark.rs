use bufreader_vs_read_string::{buffer, read_to_string};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_buffer_reader(c: &mut Criterion) {
    c.bench_function("buffer", |b| {
        b.iter(|| {
            buffer(
                black_box("W6k".to_string()),
                black_box("test.txt".to_string()),
            )
        })
    });
}

fn bench_read_to_string(c: &mut Criterion) {
    c.bench_function("read_to_string", |b| {
        b.iter(|| {
            read_to_string(
                black_box("W6k".to_string()),
                black_box("test.txt".to_string()),
            )
        })
    });
}

criterion_group!(benches, bench_buffer_reader, bench_read_to_string);
criterion_main!(benches);
