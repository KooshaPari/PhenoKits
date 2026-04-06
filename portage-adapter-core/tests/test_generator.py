"""Tests for portage_adapter_core code generator."""

import pytest
import yaml
from pathlib import Path
from portage_adapter_core.generator import AdapterGenerator


class TestAdapterGenerator:
    """Tests for AdapterGenerator class."""

    def test_generator_initialization(self):
        """Generator should initialize with defaults."""
        gen = AdapterGenerator()
        assert gen.template is not None

    def test_generate_from_dict(self):
        """Generator should create code from dict."""
        gen = AdapterGenerator()
        schema = {
            "name": "TestBench",
            "category": "coding",
            "description": "Test benchmark"
        }
        code = gen._generate_class(schema)
        assert "TestBenchAdapter" in code
        assert "NAME = 'TestBench'" in code


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
