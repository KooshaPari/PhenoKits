# AGENTS.md — packages

## Project Overview

- **Name**: packages
- **Description**: Shared TypeScript/JavaScript packages for the Phenotype ecosystem
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/packages`
- **Language Stack**: TypeScript, JavaScript
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to packages
cd /Users/kooshapari/CodeProjects/Phenotype/repos/packages

# Install dependencies
npm install

# Build
npm run build

# Test
npm test

# Lint
npm run lint
```

## Architecture

```
packages/
├── errors/                 # Error handling utilities
├── ids/                    # ID generation and validation
└── types/                  # Shared TypeScript types
```

## Quality Standards

### TypeScript Standards
- **Line length**: 100 characters
- **Formatter**: `prettier`
- **Linter**: `eslint`
- **Type checker**: `tsc --noEmit`

### Code Style
```typescript
// Use explicit types
interface Config {
  name: string;
  enabled: boolean;
}

// Export from index.ts
export { Config } from './types';
```

## Git Workflow

### Branch Naming
Format: `packages/<type>/<description>` or `<package-name>/<type>/<description>`

Examples:
- `packages/feat/shared-logger`
- `types/fix/nullable-handling`

### Commit Format
```
<type>(<package>): <description>

Examples:
- feat(errors): add structured error class
- fix(ids): resolve ULID generation bug
```

## File Structure

```
packages/
├── <package-name>/
│   ├── package.json
│   ├── tsconfig.json
│   ├── src/
│   │   └── index.ts
│   └── dist/               # Build output
```

## CLI Commands

```bash
# Root operations
npm install
npm run build
npm test

# Package-specific
cd packages/<name>
npm run build
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Build fails | Check `tsconfig.json` settings |
| Type errors | Run `tsc --noEmit` for details |
| Dependency issues | Verify `package.json` versions |

## Dependencies

- **TypeScript**: Primary language
- **Node.js**: Runtime
- **Shared**: Types, errors, IDs

## Agent Notes

When working in packages:
1. Each subdirectory is an independent package
2. May use npm workspaces or independent versioning
3. Check for tsconfig.json in each package
4. Follow TypeScript strict mode conventions
