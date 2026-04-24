# Product Requirements Document (PRD) - bed

## 1. Executive Summary

**bed** is a development environment orchestration and workspace management tool designed for modern software development workflows. It provides ephemeral, reproducible development environments that can be version-controlled, shared across teams, and instantiated on-demand. bed bridges the gap between local development convenience and production-like environment fidelity.

**Vision**: To make "it works on my machine" a thing of the past by providing every developer with production-identical, instant-on development environments that are as easy to create as running a single command.

**Mission**: Enable teams to define, version, share, and instantiate development environments with the same rigor applied to production infrastructure, eliminating environment drift and onboarding friction.

**Current Status**: Early development phase with core workspace lifecycle management implemented.

---

## 2. Problem Statement

### 2.1 Current Challenges

Modern software development faces significant environment-related friction:

**Environment Drift**: 
- Developer machines accumulate dependencies over time
- "Works on my machine" bugs consume hours of debugging
- Production bugs that don't reproduce locally
- Dependency version conflicts between projects
- System-level library mismatches

**Onboarding Friction**:
- New team members spend days setting up development environments
- Outdated or incomplete onboarding documentation
- "Shadow knowledge" not captured in docs
- Complex multi-service setups require manual coordination
- Permission and credential setup is error-prone

**Reproducibility Crisis**:
- CI/CD environments differ from local development
- Integration tests fail inconsistently
- Debugging CI failures requires pushing commits
- Old branches break due to system changes
- Historical versions can't be built

**Resource Waste**:
- Developers maintain multiple local environments
- Over-provisioned cloud dev instances run 24/7
- Context switching between projects is slow
- Experimentation is expensive

### 2.2 Root Causes

1. **Lack of Environment as Code**: Development environments are manually configured
2. **System Dependency Leakage**: Applications depend on system state
3. **Imperative Setup**: Setup scripts that aren't idempotent
4. **State Accumulation**: Long-lived environments drift from baseline
5. **Configuration Sprawl**: Settings scattered across dotfiles, env vars, and documentation

### 2.3 Impact

- 30-50% of developer time spent on environment issues (industry average)
- Delayed feature delivery due to setup overhead
- Quality issues from environment-related bugs
- Reduced experimentation and innovation
- Knowledge silos and bus factor risk
- Burnout from repetitive setup tasks

### 2.4 Target Solution

bed provides:
1. **Declarative Environments**: Define environments in code (bed.toml)
2. **Ephemeral Workspaces**: Create/destroy environments on demand
3. **Reproducible Builds**: Same environment every time, everywhere
4. **Version Control**: Environments tracked alongside application code
5. **Team Sharing**: Share working environments as easily as sharing code

---

## 3. Target Users & Personas

### 3.1 Primary Personas

#### Jamie - Senior Backend Developer
- **Role**: Senior engineer working on microservices
- **Pain Points**: Context switching between 5+ services; environment conflicts; onboarding junior devs
- **Goals**: Instant environment for any service; reproduce production issues locally; share exact setup with team
- **Technical Level**: Expert
- **Usage Pattern**: Daily environment creation/switching; debugging production issues

#### Alex - Junior Frontend Developer
- **Role**: New team member, frontend focus
- **Pain Points**: Overwhelmed by complex setup; afraid to break existing environments
- **Goals**: One-command setup; isolated experiments; safe space to learn
- **Technical Level**: Beginner-Intermediate
- **Usage Pattern**: Occasional environment recreation; learning and experimentation

#### Taylor - DevOps/Platform Engineer
- **Role**: Platform team maintaining developer infrastructure
- **Pain Points**: Supporting diverse team needs; environment-related tickets; CI/CD inconsistencies
- **Goals**: Standardized environments; self-service provisioning; CI parity with local
- **Technical Level**: Expert
- **Usage Pattern**: Defining base images; troubleshooting; CI integration

