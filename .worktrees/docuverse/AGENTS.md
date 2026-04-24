# AGENTS.md — Docuverse

Extends shelf-level AGENTS.md rules for Docuverse.

## Project Identity

- **Name**: Docuverse
- **Description**: Documentation engine with multiple output formats and plugin system
- **Language**: Go

## Project-Specific Rules

### Test-First Mandate

- **For NEW modules**: test file MUST exist before implementation file
- **For BUG FIXES**: failing test MUST be written before the fix
- **For REFACTORS**: existing tests must pass before AND after

### Quality Gates

All PRs must pass:
- `go test ./...`
- `golangci-lint run`
- `gofmt -s`

### Commit Messages

Format: `<type>(<scope>): <description>`

Types: `feat`, `fix`, `chore`, `docs`, `refactor`, `test`, `ci`

### File Organization

```
docslib.go           # Main library
parser.go            # Documentation parsing
renderer.go          # Output rendering
```

## Testing Requirements

- Unit tests for all public functions
- Integration tests for renderers
- Minimum 80% code coverage
