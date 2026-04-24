#![feature(test)]
extern crate test;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use phenotype_xdd_lib::prelude::*;

fn property_benchmark(c: &mut Criterion) {
    c.bench_function("non_empty_string creation", |b| {
        b.iter(|| NonEmptyString::new(black_box("benchmark")))
    });
}

criterion_group!(benches, property_benchmark);
criterion_main!(benches);
