# phenotype-packs

Pre-built feature packs, extension bundles, and composition templates for the Phenotype ecosystem. Enables rapid deployment of common configurations, integrations, and feature combinations.

## Overview

**phenotype-packs** provides opinionated, production-ready bundles that combine Phenotype services into composable, deployable units. Feature packs include everything from authentication systems to observability stacks, reducing configuration overhead and enabling teams to launch complex Phenotype deployments in minutes.

**Core Mission**: Enable rapid, repeatable deployment of Phenotype service combinations through pre-configured, tested, and documented feature packs.

## Technology Stack

- **Multi-Language**: Go (orchestration), Python (templates), Rust (utilities)
- **Composition**: Docker Compose, Kubernetes manifests, Terraform modules
- **Templating**: Handlebars for dynamic configuration generation
- **Testing**: Integration tests validating pack deployments
- **Documentation**: Per-pack guides and architecture diagrams

## Key Features

- **Feature Packs**: Pre-configured combinations (auth, observability, data, monitoring)
- **One-Command Deployment**: Single command bootstraps entire feature stack
- **Customization**: Template variables for environment-specific configuration
- **Dependency Management**: Automatic service ordering and health checks
- **Multi-Backend**: Docker, Kubernetes, local development support
- **Documentation**: Per-pack usage guides, architecture diagrams, troubleshooting
- **Testing**: Automated integration tests for each pack

## Quick Start

```bash
# Clone and explore
git clone <repo-url>
cd phenotype-packs

# Review available packs
ls -la packs/

# Deploy the "auth" pack locally
./deploy.sh --pack auth --environment local

# Deploy the full "enterprise" pack to Kubernetes
./deploy.sh --pack enterprise --environment kubernetes --kubeconfig ~/.kube/config

# View pack documentation
cat packs/auth/README.md

# Customize a pack via environment variables
OAUTH_PROVIDER=github STORAGE_TYPE=postgres ./deploy.sh --pack auth --environment local
```

## Project Structure

```
phenotype-packs/
├── packs/
│   ├── auth/                   # Authentication and authorization stack
│   │   ├── docker-compose.yml
│   │   ├── kubernetes/         # K8s manifests
│   │   ├── terraform/          # IaC modules
│   │   ├── variables.toml      # Configuration schema
│   │   ├── README.md           # Pack documentation
│   │   └── tests/
│   ├── observability/          # Logging, metrics, tracing stack
│   │   ├── docker-compose.yml
│   │   ├── kubernetes/
│   │   ├── README.md
│   │   └── tests/
│   ├── data/                   # Data layer (database, cache, search)
│   │   ├── docker-compose.yml
│   │   ├── kubernetes/
│   │   └── README.md
│   ├── messaging/              # Event streaming and queuing
│   │   ├── docker-compose.yml
│   │   └── README.md
│   ├── devenv/                 # Local development environment
│   │   ├── docker-compose.yml
│   │   └── README.md
│   └── enterprise/             # Full production stack
│       ├── docker-compose.yml
│       ├── kubernetes/
│       └── README.md
├── scripts/
│   ├── deploy.sh               # Main deployment script
│   ├── validate-pack.sh        # Pack validation
│   └── generate-config.sh      # Configuration generation
├── python/
│   ├── pack_validator/         # Pack schema validator
│   ├── config_generator/       # Template variable generator
│   └── tests/
├── go/
│   ├── deployer/               # Go orchestration tool
│   ├── pack/                   # Pack data structures
│   └── tests/
├── docs/
│   ├── ARCHITECTURE.md         # Pack design and composition
│   ├── CREATING_PACKS.md       # Guide to creating new packs
│   ├── CUSTOMIZATION.md        # Customization and templating
│   └── TROUBLESHOOTING.md      # Common issues and fixes
└── Makefile
```

## Available Packs

| Pack | Purpose | Services | Deployment |
|------|---------|----------|------------|
| **auth** | Authentication + authorization | AuthKit, PhenoPlugins | Docker, K8s |
| **observability** | Logging, metrics, tracing | Tracera, PhenoObservability | Docker, K8s |
| **data** | Database + caching layer | PhenoData, Redis | Docker, K8s |
| **messaging** | Event streaming + queuing | RabbitMQ, NATS | Docker, K8s |
| **devenv** | Local development setup | All services | Docker Compose |
| **enterprise** | Complete production stack | All services | K8s, Terraform |

## Related Phenotype Projects

- **PhenoDevOps**: CI/CD integration for pack deployment
- **phenotype-ops-mcp**: MCP server for pack management
- **AgilePlus**: Work tracking for pack deployments
- **cloud**: Multi-tenant platform (primary pack consumer)