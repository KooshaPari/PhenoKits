#!/usr/bin/env python3
"""
Legacy Tooling Anti-Pattern Scanner
Enforces Phenotype Technology Adoption Philosophy per CLAUDE.md

Usage:
    python legacy_tooling_scanner.py --repo-root /path/to/repo --policy rules.yaml
    python legacy_tooling_scanner.py --repo-root . --fail-on-severity high
"""

from __future__ import annotations

import argparse
import fnmatch
import json
import os
import re
import sys
from dataclasses import asdict, dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class Finding:
    """A single policy violation finding."""

    rule_id: str
    severity: str
    message: str
    file: str
    line: int
    command: str
    suggested_fix: str
    column: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {
            "rule_id": self.rule_id,
            "severity": self.severity,
            "message": self.message,
            "file": self.file,
            "line": self.line,
            "column": self.column,
            "command": self.command,
            "suggested_fix": self.suggested_fix,
        }


class RuleEngine:
    """Loads and evaluates policy rules."""

    SEVERITY_ORDER = {"low": 0, "medium": 1, "high": 2, "critical": 3}

    def __init__(self, rules_path: Path, exceptions_path: Path | None = None):
        self.rules = self._load_yaml(rules_path)
        self.exceptions = self._load_yaml(exceptions_path) if exceptions_path else {}
        self._compiled_rules: list[tuple[dict, re.Pattern]] = []
        self._compile_rules()

    def _load_yaml(self, path: Path) -> dict:
        """Load YAML file safely."""
        try:
            import yaml

            with open(path, "r", encoding="utf-8") as f:
                return yaml.safe_load(f) or {}
        except ImportError:
            # Fallback to JSON if yaml not available
            if path.suffix == ".json":
                with open(path, "r", encoding="utf-8") as f:
                    return json.load(f)
            raise
        except FileNotFoundError:
            return {}

    def _compile_rules(self) -> None:
        """Pre-compile regex patterns for performance."""
        for rule in self.rules.get("rules", []):
            patterns = rule.get("patterns", [])
            for p in patterns:
                regex = p.get("regex", "")
                if regex:
                    try:
                        compiled = re.compile(regex, re.IGNORECASE)
                        self._compiled_rules.append((rule, compiled))
                    except re.error as e:
                        print(f"Warning: Invalid regex in rule {rule.get('id')}: {e}", file=sys.stderr)

    def detect_repo_stacks(self, repo_root: Path) -> set[str]:
        """Detect which technology stacks are present in the repo."""
        stacks = set()

        # Python detection
        if any(
            (repo_root / f).exists()
            for f in ["pyproject.toml", "setup.py", "requirements.txt", "uv.lock"]
        ):
            stacks.add("python")

        # JavaScript/TypeScript detection
        if (repo_root / "package.json").exists():
            stacks.add("javascript")
            if any((repo_root / f).exists() for f in ["tsconfig.json", "bun.lockb"]):
                stacks.add("typescript")

        # Go detection
        if (repo_root / "go.mod").exists():
            stacks.add("go")

        # Rust detection
        if (repo_root / "Cargo.toml").exists():
            stacks.add("rust")

        stacks.add("general")
        return stacks

    def get_file_threshold(self, rule: dict, ext: str) -> int | None:
        """Get line threshold for file size rules."""
        thresholds = rule.get("thresholds", {})
        # Map extensions to language names
        ext_map = {
            ".py": "python",
            ".js": "javascript",
            ".ts": "typescript",
            ".tsx": "typescript",
            ".rs": "rust",
            ".go": "go",
        }
        lang = ext_map.get(ext)
        if lang and lang in thresholds:
            return thresholds[lang]
        return None

    def check_file_size(
        self, file_path: Path, repo_root: Path, rule: dict
    ) -> list[Finding]:
        """Check if file exceeds line threshold."""
        findings = []
        ext = file_path.suffix
        threshold = self.get_file_threshold(rule, ext)

        if threshold is None:
            return findings

        try:
            with open(file_path, "r", encoding="utf-8", errors="replace") as f:
                lines = f.readlines()
                line_count = len(lines)

            if line_count > threshold:
                rel_path = str(file_path.relative_to(repo_root))
                findings.append(
                    Finding(
                        rule_id=rule.get("id", "LT-UNKNOWN"),
                        severity=rule.get("severity", "medium"),
                        message=f"{rule.get('name', 'Large file')}: {line_count} lines exceeds threshold of {threshold}",
                        file=rel_path,
                        line=1,
                        column=0,
                        command=f"File has {line_count} lines",
                        suggested_fix=rule.get("suggested_fix", "Refactor into smaller modules"),
                    )
                )
        except (IOError, OSError):
            pass

        return findings

    def scan_file_content(
        self, file_path: Path, repo_root: Path, stacks: set[str]
    ) -> list[Finding]:
        """Scan a single file for rule violations."""
        findings = []
        rel_path = str(file_path.relative_to(repo_root))

        try:
            with open(file_path, "r", encoding="utf-8", errors="replace") as f:
                content = f.read()
                lines = content.splitlines()
        except (IOError, OSError):
            return findings

        for rule, pattern in self._compiled_rules:
            # Check if rule applies to this repo's stacks
            applies_to = set(rule.get("applies_to", []))
            if not applies_to.intersection(stacks):
                continue

            # Check file patterns
            files_patterns = rule.get("files", ["*"])
            if not any(fnmatch.fnmatch(rel_path, fp) for fp in files_patterns):
                continue

            # Check for regex matches
            for i, line in enumerate(lines, start=1):
                if pattern.search(line):
                    # Check exceptions
                    if self._is_excepted(rule.get("id"), rel_path, line):
                        continue

                    findings.append(
                        Finding(
                            rule_id=rule.get("id", "LT-UNKNOWN"),
                            severity=rule.get("severity", "medium"),
                            message=f"{rule.get('id')}: {rule.get('description', 'Unknown rule')}",
                            file=rel_path,
                            line=i,
                            column=line.find(pattern.search(line).group()) if pattern.search(line) else 0,
                            command=line.strip(),
                            suggested_fix=rule.get("suggested_fix", "Review and update"),
                        )
                    )

        return findings

    def _is_excepted(self, rule_id: str, file_path: str, command: str) -> bool:
        """Check if a finding matches an active exception."""
        now = datetime.now(timezone.utc)

        exceptions_list = (self.exceptions or {}).get("exceptions") or []
        for exc in exceptions_list:
            if exc.get("rule_id") != rule_id:
                continue

            # Check if exception is expired
            expires = exc.get("expires_at", "")
            if expires:
                try:
                    expiry_dt = datetime.fromisoformat(expires.replace("Z", "+00:00"))
                    # Ensure timezone-aware comparison
                    if expiry_dt.tzinfo is None:
                        expiry_dt = expiry_dt.replace(tzinfo=timezone.utc)
                    if now > expiry_dt:
                        continue  # Exception expired
                except ValueError:
                    continue

            # Check path glob
            path_glob = exc.get("path_glob", "")
            if path_glob and not fnmatch.fnmatch(file_path, path_glob):
                continue

            # Check command regex
            cmd_regex = exc.get("command_regex", "")
            if cmd_regex:
                try:
                    if not re.search(cmd_regex, command, re.IGNORECASE):
                        continue
                except re.error:
                    continue

            return True

        return False

    def scan_directory(self, repo_root: Path, stacks: set[str]) -> list[Finding]:
        """Scan entire repository directory."""
        findings = []

        # Directories to ignore
        ignore_dirs = {
            ".git",
            ".venv",
            ".env",
            ".tox",
            "node_modules",
            "target",
            "dist",
            "build",
            "__pycache__",
            ".pytest_cache",
            ".mypy_cache",
            ".ruff_cache",
        }

        for root, dirs, files in os.walk(repo_root):
            # Filter out ignored directories
            dirs[:] = [d for d in dirs if d not in ignore_dirs]

            for filename in files:
                file_path = Path(root) / filename

                # Check file size rules
                for rule in self.rules.get("rules", []):
                    if not rule.get("patterns"):  # Size rules have empty patterns
                        findings.extend(self.check_file_size(file_path, repo_root, rule))

                # Content scanning for relevant files
                if filename.endswith(
                    (".yml", ".yaml", ".sh", ".py", ".js", ".ts", ".rs", ".go", ".toml", ".json")
                ):
                    findings.extend(self.scan_file_content(file_path, repo_root, stacks))

        return findings


