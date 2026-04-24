# Agent Rules - phenotype-nexus

**This project is managed through AgilePlus.**

## Overview

Phenotype-Nexus is the central service mesh and API gateway for the Phenotype ecosystem. It provides unified service discovery, intelligent routing, load balancing, and edge security for all Phenotype services, enabling seamless communication across distributed deployments.

### Purpose & Goals

- **Mission**: Connect all Phenotype services with zero-trust security and high performance
- **Primary Goal**: Reduce service-to-service latency by 50% through intelligent routing
- **Secondary Goals**:
  - Provide unified observability across all service traffic
  - Enable canary and blue-green deployments at the edge
  - Support multi-region and multi-cloud topologies
  - Implement zero-trust security with mTLS and JWT

### Key Responsibilities

1. **Service Discovery**: Dynamic service registration and health checking
2. **Load Balancing**: Intelligent traffic distribution with circuit breaking
3. **Edge Routing**: Path-based, header-based, and canary routing
4. **Security**: mTLS, JWT validation, rate limiting, WAF
5. **Observability**: Distributed tracing, metrics, access logging
6. **Protocol Support**: HTTP/1, HTTP/2, HTTP/3, gRPC, WebSocket

## Stack

### Primary Language & Runtime
- **Language**: Go 1.24+ (control plane), Rust (data plane)
- **Runtime**: Native with aggressive generics adoption
- **Architecture**: Control plane + sidecar proxy pattern

### Core Dependencies (Control Plane - Go)
```go
// Service Mesh
istio.io/istio/pilot           // Istio control plane
github.com/envoyproxy/go-control-plane // Envoy xDS

// Service Discovery
github.com/hashicorp/consul/api
k8s.io/client-go

// API Gateway
github.com/caddyserver/caddy/v2
github.com/traefik/traefik/v3

// Security
github.com/golang-jwt/jwt/v5
golang.org/x/crypto

// Observability
go.opentelemetry.io/otel
github.com/prometheus/client_golang

// Configuration
github.com/spf13/viper
gopkg.in/yaml.v3
```

### Core Dependencies (Data Plane - Rust)
```toml
# Proxy implementation
envoy = { version = "0.3", features = ["runtime"] }
tokio = { version = "1.35", features = ["full"] }
hyper = { version = "1.1", features = ["full"] }
tower = "0.4"

# Protocols
h2 = "0.4"                        # HTTP/2
quinn = "0.10"                    # HTTP/3
 tonic = "0.11"                   # gRPC
```

### Control Plane Stack
- **Istio**: Service mesh control plane
- **Envoy**: Sidecar proxy (data plane)
- **Cert-Manager**: Certificate management
- **Flagger**: Canary deployment controller

### Build & Development Tools
- **Task Runner**: Task (Taskfile.yml)
- **Linting**: golangci-lint, clippy
- **Testing**: gotestsum, cargo-nextest
- **Documentation**: OpenAPI specs + docs site

## Quick Start

### Prerequisites

```bash
# Go 1.24+
brew install go@1.24

# Rust nightly
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly

# Kubernetes tools
brew install kubectl helm istioctl

# Task runner
brew install go-task/tap/go-task
```

### Installation

```bash
# Clone the repository
cd /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-nexus

# Install Go dependencies
go mod download

# Install Rust dependencies
cd proxy && cargo fetch

# Build both components
task build

# Verify installation
nexus --version
```

### Development Environment Setup

```bash
# Copy environment configuration
cp .env.example .env

# Start local Kubernetes cluster (kind or k3d)
kind create cluster --name nexus-dev

# Install Nexus to local cluster
task install:local

# Verify installation
kubectl get pods -n nexus
```

### Running the Services

```bash
# Development mode - control plane
task dev:control

# Development mode - data plane
task dev:proxy

# All services
task dev

# Production build
task build:release
```

### Verification

```bash
# Run all tests
task test

# Run with coverage
task test:coverage

# Check code quality
task lint
task format:check

# Health check
nexus status
```

## Architecture

### System Design

Phenotype-Nexus implements a service mesh with control and data planes:

```
┌─────────────────────────────────────────────────────────────┐
│                    Control Plane                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   xDS        │  │   Service    │  │   Certificate│     │
│  │   Server     │  │   Discovery  │  │   Manager    │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
├─────────────────────────────────────────────────────────────┤
│                    Data Plane (Sidecars)                     │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐     │
│  │ Service  │ │ Service  │ │ Service  │ │ Service  │     │
│  │  A       │ │  B       │ │  C       │ │  D       │     │
│  │ + Proxy  │ │ + Proxy  │ │ + Proxy  │ │ + Proxy  │     │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘     │
└───────┼────────────┼────────────┼────────────┼────────────┘
        │            │            │            │
        └────────────┴────────────┴────────────┘
                       mTLS Mesh
```

### Component Breakdown