#### Morgan - Tech Lead
- **Role**: Leading development team, code review focus
- **Pain Points**: PR reviews require environment setup; can't easily test contributor changes
- **Goals**: Instant PR environments; security isolation; team productivity metrics
- **Technical Level**: Expert
- **Usage Pattern**: PR reviews; team management; architecture decisions

### 3.2 Secondary Personas

#### Casey - Open Source Maintainer
- **Role**: Maintains popular open source project
- **Pain Points**: Contributors have varied environments; PRs fail due to setup issues
- **Goals**: Contributors get identical environment; reduce PR friction

#### Riley - Security Engineer
- **Role**: Security review and compliance
- **Pain Points**: Need isolated environments for security testing; audit trail requirements
- **Goals**: Ephemeral environments for security work; audit logging; no persistent secrets

### 3.3 User Needs Matrix

| Need | Jamie | Alex | Taylor | Morgan | Casey | Riley |
|------|-------|------|--------|--------|-------|-------|
| Speed | Critical | Critical | Important | Important | Important | Nice |
| Isolation | Critical | Nice | Critical | Critical | Nice | Critical |
| Reproducibility | Critical | Nice | Critical | Critical | Critical | Critical |
| Ease of Use | Nice | Critical | Nice | Nice | Critical | Nice |
| CI Integration | Nice | Nice | Critical | Important | Important | Nice |
| Cost Control | Nice | Nice | Critical | Important | Important | Nice |

---

## 4. Functional Requirements

### 4.1 Environment Definition

#### FR-ENV-001: Declarative Configuration
**Priority**: P0 (Critical)
**Description**: Define environments declaratively in bed.toml files
**Acceptance Criteria**:
- TOML-based configuration with clear schema
- Support for base image specification
- Package manager integration (npm, pip, cargo, etc.)
- Service dependencies (databases, caches, message queues)
- Environment variable definitions
- Port mapping configuration
- Volume mount specifications
- Secret management references

#### FR-ENV-002: Environment Inheritance
**Priority**: P1 (High)
**Description**: Inherit and extend base environments
**Acceptance Criteria**:
- `extends` keyword for base environment reference
- Override and merge semantics
- Multiple inheritance support
- Base environment versioning
- Local overrides without modifying shared config

#### FR-ENV-003: Template System
**Priority**: P1 (High)
**Description**: Pre-defined environment templates
**Acceptance Criteria**:
- Built-in templates (node, python, rust, go, ruby)
- Community template repository
- Template parameterization
- Custom template creation
- Template discovery and installation

#### FR-ENV-004: Secret Management
**Priority**: P0 (Critical)
**Description**: Secure handling of environment secrets
**Acceptance Criteria**:
- Integration with secret managers (1Password, Vault, AWS Secrets Manager)
- Environment-specific secret injection
- Secret rotation support
- No secrets in configuration files
- Audit logging for secret access

### 4.2 Workspace Lifecycle

#### FR-LIFE-001: Workspace Creation
**Priority**: P0 (Critical)
**Description**: Create new development workspace from configuration
**Acceptance Criteria**:
- `bed create` command with optional name
- Configuration validation before creation
- Dependency resolution and installation
- Service startup and health checks
- Progress reporting and time estimates
- Partial creation recovery on failure

#### FR-LIFE-002: Workspace Activation
**Priority**: P0 (Critical)
**Description**: Enter and work within a workspace
**Acceptance Criteria**:
- `bed activate` command with shell integration
- Automatic environment variable injection
- Tool availability (custom PATH)
- Shell prompt customization showing active workspace
- IDE integration (VS Code, JetBrains)
- tmux/screen session support

#### FR-LIFE-003: Workspace Deactivation
**Priority**: P1 (High)
**Description**: Exit workspace and return to host environment
**Acceptance Criteria**:
- `bed deactivate` command
- Clean shell environment restoration
- Process cleanup confirmation
- Unsaved work warnings

