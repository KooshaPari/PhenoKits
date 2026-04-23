# Phenotype Release Branch Governance

## Overview

This document establishes the release branch governance policy for all Phenotype ecosystem repositories.

**IMPORTANT**: All work MUST be managed through AgilePlus. See [AgilePlus Mandate](../governance/agileplus-mandate.md) for details.

## AgilePlus Integration

Release governance work is tracked in AgilePlus:
- Feature specs for release automation: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/kitty-specs/`
- Release governance docs: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/docs/process/governance.md`
- Worklog: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/.work-audit/worklog.md`

All release-related features must be specified through AgilePlus before implementation.

## Branch Classification System

### Branch Types

| Branch Type | Pattern | Purpose | Protection |
|-------------|---------|---------|------------|
| **Canonical Main** | `main` | Primary integration branch, always releasable | Required |
| **Release Channels** | `release/{channel}` | Predefined release cadences | Required |
| **Feature Branches** | `{type}/{description}` | Development work | Optional |
| **Hotfix Branches** | `hotfix/{issue}` | Emergency fixes | Required |

### Release Channels

| Channel | Branch | Purpose | Merge Policy |
|---------|--------|---------|--------------|
| **Canary** | `release/canary` | Bleeding edge, daily builds | Fast-forward only |
| **Beta** | `release/beta` | Pre-release testing | Requires CI green |
| **Stable** | `release/stable` | Production-ready | Requires CI + manual approval |

### Branch Naming Conventions

```
main                           # Canonical main
release/canary                 # Canary channel
release/beta                   # Beta channel
release/stable                 # Stable channel
release/{major}.{minor}.x     # Version-specific releases

# Feature branches
feat/{description}
fix/{description}
chore/{description}
docs/{description}
refactor/{description}
test/{description}
perf/{description}

# Hotfix branches
hotfix/{issue-id}
hotfix/{description}

# Agent-generated
agent/{agent-name}/{task}
garden/{description}
```

## Merge Policies

### Release Channel Promotion

1. **Canary** receives merges from `main` daily
2. **Beta** receives promotion from `canary` weekly (if stable)
3. **Stable** receives promotion from `beta` monthly (if stable)

### Merge Restrictions

| Source | Target | Allowed |
|--------|--------|---------|
| `feature/*` | `main` | Yes (via PR, CI required) |
| `main` | `release/canary` | Yes (daily automation) |
| `release/canary` | `release/beta` | Yes (weekly, gate required) |
| `release/beta` | `release/stable` | Yes (monthly, approval required) |
| `hotfix/*` | `release/stable` | Yes (emergency, bypass normal) |
| `hotfix/*` | `main` | Yes (emergency, then forward-merge) |

## Versioning Strategy

### Semantic Versioning

```
{major}.{minor}.{patch}-{channel}.{build}
Example: 1.2.3-stable.47
```

### Release Tags

```
v{major}.{minor}.{patch}
v{major}.{minor}.{patch}-beta.{n}
v{major}.{minor}.{patch}-stable.{n}
```

## Implementation

### Branch Protection Rules

Each repository MUST have:

1. `main` protected with:
   - Require PR reviews
   - Require CI passing
   - Require linear history (optional)
   - Dismiss stale reviews

2. `release/*` protected with:
   - Require PR reviews (1 minimum)
   - Require CI passing
   - No force push
   - No deletion

### Automation Workflows

```yaml
# Canary promotion (daily at 00:00 UTC)
cron: "0 0 * * *"
- Check main CI status
- Fast-forward merge main → release/canary
- Tag with canary timestamp

# Beta promotion (weekly on Monday)
cron: "0 0 * * 1"
- Check canary stability metrics
- Check all tests pass
- Fast-forward merge release/canary → release/beta
- Tag with beta version

# Stable promotion (monthly on 1st)
cron: "0 0 1 * *"
- Check beta stability metrics
- Manual approval required
- Fast-forward merge release/beta → release/stable
- Create version tag
```

## Repository Checklist

- [ ] `main` branch exists and is protected
- [ ] `release/canary` branch exists
- [ ] `release/beta` branch exists
- [ ] `release/stable` branch exists
- [ ] Branch protection rules configured
- [ ] Automation workflows in place

## Exceptions

Emergency hotfixes may bypass these policies with:

1. Explicit approval from repository owner
2. Post-merge review within 24 hours
3. Forward-merge to canonical branches
