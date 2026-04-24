use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_validation(c: &mut Criterion) {
    c.bench_function("valid_uuid", |b| {
        b.iter(|| {
            let uuid = black_box("550e8400-e29b-41d4-a716-446655440000");
            uuid.len() == 36
        })
    });
}

criterion_group!(benches, benchmark_validation);
criterion_main!(benches);