#### FR-LIFE-004: Workspace Destruction
**Priority**: P1 (High)
**Description**: Remove workspace and reclaim resources
**Acceptance Criteria**:
- `bed destroy` command with confirmation
- Selective destruction (keep data volumes)
- Force destruction option
- Cleanup verification
- Resource usage reporting

#### FR-LIFE-005: Workspace Suspension/Resumption
**Priority**: P2 (Medium)
**Description**: Pause workspace without destruction
**Acceptance Criteria**:
- `bed suspend` to freeze workspace state
- `bed resume` to restore suspended workspace
- State preservation across host reboots
- Resource release during suspension

### 4.3 Service Orchestration

#### FR-SVC-001: Service Dependencies
**Priority**: P0 (Critical)
**Description**: Define and manage service dependencies
**Acceptance Criteria**:
- Database services (PostgreSQL, MySQL, MongoDB, Redis)
- Message queues (RabbitMQ, Kafka, NATS)
- Search engines (Elasticsearch, Meilisearch)
- Object storage (MinIO)
- Service health checks
- Startup ordering and readiness gates

#### FR-SVC-002: Service Configuration
**Priority**: P1 (High)
**Description**: Configure dependent services
**Acceptance Criteria**:
- Version pinning for services
- Custom configuration files
- Data initialization (seeds, migrations)
- Persistent vs. ephemeral storage options
- Service-to-service networking

#### FR-SVC-003: Port Management
**Priority**: P1 (High)
**Description**: Handle port allocation and conflicts
**Acceptance Criteria**:
- Automatic port allocation for dynamic ports
- Static port mapping configuration
- Port conflict detection and resolution
- Port forwarding to host
- Port availability checking

### 4.4 Networking

#### FR-NET-001: Network Isolation
**Priority**: P1 (High)
**Description**: Isolate workspace networks
**Acceptance Criteria**:
- Private workspace networks
- Cross-workspace communication controls
- VPN integration options
- Proxy configuration support

#### FR-NET-002: Host Integration
**Priority**: P1 (High)
**Description**: Seamless host-workspace integration
**Acceptance Criteria**:
- localhost access to workspace services
- File system mounting (bidirectional)
- SSH agent forwarding
- Git credential sharing
- Browser integration (automatic port opening)

### 4.5 State Management

#### FR-STATE-001: Volume Persistence
**Priority**: P1 (High)
**Description**: Persistent storage for workspace data
**Acceptance Criteria**:
- Named volume management
- Bind mount support
- Data volume backup and restore
- Volume sharing between workspaces
- Automatic cleanup policies

#### FR-STATE-002: Checkpoint/Snapshot
**Priority**: P2 (Medium)
**Description**: Save and restore workspace state
**Acceptance Criteria**:
- `bed snapshot create` command
- Named snapshots with descriptions
- Snapshot listing and inspection
- `bed snapshot restore` command
- Branching from snapshots

### 4.6 Sharing and Collaboration

#### FR-SHARE-001: Environment Export
**Priority**: P1 (High)
**Description**: Export workspace definition for sharing
**Acceptance Criteria**:
- `bed export` with multiple formats
- Full environment specification
- Dependency lock files
- Exclusion of secrets and sensitive data

#### FR-SHARE-002: Environment Import
**Priority**: P1 (High)
**Description**: Import shared environment definitions
**Acceptance Criteria**:
- `bed import` from URL or file
- Verification of imported configuration
- Conflict resolution with existing environments
- Template installation from import

#### FR-SHARE-003: Team Registry
**Priority**: P2 (Medium)
**Description**: Share environments within team/organization
**Acceptance Criteria**:
- Private registry support
- Access control and permissions
- Environment versioning
- Usage analytics

### 4.7 IDE Integration

#### FR-IDE-001: VS Code Integration
**Priority**: P1 (High)
**Description**: First-class VS Code support
**Acceptance Criteria**:
- bed extension for VS Code
- Remote container development
- Workspace configuration generation
- Integrated terminal with bed environment
- Debug configuration setup

