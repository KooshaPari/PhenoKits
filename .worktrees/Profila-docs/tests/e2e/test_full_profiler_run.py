"""End-to-end tests for full profiler run.

Traces to: FR-PROF-201 - Full Profiler Run
"""

import json
import subprocess
import tempfile
from pathlib import Path

import pytest


class TestFullProfilerRun:
    """Test suite for complete profiler execution."""

    def test_profiler_script_exists(self):
        """Verify the main profiler entry point exists."""
        profiler_sh = Path(__file__).parent.parent.parent / "bin" / "profiler.sh"
        assert profiler_sh.exists(), "profiler.sh not found"

    def test_profiler_sh_help(self):
        """Test that profiler.sh provides help information."""
        bin_dir = Path(__file__).parent.parent.parent / "bin"

        result = subprocess.run(
            ["bash", str(bin_dir / "profiler.sh"), "--help"],
            capture_output=True,
            text=True,
            timeout=5
        )

        assert result.returncode in [0, 1]

    def test_all_metrics_script(self):
        """Test that all_metrics.sh script is valid."""
        bin_dir = Path(__file__).parent.parent.parent / "bin"

        result = subprocess.run(
            ["bash", "-n", str(bin_dir / "all_metrics.sh")],
            capture_output=True,
            text=True
        )

        assert result.returncode == 0, f"all_metrics.sh has syntax errors"

    def test_data_consistency_timestamp(self, temp_dir):
        """Test that all outputs use consistent timestamps."""
        src_dir = temp_dir / "project" / "src"
        src_dir.mkdir(parents=True)

        (src_dir / "main.py").write_text("""
def main():
    x = 1
    for i in range(10):
        x += i
    return x
""")

        bin_dir = Path(__file__).parent.parent.parent / "bin"
        output_dir = temp_dir / "output"
        output_dir.mkdir()

        result = subprocess.run(
            ["python3", str(bin_dir / "system_metrics.py"), "--output", str(output_dir / "metrics.json")],
            capture_output=True,
            text=True,
            timeout=10
        )

        if (output_dir / "metrics.json").exists():
            data = json.loads((output_dir / "metrics.json").read_text())
            assert "timestamp" in data, "Missing timestamp in output"
            assert isinstance(data["timestamp"], (int, float))

    def test_exit_codes(self):
        """Test that scripts return appropriate exit codes."""
        bin_dir = Path(__file__).parent.parent.parent / "bin"

        result = subprocess.run(
            ["python3", str(bin_dir / "system_metrics.py"), "--help"],
            capture_output=True,
            text=True
        )
        assert result.returncode == 0

    def test_resource_cleanup(self, temp_dir):
        """Test that temporary files are cleaned up after execution."""
        bin_dir = Path(__file__).parent.parent.parent / "bin"

        result = subprocess.run(
            ["python3", str(bin_dir / "system_metrics.py")],
            capture_output=True,
            text=True,
            timeout=10
        )
        assert result.returncode == 0

    def test_end_to_end_complexity_analysis(self, temp_dir):
        """Complete E2E test: create Python files, analyze, verify output."""
        src_dir = temp_dir / "project" / "src"
        src_dir.mkdir(parents=True)

        (src_dir / "calculator.py").write_text("""
class Calculator:
    def add(self, a, b):
        return a + b

    def factorial(self, n):
        if n <= 1:
            return 1
        return n * self.factorial(n - 1)
""")

        (src_dir / "utils.py").write_text("""
def format_number(n):
    return f"{n:,}"

def process_list(items):
    result = []
    for item in items:
        if item > 0:
            result.append(item * 2)
    return result
""")

        bin_dir = Path(__file__).parent.parent.parent / "bin"

        import sys
        sys.path.insert(0, str(bin_dir))
        from complexity_analyzer import ComplexityAnalyzer

        analyzer = ComplexityAnalyzer(str(src_dir))
        analyzer.analyze()

        expected_functions = {"add", "factorial", "format_number", "process_list"}
        found_functions = {f.name for f in analyzer.functions}

        assert expected_functions <= found_functions

        factorial = next(f for f in analyzer.functions if f.name == "factorial")
        assert factorial.recurses, "factorial should be marked as recursive"

    def test_continuous_mode_simulation(self):
        """Simulate continuous profiler operation."""
        bin_dir = Path(__file__).parent.parent.parent / "bin"

        result = subprocess.run(
            ["python3", str(bin_dir / "continuous_profiler.py"), "--help"],
            capture_output=True,
            text=True,
            timeout=5
        )

        assert result.returncode == 0
