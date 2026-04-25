# Packages

**Phenotype Packages Collection** — A collection of reusable npm/bun packages, design system components, and shared modules providing framework-agnostic functionality across the Phenotype JavaScript/TypeScript ecosystem.

## Overview

The `packages/` directory contains npm packages organized by function and domain, enabling code reuse across frontend applications, backend services, and monorepos. Each package is independently versioned and published to npm or private registries.

**Core Mission**: Provide production-grade, well-maintained shared packages eliminating duplication and enabling consistent patterns across all Phenotype TypeScript/JavaScript projects.

## Technology Stack

- **Language**: TypeScript (primary), JavaScript
- **Package Manager**: bun (preferred), npm compatible
- **Testing**: vitest / jest
- **Build**: esbuild / vite
- **Publishing**: npm (public and private)

## Package Categories

### Design System & UI
| Package | Purpose | Status |
|---------|---------|--------|
| **@phenotype/design** | Design tokens, CSS, VitePress theme | Archived |
| **@phenotype/components** | React/Svelte component library | Planned |

### Utilities & Infrastructure
| Package | Purpose | Status |
|---------|---------|--------|
| **@phenotype/types** | Shared TypeScript type definitions | Active |
| **@phenotype/errors** | Error handling utilities | Active |
| **@phenotype/config** | Configuration management | Planned |
| **@phenotype/http** | HTTP client with retry logic | Planned |

### Domain-Specific
| Package | Purpose | Status |
|---------|---------|--------|
| **@phenotype/auth** | Authentication and OAuth utilities | Active |
| **@phenotype/observability** | Telemetry and logging | Planned |

## Project Structure

```
packages/
├── design/                   # Design tokens and component lib
│   ├── css/
│   ├── tokens/
│   └── package.json
├── types/                    # TypeScript type definitions
│   ├── src/
│   ├── tests/
│   └── package.json
├── errors/                   # Error handling
├── auth/                     # Authentication
├── ADR.md                    # Architecture decisions
├── CHARTER.md                # Collection charter
├── PRD.md                    # Product requirements
├── PLAN.md                   # Implementation plan
└── README.md
```

## Quick Start

```bash
# Clone and navigate
cd packages/<package-name>

# Review governance
cat ../CLAUDE.md

# Install dependencies
bun install   # or npm install

# Run tests
bun run test

# Build
bun run build

# Quality checks
bun run quality
```

## Adding a New Package

1. **Create directory**:
   ```bash
   mkdir packages/my-package
   cd packages/my-package
   ```

2. **Initialize package**:
   ```bash
   bun init
   # OR
   npm init -y
   ```

3. **Update package.json**:
   ```json
   {
     "name": "@phenotype/my-package",
     "version": "0.1.0",
     "description": "My package description",
     "main": "dist/index.js",
     "types": "dist/index.d.ts"
   }
   ```

4. **Create structure**:
   ```bash
   mkdir src tests
   touch src/index.ts
   ```

5. **Follow conventions**:
   - Export types first (export { MyType })
   - Use descriptive error messages
   - Include JSDoc comments on public APIs
   - Write tests with FR traceability
   - Add to collection index

## Shared Conventions

### Error Handling
```typescript
export class PhnoError extends Error {
  constructor(message: string, public code: string) {
    super(message);
    this.name = 'PhnoError';
  }
}
```

### Configuration
```typescript
export interface Config {
  timeout?: number;
  retries?: number;
}
```

### Testing
```typescript
import { describe, it, expect } from 'vitest';

describe('FR-PKG-001: my feature', () => {
  it('should work', () => {
    expect(true).toBe(true);
  });
});
```

## Workspace Configuration

### Root package.json
```json
{
  "workspaces": [
    "packages/*"
  ]
}
```

### Shared Dependencies
Define shared versions in root `package.json`:
```json
{
  "pnpm": {
    "overrides": {
      "typescript": "~5.0",
      "vitest": "~1.0"
    }
  }
}
```

## Governance

All packages follow Phenotype governance:
- **Parent governance**: `/repos/CLAUDE.md`
- **Global governance**: `~/.claude/CLAUDE.md`
- **Collection charter**: `CHARTER.md`
- **Decision records**: `ADR.md`

## Publishing & Versioning

- **Versioning**: SemVer (major.minor.patch)
- **Changelog**: Per-package CHANGELOG.md
- **Publishing**: npm (public or private scope)
- **Release**: Automated via GitHub Actions
- **Deprecation**: 2-major-version support window

## Testing & Quality

- **Unit Tests**: vitest / jest
- **Integration Tests**: Cross-package tests
- **Type Checking**: typescript strict mode
- **Linting**: oxlint (fast Rust-based linter)
- **Formatting**: prettier / oxfmt

## Build Output

All packages produce:
- `dist/index.js` — CommonJS (for compatibility)
- `dist/index.mjs` — ES Modules (preferred)
- `dist/index.d.ts` — TypeScript definitions

## Related Collections

- **libs/** — Python/Rust libraries
- **apps/** — Applications consuming packages
- **crates/** — Rust infrastructure (parallel to packages/)

## License

MIT — Part of Phenotype organization