class ReportGenerator:
    """Generate reports in various formats."""

    @staticmethod
    def json_report(findings: list[Finding], totals: dict[str, int], output_path: Path) -> None:
        """Generate JSON report."""
        report = {
            "generated_at": datetime.now(timezone.utc).isoformat(),
            "totals": totals,
            "findings": [f.to_dict() for f in findings],
        }
        with open(output_path, "w", encoding="utf-8") as f:
            json.dump(report, f, indent=2)

    @staticmethod
    def markdown_report(findings: list[Finding], totals: dict[str, int], output_path: Path) -> None:
        """Generate Markdown report."""
        lines = [
            "# Legacy Tooling Anti-Pattern Report",
            "",
            f"Generated: {datetime.now(timezone.utc).isoformat()}Z",
            "",
            "## Summary",
            "",
            f"- **Critical**: {totals.get('critical', 0)}",
            f"- **High**: {totals.get('high', 0)}",
            f"- **Medium**: {totals.get('medium', 0)}",
            f"- **Low**: {totals.get('low', 0)}",
            f"- **Total**: {sum(totals.values())}",
            "",
            "## Findings",
            "",
        ]

        if not findings:
            lines.append("No legacy tooling anti-patterns detected.")
        else:
            # Sort by severity (high to low) then by file
            severity_order = {"critical": 0, "high": 1, "medium": 2, "low": 3}
            sorted_findings = sorted(
                findings,
                key=lambda f: (severity_order.get(f.severity, 99), f.file, f.line),
            )

            for finding in sorted_findings:
                lines.extend([
                    f"### {finding.rule_id} ({finding.severity.upper()})",
                    "",
                    f"- **File**: `{finding.file}:{finding.line}`",
                    f"- **Message**: {finding.message}",
                    f"- **Command**: `{finding.command}`",
                    f"- **Suggested Fix**: {finding.suggested_fix}",
                    "",
                ])

        with open(output_path, "w", encoding="utf-8") as f:
            f.write("\n".join(lines))

    @staticmethod
    def sarif_report(findings: list[Finding], totals: dict[str, int], output_path: Path) -> None:
        """Generate SARIF report for GitHub Advanced Security integration."""
        sarif = {
            "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
            "version": "2.1.0",
            "runs": [
                {
                    "tool": {
                        "driver": {
                            "name": "legacy-tooling-scanner",
                            "informationUri": "https://github.com/phenotype/tooling",
                            "rules": [],
                        }
                    },
                    "results": [],
                }
            ],
        }

        # Add rules and results
        rule_ids = {f.rule_id for f in findings}
        for rule_id in sorted(rule_ids):
            sarif["runs"][0]["tool"]["driver"]["rules"].append(
                {
                    "id": rule_id,
                    "name": rule_id,
                    "shortDescription": {"text": f"Legacy tooling rule {rule_id}"},
                }
            )

        for finding in findings:
            sarif["runs"][0]["results"].append(
                {
                    "ruleId": finding.rule_id,
                    "level": finding.severity if finding.severity in ["error", "warning", "note"] else "warning",
                    "message": {"text": finding.message},
                    "locations": [
                        {
                            "physicalLocation": {
                                "artifactLocation": {"uri": finding.file},
                                "region": {
                                    "startLine": finding.line,
                                    "startColumn": finding.column,
                                },
                            }
                        }
                    ],
                }
            )

        with open(output_path, "w", encoding="utf-8") as f:
            json.dump(sarif, f, indent=2)


