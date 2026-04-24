// =============================================================================
// AGENTS.md — Portalis (portkey)
// =============================================================================
// Project-specific agent rules for the Portalis LLM Gateway library.
//
// =============================================================================

# AGENTS.md — Portalis (portkey)

Extends: `phenotype-governance/AGENTS.md`

---

## Project Identity

| Field | Value |
|-------|-------|
| **Name** | Portalis (portkey) |
| **Description** | LLM Gateway abstractions for multi-provider support (OpenAI, Anthropic, Ollama, etc.) |
| **Language** | Python |
| **Location** | `/Users/kooshapari/CodeProjects/Phenotype/repos/Portalis` |
| **Language Stack** | Python 3.11+ (PEP 484 type hints) |
| **Published** | Internal (pip installable) |

---

## AgilePlus Integration

All work MUST be tracked in AgilePlus:

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/.agileplus
agileplus <command>
```

**Requirements**:

1. Check for AgilePlus spec before implementing
2. Create spec for new work: `agileplus specify --title "<feature>"`
3. Update work package status as work progresses
4. No code without corresponding AgilePlus spec

---

## Agent Expectations

### Primary Agent Selection

| Task type | Primary agent |
|-----------|---------------|
| Feature implementation | Forge |
| Code review | Muse |
| Bug investigation | Sage |
| Testing/runtime | Helios |
| Cross-project architecture | Forge + Sage |
| Research/investigation | Sage |
| Documentation | Forge (with Muse review) |

### Session Naming

Format: `<project>:<brief-task-description>`

Good: `Portalis:auth-refactor`, `Portalis:provider-impl`
Bad: `fix`, `implementation`, `agent work`

---

## Quality Standards

### Code Quality Mandate

- **All linters must pass**: `ruff check src/ tests/`
- **All type checks must pass**: `mypy src/`
- **All tests must pass**: `pytest tests/`
- **No AI slop**: Avoid placeholder TODOs, lorem ipsum, generic comments
- **Backwards incompatibility**: No shims, full migrations, clean breaks

### Test-First Mandate

- **For NEW modules**: test file MUST exist before implementation file
- **For BUG FIXES**: failing test MUST be written before the fix
- **For REFACTORS**: existing tests must pass before AND after

### FR Traceability

All tests MUST reference a Functional Requirement (FR):

```python
# Traces to: FR-XXX-NNN
@pytest.mark.frdom001
def test_feature_name():
    # Test body
```

---

## Project-Specific Rules

### Build Commands

```bash
# Run all quality checks
pytest tests/
mypy src/
ruff check src/ tests/

# Auto-format code
ruff format src/ tests/

# Install with all dependencies
pip install -e ".[all]"
```

### Test Commands

```bash
# Run all tests
pytest tests/

# Run specific test file
pytest tests/unit/test_models.py

# Run with coverage
pytest --cov=src --cov-report=term-missing

# Run with verbose output
pytest -v tests/
```

### Documentation

```bash
# Generate API docs
pdoc src/portkey --output-dir docs/api

# Serve docs locally
python -m http.server 8000 --directory docs/
```

---

## Tool Chain

This project uses the following tools:

| Tool | Purpose | Config |
|------|---------|--------|
| ruff | Python linter/formatter | `pyproject.toml` [tool.ruff] |
| mypy | Type checker | `pyproject.toml` [tool.mypy] |
| pytest | Test runner | `pyproject.toml` [tool.pytest] |
| pytest-cov | Coverage | `codecov.yml` |
| bandit | Security scanning | `pyproject.toml` [tool.bandit] |
| pip-audit | Dependency vulnerabilities | (CI) |
| vulture | Dead code detection | `pyproject.toml` [tool.vulture] |
| radon | Code metrics | `pyproject.toml` [tool.radon] |

---

## Code Review Protocol

### PR Requirements

1. One logical change per PR
2. PR title matches commit format
3. Description explains WHY, not just WHAT
4. Always link related issues
5. All tests must pass before merge

### Commit Messages

Format: `<type>(<scope>): <description>`

Types: `feat`, `fix`, `chore`, `docs`, `refactor`, `test`, `ci`

Good: `feat(portkey): add token refresh with exponential backoff`
Bad: `fix stuff`, `update`, `WIP`

---

## Error Handling

### Rate Limits (429)

When encountering API rate limits:

1. Stop immediately — do not retry
2. Report the limit type to user
3. Wait for user instruction

### Crashes

If an agent crashes mid-task:

1. Save state to conversation
2. Report what was in progress
3. Wait for user or another agent to resume

---

## Onboarding

When starting work on this project:

1. Run `pytest tests/` to verify tests pass (79 tests)
2. Run `mypy src/` to verify type checking
3. Run `ruff check src/ tests/` to verify linting
4. Read `CLAUDE.md` for project-specific context
5. Read `README.md` for architecture overview

---

## Architecture Reference

See `README.md` for detailed architecture documentation:

- Domain Layer: `src/portkey/domain/` — Pure business logic
- Application Layer: `src/portkey/application/` — Ports and use cases
- Infrastructure Layer: `src/portkey/infrastructure/` — Adapters

---

## Governance Reference

- Base rules: `platforms/thegent/governance/AGENTS.base.md`
- Tool registry: `phenotype-governance/configs/tools.toml`
- CI templates: `phenotype-governance/.github/workflows/`
- Pre-commit hooks: `.pre-commit-config.yaml`
