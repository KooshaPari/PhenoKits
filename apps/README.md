# Apps

**Phenotype Applications Collection** — Standalone, production-grade applications built on the Phenotype framework infrastructure.

## Overview

The `apps/` directory houses complete, end-to-end applications that consume and integrate Phenotype ecosystem crates and services. Each app is independently deployable and serves specific user-facing or operational functions.

**Core Mission**: Provide production-grade reference implementations and user-facing applications demonstrating Phenotype platform capabilities.

## Technology Stack

- **Framework**: Multi-stack (Rust for backends, Svelte/React for frontends)
- **Architecture**: Microservices + hexagonal ports & adapters
- **Deployment**: Container-native (Docker, Kubernetes-ready)
- **Observability**: Integrated with Phenotype observability stack (Tracera, Prometheus)

## Applications

| App | Purpose | Status | Stack |
|-----|---------|--------|-------|
| **phenotype-dev-hub** | Developer portal and workspace | Active | Node.js/TypeScript |

## Project Structure

```
apps/
├── phenotype-dev-hub/         # Developer portal
│   ├── src/
│   ├── README.md
│   └── package.json
```

## Quick Start

```bash
# Clone and navigate
cd apps/<app-name>

# Review app-specific README
cat README.md

# Install and run
pnpm install
pnpm dev
```

## Adding a New Application

1. Create directory: `mkdir apps/my-app`
2. Initialize per-app governance: `cp CLAUDE.md apps/my-app/`
3. Create comprehensive README with purpose, stack, quick start
4. Link to parent governance: see CLAUDE.md
5. Add to this collection index (above)

## Governance

All apps follow Phenotype governance:
- **Parent governance**: `/repos/CLAUDE.md`
- **Global governance**: `~/.claude/CLAUDE.md`
- Per-app CLAUDE.md for language/framework specifics

## Related Collections

- **libs/** — Reusable libraries and utilities
- **packages/** — Shared packages and modules
- **plugins/** — Plugin ecosystem

## License

MIT — Part of Phenotype organization
