# Planify

Planning and execution engine for Phenotype, providing AI-driven project planning, dependency analysis, and milestone tracking with integrated governance validation.

## Overview

Planify is a comprehensive planning system that transforms high-level goals into executable project plans with explicit task dependencies, effort estimates, and risk assessment. It integrates with AgilePlus for work tracking and provides planning primitives for agents to autonomously generate and execute project breakdowns.

## Technology Stack

- **Language**: Python 3.10+ (async-native)
- **Frameworks**: FastAPI, Pydantic
- **Key Dependencies**: networkx, pandas, dateutil
- **Persistence**: PostgreSQL, Redis
- **Architecture**: Service-oriented with DAG engine

## Key Features

- AI-assisted project planning and decomposition
- Dependency graph visualization and analysis
- Effort estimation using historical data
- Critical path analysis and scheduling
- Risk assessment and mitigation planning
- Integration with AgilePlus specs and tracking
- Milestone and goal management
- Progress tracking and burndown visualization
- Governance policy validation
- Export to multiple formats (CSV, JSON, Markdown)

## Quick Start

```bash
# Clone repository
git clone https://github.com/KooshaPari/Planify.git
cd Planify

# Review governance
cat CLAUDE.md

# Install dependencies
python -m pip install -e ".[dev]"

# Run tests
pytest tests/

# Validate governance
python -m planify.validators.governance --path .

# Start planning server
uvicorn planify.app:app --reload

# Test API
curl http://localhost:8000/api/plans
```

## Project Structure

```
Planify/
├── src/
│   ├── api/
│   │   ├── routes/
│   │   │   ├── plans.py          # Plan management
│   │   │   ├── tasks.py          # Task operations
│   │   │   ├── milestones.py     # Milestone tracking
│   │   │   └── analysis.py       # Plan analysis
│   │   └── middleware/           # API middleware
│   ├── services/
│   │   ├── planner.py            # Planning engine
│   │   ├── estimator.py          # Effort estimation
│   │   ├── scheduler.py          # Task scheduling
│   │   └── analyzer.py           # Dependency analysis
│   ├── models/
│   │   ├── plan.py               # Plan models
│   │   ├── task.py               # Task models
│   │   └── milestone.py          # Milestone models
│   ├── validators/
│   │   ├── governance.py         # Governance validation
│   │   └── dependencies.py       # Dependency checking
│   ├── storage/
│   │   ├── postgres.py           # PostgreSQL adapter
│   │   └── cache.py              # Redis caching
│   └── main.py                   # Application entrypoint
├── tests/
│   ├── unit/                     # Unit tests
│   └── integration/              # Integration tests
├── docs/
│   ├── ARCHITECTURE.md           # System design
│   ├── PLANNER_ALGORITHM.md      # Planning algorithms
│   └── EXAMPLES.md               # Usage examples
└── pyproject.toml                # Python packaging
```

## Related Phenotype Projects

- **[AgilePlus](../AgilePlus)** — Work tracking and specs
- **[phenotype-shared](../phenotype-shared)** — Shared infrastructure
- **[thegent](../thegent)** — Execution engine

## Governance & Documentation

- **CLAUDE.md** — Agent operating instructions
- **PRD.md** — Product requirements
- **ARCHITECTURE.md** — System architecture
- **License**: Apache 2.0

---

**Status**: Active development  
**Maintained by**: Phenotype Org  
**Last Updated**: 2026-04-24
