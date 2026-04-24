# AGENTS.md — worklogs

**Category:** GOVERNANCE

## Project Overview

- **Name**: worklogs
- **Description**: Centralized worklog repository for cross-project documentation and research
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/worklogs`
- **Language Stack**: Markdown documentation
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to worklogs
cd /Users/kooshapari/CodeProjects/Phenotype/repos/worklogs

# View index
cat README.md

# View specific worklog
cat ARCHITECTURE.md
cat RESEARCH.md

# Aggregate worklogs
./aggregate.sh all
```

## Architecture

```
worklogs/
├── aggregate.sh              # Aggregation script
├── AGENT_ONBOARDING.md       # Agent onboarding guide
├── ARCHITECTURE.md           # Architecture decisions
├── DEPENDENCIES.md           # Dependency tracking
├── DUPLICATION.md            # Duplication findings
├── GOVERNANCE.md             # Governance notes
├── INTEGRATION.md            # Integration notes
├── PERFORMANCE.md            # Performance findings
├── README.md                 # Worklog index
├── RECOVERY_VERIFICATION.md  # Recovery documentation
├── RESEARCH.md               # Research findings
├── REPO_CONSOLIDATION_COMPLETE.md  # Consolidation notes
├── WORKFLOW_ROLLOUT_PRs.md   # Workflow documentation
└── PHENOSDK_MIGRATION_CLEANUP_COMPLETE.md  # Migration notes
```

## Quality Standards

### Worklog Standards
- UTF-8 encoding only
- **Line length**: 100 characters
- Clear date/project attribution
- Use project tags: `[AgilePlus]`, `[thegent]`, `[heliosCLI]`, etc.

### Categories
| Category | File | Purpose |
|----------|------|---------|
| ARCHITECTURE | `ARCHITECTURE.md` | ADRs, library extraction |
| DUPLICATION | `DUPLICATION.md` | Cross-project duplication |
| DEPENDENCIES | `DEPENDENCIES.md` | External deps, forks |
| INTEGRATION | `INTEGRATION.md` | External integrations |
| PERFORMANCE | `PERFORMANCE.md` | Optimization findings |
| RESEARCH | `RESEARCH.md` | Starred repo analysis |
| GOVERNANCE | `GOVERNANCE.md` | Policy, evidence |

## Git Workflow

### Branch Naming
Format: `worklogs/<type>/<description>`

Examples:
- `worklogs/feat/architecture-note`
- `worklogs/research/new-library`

### Commit Format
```
<type>(worklogs): <description>

Examples:
- docs(worklogs): add research on X
- chore(worklogs): aggregate logs
```

## File Structure

```
worklogs/
├── README.md                 # Index and guide
├── AGENT_ONBOARDING.md       # Onboarding
├── aggregate.sh              # Aggregation tool
├── [Category files]
│   ├── ARCHITECTURE.md
│   ├── DEPENDENCIES.md
│   ├── DUPLICATION.md
│   ├── GOVERNANCE.md
│   ├── INTEGRATION.md
│   ├── PERFORMANCE.md
│   └── RESEARCH.md
└── [Specific worklogs]
```

## CLI Commands

```bash
# View index
cat README.md

# Aggregate by project
./aggregate.sh project

# Aggregate by priority
./aggregate.sh priority

# Aggregate all
./aggregate.sh all

# Create new entry
# Edit appropriate [CATEGORY].md file
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Where to log | Check README.md category guide |
| Encoding issues | Use UTF-8, no smart quotes |
| Aggregation fails | Check aggregate.sh permissions |

## When to Write Worklogs

Write for:
- Research completions
- Decisions made
- Issues found (duplication, performance)
- Work completions
- Planning (fork candidates, migration plans)

## Dependencies

- **AgilePlus**: Work tracking integration
- **All projects**: Source of worklog content

## Agent Notes

When working in worklogs:
1. This is the centralized worklog repository
2. All agents should contribute relevant findings
3. Use proper project tags
4. Run aggregate.sh to generate summaries
5. See AGENT_ONBOARDING.md for detailed guidance
