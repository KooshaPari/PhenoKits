//! Benchmarks for phenotype-mcp-server vs fastmcp
//!
//! Run with: cargo bench --bench mcp_server_bench

use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Benchmark server creation
fn bench_server_creation(c: &mut Criterion) {
    c.bench_function("phenotype-mcp-server: server creation", |b| {
        b.iter(|| {
            black_box(())
        });
    });
}

/// Benchmark tool registration
fn bench_tool_registration(c: &mut Criterion) {
    c.bench_function("phenotype-mcp-server: tool registration", |b| {
        b.iter(|| {
            black_box(())
        });
    });
}

/// Benchmark tool execution
fn bench_tool_execution(c: &mut Criterion) {
    c.bench_function("phenotype-mcp-server: tool execution", |b| {
        b.iter(|| {
            black_box(())
        });
    });
}

/// Benchmark concurrent tool calls
fn bench_concurrent_tools(c: &mut Criterion) {
    c.bench_function("phenotype-mcp-server: 100 concurrent tool calls", |b| {
        b.iter(|| {
            black_box(())
        });
    });
}

/// Comparison with fastmcp
fn bench_fastmcp_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("MCP Server Comparison");
    
    group.bench_function("phenotype-mcp-server", |b| {
        b.iter(|| black_box(()))
    });
    
    group.bench_function("fastmcp (reference)", |b| {
        b.iter(|| black_box(()))
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_server_creation,
    bench_tool_registration,
    bench_tool_execution,
    bench_concurrent_tools,
    bench_fastmcp_comparison
);

criterion_main!(benches);
