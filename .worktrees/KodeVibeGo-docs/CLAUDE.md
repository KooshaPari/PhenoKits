# CLAUDE.md

## Agent Instructions

- Review AGENTS.md for comprehensive project context
- Follow Go-specific conventions (gofmt, golint, go vet)
- Prefer small, focused changes with clear commit messages
- Update session docs in `docs/sessions/` for significant work

## Project Context

- **Name**: KodeVibeGo
- **Language**: Go
- **Type**: Go application/service
- **Purpose**: Part of the Phenotype ecosystem

## Key Files

- `go.mod` - Module definition
- `go.sum` - Dependency checksums
- `main.go` or `cmd/` - Application entrypoint
- `*.go` - Go source files

## Development Commands

```bash
# Build
go build ./...

# Test
go test ./...
go test -v ./...  # Verbose
go test -race ./...  # Race detection

# Lint
go vet ./...
golangci-lint run  # If configured

# Format
gofmt -w .
go fmt ./...

# Dependencies
go mod tidy
go mod download
```

## Go Conventions

- Follow standard Go formatting (`gofmt`)
- Use meaningful package names
- Keep functions focused and small
- Handle errors explicitly
- Document exported functions with comments
- Use `gofmt`, `go vet`, and `golint` before committing

## Phenotype Org Rules

- UTF-8 encoding only
- Worktree discipline: canonical repo stays on `main`
- No agent directories committed (`.claude/`, `.codex/`, `.cursor/`)
- Run tests before committing
- Update AGENTS.md if patterns change
