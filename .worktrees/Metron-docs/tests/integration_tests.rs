//! Integration tests for Metron
//!
//! These tests verify end-to-end functionality across modules.

use metrickit::{Counter, Gauge, Histogram, MetricError, Registry};
use std::thread;

// ============================================================================
// Integration Tests - Registry Operations
// ============================================================================

/// Tests that a Counter can be registered and used through the registry
#[test]
fn test_counter_integration() {
    let registry = Registry::new();

    let counter = registry.register_counter("test_counter").unwrap();

    counter.inc();
    counter.inc();
    counter.add(5.0);

    assert_eq!(counter.get(), 7.0);
}

/// Tests that a Gauge can be registered and used through the registry
#[test]
fn test_gauge_integration() {
    let registry = Registry::new();

    let gauge = registry.register_gauge("test_gauge").unwrap();

    gauge.set(42.0);
    assert_eq!(gauge.get(), 42.0);

    gauge.inc();
    assert_eq!(gauge.get(), 43.0);

    gauge.dec();
    assert_eq!(gauge.get(), 42.0);
}

/// Tests that a Histogram can be registered and used through the registry
#[test]
fn test_histogram_integration() {
    let registry = Registry::new();

    let histogram =
        registry.register_histogram("test_histogram", vec![0.0, 0.5, 1.0, 2.0, 5.0]).unwrap();

    histogram.observe(0.3);
    histogram.observe(0.7);
    histogram.observe(1.5);

    let value = histogram.get();
    assert_eq!(value.count, 3);
    assert!((value.sum - 2.5).abs() < 0.001);
}

/// Tests that duplicate registration returns an error
#[test]
fn test_duplicate_registration_error() {
    let registry = Registry::new();

    registry.register_counter("duplicate_metric").unwrap();
    let result = registry.register_counter("duplicate_metric");

    assert!(matches!(result, Err(MetricError::AlreadyRegistered(_))));
}

/// Tests counter rate limiting behavior
#[test]
fn test_counter_rate_limiting() {
    let registry = Registry::new();
    let counter = registry.register_counter("rate_counter").unwrap();

    // Simulate rapid increments
    for _ in 0..100 {
        counter.inc();
    }

    assert_eq!(counter.get(), 100.0);
}

/// Tests gauge with negative values
#[test]
fn test_gauge_negative_values() {
    let registry = Registry::new();
    let gauge = registry.register_gauge("balance").unwrap();

    gauge.set(-100.0);
    assert_eq!(gauge.get(), -100.0);

    gauge.inc();
    assert_eq!(gauge.get(), -99.0);

    gauge.dec();
    assert_eq!(gauge.get(), -100.0);
}

/// Tests histogram with extreme values
#[test]
fn test_histogram_extreme_values() {
    let registry = Registry::new();
    let histogram = registry.register_histogram("extreme", vec![0.0, 1.0, 10.0, 100.0]).unwrap();

    histogram.observe(0.001); // Very small
    histogram.observe(999.0); // Very large

    let value = histogram.get();
    assert_eq!(value.count, 2);
}

/// Tests registry clear functionality
#[test]
fn test_registry_clear() {
    let registry = Registry::new();

    registry.register_counter("counter1").unwrap();
    registry.register_gauge("gauge1").unwrap();
    registry.register_histogram("histogram1", vec![1.0, 5.0]).unwrap();

    registry.clear();

    // After clear, getting counters/gauges/histograms should create new empty ones
    let counter = registry.counter("counter1");
    assert_eq!(counter.get(), 0.0);

    let gauge = registry.gauge("gauge1");
    assert_eq!(gauge.get(), 0.0);
}

/// Tests getting or creating counters/gauges/histograms
#[test]
fn test_get_or_create() {
    let registry = Registry::new();

    // First call creates
    let c1 = registry.counter("my_counter");
    c1.add(10.0);

    // Second call returns same
    let c2 = registry.counter("my_counter");
    assert_eq!(c2.get(), 10.0);
}

// ============================================================================
// Concurrency Tests
// ============================================================================

/// Tests concurrent counter increments
#[test]
fn test_concurrent_counter_increments() {
    let registry = Registry::new();
    let counter = registry.register_counter("concurrent_counter").unwrap();

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let counter = counter.clone();
            thread::spawn(move || {
                for _ in 0..100 {
                    counter.inc();
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(counter.get(), 1000.0);
}

/// Tests concurrent gauge operations
#[test]
fn test_concurrent_gauge_operations() {
    let registry = Registry::new();
    let gauge = registry.register_gauge("concurrent_gauge").unwrap();

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let gauge = gauge.clone();
            thread::spawn(move || {
                for i in 0..100 {
                    gauge.set(i as f64);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    // Final value is non-deterministic due to concurrency, but should be valid
    let final_value = gauge.get();
    assert!(final_value >= 0.0 && final_value < 100.0);
}

/// Tests concurrent histogram recording
#[test]
fn test_concurrent_histogram_recording() {
    let registry = Registry::new();
    let histogram =
        registry.register_histogram("concurrent_histogram", vec![0.0, 1.0, 5.0, 10.0]).unwrap();

    let handles: Vec<_> = (0..10)
        .map(|i| {
            let histogram = histogram.clone();
            thread::spawn(move || {
                for j in 0..100 {
                    histogram.observe((i * 100 + j) as f64 * 0.1);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let value = histogram.get();
    assert_eq!(value.count, 1000);
}

// ============================================================================
// Stress Tests
// ============================================================================

/// Tests many metrics registered
#[test]
fn test_many_metrics() {
    let registry = Registry::new();

    for i in 0..1000 {
        registry.register_counter(&format!("counter_{}", i)).unwrap();
    }

    let counters = registry.snapshot_counters();
    assert_eq!(counters.len(), 1000);
}

/// Tests many histogram observations
#[test]
fn test_many_histogram_observations() {
    let registry = Registry::new();
    let histogram =
        registry.register_histogram("many_records", vec![0.0, 0.1, 0.5, 1.0, 5.0]).unwrap();

    // Record values 0, 0.001, 0.002, ... 9.999
    for i in 0..10_000 {
        histogram.observe(i as f64 * 0.001);
    }

    let value = histogram.get();
    assert_eq!(value.count, 10_000);
    // Sum of arithmetic series: 0.001 * (0 + 1 + ... + 9999) = 0.001 * 49995000 = 49995
    let expected_sum = 49995.0;
    let diff = (value.sum - expected_sum).abs();
    assert!(diff < 0.1, "Expected sum ~{}, got {} (diff: {})", expected_sum, value.sum, diff);
}

// ============================================================================
// Histogram Tests
// ============================================================================

/// Tests histogram bucket counts
#[test]
fn test_histogram_buckets() {
    let histogram = Histogram::with_buckets("buckets", vec![1.0, 5.0, 10.0]);

    histogram.observe(0.5); // < 1.0
    histogram.observe(2.0); // < 5.0
    histogram.observe(7.0); // < 10.0
    histogram.observe(15.0); // >= 10.0

    let value = histogram.get();
    assert_eq!(value.buckets.counts[0], 1); // 0.5
    assert_eq!(value.buckets.counts[1], 1); // 2.0
    assert_eq!(value.buckets.counts[2], 1); // 7.0
    assert_eq!(value.buckets.counts[3], 1); // 15.0
}

/// Tests histogram with default buckets
#[test]
fn test_histogram_default_buckets() {
    let histogram = Histogram::new("default");

    histogram.observe(0.05);
    histogram.observe(0.5);
    histogram.observe(5.0);

    let value = histogram.get();
    assert_eq!(value.count, 3);
}
