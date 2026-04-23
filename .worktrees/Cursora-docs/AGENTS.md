# AGENTS.md — Cursora

## Project Identity

- **Name**: Cursora
- **Description**: Cursor-based pagination utilities for TypeScript/Node.js
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/Cursora`
- **Language Stack**: TypeScript, Node.js
- **Type**: Library/Utility

## Agent Responsibilities

### Forge (Implementation)
- Implement pagination strategies per FR specifications
- Add new cursor formats as needed
- Maintain TypeScript type safety
- Write unit tests with FR traceability

### Helios (Testing)
- Run `npm test` before any PR
- Verify edge cases (empty results, boundary conditions)
- Performance test cursor encoding/decoding

## Development Commands

```bash
npm install    # Install dependencies
npm test       # Run tests
npm run build  # TypeScript compilation
```

## Quality Standards

- **TypeScript strict mode** enabled
- **No implicit any** errors
- **Test coverage**: Minimum 80% for cursor encoding logic
- **FR traceability**: All tests MUST reference FR identifiers

## Branch Discipline

- Feature branches: `feat/<feature-name>`
- Bug fixes: `fix/<issue-name>`
- Worktrees preferred for parallel work

## CI/CD

- GitHub Actions workflow in `.github/workflows/`
- Must pass before merge to main
