# AGENTS.md — PhenoEvents

## Project Overview

- **Name**: PhenoEvents
- **Description**: Event management and webhook handling for the Phenotype ecosystem
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/PhenoEvents`
- **Language Stack**: TypeScript/JavaScript (likely), configuration
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to PhenoEvents
cd /Users/kooshapari/CodeProjects/Phenotype/repos/PhenoEvents

# Check available commands in subdirectories
cd pheno-events && cat README.md 2>/dev/null || ls -la
cd webhook && cat README.md 2>/dev/null || ls -la
```

## Architecture

```
PhenoEvents/
├── pheno-events/             # Event management system
└── webhook/                  # Webhook handlers
```

## Quality Standards

### Code Standards
- Follow conventions of parent project
- **Line length**: 100 characters
- Use appropriate linter for detected language

### Event Handling
- Idempotent event processing
- Proper error handling with dead letter queues
- Schema validation for events

## Git Workflow

### Branch Naming
Format: `phenoevents/<type>/<description>`

Examples:
- `phenoevents/feat/webhook-auth`
- `phenoevents/fix/event-dedup`

### Commit Format
```
<type>(phenoevents): <description>

Examples:
- feat(pheno-events): add event persistence
- fix(webhook): resolve signature verification
```

## File Structure

```
PhenoEvents/
├── pheno-events/
│   ├── src/
│   └── [config files]
└── webhook/
    ├── src/
    └── [config files]
```

## CLI Commands

```bash
# Check structure
ls -la pheno-events/
ls -la webhook/

# Build (if applicable)
npm install
npm run build
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Event not firing | Check event bus connection |
| Webhook failing | Verify endpoint and auth |
| Schema errors | Validate against event schema |

## Dependencies

- **pheno/**: Event definitions
- **AgilePlus**: Event tracking
- **HexaKit**: Event infrastructure

## Agent Notes

When working in PhenoEvents:
1. This appears to be a minimal/placeholder project
2. Check if active development should happen elsewhere
3. May be superseded by phenotype-event-sourcing crate