#### FR-IDE-002: JetBrains Integration
**Priority**: P2 (Medium)
**Description**: IntelliJ, PyCharm, GoLand, etc. support
**Acceptance Criteria**:
- Remote development support
- bed plugin integration
- Project configuration import

#### FR-IDE-003: Terminal Editor Support
**Priority**: P2 (Medium)
**Description**: Support for vim, emacs, helix
**Acceptance Criteria**:
- Environment variable setup
- Tool availability in PATH
- LSP server integration

---

## 5. Non-Functional Requirements

### 5.1 Performance

#### NFR-PERF-001: Creation Time
**Priority**: P0 (Critical)
**Description**: Fast workspace creation
**Requirements**:
- Warm cache: < 10 seconds
- Cold start with common base: < 60 seconds
- Complex environment: < 5 minutes
- Progress indicators for long operations

#### NFR-PERF-002: Activation Time
**Priority**: P0 (Critical)
**Description**: Fast workspace activation
**Requirements**:
- < 1 second to activate existing workspace
- No measurable shell latency
- Lazy loading of heavy components

#### NFR-PERF-003: Resource Efficiency
**Priority**: P1 (High)
**Description**: Efficient resource usage
**Requirements**:
- Disk: Deduplicated layers, compression
- Memory: Shared base images, swap optimization
- CPU: No background processes when inactive
- Network: Cached layers, offline capability

### 5.2 Reliability

#### NFR-REL-001: Reproducibility
**Priority**: P0 (Critical)
**Description**: Identical environments across invocations
**Requirements**:
- Bit-for-bit reproducible where possible
- Lock file for exact dependency versions
- Timestamp recording for time-sensitive builds
- Network-isolated builds option

#### NFR-REL-002: Recovery
**Priority**: P1 (High)
**Description**: Graceful handling of failures
**Requirements**:
- Partial creation cleanup on failure
- Automatic retry with exponential backoff
- State recovery mechanisms
- Clear error messages with remediation

#### NFR-REL-003: Data Safety
**Priority**: P0 (Critical)
**Description**: Protect user data
**Requirements**:
- No data loss on workspace destruction (explicit opt-in)
- Atomic operations for critical changes
- Backup recommendations and tooling
- Safe upgrade paths

### 5.3 Security

#### NFR-SEC-001: Isolation
**Priority**: P0 (Critical)
**Description**: Strong workspace isolation
**Requirements**:
- Process isolation (namespaces, cgroups)
- Filesystem isolation (chroot, overlay)
- Network isolation (virtual networks)
- Resource limits enforcement

#### NFR-SEC-002: Secret Handling
**Priority**: P0 (Critical)
**Description**: Secure secret management
**Requirements**:
- Secrets never in logs
- Memory-only secret storage where possible
- Encrypted secret persistence
- Secret rotation support
- Audit logging

#### NFR-SEC-003: Supply Chain
**Priority**: P1 (High)
**Description**: Secure dependency management
**Requirements**:
- Image signing and verification
- SBOM generation
- Vulnerability scanning integration
- Reproducible builds

### 5.4 Usability

#### NFR-USE-001: Learning Curve
**Priority**: P1 (High)
**Description**: Easy to learn
**Requirements**:
- Simple getting started (5 minutes)
- Progressive complexity
- Sensible defaults
- Clear documentation

#### NFR-USE-002: Error Messages
**Priority**: P1 (High)
**Description**: Helpful error reporting
**Requirements**:
- Actionable error messages
- Suggested fixes
- Link to relevant documentation
- Verbose mode for debugging

#### NFR-USE-003: Documentation
**Priority**: P0 (Critical)
**Description**: Comprehensive documentation
**Requirements**:
- Quick start guide
- Full reference documentation
- Troubleshooting guide
- Video tutorials
- API documentation

