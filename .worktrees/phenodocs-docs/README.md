# PhenoDocs

> VitePress Federation Hub — Aggregate documentation from multiple projects into a unified portal

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Bun](https://img.shields.io/badge/bun-1.x-black?logo=bun)](https://bun.sh/)
[![Python](https://img.shields.io/badge/Python-3.14+-blue.svg)](https://www.python.org/)
[![VitePress](https://img.shields.io/badge/VitePress-1.x-green.svg)](https://vitepress.dev/)
[![Deploy](https://github.com/kooshapari/phenodocs/actions/workflows/deploy.yml/badge.svg)](https://github.com/kooshapari/phenodocs/actions/workflows/deploy.yml)
[![GitHub Pages](https://img.shields.io/badge/GitHub%20Pages-Ready-green)](https://kooshapari.github.io/phenodocs/)

**Live Demo**: https://kooshapari.github.io/phenodocs/

## Shipping / full turn

Every merged wave should land via **pull request** to **`main`** (or a **release** branch), update **`CHANGELOG.md`** when the change is user-visible, and keep **`pnpm run build`** green for doc config changes. See **`docs/guides/full-turn-delivery.md`**.

## Overview

PhenoDocs solves the problem of maintaining multiple documentation sites by providing a federation layer that:

- **Aggregates** docs from multiple projects into one searchable portal
- **Auto-generates** navigation from project structures
- **Maintains** separate views for different audiences (lab, docs, audit, kb)
- **Indexes** content for both human and AI consumption

## Features

- 🚀 **Fast** — Static site generation with VitePress
- 🔗 **Federated** — Pull docs from multiple repositories
- 🤖 **AI-Ready** — `.llms.txt` generation for LLM context
- 🔍 **Searchable** — Full-text search across all projects
- 📊 **Layered** — Separate views for ideas, specs, audits, knowledge
- ⚙️ **Automated** — Git hooks for automatic updates

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/yourorg/phenodocs.git
cd phenodocs

# Install JS deps (Bun) and Python tooling (uv, CPython 3.14+)
curl -fsSL https://bun.sh/install | bash   # if Bun not installed
curl -LsSf https://astral.sh/uv/install.sh | sh   # if uv not installed
bun install
uv sync --group dev
```

After cloning, use **`gh pr list`** / **`gh pr view`** (GitHub CLI) to see open PRs and merge status; **`gh pr view --web`** opens the current branch’s PR in the browser.

### Running Locally

```bash
# Development server
bun run dev

# Build for production
bun run build

# Preview production build
bun run preview

# Oxlint + vue-tsc + link stub
bun run check
```

### Generating the Hub

From a project using `docs_engine`:

```bash
docs hub --hub-dir ../phenodocs
```

Or programmatically:

```python
from docs_engine.hub.generator import HubGenerator

projects = {
    "thegent": "/path/to/thegent/docs",
    "pheno-sdk": "/path/to/pheno-sdk/docs",
    "cliproxy": "/path/to/cliproxy/docs"
}

gen = HubGenerator(
    hub_dir="phenodocs",
    projects=projects
)
gen.generate()
```

## Project Structure

```
phenodocs/
├── .vitepress/
│   ├── config.ts          # VitePress configuration
│   └── theme/
│       └── index.ts       # Custom theme
├── docs/                  # Hub documentation
│   ├── index.md          # Hub landing page
│   ├── guide/
│   │   └── getting-started.md
│   └── reference/
│       └── api.md
├── projects/             # Aggregated project docs (git submodule or external)
├── package.json
└── README.md
```

## Documentation Layers

PhenoDocs organizes content into five layers:

| Layer | View | Content Type |
|-------|------|--------------|
| 0 | (internal) | Raw/ephemeral (conversation dumps, scratch notes) |
| 1 | `/lab/` | Ideas, research, debug logs |
| 2 | `/docs/` | PRDs, ADRs, specifications |
| 3 | `/audit/` | Changelogs, completion reports |
| 4 | `/kb/` | Retrospectives, knowledge extracts |

## Configuration

### VitePress Config

```typescript
// .vitepress/config.ts
import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'PhenoDocs',
  description: 'Federated documentation hub',
  themeConfig: {
    nav: [
      { text: 'Lab', link: '/lab/' },
      { text: 'Docs', link: '/docs/' },
      { text: 'Audit', link: '/audit/' },
      { text: 'KB', link: '/kb/' }
    ],
    sidebar: {
      '/docs/': [
        {
          text: 'Specifications',
          items: [
            { text: 'PRD', link: '/docs/prd' },
            { text: 'ADRs', link: '/docs/adr' }
          ]
        }
      ]
    }
  }
})
```

### Adding a New Project

1. Add project to `HubGenerator` projects dict
2. Ensure project has proper frontmatter
3. Regenerate hub: `docs hub`
4. Commit changes

## Integration with Docs Engine

PhenoDocs works with the [docs_engine](https://github.com/yourorg/docs_engine) Python package:

```python
# CLI
docs hub --hub-dir ../phenodocs

# MCP Tool
thegent_doc_hub_generate(hub_dir="../phenodocs")
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a PR

### Building

```bash
bun run build
bun run preview
```

## License

MIT License — see [LICENSE](LICENSE) for details.

## Related Projects

- [docs_engine](https://github.com/yourorg/docs_engine) — Agent-driven documentation lifecycle
- [thegent](https://github.com/yourorg/thegent) — AI agent system
- [pheno-sdk](https://github.com/yourorg/pheno-sdk) — SDK for Pheno APIs
