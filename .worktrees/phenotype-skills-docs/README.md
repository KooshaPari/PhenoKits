# Phenotype Skills

> Modular Skill System for Agent Orchestration

A comprehensive skill framework for building extensible agent capabilities with hot-reloading, versioning, and dependency management.

## Features

- **Hot Reloading**: Update skills without restarting agents
- **Version Management**: Semantic versioning for skill compatibility
- **Dependency Resolution**: Automatic dependency graph management
- **Multi-Language**: Rust, Python, TypeScript skill support
- **Sandboxing**: Secure execution environment for untrusted skills
- **Registry**: Centralized skill discovery and distribution

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Phenotype Skills System                             │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         Skill Registry                               │   │
│  │                                                                      │   │
│  │   ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │   │
│  │   │  Skill       │  │  Skill       │  │  Skill       │         │   │
│  │   │  (v1.2.0)    │  │  (v2.0.1)    │  │  (v0.9.0)    │         │   │
│  │   │  web_search  │  │  code_gen    │  │  file_parse  │         │   │
│  │   └──────────────┘  └──────────────┘  └──────────────┘         │   │
│  │                                                                      │   │
│  │   Dependencies, Versions, Metadata                                   │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         Skill Loader                                 │   │
│  │                                                                      │   │
│  │   Dynamic Loading → Hot Reload → Dependency Resolution              │   │
│  │                                                                      │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         Skill Sandbox                                │   │
│  │                                                                      │   │
│  │   bwrap │ gVisor │ WASM │ Firecracker                              │   │
│  │                                                                      │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         Skill Runtime                                │   │
│  │                                                                      │   │
│  │   Python │ Rust │ TypeScript │ WebAssembly                         │   │
│  │                                                                      │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Quick Start

```bash
# Install
pip install phenotype-skills

# Create a skill
skills new my_skill --template python

# Register a skill
skills register ./my_skill

# Run a skill
skills run my_skill --input '{"query": "hello"}'

# List installed skills
skills list
```

## Skill Types

| Type | Language | Runtime | Use Case |
|------|----------|---------|----------|
| `native` | Python | CPython | Data processing, ML |
| `compiled` | Rust | Native | High-performance tasks |
| `script` | TypeScript | Deno/Node | Web integration |
| `wasm` | Any | Wasmtime | Portable, sandboxed |

## Creating a Skill

```python
# skills/my_skill/__init__.py
from phenotype_skills import Skill, skill

@skill(
    name="web_search",
    version="1.0.0",
    description="Search the web",
    author="phenotype",
    runtime="python"
)
class WebSearchSkill(Skill):
    async def execute(self, query: str) -> dict:
        # Implementation
        return {"results": [...]}

    def schema(self) -> dict:
        return {
            "input": {
                "type": "object",
                "properties": {
                    "query": {"type": "string"}
                }
            },
            "output": {
                "type": "object",
                "properties": {
                    "results": {"type": "array"}
                }
            }
        }
```

## Configuration

```yaml
# ~/.config/phenotype/skills.yaml
registry:
  url: https://skills.phenotype.dev
  auth_token: ${SKILLS_TOKEN}

runtime:
  default: python
  python:
    version: "3.11"
    packages: ["numpy", "pandas"]
  rust:
    toolchain: stable

sandbox:
  enabled: true
  type: gvisor  # bwrap, gvisor, firecracker
  limits:
    cpu: 1.0
    memory: 512MB
    timeout: 30s

logging:
  level: info
  format: json
```

## Built-in Skills

| Skill | Description | Runtime |
|-------|-------------|---------|
| `web_search` | Search engines | Python |
| `file_parser` | Parse documents | Python |
| `code_analysis` | AST analysis | Python |
| `git_ops` | Git operations | Rust |
| `image_proc` | Image processing | Python |
| `http_client` | HTTP requests | Rust |
| `database` | SQL operations | Python |
| `crypto` | Cryptographic ops | Rust |

## Skill Registry

```bash
# Search for skills
skills search "code analysis"

# Install from registry
skills install phenotype/code-analysis

# Publish to registry
skills publish --registry https://skills.phenotype.dev

# Update skills
skills update --all
```

## Sandboxing

```python
# High-security skill
@skill(
    name="untrusted_skill",
    sandbox="gvisor",  # Strongest isolation
    limits={
        "cpu": 0.5,
        "memory": "256MB",
        "timeout": 10,
        "network": False  # No network access
    }
)
class UntrustedSkill(Skill):
    pass
```

## Documentation

- [SPEC.md](./SPEC.md) - Full specification
- [PLAN.md](./PLAN.md) - Implementation roadmap
- [docs/](docs/) - VitePress documentation

## License

MIT
