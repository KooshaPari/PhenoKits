"""Smoke tests for the complexity analyzer module."""

import sys
import textwrap
from pathlib import Path

# Add bin/ to path so we can import the analyzer
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "bin"))

from complexity_analyzer import ComplexityAnalyzer, Function


class TestFunctionDataclass:
    """Tests for the Function dataclass."""

    def test_function_defaults(self):
        func = Function(name="foo", line=1)
        assert func.name == "foo"
        assert func.line == 1
        assert func.complexity == 1
        assert func.cyclomatic == 1
        assert func.params == 0
        assert func.recurses is False
        assert func.loops == 0
        assert func.conditionals == 0

    def test_function_custom_values(self):
        func = Function(name="bar", line=10, complexity=5, loops=2)
        assert func.complexity == 5
        assert func.loops == 2


class TestComplexityAnalyzer:
    """Tests for ComplexityAnalyzer."""

    def test_init(self, tmp_path):
        analyzer = ComplexityAnalyzer(str(tmp_path))
        assert analyzer.root == tmp_path
        assert analyzer.functions == []
        assert analyzer.total_lines == 0
        assert analyzer.total_functions == 0

    def test_analyze_empty_dir(self, tmp_path):
        analyzer = ComplexityAnalyzer(str(tmp_path))
        result = analyzer.analyze()
        assert result is analyzer
        assert analyzer.total_functions == 0

    def test_analyze_simple_file(self, tmp_path):
        py_file = tmp_path / "sample.py"
        py_file.write_text(textwrap.dedent("""\
            def hello(name):
                return f"Hello, {name}"

            def add(a, b):
                if a > 0:
                    return a + b
                return b
        """))
        analyzer = ComplexityAnalyzer(str(tmp_path))
        analyzer.analyze()
        assert analyzer.total_functions == 2
        names = [f.name for f in analyzer.functions]
        assert "hello" in names
        assert "add" in names

    def test_analyze_skips_venv(self, tmp_path):
        venv_dir = tmp_path / "venv"
        venv_dir.mkdir()
        (venv_dir / "module.py").write_text("def secret(): pass")
        analyzer = ComplexityAnalyzer(str(tmp_path))
        analyzer.analyze()
        assert analyzer.total_functions == 0
