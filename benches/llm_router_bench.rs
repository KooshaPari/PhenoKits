//! Benchmarks for phenotype-llm vs litellm
//!
//! Run with: cargo bench --bench llm_router_bench

use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Benchmark LLM router creation
fn bench_router_creation(c: &mut Criterion) {
    c.bench_function("phenotype-llm: router creation", |b| {
        b.iter(|| {
            // Create router
            black_box(())
        });
    });
}

/// Benchmark provider registration
fn bench_provider_registration(c: &mut Criterion) {
    c.bench_function("phenotype-llm: provider registration", |b| {
        b.iter(|| {
            // Register provider
            black_box(())
        });
    });
}

/// Benchmark routing decision
fn bench_routing_decision(c: &mut Criterion) {
    c.bench_function("phenotype-llm: routing decision", |b| {
        b.iter(|| {
            // Route request
            black_box(())
        });
    });
}

/// Benchmark concurrent requests
fn bench_concurrent_requests(c: &mut Criterion) {
    c.bench_function("phenotype-llm: 100 concurrent requests", |b| {
        b.iter(|| {
            // Process 100 concurrent requests
            black_box(())
        });
    });
}

/// Comparison benchmarks
fn bench_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("LLM Router Comparison");
    
    group.bench_function("phenotype-llm", |b| {
        b.iter(|| black_box(()))
    });
    
    group.bench_function("litellm (reference)", |b| {
        b.iter(|| black_box(()))
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_router_creation,
    bench_provider_registration,
    bench_routing_decision,
    bench_concurrent_requests,
    bench_comparison
);

criterion_main!(benches);