### 5.5 Portability

#### NFR-PORT-001: Platform Support
**Priority**: P0 (Critical)
**Description**: Cross-platform availability
**Requirements**:
- macOS (Intel and Apple Silicon)
- Linux (major distributions)
- Windows (WSL2 and native where possible)
- Consistent behavior across platforms

#### NFR-PORT-002: Backend Flexibility
**Priority**: P2 (Medium)
**Description**: Multiple backend options
**Requirements**:
- Docker (default)
- Podman
- Containerd
- VM backends (Lima, colima)
- Kubernetes (advanced)

### 5.6 Compatibility

#### NFR-COMP-001: Docker Compatibility
**Priority**: P0 (Critical)
**Description**: Docker ecosystem compatibility
**Requirements**:
- Dockerfile parsing and execution
- Docker Compose support
- Image registry compatibility
- Volume and network compatibility

#### NFR-COMP-002: DevContainer Spec
**Priority**: P2 (Medium)
**Description**: VS Code DevContainer compatibility
**Requirements**:
- devcontainer.json parsing
- Feature parity where applicable
- Migration path from DevContainers

---

## 6. User Stories

### 6.1 Environment Setup Stories

#### US-SETUP-001: New Project Onboarding
**As a** new team member
**I want to** set up the project environment with one command
**So that** I can start contributing immediately
**Acceptance Criteria**:
- Clone repository
- Run `bed create`
- Environment ready in under 2 minutes
- All services running and accessible
- Documentation is accessible at localhost:3000

#### US-SETUP-002: Multi-Service Project
**As a** backend developer
**I want to** work on a microservice in isolation
**So that** I don't need to run the entire system locally
**Acceptance Criteria**:
- Define service dependencies in bed.toml
- `bed create` starts only required services
- Mock/stub services for external dependencies
- Can run integration tests against real dependencies

#### US-SETUP-003: Legacy Project Support
**As a** developer on a legacy project
**I want to** create an environment matching production
**So that** I can safely make changes
**Acceptance Criteria**:
- Specify exact dependency versions
- Document system requirements
- Reproduce production database state
- Test upgrades safely

### 6.2 Daily Development Stories

#### US-DEV-001: Context Switching
**As a** developer working on multiple projects
**I want to** quickly switch between different environments
**So that** I can work efficiently across projects
**Acceptance Criteria**:
- `bed list` shows all workspaces
- `bed activate <workspace>` switches instantly
- Each workspace has isolated state
- No conflicts between projects

#### US-DEV-002: Experimentation
**As a** developer testing a new approach
**I want to** create a throwaway environment
**So that** I can experiment safely
**Acceptance Criteria**:
- Quick temporary workspace creation
- Easy copy from existing workspace
- Simple destruction when done
- No impact on main work

#### US-DEV-003: Debugging Production
**As a** developer investigating a production bug
**I want to** recreate the production environment locally
**So that** I can reproduce and fix the issue
**Acceptance Criteria**:
- Import production environment specification
- Anonymized production data import
- Same dependency versions as production
- Debug tools available

### 6.3 Team Collaboration Stories

#### US-TEAM-001: Sharing Setup
**As a** tech lead
**I want to** share my working environment with the team
**So that** everyone has a consistent setup
**Acceptance Criteria**:
- Export environment definition
- Team members can import and use
- Version controlled alongside code
- Update notifications

#### US-TEAM-002: Onboarding Automation
**As a** hiring manager
**I want to** reduce new hire onboarding time
**So that** new team members are productive quickly
**Acceptance Criteria**:
- New hire runs single command
- Environment identical to team standard
- All access credentials pre-configured
- First commit possible within first hour

#### US-TEAM-003: CI/CD Parity
**As a** DevOps engineer
**I want to** use the same environment locally and in CI
**So that** "works locally" means "works in CI"
**Acceptance Criteria**:
- bed environments usable in CI pipelines
- Identical configuration between local and CI
- Reproduce CI failures locally
- Fast CI environment startup

