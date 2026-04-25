# Libs

**Phenotype Libraries Collection** — A collection of reusable, domain-agnostic libraries and utility modules providing cross-cutting concerns and foundational infrastructure for the Phenotype ecosystem.

## Overview

The `libs/` directory contains Python-based libraries organized by domain, offering shared utilities for configuration management, caching, resilience patterns, HTTP clients, LLM integration, and more. Each library is independently versioned and publishable to PyPI or private registries.

**Core Mission**: Provide battle-tested, well-maintained shared libraries eliminating duplication and enabling consistent patterns across all Phenotype projects.

## Technology Stack

- **Language**: Python 3.9+
- **Package Management**: poetry / pip
- **Testing**: pytest + pytest-cov
- **Distribution**: PyPI (selected libraries) or internal registries

## Library Categories

### Infrastructure & Foundation
| Library | Purpose | Status |
|---------|---------|--------|
| **pheno_config** | Configuration management with validation | Active |
| **pheno_http** | HTTP client with retry/circuit-breaker | Active |
| **pheno_cache** | Caching patterns and TTL management | Active |
| **pheno_retry** | Resilience patterns and exponential backoff | Active |

### Domain-Specific
| Library | Purpose | Status |
|---------|---------|--------|
| **pheno_agent** | Agent orchestration and dispatch | Active |
| **pheno_llm** | LLM integration and model routing | Active |
| **pheno_governance** | Governance utilities and compliance | Active |
| **pheno_adapters** | Port implementations and adapters | Active |

## Project Structure

```
libs/
├── pheno_adapters/           # Adapter implementations
│   ├── src/
│   ├── tests/
│   └── pyproject.toml
├── pheno_agent/              # Agent orchestration
│   ├── src/
│   ├── tests/
│   └── pyproject.toml
├── pheno_cache/              # Caching utilities
├── pheno_config/             # Configuration management
├── pheno_governance/         # Governance utilities
├── pheno_http/               # HTTP client
├── pheno_llm/                # LLM integration
├── pheno_retry/              # Resilience patterns
├── ADR.md                    # Architecture decisions
├── CHARTER.md                # Library charter
├── PRD.md                    # Product requirements
├── PLAN.md                   # Implementation plan
├── AGENTS.md                 # Agent instructions
└── README.md
```

## Quick Start

```bash
# Clone and navigate
cd libs/<library-name>

# Review governance
cat ../CLAUDE.md

# Install dependencies
pip install -e .

# Run tests
pytest tests/ -v --cov

# Quality checks
task quality
```

## Adding a New Library

1. **Create directory**:
   ```bash
   mkdir libs/pheno_mylib
   ```

2. **Initialize Python project**:
   ```bash
   poetry init --name pheno_mylib --path libs/pheno_mylib
   ```

3. **Create structure**:
   ```bash
   cd libs/pheno_mylib
   mkdir src/pheno_mylib tests
   touch src/pheno_mylib/__init__.py
   ```

4. **Follow conventions**:
   - Use Pydantic for data models
   - Use tenacity for retries
   - Include comprehensive docstrings
   - Write tests with FR traceability
   - Add to collection index

## Shared Conventions

### Error Handling
```python
from pheno_errors import PhnoError

class MyLibError(PhnoError):
    """Base error for my library."""
    pass
```

### Configuration
```python
from pheno_config import Config

class MyLibConfig(Config):
    timeout: int = 30
    max_retries: int = 3
```

### Testing
```python
import pytest

@pytest.mark.traces_to("FR-MYLIB-001")
def test_my_feature():
    assert True
```

## Governance

All libs follow Phenotype governance:
- **Parent governance**: `/repos/CLAUDE.md`
- **Global governance**: `~/.claude/CLAUDE.md`
- **Collection charter**: `CHARTER.md`
- **Decision records**: `ADR.md`

## Related Collections

- **packages/** — Shared packages and modules
- **apps/** — Applications consuming libraries
- **crates/** — Rust infrastructure (parallel to libs/)

## Publishing & Versioning

- **Versioning**: SemVer (major.minor.patch)
- **Changelog**: Per-library CHANGELOG.md
- **Publishing**: Internal PyPI or public PyPI (as appropriate)
- **Deprecation**: 2-major-version support window

## License

MIT — Part of Phenotype organization
