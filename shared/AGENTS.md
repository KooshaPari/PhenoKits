# AGENTS.md — shared

## Project Overview

- **Name**: shared
- **Description**: Shared UI components and utilities for Phenotype applications
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/shared`
- **Language Stack**: TypeScript/JavaScript (UI-focused)
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to project
cd /Users/kooshapari/CodeProjects/Phenotype/repos/shared

# Check components
ls -la ui-*/
ls -la authkit-config/
```

## Architecture

```
shared/
├── authkit-config/           # AuthKit configuration
├── docs/                     # Documentation
├── ui-api/                   # UI API layer
├── ui-components/            # Shared UI components
├── ui-hooks/                 # React hooks
├── ui-store/                 # State management
└── ui-utils/                 # UI utilities
```

## Quality Standards

### TypeScript/JavaScript Standards
- **Line length**: 100 characters
- **Formatter**: `prettier`
- **Linter**: `eslint`
- **Type checker**: `tsc --noEmit`

### UI Standards
- Component documentation (Storybook if applicable)
- Accessibility (a11y) compliance
- Responsive design patterns

## Git Workflow

### Branch Naming
Format: `shared/<type>/<description>` or `ui-<area>/<type>/<description>`

Examples:
- `shared/feat/new-component`
- `ui-components/fix/button-styling`

## File Structure

```
shared/
├── authkit-config/
├── docs/
├── ui-api/
├── ui-components/
├── ui-hooks/
├── ui-store/
└── ui-utils/
```

## CLI Commands

```bash
# Standard Node workflow
npm install
npm run build
npm test

# Check specific UI package
cd ui-components && npm run build
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Component not rendering | Check ui-components |
| State issues | Check ui-store |
| API errors | Check ui-api |

## Dependencies

- **AuthKit**: Authentication integration
- **packages/**: Shared types
- **frontend/**: UI utilities

## Agent Notes

When working in shared:
1. Shared UI library for Phenotype apps
2. Component-based architecture
3. Follow React/TypeScript best practices
4. Coordinate with AuthKit for auth UI