### 6.4 Advanced Use Cases

#### US-ADV-001: Security Research
**As a** security researcher
**I want to** analyze code in an isolated environment
**So that** I can safely examine potentially malicious code
**Acceptance Criteria**:
- Network-isolated workspace
- No access to sensitive resources
- Easy reset to clean state
- Audit logging of all actions

#### US-ADV-002: Training Environment
**As a** trainer
**I want to** provide identical environments to all students
**So that** instruction is consistent
**Acceptance Criteria**:
- Pre-configured training environments
- Reset to initial state between sessions
- Share exercises as bed configurations
- Progress checkpoints

---

## 7. Feature Specifications

### 7.1 Command Structure

```
bed
├── init              # Initialize bed in current project
├── create            # Create new workspace
├── activate          # Enter workspace
├── deactivate        # Exit workspace
├── list              # List workspaces
├── status            # Show workspace status
├── destroy           # Remove workspace
├── snapshot          # Manage snapshots
│   ├── create
│   ├── list
│   ├── restore
│   └── delete
├── export            # Export environment
├── import            # Import environment
├── template          # Manage templates
│   ├── list
│   ├── install
│   └── create
├── service           # Manage services
│   ├── start
│   ├── stop
│   ├── restart
│   ├── logs
│   └── status
├── config            # Configuration management
│   ├── get
│   ├── set
│   └── validate
└── doctor            # Diagnose issues
```

### 7.2 Configuration Schema

```toml
# bed.toml - Environment Definition
name = "my-project"
description = "Development environment for my-project"
version = "1.0.0"

[base]
image = "node:18"
template = "node-fullstack"

[packages]
system = ["git", "curl", "jq"]
npm = ["typescript", "ts-node", "nodemon"]
python = { version = "3.11", packages = ["black", "pytest"] }

[services]
[services.postgres]
image = "postgres:15"
ports = ["5432:5432"]
env = { POSTGRES_PASSWORD = "${secrets.db_password}" }
volumes = ["postgres_data:/var/lib/postgresql/data"]
health_check = { command = "pg_isready", interval = "5s" }

[services.redis]
image = "redis:7-alpine"
ports = ["6379:6379"]

[environment]
NODE_ENV = "development"
DATABASE_URL = "postgres://postgres:${secrets.db_password}@localhost:5432/dev"
REDIS_URL = "redis://localhost:6379"

[mounts]
source = "."
target = "/workspace"

[ports]
app = "3000"
api = "8080"

[scripts]
post_create = "npm install && npm run db:migrate"
pre_activate = "npm run db:ensure"
post_destroy = "echo 'Workspace destroyed'"

[secrets]
provider = "1password"
vault = "Development"
items = ["db_password", "api_key"]
```

### 7.3 Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Layer                           │
├─────────────────────────────────────────────────────────────┤
│                     Core Engine                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │  Parser     │  │  Lifecycle  │  │  Service Orchestr.│  │
│  │  (TOML)     │  │  Manager    │  │                     │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                    Backend Adapters                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Docker    │  │   Podman    │  │   VM (Lima, etc)    │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                    Integration Layer                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Secrets   │  │   Registry  │  │   IDE Plugins       │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### 7.4 Workspace States

```
┌──────────┐    create     ┌──────────┐   activate   ┌──────────┐
│ Defined  │ ─────────────>│  Created │ ────────────>│  Active  │
│  (TOML)  │               │          │              │          │
└──────────┘               └──────────┘              └────┬─────┘
                                                          │
                    ┌───────────────────────────────────────┘
                    │ deactivate
                    ▼
              ┌──────────┐
              │Inactive/ │
              │Suspended │
              └────┬─────┘
                   │ destroy
                   ▼
              ┌──────────┐
              │Destroyed │
              └──────────┘
```

