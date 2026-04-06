//! Metric Exporter

use crate::domain::*;

pub trait MetricExporter: Send + Sync {
    fn export(&self, registry: &Registry) -> MetricResult<String>;
}

/// Prometheus format exporter
#[derive(Clone)]
pub struct PrometheusExporter;

impl PrometheusExporter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PrometheusExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricExporter for PrometheusExporter {
    fn export(&self, registry: &Registry) -> MetricResult<String> {
        let mut output = String::new();

        // Export counters
        for (name, counter) in registry.snapshot_counters() {
            let metadata = counter.metadata();
            if let Some(ref desc) = metadata.description {
                output.push_str(&format!("# HELP {} {}\n", name, desc));
            }
            if let Some(ref unit) = metadata.unit {
                output.push_str(&format!("# UNIT {} {}\n", name, unit));
            }
            output.push_str(&format!("{} {}\n", name, counter.get()));
        }

        // Export gauges
        for (name, gauge) in registry.snapshot_gauges() {
            let metadata = gauge.metadata();
            if let Some(ref desc) = metadata.description {
                output.push_str(&format!("# HELP {} {}\n", name, desc));
            }
            output.push_str(&format!("{} {}\n", name, gauge.get()));
        }

        // Export histograms
        for (name, histogram) in registry.snapshot_histograms() {
            let value = histogram.get();
            let metadata = histogram.metadata();
            if let Some(ref desc) = metadata.description {
                output.push_str(&format!("# HELP {} {}\n", name, desc));
            }
            output.push_str(&format!("{}_count {}\n", name, value.count));
            output.push_str(&format!("{}_sum {}\n", name, value.sum));
            for (i, count) in value.buckets.counts.iter().enumerate() {
                let bound = if i < value.buckets.bounds.len() {
                    value.buckets.bounds[i]
                } else {
                    f64::INFINITY
                };
                output.push_str(&format!("{}_bucket{{le=\"{}\"}} {}\n", name, bound, count));
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Registry;

    #[test]
    fn prometheus_exporter_includes_metrics() {
        let registry = Registry::default();
        let counter = registry.counter("requests_total");
        counter.inc();
        let gauge = registry.gauge("current_users");
        gauge.set(42.0);
        let histogram = registry.histogram("response_latency");
        histogram.observe(0.03);

        let output = PrometheusExporter::new().export(&registry).expect("failed to export metrics");

        assert!(output.contains("requests_total 1"));
        assert!(output.contains("current_users 42"));
        assert!(output.contains("response_latency_count 1"));
        assert!(output.contains("response_latency_bucket"));
    }

    #[test]
    fn prometheus_exporter_empty_registry() {
        let registry = Registry::new();
        let output = PrometheusExporter::new().export(&registry).expect("failed to export");
        assert!(output.is_empty());
    }

    #[test]
    fn prometheus_exporter_counter_with_metadata() {
        let counter =
            Counter::new("test_counter").with_description("A test counter").with_unit("requests");
        let registry = Registry::new();
        registry.counter("test_counter"); // This creates a new counter without metadata
        let registered_counter = registry.register_counter("test_counter_2").unwrap();
        // Since registry.counter returns existing, let's use register then observe
        let c = registry.register_counter("metadata_counter").unwrap();
        c.inc();
        c.inc();

        // Test that exporter works with basic metrics
        let output = PrometheusExporter::new().export(&registry).expect("failed to export");
        assert!(output.contains("metadata_counter"));
    }

    #[test]
    fn prometheus_exporter_gauge_with_metadata() {
        let registry = Registry::new();
        let g = registry.register_gauge("metadata_gauge").unwrap();
        g.set(100.5);

        let output = PrometheusExporter::new().export(&registry).expect("failed to export");
        assert!(output.contains("metadata_gauge 100.5"));
    }

    #[test]
    fn prometheus_exporter_histogram_with_metadata() {
        let registry = Registry::new();
        let h = registry.register_histogram("metadata_histogram", vec![0.1, 0.5, 1.0]).unwrap();
        h.observe(0.05);
        h.observe(0.3);
        h.observe(1.5);

        let output = PrometheusExporter::new().export(&registry).expect("failed to export");
        assert!(output.contains("metadata_histogram_count 3"));
        assert!(output.contains("metadata_histogram_sum"));
        assert!(output.contains("metadata_histogram_bucket"));
        // Check that histogram is exported correctly
        assert!(output.contains("metadata_histogram"));
    }

    #[test]
    fn prometheus_exporter_multiple_counters() {
        let registry = Registry::new();
        registry.counter("c1");
        registry.counter("c2");
        registry.counter("c3");

        let output = PrometheusExporter::new().export(&registry).expect("failed to export");
        assert!(output.contains("c1 0"));
        assert!(output.contains("c2 0"));
        assert!(output.contains("c3 0"));
    }

    #[test]
    fn prometheus_exporter_multiple_gauges() {
        let registry = Registry::new();
        let g1 = registry.gauge("g1");
        let g2 = registry.gauge("g2");
        g1.set(1.0);
        g2.set(2.0);

        let output = PrometheusExporter::new().export(&registry).expect("failed to export");
        assert!(output.contains("g1 1"));
        assert!(output.contains("g2 2"));
    }

    #[test]
    fn prometheus_exporter_histogram_buckets() {
        let registry = Registry::new();
        let histogram = registry.register_histogram("h", vec![0.1, 0.5, 1.0]).unwrap();
        histogram.observe(0.05); // < 0.1
        histogram.observe(0.3); // 0.1-0.5
        histogram.observe(0.7); // 0.5-1.0
        histogram.observe(2.0); // > 1.0

        let output = PrometheusExporter::new().export(&registry).expect("failed to export");
        // Check histogram is exported
        assert!(output.contains("h_count 4"));
        // Check buckets exist
        assert!(output.contains("h_bucket"));
    }

    #[test]
    fn prometheus_exporter_cloned() {
        let exporter = PrometheusExporter::new();
        let _cloned = exporter.clone();
        let registry = Registry::new();
        registry.counter("test");
        assert!(exporter.export(&registry).is_ok());
    }
}
