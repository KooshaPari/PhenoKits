//! Metric Registry

use parking_lot::RwLock;
use std::collections::HashMap;

use super::{Counter, Gauge, Histogram, MetricError};

/// Metric registry
pub struct Registry {
    counters: RwLock<HashMap<String, Counter>>,
    gauges: RwLock<HashMap<String, Gauge>>,
    histograms: RwLock<HashMap<String, Histogram>>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        Self {
            counters: RwLock::new(HashMap::new()),
            gauges: RwLock::new(HashMap::new()),
            histograms: RwLock::new(HashMap::new()),
        }
    }

    /// Register a counter
    pub fn register_counter(&self, name: &str) -> Result<Counter, MetricError> {
        let counter = Counter::new(name);
        let mut counters = self.counters.write();
        if counters.contains_key(name) {
            return Err(MetricError::AlreadyRegistered(name.into()));
        }
        counters.insert(name.into(), counter.clone());
        Ok(counter)
    }

    /// Get or create a counter
    pub fn counter(&self, name: &str) -> Counter {
        let mut counters = self.counters.write();
        if let Some(c) = counters.get(name) {
            return c.clone();
        }
        let counter = Counter::new(name);
        counters.insert(name.into(), counter.clone());
        counter
    }

    /// Register a gauge
    pub fn register_gauge(&self, name: &str) -> Result<Gauge, MetricError> {
        let gauge = Gauge::new(name);
        let mut gauges = self.gauges.write();
        if gauges.contains_key(name) {
            return Err(MetricError::AlreadyRegistered(name.into()));
        }
        gauges.insert(name.into(), gauge.clone());
        Ok(gauge)
    }

    /// Get or create a gauge
    pub fn gauge(&self, name: &str) -> Gauge {
        let mut gauges = self.gauges.write();
        if let Some(g) = gauges.get(name) {
            return g.clone();
        }
        let gauge = Gauge::new(name);
        gauges.insert(name.into(), gauge.clone());
        gauge
    }

    /// Register a histogram
    pub fn register_histogram(
        &self,
        name: &str,
        bounds: Vec<f64>,
    ) -> Result<Histogram, MetricError> {
        let histogram = Histogram::with_buckets(name, bounds);
        let mut histograms = self.histograms.write();
        if histograms.contains_key(name) {
            return Err(MetricError::AlreadyRegistered(name.into()));
        }
        histograms.insert(name.into(), histogram.clone());
        Ok(histogram)
    }

    /// Get or create a histogram
    pub fn histogram(&self, name: &str) -> Histogram {
        let mut histograms = self.histograms.write();
        if let Some(h) = histograms.get(name) {
            return h.clone();
        }
        let histogram = Histogram::new(name);
        histograms.insert(name.into(), histogram.clone());
        histogram
    }

    /// Clear all metrics
    pub fn clear(&self) {
        self.counters.write().clear();
        self.gauges.write().clear();
        self.histograms.write().clear();
    }

    /// Snapshot of all counters for export
    pub fn snapshot_counters(&self) -> Vec<(String, Counter)> {
        self.counters.read().iter().map(|(name, counter)| (name.clone(), counter.clone())).collect()
    }

    /// Snapshot of all gauges for export
    pub fn snapshot_gauges(&self) -> Vec<(String, Gauge)> {
        self.gauges.read().iter().map(|(name, gauge)| (name.clone(), gauge.clone())).collect()
    }

    /// Snapshot of all histograms for export
    pub fn snapshot_histograms(&self) -> Vec<(String, Histogram)> {
        self.histograms
            .read()
            .iter()
            .map(|(name, histogram)| (name.clone(), histogram.clone()))
            .collect()
    }

    /// Get total number of registered metrics
    pub fn len(&self) -> usize {
        self.counters.read().len() + self.gauges.read().len() + self.histograms.read().len()
    }

    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_is_empty() {
        let registry = Registry::new();
        assert!(registry.is_empty());
    }

    #[test]
    fn test_registry_len_empty() {
        let registry = Registry::new();
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_registry_len_with_metrics() {
        let registry = Registry::new();
        registry.counter("test_counter");
        registry.gauge("test_gauge");
        registry.histogram("test_histogram");
        assert_eq!(registry.len(), 3);
    }

    #[test]
    fn test_register_counter_error_on_duplicate() {
        let registry = Registry::new();
        registry.register_counter("test").unwrap();
        let result = registry.register_counter("test");
        assert!(result.is_err());
        // Verify it's an AlreadyRegistered error using pattern match
        if let Err(MetricError::AlreadyRegistered(_)) = result {
            // Expected
        } else {
            panic!("Expected AlreadyRegistered error");
        }
    }

    #[test]
    fn test_register_gauge_error_on_duplicate() {
        let registry = Registry::new();
        registry.register_gauge("test").unwrap();
        let result = registry.register_gauge("test");
        assert!(result.is_err());
    }

    #[test]
    fn test_register_histogram_error_on_duplicate() {
        let registry = Registry::new();
        registry.register_histogram("test", vec![0.1, 0.5, 1.0]).unwrap();
        let result = registry.register_histogram("test", vec![0.1, 0.5]);
        assert!(result.is_err());
    }

    #[test]
    fn test_clear_removes_all_metrics() {
        let registry = Registry::new();
        registry.counter("c1");
        registry.gauge("g1");
        registry.histogram("h1");
        assert_eq!(registry.len(), 3);
        registry.clear();
        assert!(registry.is_empty());
    }

    #[test]
    fn test_snapshot_empty() {
        let registry = Registry::new();
        assert!(registry.snapshot_counters().is_empty());
        assert!(registry.snapshot_gauges().is_empty());
        assert!(registry.snapshot_histograms().is_empty());
    }

    #[test]
    fn test_snapshot_contains_registered_metrics() {
        let registry = Registry::new();
        registry.counter("counter1");
        registry.gauge("gauge1");
        registry.histogram("histogram1");

        let counters = registry.snapshot_counters();
        let gauges = registry.snapshot_gauges();
        let histograms = registry.snapshot_histograms();

        assert_eq!(counters.len(), 1);
        assert_eq!(gauges.len(), 1);
        assert_eq!(histograms.len(), 1);
    }

    #[test]
    fn test_counter_returns_existing() {
        let registry = Registry::new();
        let c1 = registry.counter("test");
        c1.inc();
        let c2 = registry.counter("test");
        // Both should reference same counter
        assert_eq!(c2.get(), 1.0);
    }

    #[test]
    fn test_gauge_returns_existing() {
        let registry = Registry::new();
        let g1 = registry.gauge("test");
        g1.set(10.0);
        let g2 = registry.gauge("test");
        assert_eq!(g2.get(), 10.0);
    }

    #[test]
    fn test_histogram_returns_existing() {
        let registry = Registry::new();
        let h1 = registry.histogram("test");
        h1.observe(0.5);
        let h2 = registry.histogram("test");
        assert_eq!(h2.get().count, 1);
    }

    #[test]
    fn test_counter_get_or_create_separate() {
        let registry = Registry::new();
        registry.counter("a");
        registry.counter("b");
        assert_eq!(registry.len(), 2);
    }
}