---

## 8. Success Metrics

### 8.1 Adoption Metrics

| Metric | Baseline | Target (6mo) | Target (12mo) |
|--------|----------|--------------|---------------|
| Active Workspaces | 0 | 500 | 5,000 |
| Teams Using bed | 0 | 20 | 100 |
| Environment Creation/Week | 0 | 1,000 | 10,000 |
| GitHub Stars | 0 | 300 | 1,500 |

### 8.2 Efficiency Metrics

| Metric | Baseline | Target |
|--------|----------|--------|
| Setup Time | Days | < 10 minutes |
| Context Switch Time | 30+ minutes | < 1 minute |
| Environment Bug Reports | High | -50% |
| New Hire Productivity | Day 3 | Day 1 |

### 8.3 Technical Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Creation Success Rate | > 95% | Telemetry |
| Average Creation Time | < 60s | Performance tests |
| Test Coverage | > 85% | Code coverage |
| Platform Parity | 100% | Test matrix |

### 8.4 Satisfaction Metrics

| Metric | Target | Method |
|--------|--------|--------|
| Developer NPS | > 40 | Quarterly survey |
| Support Tickets | < 10/week | Ticket tracking |
| Documentation Rating | > 4/5 | In-app feedback |

---

## 9. Release Criteria

### 9.1 Version 0.1 (Alpha)

**Target**: Q2 2026

**Must Have**:
- [ ] Docker backend
- [ ] TOML configuration
- [ ] Create/destroy lifecycle
- [ ] Service orchestration
- [ ] Basic IDE integration (VS Code)
- [ ] macOS and Linux support

### 9.2 Version 0.5 (Beta)

**Target**: Q3 2026

**Must Have**:
- [ ] All alpha features stable
- [ ] Secret management integration
- [ ] Template system
- [ ] Windows support
- [ ] Snapshot/checkpoint
- [ ] Team registry
- [ ] CI/CD integration

### 9.3 Version 1.0 (GA)

**Target**: Q4 2026

**Must Have**:
- [ ] All beta features stable
- [ ] Security audit passed
- [ ] Performance benchmarks met
- [ ] Documentation complete
- [ ] 90% test coverage
- [ ] Migration path from DevContainers

### 9.4 Exit Criteria

**Release Ready When**:
1. All acceptance criteria met
2. Security review complete
3. Performance targets achieved
4. Documentation published
5. No P0/P1 bugs open
6. Community feedback incorporated

---

## 10. Appendix

### 10.1 Glossary

- **Workspace**: An isolated development environment instance
- **Environment**: The configuration defining a workspace
- **Service**: A dependency like a database or message queue
- **Snapshot**: A saved point-in-time state of a workspace
- **Template**: A reusable environment configuration
- **Backend**: The container/VM technology powering workspaces

### 10.2 Competitive Analysis

| Feature | bed | DevContainers | Nix | devenv | Gitpod |
|---------|-----|---------------|-----|--------|--------|
| Local First | Yes | Yes | Yes | Yes | No |
| Declarative | Yes | Yes | Yes | Yes | Partial |
| Ephemeral | Yes | No | No | No | Yes |
| Multi-Backend | Yes | No | N/A | N/A | No |
| IDE Agnostic | Yes | Partial | Yes | Yes | Yes |
| Secret Mgmt | Yes | No | Partial | Partial | Yes |
| Team Sharing | Yes | Partial | Yes | Partial | Yes |

### 10.3 Technology Stack

- **Core**: Rust (performance, safety)
- **Configuration**: TOML (readability)
- **Backend**: Docker/Podman API
- **Networking**: Custom bridge networks
- **Storage**: Overlay filesystems
- **Secrets**: 1Password/Vault SDKs

---

*Document Version*: 1.0  
*Last Updated*: 2026-04-05  
*Author*: Phenotype Architecture Team  
*Status*: Draft for Review
