# AGENTS.md — phenotype-colab-extensions

<!-- Base: platforms/thegent/governance/AGENTS.base.md -->

## Project Identity & Work Management

### Project Overview

- **Name**: phenotype-colab-extensions
- **Description**: Extensions and specs for Phenotype's colab fork (upstream: blackboardsh/colab)
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-colab-extensions`
- **Language Stack**: TypeScript, Task (go-task)
- **Published**: Internal (Phenotype ecosystem)

### AgilePlus Integration

All work MUST be tracked in AgilePlus:
- Reference: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus`
- CLI: `cd AgilePlus && agileplus <command>`
- Specs: `AgilePlus/kitty-specs/<feature-id>/`
- Worklog: `AgilePlus/.work-audit/worklog.md`

**Requirements**:
1. Check for AgilePlus spec before implementing
2. Create spec for new work: `agileplus specify --title "<feature>"`
3. Update work package status as work progresses
4. No code without corresponding AgilePlus spec

---

## Repository Mental Model

### Project Structure

```
phenotype-colab-extensions/
├── src/
│   ├── specs/               # PRD, FR, ADR for colab integration
│   ├── webflow-plugin/      # Webflow integration plugin
│   ├── workflows/           # CI/CD workflow definitions
│   └── Taskfile.yml         # Automation entry point
├── ADR.md                   # Architecture decision records
├── CHARTER.md               # Project charter
├── PLAN.md                  # Project plan
├── PRD.md                   # Product requirements
├── FUNCTIONAL_REQUIREMENTS.md # Functional requirements
├── README.md                # Project overview
├── SECURITY_AUDIT.md        # Security review
└── UPSTREAM_SYNC.md         # Upstream synchronization guide
```

### Extension Boundaries

- **Never modify**: `app/`, `src/main/`, `src/renderers/`, `src/pty/` (upstream directories)
- **Extension code**: Must live in `src/` with clear separation
- **Sync validation**: CI checks for path conflicts before merge

### Key Constraints

- No intermingling with upstream code
- All extensions use colab plugin API exclusively
- Minimal entitlements in plugin manifests
- UTF-8 encoding for all text files

---

## Development Environment

### Prerequisites

- Node.js 18+
- Task CLI (go-task)
- Colab development environment

### Build Commands

```bash
# Build plugin
cd src && task build

# Type checking
cd src && task typecheck

# Run linting
cd src && task lint

# Sync upstream
cd src && task sync:upstream

# Validate no conflicts
cd src && task sync:check
```

### Testing

```bash
# Run all tests
cd src && task test

# Check extension isolation
cd src && task sync:check
```

---

## Session Documentation

All agents MUST maintain session documentation for research, decisions, and findings:

### Location

- Default: `docs/sessions/<session-id>/`

### Standard Session Structure

```
docs/sessions/<session-id>/
├── README.md           # Overview and context
├── 01_RESEARCH.md      # Findings and analysis
├── 02_PLAN.md          # Design and approach
├── 03_IMPLEMENTATION.md # Code changes and rationale
├── 04_VALIDATION.md    # Tests and verification
└── 05_KNOWN_ISSUES.md  # Blockers and follow-ups
```

---

## Quality Standards

### Code Quality Mandate

- **TypeScript strict mode**: `strict: true` in tsconfig
- **Linting**: Biome for formatting and linting
- **No AI slop**: Avoid placeholder TODOs, lorem ipsum, generic comments
- **Backwards incompatibility**: No shims, full migrations, clean breaks

### Test-First Mandate

- **For NEW plugins**: test directory MUST exist before implementation
- **For BUG FIXES**: failing test MUST be written before the fix
- **For REFACTORS**: existing tests must pass before AND after

### FR Traceability

All tests MUST reference a Functional Requirement (FR):

```typescript
/**
 * Traces to: FR-COLABEXT-NNN
 */
describe('webflow sync', () => {
  it('should sync DevLink components', () => {
    // Test body
  });
});
```

---

## Governance Reference

See thegent governance base for complete guidance on:

1. **Core Agent Expectations** — Autonomous operation, when to ask vs. decide
2. **Standard Operating Loop (SWE Autopilot)** — Review, Research, Plan, Execute, Size-Check, Test, Review & Polish, Repeat
3. **File Size & Modularity Mandate** — ≤500 line hard limit, decomposition patterns
4. **Research-First Development** — Codebase research, web research, documentation
5. **Branch Discipline** — Worktree usage, PR workflow, git best practices
6. **Child-Agent and Delegation Policy** — When to spawn subagents, parallel vs. sequential
7. **Tool Usage & CLI Priority** — CLI as primary interface, read-only tools first
8. **Naming Conventions** — Session naming, file naming, branch naming

Location: `platforms/thegent/governance/AGENTS.base.md`

---

## Upstream Sync Guidelines

### Sync Process

1. Read `UPSTREAM_SYNC.md` before any sync attempt
2. Run `task sync:check` to validate no conflicts
3. Run `task sync:upstream` to pull and apply
4. Verify CI passes after sync
5. Update `UPSTREAM_SYNC.md` with sync results

### What to Preserve

- All files in `src/`
- Plugin manifests
- CI workflow definitions
- Spec documents

### What to Never Modify

- Upstream source directories
- Core colab functionality
- Upstream plugin API

---

## Quick Reference Commands

```bash
# Validate extension isolation
cd src && task sync:check

# Run CI locally
cd src && task ci

# Build plugin
cd src && task build

# Type checking
cd src && task typecheck

# Run tests
cd src && task test
```

---

## UTF-8 Encoding

All markdown files must use UTF-8. Avoid smart quotes, em-dashes, and special characters.

```bash
# Validate encoding (in AgilePlus repo)
cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
agileplus validate-encoding --all --fix
```
