# AgilePlus Governance: Mandatory Project Management Framework

## Overview

This document establishes AgilePlus as the **mandatory project management framework** for all Phenotype organization repositories. All work must be tracked, specified, planned, and executed through AgilePlus.

## Mandate

**All code, features, and work items MUST be managed through AgilePlus.** This is not optional.

### Core Requirements

1. **No Code Without Spec**: Every feature, bugfix, or enhancement MUST have a corresponding AgilePlus spec before implementation begins.

2. **Spec-Plan-Tasks-Implement Chain**: All work follows this chain:
   ```
   specify -> research -> plan -> tasks -> implement -> review -> merge -> ship
   ```

3. **Work Package Tracking**: All implementation work MUST be broken into work packages (WP01, WP02, etc.) with explicit state tracking.

4. **Worklogs Required**: Each project MUST maintain worklogs covering:
   - **Past**: Completed features with lessons learned
   - **Present**: Active work packages with current status
   - **Future**: Planned features in backlog

## AgilePlus Reference

### Framework Location

- **Repository**: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus`
- **Database**: `.agileplus/agileplus.db`
- **Documentation**: `docs/`
- **Workflows**: `docs/workflow/` (specify, research, plan, implement, review, merge, ship)
- **Process**: `docs/process/` (constitution, governance, retrospective)
- **Concepts**: `docs/concepts/` (core philosophy and principles)

### CLI Commands

```bash
# Navigate to AgilePlus repo
cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus

# Specify a new feature
agileplus specify --title "<feature>" --description "<description>"

# List all features
agileplus list

# Show feature details
agileplus show <feature-id>

# Update work package status
agileplus status <feature-id> --wp <wp-id> --state <state>

# Validate encoding (required for all markdown files)
agileplus validate-encoding --feature <feature-id>
agileplus validate-encoding --feature <feature-id> --fix
```

## Feature Lifecycle

### State Machine

```
specified -> planned -> tasks_created -> in_progress -> for_review -> merged -> shipped
      |                        |
      v                        v
  superseded              blocked/stuck
```

### State Definitions

| State | Description |
|-------|-------------|
| `specified` | Feature spec created, requirements defined |
| `planned` | Architecture decisions made, plan documented |
| `tasks_created` | Work packages defined and assigned |
| `in_progress` | Active implementation underway |
| `for_review` | Implementation complete, awaiting review |
| `blocked/stuck` | Impediment identified, needs resolution |
| `merged` | Code merged to main branch |
| `shipped` | Feature released to users |
| `superseded` | Replaced by another feature |

## Documentation Requirements

### Required Artifacts Per Feature

| Artifact | File | Description |
|----------|------|-------------|
| Specification | `spec.md` | What the feature does and why |
| Plan | `plan.md` | Architecture and implementation approach |
| Work Packages | `tasks/WP*.md` | Individual work items |
| Metadata | `meta.json` | Status, dates, complexity, estimates |

### Directory Structure

```
<project>/
  kitty-specs/
    <feature-id>/
      spec.md          # Feature specification
      plan.md          # Architecture and plan
      meta.json        # Feature metadata
      tasks/
        WP01.md        # Work package 1
        WP02.md        # Work package 2
        ...
```

### Work Package Template

```markdown
# WP01: <Work Package Title>

## Status
[in_progress]

## Summary
Brief description of this work package.

## Tasks
- [ ] Task 1
- [ ] Task 2

## Dependencies
- WP00 (must complete first)

## Notes
Additional context and notes.

## Completion Criteria
- Criteria 1
- Criteria 2
```

## Worklog Requirements

### Per-Project Worklogs

Each project repository MUST maintain a `worklog.md` covering:

1. **Past Work** (Completed)
   - Feature summaries
   - Lessons learned
   - Key decisions made

2. **Present Work** (In Progress)
   - Active work packages
   - Current blockers
   - Next steps

3. **Future Work** (Planned)
   - Backlog items
   - Dependencies
   - Estimated timelines

### Cross-Project Worklogs

The AgilePlus repository maintains:
- `.work-audit/worklog.md` - Organization-wide work tracking
- `.work-audit/repo-manifest.json` - Repository state snapshots
- `.work-audit/raw-goals.json` - Goal tracking data

## Enforcement

### CI/Git Hooks

AgilePlus provides pre-commit validation:
```bash
# Validate feature encoding
agileplus validate-encoding --feature <id>

# Check all features
agileplus validate-encoding --all
```

### Agent Compliance

All AI agents MUST:
1. Check for existing AgilePlus spec before implementing
2. Create spec through AgilePlus CLI for new work
3. Update work package status as work progresses
4. Document all decisions in feature worklogs

### Review Requirements

Pull requests MUST include:
- Link to AgilePlus feature spec
- Work package status updates
- Evidence of encoding validation

## Anti-Patterns

### Prohibited

- ❌ Writing code without a corresponding AgilePlus spec
- ❌ Implementing features not tracked in AgilePlus
- ❌ Skipping the spec-plan-implement chain
- ❌ Using non-UTF-8 characters in documentation
- ❌ Committing code with encoding issues

### Required

- ✅ Every feature has `spec.md`, `plan.md`, and `tasks/WP*.md`
- ✅ All work packages have explicit status
- ✅ Encoding validated before commit
- ✅ Constitution followed for governance decisions

## Exception Process

Exceptions to this mandate require:
1. Explicit human approval comment
2. Written justification for the exception
3. Plan to bring work into compliance within 30 days

## References

- AgilePlus README: `AgilePlus/README.md`
- Constitution: `AgilePlus/docs/process/constitution.md`
- Specify Workflow: `AgilePlus/docs/workflow/specify.md`
- Plan Workflow: `AgilePlus/docs/workflow/plan.md`
- Implement Workflow: `AgilePlus/docs/workflow/implement.md`