#### 1. Control Plane
- **xDS Server**: Envoy Discovery Service (LDS, RDS, CDS, EDS)
- **Service Discovery**: Kubernetes, Consul, or custom
- **Certificate Manager**: Automatic mTLS certificate rotation

#### 2. Data Plane (Envoy Proxy)
- **Listener**: Ingress traffic handling
- **Router**: Traffic routing and load balancing
- **Cluster**: Upstream service connections
- **Filter**: Authentication, rate limiting, transformation

#### 3. Edge Gateway
- **Ingress Controller**: External traffic entry
- **API Gateway**: Rate limiting, authentication, routing
- **WAF**: Web Application Firewall rules

### Traffic Flow

```
Client Request
      ↓
  Edge Gateway (TLS termination, WAF)
      ↓
  VirtualService (routing rules)
      ↓
  DestinationRule (load balancing)
      ↓
  Sidecar Proxy (mTLS to upstream)
      ↓
  Target Service
```

### Security Architecture

```
┌─────────────────────────────────────────┐
│         Zero Trust Security             │
├─────────────────────────────────────────┤
│                                         │
│  External → JWT/OAuth Validation → mTLS │
│                                         │
│  Service → mTLS + RBAC → Service        │
│                                         │
│  Certificates: Automatic rotation       │
│  via cert-manager + SPIFFE              │
│                                         │
└─────────────────────────────────────────┘
```

## Quality Standards

### Testing Requirements

#### Test Coverage
- **Go Control Plane**: 80% minimum coverage
- **Rust Proxy**: 75% minimum coverage
- **Critical Paths**: 95% coverage for routing logic

#### Test Categories
```bash
# Go tests
task test:go

# Rust tests
task test:rust

# Integration tests
task test:integration

# E2E tests with real cluster
task test:e2e
```

### Code Quality

#### Go Standards
```bash
# Linting
golangci-lint run --config=.golangci.yml

# Formatting
go fmt ./...
gofumpt -l -w .
```

#### Rust Standards
```bash
# Clippy
cargo clippy --all-targets -- -D warnings

# Formatting
cargo fmt --check
```

### Reliability Standards

| Metric | Target | Measurement |
|--------|--------|-------------|
| P99 Latency | < 5ms | Sidecar to sidecar |
| Availability | 99.99% | Control plane uptime |
| Config propagation | < 2s | xDS to all proxies |
| Certificate rotation | Zero-downtime | mTLS cert update |

## Git Workflow

### Branch Strategy

```
main
  │
  ├── feature/http3-support
  │   └── PR #67 → squash merge ──┐
  │                               │
  ├── feature/canary-advanced     │
  │   └── PR #68 → squash merge ──┤
  │                               │
  ├── fix/proxy-memory-leak       │
  │   └── PR #69 → squash merge ──┤
  │                               │
  └── hotfix/security-patch ──────┘
      └── PR #70 → merge commit
```

### Branch Naming

```
feature/<component>-<description>
fix/<component>-<issue>
perf/<optimization>
refactor/<scope>
docs/<topic>
chore/<maintenance>
hotfix/<critical>
```

### Commit Conventions

```
feat(routing): add HTTP/3 support for edge gateway

Enables HTTP/3 (QUIC) for external traffic termination.
Reduces connection establishment time by 50%.

Closes #123

fix(proxy): resolve memory leak in long-lived connections

Connections held open for > 24h accumulated buffers.
Now properly releases memory on connection close.
```

### Pull Request Process

1. **Pre-PR Checklist**:
   ```bash
   task lint
   task test
   task format:check
   ```

2. **PR Requirements**:
   - Link to AgilePlus spec
   - Performance benchmarks for proxy changes
   - Architecture diagram for routing changes

3. **Review Requirements**:
   - 1 approval from nexus team
   - CI passes including integration tests

4. **Merge Strategy**:
   - Squash merge for features
   - Regular merge for hotfixes

## File Structure

```
phenotype-nexus/
├── control-plane/              # Go control plane
│   ├── cmd/
│   │   └── nexus/
│   ├── pkg/
│   │   ├── xds/               # xDS server
│   │   ├── discovery/         # Service discovery
│   │   ├── authz/             # Authorization
│   │   └── config/            # Configuration
│   ├── api/                   # API definitions
│   ├── go.mod
│   └── go.sum
│
├── proxy/                      # Rust data plane
│   ├── src/
│   │   ├── main.rs
│   │   ├── listener.rs        # Traffic listener
│   │   ├── router.rs          # Routing logic
│   │   ├── cluster.rs         # Upstream handling
│   │   ├── filter.rs          # Filter chain
│   │   ├── tls.rs             # TLS handling
│   │   └── metrics.rs         # Metrics
│   ├── Cargo.toml
│   └── Cargo.lock
│
├── gateway/                    # Edge gateway
│   ├── caddy/                 # Caddy configs
│   └── envoy/                 # Envoy configs
│
├── helm/                       # Helm charts
│   └── nexus/
│
├── manifests/                  # Kubernetes manifests
│   ├── crd/
│   └── example/
│
├── tests/
│   ├── integration/
│   └── e2e/
│
├── docs/
│   ├── architecture.md
│   └── routing.md
│
├── Taskfile.yml                # Task definitions
├── go.work                     # Go workspace
├── Cargo.toml                  # Rust workspace
├── README.md
├── CHANGELOG.md
└── AGENTS.md                   # This file
```

