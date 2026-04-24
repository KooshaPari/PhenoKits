# AGENTS.md — KWatch

## Project Overview

- **Name**: KWatch (Kubernetes Monitoring & Alerting)
- **Description**: Kubernetes monitoring and alerting platform with custom metrics, intelligent alerts, and automated remediation
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/KWatch`
- **Language Stack**: Rust (Edition 2024), Go (operators), Kubernetes
- **Published**: Private (Phenotype org)

## Quick Start Commands

```bash
# Clone and setup
git clone https://github.com/KooshaPari/KWatch.git
cd KWatch

# Install Rust toolchain
rustup update nightly
rustup default nightly

# Build
cargo build --release

# Deploy to Kubernetes
kubectl apply -f deploy/

# Run tests
cargo test
```

## Architecture

### Monitoring Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                     Data Collection Layer                                │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐         │
│  │   Prometheus      │  │   Custom          │  │   Logs            │         │
│  │   Metrics         │  │   Metrics         │  │   (Fluent)        │         │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘         │
└───────────┼────────────────────┼────────────────────┼────────────────┘
            │                    │                    │
            ▼                    ▼                    ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      KWatch Core (Rust)                                  │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │                    Monitoring Engine                             │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐            │   │
│  │  │   Analyzer   │  │   Alert    │  │   Correl   │            │   │
│  │  │   Engine     │  │   Manager  │  │   Engine   │            │   │
│  │  └────────────┘  └────────────┘  └────────────┘            │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐            │   │
│  │  │   Forecast │  │   Silencer │  │   Remediate│            │   │
│  │  │   Engine     │  │            │  │            │            │   │
│  │  └────────────┘  └────────────┘  └────────────┘            │   │
│  └──────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
            │
            ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      Notification Layer                                  │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐         │
│  │   Slack           │  │   PagerDuty       │  │   Webhook         │         │
│  │   Email           │  │   SMS             │  │   Custom          │         │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘         │
└─────────────────────────────────────────────────────────────────────┘
```

### Alert Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Alert Processing Flow                               │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Metric ──▶ Rule ──▶ Trigger ──▶ Correlation ──▶ Notify ──▶ Remediate │
│                                                                      │
│   CPU > 80%   If sustained   Group by      Send to     Auto-scale   │
│   for 5min    for 5min       service       on-call     if configured │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

## Quality Standards

### Rust Code Quality

- **Formatter**: `rustfmt` (nightly)
- **Linter**: `clippy -- -D warnings`
- **Tests**: `cargo nextest run` with coverage >80%

### Monitoring Standards

- Sub-10s metric freshness
- 99.9% alert delivery
- Automatic alert grouping
- Noise reduction via ML

### Test Requirements

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Kubernetes tests
kubectl apply -f tests/manifests/
```

## Git Workflow

### Branch Naming

Format: `<type>/<component>/<description>`

Types: `feat`, `fix`, `docs`, `refactor`

Examples:
- `feat/alerts/add-ml-silencing`
- `fix/collectors/memory-leak`
- `refactor/engine/use-streams`

## CLI Commands

```bash
# Deploy operator
cargo run --bin kwatch -- deploy

# Check cluster health
cargo run --bin kwatch -- health

# List alerts
cargo run --bin kwatch -- alerts list

# Silence alert
cargo run --bin kwatch -- alerts silence --id alert-123 --duration 1h
```

## Environment Variables

```bash
# Kubernetes
KUBECONFIG=/path/to/kubeconfig
KWATCH_NAMESPACE=monitoring

# Alerting
ALERT_SLACK_WEBHOOK=https://hooks.slack.com/...
ALERT_PAGERDUTY_KEY=xxx

# Storage
PROMETHEUS_URL=http://prometheus:9090
```

---

Last Updated: 2026-04-05
Version: 1.0.0
