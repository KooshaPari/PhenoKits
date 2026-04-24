# AGENTS.md — frontend

## Project Overview

- **Name**: frontend
- **Description**: Frontend utilities and components for Phenotype ecosystem - Go-based UI with TypeScript integrations
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/frontend`
- **Language Stack**: Go (primary), TypeScript (integrations)
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to frontend
cd /Users/kooshapari/CodeProjects/Phenotype/repos/frontend

# Go components
go build ./...
go test ./...

# TypeScript components (if needed)
npm install  # if package.json exists
```

## Architecture

```
frontend/
├── api_client.go           # Go API client utilities
├── form.go                 # Form handling
├── sentry-client.ts        # Sentry error tracking client
├── sentry.config.json      # Sentry configuration
├── SOTA.md                 # State of the Art documentation
├── state.go                # State management
├── ui.go                   # UI components/helpers
└── webpack.sentry.config.js # Webpack Sentry integration
```

## Quality Standards

### Go Standards
- **Line length**: 100 characters
- **Formatter**: `gofmt` or `goimports`
- **Linter**: `golangci-lint` (if configured)
- **Tests**: `go test ./...`

### TypeScript Standards
- **Formatter**: `prettier`
- **Linter**: `eslint`
- **Type checker**: `tsc --noEmit`

## Git Workflow

### Branch Naming
Format: `frontend/<type>/<description>`

Types: `feat`, `fix`, `chore`, `docs`, `refactor`, `test`

Examples:
- `frontend/feat/api-client-v2`
- `frontend/fix/sentry-integration`

### Commit Format
```
<type>(frontend): <description>

Examples:
- feat(frontend): add form validation helpers
- fix(frontend): resolve sentry client initialization
```

## File Structure

```
frontend/
├── *.go                    # Go source files
├── *.ts                    # TypeScript files
├── *.json                  # Configuration files
├── *.js                    # JavaScript configs
└── SOTA.md                 # Research/documentation
```

## CLI Commands

```bash
# Go operations
go build ./...
go test ./...
go fmt ./...

# Linting (if golangci-lint available)
golangci-lint run

# TypeScript (if applicable)
npx tsc --noEmit
npx eslint .
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Go build fails | Check `go.mod` dependencies |
| Sentry issues | Verify `sentry.config.json` |
| TypeScript errors | Check for missing type definitions |

## Dependencies

- **Go**: Core implementation
- **TypeScript**: Sentry client integration
- **Sentry**: Error tracking service

## Agent Notes

When working in frontend:
1. This is a shared frontend utilities package
2. Go files provide backend-for-frontend patterns
3. TypeScript files are for third-party integrations (Sentry)
4. Check SOTA.md for architectural decisions
