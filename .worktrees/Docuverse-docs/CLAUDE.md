# CLAUDE.md — Docuverse

## Project Identity

- **Name**: Docuverse
- **Description**: Documentation engine with multiple output formats and plugin system
- **Location**: `remote-clones/Docuverse/`
- **Language**: Go
- **License**: MIT

## Architecture

Hexagonal (Ports & Adapters):
- Parser trait is the port (interface)
- Renderer implementations are adapters (HTML, Markdown, PDF, etc.)
- Plugin system for extensibility

## Quick Commands

```bash
# Build
go build ./...

# Test
go test -v ./...

# Lint
golangci-lint run

# Format
gofmt -s -w .

# Documentation
godoc -http=:8080
```

## Key Files

| Path | Purpose |
|------|---------|
| `docslib.go` | Main library |
| `parser.go` | Documentation parsing |
| `renderer.go` | Output rendering |
| `plugin.go` | Plugin system |

## Testing Requirements

- Unit tests for all public functions
- Integration tests for renderers
- Plugin system tests
- Minimum 80% code coverage
