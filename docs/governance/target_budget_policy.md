# Target/ Directory Budget Policy

Status: **ACTIVE** as of 2026-04-24
Severity: Critical (disk bloat has caused 100% full filesystem events)

## Overview

Rust `target/` directories accumulate build artifacts rapidly, especially in multi-agent parallel cargo builds. Disk exhaustion can block all development and CI/CD.

**Incident:** April 24, 2026 — `/repos` filled to 100% (65GB target/ bloat) during high-concurrency agent-orchestrator dispatches. This policy prevents recurrence.

## Automated Controls

### 1. target-pruner Binary

**Location:** `/repos/FocalPoint/tooling/target-pruner/`

Standalone Rust binary for managing target/ directories.

#### Usage

```bash
# Dry-run: list top 10 target/ dirs by size
./FocalPoint/target/release/target-pruner --dry-run

# Prune targets older than 7 days (keeps active repos)
./FocalPoint/target/release/target-pruner --prune \
  --keep-active-days 2 \
  --max-size 2G

# Verbose output
./FocalPoint/target/release/target-pruner --prune --verbose
```

#### Options

| Flag | Default | Purpose |
|------|---------|---------|
| `--dry-run` | true | List targets without removal |
| `--prune` | false | Remove targets eligible for pruning |
| `--keep-active-days` | 2 | Skip repos with commits <N days old |
| `--max-size` | 2G | Per-repo target/ max size |
| `--top-n` | 10 | Number of entries to display |
| `--config` | `~/.config/phenotype/target-budget.toml` | Config file path |
| `--verbose` | false | Enable verbose logging |

### 2. Pre-Dispatch Disk Check

**Location:** `FocalPoint/tooling/agent-orchestrator/src/disk_check.rs`

The agent-orchestrator validates disk budget before dispatching >3 parallel agents:

```rust
// In cmd_lanes_dispatch()
disk_check::validate_pre_dispatch(1)?;  // 20GB free required for concurrent cargo
```

**Behavior:**
- If <20GB free and spawning >3 parallel agents: dispatch **refuses** with clear error
- Error message includes recovery command: `target-pruner --prune`
- Prevents multi-agent cargo builds from filling disk

### 3. Weekly Pruning Workflow

**Location:** `.github/workflows/disk-budget-weekly.yml`

Scheduled GitHub Actions workflow (runs on self-hosted runners only):

```bash
# Weekly (Sunday 02:00 UTC)
# Runs in dry-run mode by default
./target-pruner --dry-run --top-n 15

# Can be triggered manually with prune=true to execute removal
```

**Constraints:**
- **Self-hosted runners only** — avoids GitHub Actions billing
- Scheduled for off-peak hours (02:00 UTC)
- Dry-run by default; manual dispatch required for pruning

## Configuration

### Default Config

File: `~/.config/phenotype/target-budget.toml`

```toml
[global]
max_size_bytes = 2147483648  # 2 GB per repo
prune_days = 7               # Eligible after 7 days unused
keep_active_days = 2         # Skip repos with recent commits

[repos.AgilePlus]
max_size_bytes = 3221225472  # 3 GB (high-frequency builds)
keep_active_days = 1         # Very active

[repos.PhenoAgent]
max_size_bytes = 1610612736  # 1.5 GB
prune_days = 5
```

### Per-Repo Overrides

Add entries to `[repos.<name>]` to customize budget for specific repos:

```toml
[repos."my-repo"]
max_size_bytes = 536870912      # 512 MB for lean repos
keep_active_days = 3             # Grace period
exclude = false                  # Set true to skip entirely
```

## Emergency Recovery

If `/repos` reaches >95% full:

### 1. Immediate: Force Prune

```bash
/repos/FocalPoint/target/release/target-pruner \
  --prune \
  --keep-active-days 1 \
  --max-size 1G \
  --verbose
```

### 2. Check Status

```bash
df -h /repos
ls -lahS /repos/*/target/ | head -20
```

### 3. Manual Cleanup (if needed)

```bash
# Remove oldest target/ in a specific repo (one-time manual)
find /repos/MyProject/target -type d -name "release" -o -name "debug" | \
  xargs du -sh | sort -h | tail -5 | awk '{print $2}' | \
  xargs rm -rf
```

### 4. Notify Agents

If disk is still full after pruning:

```bash
# Kill concurrent cargo/build processes
pkill -f "cargo build"
pkill -f "cargo test"

# Wait 30s for cleanup
sleep 30

# Re-check
df -h /repos
```

## Pre-Dispatch Validation Rules

**When:**
- Agent orchestrator dispatches agents
- Spawning >3 parallel agents (high cargo concurrency)

**Check:**
1. Read `/repos` filesystem free space
2. If <20GB available: **REFUSE dispatch** with error
3. Error suggests: `target-pruner --prune --keep-active-days 1`

**Rationale:**
- 3 parallel `cargo build` processes can consume 5-10GB in minutes
- 20GB safety margin prevents mid-build exhaustion
- Conservative threshold; can be tuned per environment

## Tuning & Thresholds

### Conservative (tight disk)
```bash
target-pruner --prune --keep-active-days 1 --max-size 512M
```

### Balanced (default)
```bash
target-pruner --prune --keep-active-days 2 --max-size 2G
```

### Generous (large disk)
```bash
target-pruner --prune --keep-active-days 5 --max-size 5G
```

## Monitoring

### Manual Checks

```bash
# Current top targets
target-pruner --dry-run --top-n 20

# Repo-specific check
du -sh /repos/MyProject/target

# Free space trend (run weekly)
df -h /repos | tail -1
```

### Metrics to Watch

- Total `/repos` free space (alert if <20GB)
- Largest target/ directory (alert if >3GB)
- Frequency of prune runs (should be weekly or less)

## References

- **Binary:** `/repos/FocalPoint/tooling/target-pruner/`
- **Agent Orchestrator:** `/repos/FocalPoint/tooling/agent-orchestrator/src/disk_check.rs`
- **Workflow:** `/.github/workflows/disk-budget-weekly.yml`
- **Incident Report:** (April 24, 2026 — 100% disk full event)

## Glossary

| Term | Definition |
|------|-----------|
| **target/** | Rust build artifact directory; contains binaries, deps, incremental builds |
| **Eligibility** | A target/ is eligible for pruning if: (a) last access >7 days ago AND (b) repo has no commits <2 days old |
| **Grace Period** | `keep_active_days` parameter; skips repos in active development |
| **Budget** | Maximum size per repo's target/ directory; hard cap enforced by pruner |
| **Dry-run** | List targets without removal; safe for audit and planning |

---

**Last Updated:** 2026-04-24
**Owner:** DevOps / Platform Engineering
**Escalation:** Fill /repos to >95% → run recovery playbook; if persists → investigate concurrent build queues
