"""Tests for portage_adapter_core base classes."""

import pytest
from portage_adapter_core import BaseAdapter, RequireNameMeta


class TestRequireNameMeta:
    """Tests for RequireNameMeta metaclass."""

    def test_base_class_allowed_without_name(self):
        """Base class should not require NAME attribute."""
        class TestAdapter(BaseAdapter):
            pass
        # Should not raise

    def test_subclass_requires_name(self):
        """Subclasses must define NAME attribute."""
        with pytest.raises(TypeError, match="must define 'NAME'"):
            class InvalidAdapter(BaseAdapter):
                pass

    def test_valid_subclass_with_name(self):
        """Subclasses with NAME should work."""
        class ValidAdapter(BaseAdapter):
            NAME = "valid"
        assert ValidAdapter.NAME == "valid"


class TestBaseAdapter:
    """Tests for BaseAdapter class."""

    def test_base_class_instantiation(self):
        """Base class should be abstract."""
        with pytest.raises(TypeError):
            BaseAdapter()

    def test_subclass_instantiation(self):
        """Subclass with NAME should instantiate."""
        class MyAdapter(BaseAdapter):
            NAME = "my"
        
        adapter = MyAdapter()
        assert adapter.NAME == "my"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
