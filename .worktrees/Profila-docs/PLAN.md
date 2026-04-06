# Plan -- profiler

**Version:** 1.0
**Status:** Active
**Last Updated:** 2026-03-27

Phased WBS with DAG dependencies. All phases are agent-executable with no human checkpoints.

---

## Phase 1: Core Shell Profiling (Complete)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P1.1 | Bash entry-point `profiler.sh` with subcommand dispatch | -- | Done |
| P1.2 | `all_metrics.sh` for full OS-level memory and CPU snapshot | P1.1 | Done |
| P1.3 | `network_profiler.sh` for TCP/UDP connections and interface byte counts | P1.1 | Done |
| P1.4 | `disk_profiler.sh` for read/write throughput and file-handle count | P1.1 | Done |
| P1.5 | `full_system_audit.sh` consolidating all resource sections | P1.2, P1.3, P1.4 | Done |
| P1.6 | `pgrep`-based PID resolution, cross-platform macOS/Linux | P1.1 | Done |
| P1.7 | Timestamped output files to `reports/` directory | P1.1 | Done |

---

## Phase 2: Python Analysis Tools (Complete)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P2.1 | `complexity_analyzer.py` -- AST-based cyclomatic complexity for Python | P1.1 | Done |
| P2.2 | `--lang` flag supporting `rust`, `go`, `typescript` via regex heuristics | P2.1 | Done |
| P2.3 | HIGH-flag exit code 1 when cyclomatic > 10 or cognitive > 15 | P2.1 | Done |
| P2.4 | `continuous_profiler.py` -- CSV streaming at configurable interval | P1.6 | Done |
| P2.5 | SIGINT handler for clean flush-and-exit in continuous mode | P2.4 | Done |
| P2.6 | `generate_charts.py` -- matplotlib PNG generation from CSV | P2.4 | Done |

---

## Phase 3: Setup and Build Instrumentation (Complete)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P3.1 | `profiler_setup.sh` -- dependency installation and env verification | P1.1 | Done |
| P3.2 | `build_for_profiling.sh` -- Rust/Go instrumented builds with debug symbols | P1.1 | Done |

---

## Phase 4: Spec Documentation (Complete)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P4.1 | `PRD.md` -- epics E1-E5 with acceptance criteria | P1.1 | Done |
| P4.2 | `FUNCTIONAL_REQUIREMENTS.md` -- FR-MET/NET/DSK/AUD/CPX/CNT/CHT requirements | P4.1 | Done |
| P4.3 | `ADR.md` -- ADR-001 through ADR-006 for key architecture decisions | P4.1 | Done |
| P4.4 | `PLAN.md` -- this document | P4.1 | Done |

---

## Phase 5: Test Coverage (Planned)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P5.1 | Unit tests for `complexity_analyzer.py` (FR-CPX-001 through FR-CPX-004) | P2.1 | Planned |
| P5.2 | Unit tests for `continuous_profiler.py` SIGINT flush (FR-CNT-003) | P2.5 | Planned |
| P5.3 | Unit tests for `generate_charts.py` PNG output (FR-CHT-001, FR-CHT-002) | P2.6 | Planned |
| P5.4 | Integration test: `profiler.sh quick <target>` exits non-zero on missing process (FR-MET-002) | P1.1 | Planned |
| P5.5 | Integration test: continuous CSV header-first format (FR-CNT-001) | P2.4 | Planned |

---

## DAG Summary

```
P1.1
 +-- P1.2 --.
 +-- P1.3 ---+-- P1.5
 +-- P1.4 --'
 +-- P1.6 --.
 |           +-- P2.4 -- P2.5 -- P2.6
 +-- P1.7   +-- P3.2
P2.1 -- P2.2
     \-- P2.3
P4.1 -- P4.2
     \-- P4.3
     \-- P4.4
P2.1 -- P5.1
P2.5 -- P5.2
P2.6 -- P5.3
P1.1 -- P5.4
P2.4 -- P5.5
```
