"""
Integration tests for phenotype-evaluation harbor metrics system.
Traces to: FR-EVAL-001 (metrics computation), FR-EVAL-002 (factory),
           FR-EVAL-003 (null/missing reward handling)
"""

import sys
import os
import pytest

# Add src to path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "..", "src"))

from harbor.metrics.sum import Sum
from harbor.metrics.min import Min
from harbor.metrics.max import Max
from harbor.metrics.mean import Mean
from harbor.metrics.factory import MetricFactory
from harbor.models.metric.type import MetricType


# ---- Sum metric ----

class TestSumMetric:
    """FR-EVAL-001: Sum metric computes correctly."""

    def test_sum_basic(self):
        """Traces to: FR-EVAL-001"""
        result = Sum().compute([{"score": 10}, {"score": 20}, {"score": 30}])
        assert result == {"sum": 60}

    def test_sum_with_none_treats_as_zero(self):
        """Traces to: FR-EVAL-003 -- None rewards are treated as 0."""
        result = Sum().compute([{"score": 5}, None, {"score": 15}])
        assert result == {"sum": 20}

    def test_sum_all_none(self):
        """Traces to: FR-EVAL-003"""
        result = Sum().compute([None, None, None])
        assert result == {"sum": 0}

    def test_sum_single_reward(self):
        """Traces to: FR-EVAL-001"""
        result = Sum().compute([{"value": 42}])
        assert result == {"sum": 42}

    def test_sum_floats(self):
        """Traces to: FR-EVAL-001 -- works with float rewards."""
        result = Sum().compute([{"v": 1.5}, {"v": 2.5}])
        assert result == {"sum": 4.0}

    def test_sum_raises_on_multi_key_reward(self):
        """Traces to: FR-EVAL-001 -- rejects ambiguous rewards."""
        with pytest.raises(ValueError, match="Expected exactly one key"):
            Sum().compute([{"a": 1, "b": 2}])


# ---- Min metric ----

class TestMinMetric:
    """FR-EVAL-001: Min metric computes correctly."""

    def test_min_basic(self):
        """Traces to: FR-EVAL-001"""
        result = Min().compute([{"s": 30}, {"s": 10}, {"s": 20}])
        assert result == {"min": 10}

    def test_min_with_none(self):
        """Traces to: FR-EVAL-003 -- None treated as 0, so min can be 0."""
        result = Min().compute([{"s": 5}, None, {"s": 15}])
        assert result == {"min": 0}

    def test_min_single(self):
        """Traces to: FR-EVAL-001"""
        result = Min().compute([{"val": 99}])
        assert result == {"min": 99}

    def test_min_negative_values(self):
        """Traces to: FR-EVAL-001"""
        result = Min().compute([{"v": -5}, {"v": 0}, {"v": 5}])
        assert result == {"min": -5}


# ---- Max metric ----

class TestMaxMetric:
    """FR-EVAL-001: Max metric computes correctly."""

    def test_max_basic(self):
        """Traces to: FR-EVAL-001"""
        result = Max().compute([{"s": 30}, {"s": 10}, {"s": 20}])
        assert result == {"max": 30}

    def test_max_with_none(self):
        """Traces to: FR-EVAL-003 -- None treated as 0."""
        result = Max().compute([None, {"s": 7}, None])
        assert result == {"max": 7}

    def test_max_all_none(self):
        """Traces to: FR-EVAL-003"""
        result = Max().compute([None, None])
        assert result == {"max": 0}

    def test_max_floats(self):
        """Traces to: FR-EVAL-001"""
        result = Max().compute([{"v": 1.1}, {"v": 9.9}, {"v": 5.5}])
        assert result == {"max": 9.9}


# ---- Mean metric ----

class TestMeanMetric:
    """FR-EVAL-001: Mean metric computes correctly."""

    def test_mean_basic(self):
        """Traces to: FR-EVAL-001"""
        result = Mean().compute([{"s": 10}, {"s": 20}, {"s": 30}])
        assert result == {"mean": 20.0}

    def test_mean_with_none(self):
        """Traces to: FR-EVAL-003 -- None is 0, affects mean."""
        result = Mean().compute([{"s": 10}, None, {"s": 20}])
        # (10 + 0 + 20) / 3 = 10.0
        assert result == {"mean": 10.0}

    def test_mean_single(self):
        """Traces to: FR-EVAL-001"""
        result = Mean().compute([{"v": 7}])
        assert result == {"mean": 7.0}


# ---- MetricFactory integration ----

class TestMetricFactory:
    """FR-EVAL-002: MetricFactory creates the correct metric type."""

    def test_factory_creates_sum(self):
        """Traces to: FR-EVAL-002"""
        metric = MetricFactory.create_metric(MetricType.SUM)
        assert isinstance(metric, Sum)
        result = metric.compute([{"x": 5}, {"x": 5}])
        assert result == {"sum": 10}

    def test_factory_creates_min(self):
        """Traces to: FR-EVAL-002"""
        metric = MetricFactory.create_metric(MetricType.MIN)
        assert isinstance(metric, Min)

    def test_factory_creates_max(self):
        """Traces to: FR-EVAL-002"""
        metric = MetricFactory.create_metric(MetricType.MAX)
        assert isinstance(metric, Max)

    def test_factory_creates_mean(self):
        """Traces to: FR-EVAL-002"""
        metric = MetricFactory.create_metric(MetricType.MEAN)
        assert isinstance(metric, Mean)

    def test_factory_raises_on_unsupported_type(self):
        """Traces to: FR-EVAL-002 -- unknown type raises ValueError."""
        # UV_SCRIPT requires kwargs, so skip that; test that factory raises
        # on a completely invalid string-enum value
        import enum
        FakeType = enum.Enum("FakeType", {"FAKE": "fake"})
        with pytest.raises((ValueError, KeyError)):
            MetricFactory.create_metric(FakeType.FAKE)  # type: ignore[arg-type]

    def test_factory_all_types_are_registered(self):
        """Traces to: FR-EVAL-002 -- all MetricType values except UV_SCRIPT have direct mapping."""
        non_script_types = [
            MetricType.SUM, MetricType.MIN, MetricType.MAX, MetricType.MEAN
        ]
        for metric_type in non_script_types:
            metric = MetricFactory.create_metric(metric_type)
            assert metric is not None, f"Factory should produce metric for {metric_type}"


# ---- MetricType enum ----

class TestMetricType:
    """FR-EVAL-002: MetricType enum has expected values."""

    def test_metric_type_string_values(self):
        """Traces to: FR-EVAL-002"""
        assert MetricType.SUM.value == "sum"
        assert MetricType.MIN.value == "min"
        assert MetricType.MAX.value == "max"
        assert MetricType.MEAN.value == "mean"
        assert MetricType.UV_SCRIPT.value == "uv-script"

    def test_metric_type_from_string(self):
        """Traces to: FR-EVAL-002 -- MetricType is str Enum, can be created from string."""
        assert MetricType("sum") == MetricType.SUM
        assert MetricType("min") == MetricType.MIN
        assert MetricType("max") == MetricType.MAX
        assert MetricType("mean") == MetricType.MEAN
