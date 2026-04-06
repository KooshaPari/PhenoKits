# phenotype-nexus User Stories

> Real-world usage examples

## Story Collection

| Story | Complexity | Time | Status |
|-------|------------|------|--------|
| [Hello World](./hello-world) | Beginner | 2 min | Complete |
| [Core Integration](./core-integration) | Intermediate | 10 min | Complete |
| [Production Deploy](./production-deploy) | Advanced | 30 min | Complete |

## Story Details

### Hello World

A minimal example to get phenotype-nexus running with basic service registration
and discovery. Perfect for evaluating the library in isolation.

### Core Integration

Integrates phenotype-nexus into a multi-service microservice architecture with:
- Service registration on startup
- Dynamic service discovery
- Round-robin load balancing
- Basic health tracking

### Production Deploy

Full production deployment with:
- Kubernetes deployment manifests
- Circuit breaker implementation
- TTL-based registration refresh
- Prometheus metrics and Grafana dashboards
- SLO configuration and error budgets

## Story Format

Each story includes:
- **GIF Demo**: Visual walkthrough
- **Code Blocks**: Copy-paste ready
- **Mermaid Diagrams**: Architecture visualization
- **Performance Metrics**: Expected latency/throughput