## CLI

### Core Commands

```bash
# Service Management
nexus service list             # List services
nexus service register <name>  # Register service
nexus service deregister <name> # Deregister
nexus service status <name>    # Check status

# Routing
nexus route list               # List routes
nexus route add <path> <service>  # Add route
nexus route delete <path>      # Delete route

# Certificates
nexus cert list                # List certificates
nexus cert rotate <name>       # Rotate certificate
nexus cert status <name>       # Check cert status

# Traffic Management
nexus traffic shift <service> <percent>  # Canary shift
nexus traffic mirror <from> <to>         # Traffic mirroring
nexus traffic split <service> <weights>  # A/B testing

# Observability
nexus metrics                  # Show metrics
nexus logs <service>           # View logs
nexus trace <request-id>       # Trace request

# Diagnostics
nexus doctor                   # Health check
nexus proxy-config <pod>       # Get proxy config
nexus proxy-stats <pod>          # Get proxy stats
```

### Configuration

```yaml
# nexus.yaml
apiVersion: nexus.phenotype.dev/v1
kind: MeshConfig
spec:
  mtls:
    enabled: true
    mode: STRICT
  
  gateway:
    enabled: true
    tls:
      mode: SIMPLE
      credentialName: gateway-cert
  
  observability:
    tracing:
      sampling: 1.0
    metrics:
      enabled: true
  
  security:
    rateLimit:
      requestsPerSecond: 1000
    jwt:
      jwksUri: https://auth.phenotype.dev/.well-known/jwks.json
```

## Troubleshooting

### Common Issues

#### Issue: Sidecar proxy not starting

**Symptoms:**
- Pod stuck in Init state
- "istio-proxy" container crash looping

**Diagnosis:**
```bash
# Check proxy logs
kubectl logs <pod> -c istio-proxy

# Check xDS connection
istioctl proxy-status
```

**Resolution:**
```bash
# Restart proxy
kubectl rollout restart deployment/<name>

# Force config reload
istioctl proxy-config bootstrap <pod>
```

---

#### Issue: mTLS connection failures

**Symptoms:**
```
Connection refused: certificate validation failed
```

**Diagnosis:**
```bash
# Check certificate status
nexus cert status <service>

# Verify mTLS mode
kubectl get peerauthentication -A
```

**Resolution:**
```bash
# Force certificate rotation
nexus cert rotate <service>

# Check for clock skew
ntpdate -q time.google.com
```

---

#### Issue: High latency through proxy

**Diagnosis:**
```bash
# Check proxy metrics
nexus proxy-stats <pod> | grep latency

# Compare direct vs proxied
nexus trace <request-id>
```

**Resolution:**
```bash
# Adjust buffer sizes
nexus config set proxy.bufferSize 64KB

# Enable connection pooling
nexus config set cluster.connectionPool.maxConnections 100
```

---

#### Issue: Routing rules not applying

**Diagnosis:**
```bash
# Check config propagation
istioctl proxy-config route <pod>

# Verify VirtualService
kubectl get virtualservice <name> -o yaml
```

**Resolution:**
```bash
# Force config reload
istioctl proxy-config route <pod> --reset

# Check for config errors
istioctl analyze
```

---

### Debug Mode

```bash
# Enable debug logging
export NEXUS_LOG_LEVEL=debug

# Proxy debug logging
istioctl proxy-config log <pod> --level debug

# Collect envoy config
istioctl proxy-config all <pod> -o yaml > proxy-config.yaml
```

### Emergency Procedures

```bash
# Bypass mesh for emergency
kubectl label namespace <name> istio-injection=disabled
kubectl rollout restart deployment/<name> -n <name>

# Reset all proxy configs
istioctl proxy-config bootstrap <pod> --force
```

---

## Agent Self-Correction & Verification Protocols

### Critical Rules

1. **Security First**
   - Never disable mTLS in production
   - All changes audited
   - Zero-trust by default

2. **Reliability**
   - No single point of failure
   - Graceful degradation
   - Automatic recovery

3. **Observability**
   - Every request traced
   - Metrics for all components
   - Alert on anomalies

4. **AgilePlus Integration**
   - Reference mesh specs
   - Update for new routing patterns

---

*This AGENTS.md is a living document. Update it as phenotype-nexus evolves.*
