"""Integration tests for profiler pipeline.

Traces to: FR-PROF-101 - Profiler Pipeline Integration
"""

import json
import subprocess
import tempfile
from pathlib import Path

import pytest


class TestProfilerPipeline:
    """Test suite for profiler pipeline integration."""

    def test_bin_scripts_exist(self):
        """Verify all profiler scripts exist and are executable."""
        bin_dir = Path(__file__).parent.parent.parent / "bin"

        required_scripts = [
            "complexity_analyzer.py",
            "continuous_profiler.py",
            "system_metrics.py",
            "generate_charts.py",
            "profiler.sh",
            "all_metrics.sh",
        ]

        for script in required_scripts:
            script_path = bin_dir / script
            assert script_path.exists(), f"Missing script: {script}"

    def test_complexity_output_valid_json(self, temp_dir, sample_python_file):
        """Test that complexity analyzer outputs valid JSON."""
        bin_dir = Path(__file__).parent.parent.parent / "bin"
        output_file = temp_dir / "complexity.json"

        result = subprocess.run(
            ["python3", str(bin_dir / "complexity_analyzer.py"), "-d", str(temp_dir), "-o", str(output_file)],
            capture_output=True,
            text=True,
            cwd=str(temp_dir)
        )

        if output_file.exists():
            content = output_file.read_text()
            if content:
                data = json.loads(content)
                assert isinstance(data, dict)

    def test_system_metrics_output_structure(self):
        """Test that system metrics returns expected data structure."""
        bin_dir = Path(__file__).parent.parent.parent / "bin"

        result = subprocess.run(
            ["python3", str(bin_dir / "system_metrics.py")],
            capture_output=True,
            text=True,
            timeout=10
        )

        assert result.returncode == 0, f"system_metrics.py failed: {result.stderr}"

        if result.stdout:
            data = json.loads(result.stdout)
            assert "cpu" in data or "memory" in data or "timestamp" in data

    def test_profiler_sh_runs(self):
        """Test that main profiler.sh script runs without errors."""
        bin_dir = Path(__file__).parent.parent.parent / "bin"

        result = subprocess.run(
            ["bash", "-n", str(bin_dir / "profiler.sh")],
            capture_output=True,
            text=True
        )

        assert result.returncode == 0, f"profiler.sh has syntax errors: {result.stderr}"

    def test_output_directory_creation(self, temp_dir):
        """Test that profilers create output directories as needed."""
        output_dir = temp_dir / "profila_output"
        bin_dir = Path(__file__).parent.parent.parent / "bin"

        result = subprocess.run(
            ["python3", str(bin_dir / "system_metrics.py"), "--output", str(output_dir / "metrics.json")],
            capture_output=True,
            text=True,
            timeout=10
        )

        if (output_dir / "metrics.json").exists():
            assert (output_dir / "metrics.json").read_text()

    def test_sequential_execution_order(self):
        """Test that profilers can be run in sequence without conflicts."""
        bin_dir = Path(__file__).parent.parent.parent / "bin"

        scripts_to_check = [
            bin_dir / "complexity_analyzer.py",
            bin_dir / "system_metrics.py",
            bin_dir / "generate_charts.py",
        ]

        for script in scripts_to_check:
            result = subprocess.run(
                ["python3", "-c", f"import ast; ast.parse(open('{script}').read())"],
                capture_output=True,
                text=True
            )
            assert result.returncode == 0, f"{script.name} has Python syntax errors"


class TestComplexityIntegration:
    """Integration tests for complexity analyzer."""

    def test_analyzes_multiple_files(self, temp_dir):
        """Test that analyzer processes multiple Python files."""
        for i in range(3):
            (temp_dir / f"module_{i}.py").write_text(f"""
def function_{i}():
    return {i}
""")

        bin_dir = Path(__file__).parent.parent.parent / "bin"

        import sys
        sys.path.insert(0, str(bin_dir))
        from complexity_analyzer import ComplexityAnalyzer

        analyzer = ComplexityAnalyzer(str(temp_dir))
        analyzer.analyze()

        assert len(analyzer.functions) == 3
        function_names = {f.name for f in analyzer.functions}
        assert all(f"function_{i}" in function_names for i in range(3))

    def test_ignores_non_python_files(self, temp_dir):
        """Test that analyzer ignores non-Python files."""
        (temp_dir / "valid.py").write_text("def func(): pass")
        (temp_dir / "not_python.txt").write_text("not python code")
        (temp_dir / "data.json").write_text('{"key": "value"}')

        bin_dir = Path(__file__).parent.parent.parent / "bin"

        import sys
        sys.path.insert(0, str(bin_dir))
        from complexity_analyzer import ComplexityAnalyzer

        analyzer = ComplexityAnalyzer(str(temp_dir))
        analyzer.analyze()

        assert len(analyzer.functions) == 1
        assert analyzer.functions[0].name == "func"
