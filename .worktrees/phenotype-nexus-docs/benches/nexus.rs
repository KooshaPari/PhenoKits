#![feature(test)]
extern crate test;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nexus::registry::{ServiceInstance, ServiceRegistry};
use std::time::Duration;

fn bench_service_instance_new(c: &mut Criterion) {
    c.bench_function("service_instance_new", |b| {
        b.iter(|| ServiceInstance::new(black_box("users"), black_box("localhost:8080")));
    });
}

fn bench_service_instance_with_tag(c: &mut Criterion) {
    let instance = ServiceInstance::new("users", "localhost:8080");
    c.bench_function("service_instance_with_tag", |b| {
        b.iter(|| instance.clone().with_tag(black_box("env"), black_box("prod")));
    });
}

fn bench_service_registry_new(c: &mut Criterion) {
    c.bench_function("service_registry_new", |b| {
        b.iter(|| ServiceRegistry::new());
    });
}

fn bench_service_registry_register(c: &mut Criterion) {
    let registry = ServiceRegistry::new();
    let instance = ServiceInstance::new("users", "localhost:8080");
    c.bench_function("service_registry_register", |b| {
        b.iter(|| registry.register(black_box(instance.clone())));
    });
}

fn bench_service_registry_discover(c: &mut Criterion) {
    let registry = ServiceRegistry::new();
    registry.register(ServiceInstance::new("users", "localhost:8080"));
    registry.register(ServiceInstance::new("users", "localhost:8081"));
    registry.register(ServiceInstance::new("orders", "localhost:9090"));
    c.bench_function("service_registry_discover", |b| {
        b.iter(|| registry.discover(black_box("users")));
    });
}

fn bench_service_registry_discover_next(c: &mut Criterion) {
    let registry = ServiceRegistry::new();
    registry.register(ServiceInstance::new("users", "localhost:8080"));
    registry.register(ServiceInstance::new("users", "localhost:8081"));
    c.bench_function("service_registry_discover_next", |b| {
        b.iter(|| registry.discover_next(black_box("users")));
    });
}

fn bench_service_registry_discover_by_tag(c: &mut Criterion) {
    let registry = ServiceRegistry::new();
    registry.register(ServiceInstance::new("users", "localhost:8080").with_tag("env", "prod"));
    registry.register(ServiceInstance::new("users", "localhost:8081").with_tag("env", "dev"));
    c.bench_function("service_registry_discover_by_tag", |b| {
        b.iter(|| registry.discover_by_tag(black_box("env"), black_box("prod")));
    });
}

fn bench_service_registry_set_health(c: &mut Criterion) {
    let registry = ServiceRegistry::new();
    registry.register(ServiceInstance::new("users", "localhost:8080"));
    c.bench_function("service_registry_set_health", |b| {
        b.iter(|| registry.set_health(black_box("users"), black_box(false)));
    });
}

fn bench_service_registry_len(c: &mut Criterion) {
    let registry = ServiceRegistry::new();
    registry.register(ServiceInstance::new("users", "localhost:8080"));
    registry.register(ServiceInstance::new("orders", "localhost:9090"));
    c.bench_function("service_registry_len", |b| {
        b.iter(|| registry.len());
    });
}

fn bench_service_registry_service_names(c: &mut Criterion) {
    let registry = ServiceRegistry::new();
    registry.register(ServiceInstance::new("users", "localhost:8080"));
    registry.register(ServiceInstance::new("orders", "localhost:9090"));
    c.bench_function("service_registry_service_names", |b| {
        b.iter(|| registry.service_names());
    });
}

fn bench_service_registry_deregister(c: &mut Criterion) {
    let registry = ServiceRegistry::new();
    registry.register(ServiceInstance::new("users", "localhost:8080"));
    c.bench_function("service_registry_deregister", |b| {
        b.iter(|| registry.deregister(black_box("users")));
    });
}

criterion_group!(
    benches,
    bench_service_instance_new,
    bench_service_instance_with_tag,
    bench_service_registry_new,
    bench_service_registry_register,
    bench_service_registry_discover,
    bench_service_registry_discover_next,
    bench_service_registry_discover_by_tag,
    bench_service_registry_set_health,
    bench_service_registry_len,
    bench_service_registry_service_names,
    bench_service_registry_deregister
);
criterion_main!(benches);
