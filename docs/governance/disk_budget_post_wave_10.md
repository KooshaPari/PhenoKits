# Disk Budget Post-Wave 10 Policy

**Document**: Lessons learned and governance update from Wave 10 disk-full incident (2026-04-24).

## Context

Wave 10 agent swarm (50+ concurrent agents) triggered out-of-space error on `/System/Volumes/Data`:
- **Free space collapse**: 10GB → 0GB in ~4 hours
- **Root cause**: Rust `target/` directories bloat (multi-crate workspace compiles across 10+ concurrent worktrees)
- **Impact**: Blocked all agents, required manual disk triage, recovery commits pending

## Problem Summary

1. **Cargo build artifacts**: Rust workspaces accumulate `target/` subdirs at all levels:
   - Workspace root: `repos/target/` (primary, ~2-5GB)
   - Per-worktree: `repos/.worktrees/*/target/` (clones, ~1GB each × N worktrees)
   - Nested crates: `repos/*/crates/*/target/` (rare, 100-500MB)

2. **Multi-agent concurrency**: 10-50 agents building in parallel = 10-50 `target/` directories simultaneously

3. **No proactive cleanup**: Cargo does not garbage-collect old targets; `cargo clean` is per-project, not org-wide

4. **APFS limitation**: `~/.Trash` does not reclaim space until emptied explicitly (hidden until `rm -rf`)

## Solution: `target-pruner` Tooling

### Tool: `tooling/target-pruner.rs`

A Rust binary (not shell) that:
- Scans the repos tree for `target/` dirs >500MB (configurable)
- Reports size, entry count, path
- Supports `--dry-run` (safe preview) and `--prune` (delete)
- Integrates with CI gates and pre-dispatch checks

### Usage

```bash
# Dry-run: report oversized targets without deleting
cargo run --release --bin target-pruner -- --mode dry-run --threshold 500

# Prune: delete all targets >500MB
cargo run --release --bin target-pruner -- --mode prune --threshold 500 --verbose

# Custom threshold
cargo run --release --bin target-pruner -- \
  --path /Users/kooshapari/CodeProjects/Phenotype/repos \
  --threshold 300 \
  --mode dry-run
```

### Build & Install

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos
cargo build --release --bin target-pruner
# Binary: target/release/target-pruner
```

## Governance Changes

### 1. Pre-Dispatch Health Check

Before launching agent swarm (≥5 agents), verify disk space:

```bash
df -h /System/Volumes/Data | tail -1
# Must show ≥15GB free. If <10GB, run:
./target-pruner --path . --mode prune --threshold 500
```

### 2. Worktree Cleanup Protocol

When creating new worktrees, clean old ones:

```bash
# List all worktrees
git worktree list

# Remove unused worktree (deletes target/)
git worktree remove .worktrees/old-topic
```

### 3. CI/CD Disk Checks

Add health check to quality-gate.sh:

```bash
FREE_GB=$(df -B1 /System/Volumes/Data | tail -1 | awk '{print int($4 / 1e9)}')
if [ "$FREE_GB" -lt 10 ]; then
    echo "Disk space critical ($FREE_GB GB). Run target-pruner." >&2
    exit 1
fi
```

### 4. Cargo Configuration

Add to `~/.cargo/config.toml` to limit artifact size:

```toml
[build]
jobs = 4  # Reduce from default (physical cores) to 4
```

Add to `Cargo.toml` workspace root:

```toml
# .cargo/config.local.toml (per-session override)
[build]
jobs = 2  # Limit parallelism in multi-agent environments
```

### 5. GitHub Actions Billing Note

Disk bloat also affects Actions:
- Standard GitHub runner: ~14GB disk
- If any workflow tries to build all workspaces concurrently, disk fills instantly
- **Do NOT use matrix strategies for Rust workspace tests** without cleanup step

Recommended: Add cleanup step in workflows:

```yaml
- name: Cleanup
  run: rm -rf target/ .worktrees/*/target/
```

## Monitoring

### Automated Checks (to implement)

| Check | Frequency | Threshold | Action |
|-------|-----------|-----------|--------|
| Disk free | On agent dispatch | <10GB | Pause, run pruner |
| Target dirs | Daily (7 AM UTC) | >1GB/dir | Log size report |
| Worktree age | Weekly | >2 weeks unused | Alert user |

### Manual Audit

```bash
# Find all target/ dirs, sorted by size
find /Users/kooshapari/CodeProjects/Phenotype/repos \
  -type d -name target -exec du -sh {} \; | sort -rh | head -20
```

## Estimation: Disk Impact Post-Cleanup

Before wave 10 triage: ~140GB used (71GB free on 211GB disk)
- Occupied by: worktrees, target/, archives, workspace nodes

After target-pruner full run (--prune --threshold 500):
- Estimated reclaim: 25-40GB (20-30 oversized target dirs)
- Free space post-cleanup: ~95-115GB

**Sustainable disk model**: Maintain >30GB free at all times; run pruner weekly.

## APFS Special Cases

- **`~/.Trash` trap**: Files moved to Trash are not freed until bin is emptied.
  - Workaround: Use `rm -rf` directly (permanent, immediate)
  - Risk: No undo; verify with `--dry-run` first

- **Snapshot overhead**: APFS snapshots can consume 5-10% of disk (invisible in `df`).
  - Check: `tmutil listlocalsnapshots /`
  - Remove: `tmutil deletelocalsnapshots <date>`

## References

- Cargo build cache: https://doc.rust-lang.org/cargo/guide/build-cache.html
- APFS space management: https://support.apple.com/guide/disk-utility/
- Wave 10 incident: Recovery commit (`chore(disk): rescue commits...`)

## Rollout

1. ✅ Deploy `target-pruner.rs` to `repos/tooling/`
2. ✅ Document governance (this file)
3. **TODO**: Add disk-check hook to quality-gate.sh
4. **TODO**: Implement weekly monitoring routine (agent-scheduled)
5. **TODO**: Update CLAUDE.md worktree rules with cleanup checklist