def main() -> int:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Legacy Tooling Anti-Pattern Scanner",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s --repo-root . --policy policy/rules.yaml
  %(prog)s --repo-root /path/to/repo --fail-on-severity high
  %(prog)s --repo-root . --report-only --output-json report.json
""",
    )

    parser.add_argument("--repo-root", default=".", help="Path to repository root (default: .)")
    parser.add_argument("--policy", default="", help="Path to policy rules YAML file")
    parser.add_argument("--exceptions", default="", help="Path to exceptions YAML file")
    parser.add_argument("--fail-on-severity", default="high", choices=["none", "low", "medium", "high", "critical"], help="Minimum severity to fail (default: high)")
    parser.add_argument("--report-only", action="store_true", help="Report only, don't fail")
    parser.add_argument("--output-json", default="legacy-tooling-report.json", help="JSON output path")
    parser.add_argument("--output-md", default="legacy-tooling-report.md", help="Markdown output path")
    parser.add_argument("--output-sarif", default="", help="SARIF output path (optional)")
    parser.add_argument("--verbose", "-v", action="store_true", help="Verbose output")

    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    if not repo_root.exists():
        print(f"Error: Repository root does not exist: {repo_root}", file=sys.stderr)
        return 1

    # Determine policy path
    if args.policy:
        policy_path = Path(args.policy)
    else:
        # Try to find policy relative to script location or repo
        script_dir = Path(__file__).parent.resolve()
        policy_path = script_dir.parent / "policy" / "rules.yaml"
        if not policy_path.exists():
            policy_path = repo_root / "tooling" / "legacy-enforcement" / "policy" / "rules.yaml"

    if not policy_path.exists():
        print(f"Error: Policy file not found: {policy_path}", file=sys.stderr)
        return 1

    # Determine exceptions path
    exceptions_path = None
    if args.exceptions:
        exceptions_path = Path(args.exceptions)
    else:
        exc_path = policy_path.parent / "exceptions.yaml"
        if exc_path.exists():
            exceptions_path = exc_path

    if args.verbose:
        print(f"Scanning repository: {repo_root}")
        print(f"Policy: {policy_path}")
        if exceptions_path:
            print(f"Exceptions: {exceptions_path}")

    # Initialize rule engine
    try:
        engine = RuleEngine(policy_path, exceptions_path)
    except Exception as e:
        print(f"Error loading policy: {e}", file=sys.stderr)
        return 1

    # Detect stacks
    stacks = engine.detect_repo_stacks(repo_root)
    if args.verbose:
        print(f"Detected stacks: {', '.join(stacks)}")

    # Scan repository
    findings = engine.scan_directory(repo_root, stacks)

    # Calculate totals
    severity_order = {"low": 0, "medium": 1, "high": 2, "critical": 3}
    totals = {"critical": 0, "high": 0, "medium": 0, "low": 0}
    for finding in findings:
        totals[finding.severity] = totals.get(finding.severity, 0) + 1

    # Generate reports
    ReportGenerator.json_report(findings, totals, Path(args.output_json))
    ReportGenerator.markdown_report(findings, totals, Path(args.output_md))
    if args.output_sarif:
        ReportGenerator.sarif_report(findings, totals, Path(args.output_sarif))

    # Console output
    print(f"\nLegacy Tooling Anti-Pattern Scan Complete")
    print(f"Repository: {repo_root}")
    print(f"Stacks: {', '.join(stacks)}")
    print(f"\nTotals:")
    fail_threshold = severity_order.get(args.fail_on_severity, 999)
    for sev in ["critical", "high", "medium", "low"]:
        count = totals[sev]
        indicator = "❌" if count > 0 and severity_order[sev] >= fail_threshold else "✓"
        print(f"  {indicator} {sev.upper()}: {count}")

    if findings:
        print(f"\nTop findings by severity:")
        sorted_findings = sorted(
            findings,
            key=lambda f: (severity_order.get(f.severity, 0), f.file),
            reverse=True,
        )[:10]
        for finding in sorted_findings:
            print(f"  [{finding.severity.upper()}] {finding.file}:{finding.line} - {finding.rule_id}")

    print(f"\nReports generated:")
    print(f"  - JSON: {args.output_json}")
    print(f"  - Markdown: {args.output_md}")
    if args.output_sarif:
        print(f"  - SARIF: {args.output_sarif}")

    # Determine exit code
    if args.report_only:
        return 0

    # Check if any findings meet the fail threshold
    threshold_level = severity_order[args.fail_on_severity]
    for finding in findings:
        if severity_order.get(finding.severity, 0) >= threshold_level:
            print(f"\nFailed: Found {finding.severity.upper()} severity violation ({finding.rule_id})")
            return 2

    return 0


if __name__ == "__main__":
    sys.exit(main())
