//! Metric Types

use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Metric type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

/// Metric metadata
#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct MetricMetadata {
    pub name: String,
    pub description: Option<String>,
    pub unit: Option<String>,
    pub metric_type: MetricType,
}

impl Hash for MetricMetadata {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.metric_type.hash(state);
    }
}

impl PartialEq for MetricMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.metric_type == other.metric_type
    }
}

impl MetricMetadata {
    pub fn new(name: impl Into<String>, metric_type: MetricType) -> Self {
        Self { name: name.into(), description: None, unit: None, metric_type }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_type_variants() {
        assert_eq!(format!("{:?}", MetricType::Counter), "Counter");
        assert_eq!(format!("{:?}", MetricType::Gauge), "Gauge");
        assert_eq!(format!("{:?}", MetricType::Histogram), "Histogram");
        assert_eq!(format!("{:?}", MetricType::Summary), "Summary");
    }

    #[test]
    fn test_metric_type_equality() {
        assert_eq!(MetricType::Counter, MetricType::Counter);
        assert_ne!(MetricType::Counter, MetricType::Gauge);
    }

    #[test]
    fn test_metric_metadata_creation() {
        let meta = MetricMetadata::new("requests_total", MetricType::Counter);
        assert_eq!(meta.name, "requests_total");
        assert_eq!(meta.metric_type, MetricType::Counter);
        assert!(meta.description.is_none());
        assert!(meta.unit.is_none());
    }

    #[test]
    fn test_metric_metadata_with_description() {
        let meta = MetricMetadata::new("latency", MetricType::Histogram)
            .with_description("Request latency in milliseconds");
        assert_eq!(meta.description, Some("Request latency in milliseconds".to_string()));
    }

    #[test]
    fn test_metric_metadata_with_unit() {
        let meta = MetricMetadata::new("bytes", MetricType::Gauge).with_unit("bytes");
        assert_eq!(meta.unit, Some("bytes".to_string()));
    }

    #[test]
    fn test_metric_metadata_chaining() {
        let meta = MetricMetadata::new("cpu_usage", MetricType::Gauge)
            .with_description("CPU usage percentage")
            .with_unit("percent");
        assert_eq!(meta.name, "cpu_usage");
        assert_eq!(meta.metric_type, MetricType::Gauge);
        assert_eq!(meta.description, Some("CPU usage percentage".to_string()));
        assert_eq!(meta.unit, Some("percent".to_string()));
    }

    #[test]
    fn test_metric_metadata_serialization() {
        let meta = MetricMetadata::new("test_metric", MetricType::Counter)
            .with_description("A test metric");
        let json = serde_json::to_string(&meta).unwrap();
        assert!(json.contains("test_metric"));
        assert!(json.contains("Counter"));
    }

    #[test]
    fn test_metric_metadata_clone() {
        let meta = MetricMetadata::new("clone_test", MetricType::Summary);
        let cloned = meta.clone();
        assert_eq!(meta.name, cloned.name);
        assert_eq!(meta.metric_type, cloned.metric_type);
    }

    #[test]
    fn test_metric_metadata_all_types() {
        for metric_type in
            [MetricType::Counter, MetricType::Gauge, MetricType::Histogram, MetricType::Summary]
        {
            let meta = MetricMetadata::new("test", metric_type);
            assert_eq!(meta.metric_type, metric_type);
        }
    }

    #[test]
    fn test_metric_metadata_unicode_name() {
        let meta = MetricMetadata::new("指标_メトリクス", MetricType::Counter);
        assert_eq!(meta.name, "指标_メトリクス");
    }

    #[test]
    fn test_metric_metadata_special_chars() {
        let meta = MetricMetadata::new("metric-with-dashes.and_dots", MetricType::Gauge)
            .with_description("Description with \"quotes\"")
            .with_unit("unit/second");
        assert_eq!(meta.name, "metric-with-dashes.and_dots");
    }

    #[test]
    fn test_metric_metadata_empty_name() {
        let meta = MetricMetadata::new("", MetricType::Counter);
        assert_eq!(meta.name, "");
    }

    #[test]
    fn test_metric_metadata_long_name() {
        let long_name = "a".repeat(1000);
        let meta = MetricMetadata::new(long_name.clone(), MetricType::Counter);
        assert_eq!(meta.name, long_name);
    }

    #[test]
    fn test_metric_metadata_deserialization_roundtrip() {
        let meta = MetricMetadata::new("test_metric", MetricType::Histogram)
            .with_description("Test description")
            .with_unit("ms");

        let json = serde_json::to_string(&meta).unwrap();
        let parsed: MetricMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.name, meta.name);
        assert_eq!(parsed.metric_type, meta.metric_type);
        assert_eq!(parsed.description, meta.description);
        assert_eq!(parsed.unit, meta.unit);
    }

    #[test]
    fn test_metric_metadata_hash() {
        use std::collections::HashSet;
        let meta1 = MetricMetadata::new("test", MetricType::Counter);
        let meta2 = MetricMetadata::new("test", MetricType::Counter);
        let meta3 = MetricMetadata::new("other", MetricType::Counter);

        let mut set = HashSet::new();
        set.insert(meta1);
        set.insert(meta2);
        set.insert(meta3);

        // meta1 and meta2 should be deduplicated
        assert_eq!(set.len(), 2);
    }
}

// Property-based tests
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_metric_type_serialization_roundtrip(metric_type: u8) {
            let metric_type = match metric_type % 4 {
                0 => MetricType::Counter,
                1 => MetricType::Gauge,
                2 => MetricType::Histogram,
                _ => MetricType::Summary,
            };
            let json = serde_json::to_string(&metric_type).unwrap();
            let parsed: MetricType = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(parsed, metric_type);
        }

        #[test]
        fn test_metric_metadata_name_roundtrip(name: String) {
            let meta = MetricMetadata::new(name.clone(), MetricType::Counter);
            prop_assert_eq!(meta.name, name);
        }

        #[test]
        fn test_metric_metadata_description_roundtrip(desc: String) {
            let meta = MetricMetadata::new("test", MetricType::Counter)
                .with_description(desc.clone());
            prop_assert_eq!(meta.description.unwrap(), desc);
        }
    }
}
