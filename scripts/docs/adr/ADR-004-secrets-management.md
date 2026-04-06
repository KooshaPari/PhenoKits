# ADR-004: Secrets Management Strategy

**Status**: Proposed  
**Date**: 2026-04-04  
**Deciders**: Phenotype Architecture Team  

---

## Context

The Phenotype ecosystem manages multiple types of sensitive credentials across repositories:

- **AI Testing APIs**: Qodo, Applitools, TestRigor
- **Notification Services**: Slack webhooks, PagerDuty keys
- **Observability**: Prometheus endpoints (potentially sensitive)
- **GitHub**: API tokens for automation

Currently, `setup-ai-testing-secrets.sh` provides a basic mechanism for configuring GitHub repository secrets using the `gh` CLI. However, as the ecosystem grows, we need a comprehensive strategy for:

1. Secret storage (where credentials live)
2. Secret distribution (how they reach applications)
3. Secret rotation (how often they change)
4. Secret auditing (who accessed what, when)
5. Environment separation (dev/staging/prod isolation)

This ADR establishes the secrets management architecture for the Phenotype organization.

---

## Decision Drivers

| Driver | Priority | Notes |
|--------|----------|-------|
| Security | Critical | Prevent credential leaks |
| Operational Simplicity | High | Must not impede development |
| Cost | High | Prefer free/OSS solutions |
| Auditability | Medium | Track access and changes |
| Rotation | Medium | Automated rotation preferred |
| Multi-environment | Medium | Dev/staging/prod isolation |
| Recovery | Medium | Disaster recovery capability |

---

## Threat Model

### Assets

| Asset | Sensitivity | Exposure Risk |
|-------|-------------|---------------|
| Qodo API Key | High | Unauthorized AI tool access |
| Applitools API Key | High | Visual testing abuse |
| Slack Webhook URL | Medium | Spam/fake notifications |
| PagerDuty Key | High | False incident creation |
| Prometheus URL | Low-Medium | Infrastructure reconnaissance |

### Threats

| Threat | Likelihood | Impact | Mitigation Priority |
|--------|------------|--------|---------------------|
| Accidental commit | High | High | Critical |
| Developer machine compromise | Medium | High | High |
| CI log exposure | Medium | Medium | High |
| Insider threat | Low | High | Medium |
| Vendor breach (Qodo, etc.) | Low | High | Medium |

---

## Options Considered

### Option 1: GitHub Secrets + Local Env (Current, Extended)

**Description**: Continue using GitHub repository secrets for CI/CD, local environment variables for development, with added structure and tooling.

**Architecture**:
```
┌─────────────────────────────────────────────────────────────────────────┐
│                    GitHub Secrets + Local Environment                   │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Development                                                            │
│  ┌─────────────────────────────────────────────────────────────────┐  │
│  │  ~/.config/phenotype/secrets.env                                 │  │
│  │  (sourced by scripts, gitignored)                                 │  │
│  │                                                                  │  │
│  │  export QODO_API_KEY="sk-..."                                    │  │
│  │  export SLACK_WEBHOOK="https://..."                              │  │
│  └─────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  CI/CD (GitHub Actions)                                                 │
│  ┌─────────────────────────────────────────────────────────────────┐  │
│  │  Repository Secrets                                              │  │
│  │  - QODO_API_KEY (per repo)                                       │  │
│  │  - SLACK_WEBHOOK (per repo)                                      │  │
│  │                                                                  │  │
│  │  Environment Secrets (optional)                                │  │
│  │  - STAGING_PROMETHEUS_URL                                        │  │
│  │  - PROD_PROMETHEUS_URL                                           │  │
│  └─────────────────────────────────────────────────────────────────┘  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

**Pros**:
- No additional infrastructure
- Native GitHub integration
- Zero cost
- Well-understood by team

**Cons**:
- No automatic rotation
- Limited audit logging
- No cross-repo secret sharing
- Manual secret distribution to developers

**Cost**: Free

---

### Option 2: HashiCorp Vault (Selected for Future)

**Description**: Deploy HashiCorp Vault for enterprise-grade secret management with dynamic credentials, automatic rotation, and comprehensive audit logging.

**Architecture**:
```
┌─────────────────────────────────────────────────────────────────────────┐
│                         HashiCorp Vault Architecture                      │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐  │
│  │                    Vault Server                                  │  │
│  │                                                                  │  │
│  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐       │  │
│  │  │   KV Secrets  │  │  Dynamic      │  │   PKI         │       │  │
│  │  │   (Static)    │  │  (Database)   │  │   (TLS)       │       │  │
│  │  └───────┬───────┘  └───────┬───────┘  └───────┬───────┘       │  │
│  │          │                  │                  │                │  │
│  │          └──────────────────┴──────────────────┘                │  │
│  │                             │                                  │  │
│  │  ┌──────────────────────────┴──────────────────────────┐     │  │
│  │  │                    Auth Methods                        │     │  │
│  │  │  - GitHub (for CI)                                   │     │  │
│  │  │  - AppRole (for apps)                                │     │  │
│  │  │  - OIDC (for developers)                             │     │  │
│  │  └──────────────────────────────────────────────────────┘     │  │
│  └─────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  Access Patterns:                                                       │
│  ┌──────────────┐     ┌──────────────┐     ┌──────────────┐            │
│  │  Developer   │     │  CI/CD       │     │  Application │            │
│  │  (OIDC)      │────►│  (GitHub     │────►│  (AppRole)   │            │
│  │              │     │  Auth)       │     │              │            │
│  └──────────────┘     └──────────────┘     └──────────────┘            │
│                                                                         │
│  Audit:                                                                 │
│  ┌─────────────────────────────────────────────────────────────────┐  │
│  │  Audit Log → Splunk/ELK → Alerting                              │  │
│  └─────────────────────────────────────────────────────────────────┘  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

**Pros**:
- Industry standard for secret management
- Dynamic secrets (automatic rotation)
- Comprehensive audit logging
- Multiple auth methods
- KV v2 with versioning
- Automatic TLS certificates
- Policy-as-code access control

**Cons**:
- Operational complexity (HA deployment)
- Learning curve for team
- Infrastructure cost (~$50-200/month for small setup)
- Requires ongoing maintenance

**Cost**: ~$100/month (self-hosted on AWS/GCP)

---

### Option 3: Doppler (SaaS)

**Description**: Use Doppler managed service for secrets management with excellent developer experience.

**Architecture**:
```
┌─────────────────────────────────────────────────────────────────────────┐
│                           Doppler Architecture                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Doppler Cloud                                                          │
│  ┌─────────────────────────────────────────────────────────────────┐  │
│  │  Projects:                                                       │  │
│  │  - AgilePlus (dev, staging, prod)                              │  │
│  │  - heliosCLI (dev, staging, prod)                                │  │
│  │  - scripts (shared secrets)                                      │  │
│  │                                                                  │  │
│  │  Features:                                                       │  │
│  │  - Secret referencing (QODO_API_KEY → project://...)            │  │
│  │  - Version history                                             │  │
│  │  - Access logs                                                   │  │
│  │  - Personal secrets override                                     │  │
│  └─────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  Integration:                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                  │
│  │  CLI         │  │  GitHub      │  │  Docker      │                  │
│  │  (doppler    │  │  Action      │  │  Inject      │                  │
│  │   run)       │  │              │  │              │                  │
│  └──────────────┘  └──────────────┘  └──────────────┘                  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

**Pros**:
- Excellent developer experience
- No infrastructure to manage
- GitHub Actions integration
- Secret referencing across projects
- Version history
- Reasonable pricing

**Cons**:
- Vendor lock-in
- SaaS dependency
- Cost scales with team
- Data in third-party system

**Cost**: $5-18/seat/month (5 developers = $25-90/month)

---

### Option 4: 1Password Secrets Automation

**Description**: Use 1Password for both personal passwords and service secrets with Secrets Automation feature.

**Pros**:
- Unified with team password manager
- Excellent security pedigree
- Familiar UI for developers
- Biometric unlock

**Cons**:
- Secrets Automation is relatively new
- Less mature API than competitors
- No dynamic secrets
- Higher per-seat cost

**Cost**: $20/dev/month minimum

---

### Option 5: AWS/GCP/Azure Native

**Description**: Use cloud provider secret managers (AWS Secrets Manager, GCP Secret Manager, Azure Key Vault).

**Pros**:
- Native cloud integration
- IAM-based access control
- Automatic rotation (AWS)
- High availability

**Cons**:
- Vendor lock-in to specific cloud
- Cross-cloud usage complex
- Cost varies by provider
- Requires cloud account

**Cost**: $0.40/secret/month (AWS), $0.06/10K ops (GCP)

---

## Decision

**Chosen Approach**: Hybrid (Option 1 + Option 2 Path)

**Immediate (Q2 2026)**: Enhanced GitHub Secrets + Local Environment
**Future (Q4 2026)**: Migrate to HashiCorp Vault

**Rationale**:
1. **Current scale**: GitHub Secrets sufficient for 5 repositories, 5 developers
2. **Security needs**: gitleaks + pre-commit provides adequate leak prevention
3. **Cost sensitivity**: Free solution preferred at current stage
4. **Migration path**: Clean upgrade path to Vault when needed
5. **Operational bandwidth**: No time for Vault operations currently

**Triggers for Vault Migration**:
- Team grows beyond 10 developers
- Need for automatic secret rotation
- Compliance requirements (SOC2, etc.)
- Multi-cloud secret sharing needed
- Dynamic database credentials needed

---

## Implementation

### Phase 1: Enhanced GitHub Secrets (Immediate)

#### 1.1 Secret Naming Convention

```
# Repository-specific secrets
<REPO>_<ENV>_<NAME>

Examples:
AGILEPLUS_DEV_QODO_API_KEY
AGILEPLUS_PROD_QODO_API_KEY
HELIOSCLI_STAGING_SLACK_WEBHOOK

# Shared secrets (in all repos)
SHARED_<NAME>

Examples:
SHARED_QODO_API_KEY
SHARED_SLACK_WEBHOOK
```

#### 1.2 Local Development Setup

Create `scripts/setup-local-secrets.sh`:

```bash
#!/bin/bash
# Setup local development secrets

SECRETS_DIR="${HOME}/.config/phenotype"
SECRETS_FILE="${SECRETS_DIR}/secrets.env"

mkdir -p "$SECRETS_DIR"

if [ ! -f "$SECRETS_FILE" ]; then
    cat > "$SECRETS_FILE" << 'EOF'
# Phenotype Local Secrets
# This file is sourced by scripts. DO NOT COMMIT.

# AI Testing
export QODO_API_KEY=""
export APPLITOOLS_API_KEY=""
export TESTRIGOR_API_KEY=""

# Notifications
export SLACK_WEBHOOK=""
export PAGERDUTY_KEY=""

# Observability
export PROMETHEUS_URL="http://localhost:9090"
EOF
    chmod 600 "$SECRETS_FILE"
    echo "Created $SECRETS_FILE"
    echo "Please edit it and add your secrets"
else
    echo "$SECRETS_FILE already exists"
fi
```

#### 1.3 Secret Loading Pattern

Update scripts to load local secrets:

```bash
#!/bin/bash
# Source local secrets if available
if [ -f "${HOME}/.config/phenotype/secrets.env" ]; then
    # shellcheck source=/dev/null
    source "${HOME}/.config/phenotype/secrets.env"
fi

# Check required secrets
if [ -z "${QODO_API_KEY:-}" ]; then
    echo "Error: QODO_API_KEY not set"
    echo "Run: scripts/setup-local-secrets.sh"
    exit 1
fi
```

### Phase 2: Secret Distribution Tooling

#### 2.1 Enhanced setup-ai-testing-secrets.sh

Extend to support environment-specific secrets:

```bash
#!/bin/bash
# Setup AI Testing Infrastructure secrets
# Usage: ./scripts/setup-ai-testing-secrets.sh [dev|staging|prod]

ENV=${1:-dev}
REPO_SLUG="${REPO_SLUG:-KooshaPari/$(basename "$PWD")}"

echo "Setting up $ENV secrets for $REPO_SLUG"

# Map environment to secret names
case $ENV in
    dev)
        QODO_KEY="${QODO_API_KEY_DEV:-$QODO_API_KEY}"
        ;;
    staging)
        QODO_KEY="${QODO_API_KEY_STAGING:-$QODO_API_KEY}"
        ;;
    prod)
        QODO_KEY="${QODO_API_KEY_PROD:-$QODO_API_KEY}"
        ;;
esac

# Set secrets with environment prefix
echo "$QODO_KEY" | gh secret set "${ENV^^}_QODO_API_KEY" --repo "$REPO_SLUG"
```

#### 2.2 Secret Sync Tool

Create `scripts/sync-secrets.sh` for cross-repo sync:

```bash
#!/bin/bash
# Sync shared secrets across repositories

SHARED_SECRETS=("QODO_API_KEY" "SLACK_WEBHOOK")
REPOS=("AgilePlus" "helioscli" "heliosApp" "thegent")

for secret in "${SHARED_SECRETS[@]}"; do
    value="${!secret}"
    if [ -z "$value" ]; then
        echo "Warning: $secret not set in environment"
        continue
    fi
    
    for repo in "${REPOS[@]}"; do
        echo "Syncing $secret to $repo..."
        echo "$value" | gh secret set "$secret" --repo "KooshaPari/$repo"
    done
done
```

### Phase 3: Audit and Monitoring

#### 3.1 Secret Access Logging

Create wrapper for secret access:

```bash
#!/bin/bash
# Log secret access
# Usage: with_secret SECRET_NAME command...

SECRET_NAME="$1"
shift

echo "$(date -Iseconds) ACCESS $SECRET_NAME $USER $(pwd)" >> ~/.config/phenotype/secret-audit.log

# Get secret value
SECRET_VALUE="${!SECRET_NAME}"
if [ -z "$SECRET_VALUE" ]; then
    echo "Error: Secret $SECRET_NAME not available"
    exit 1
fi

# Run command with secret available
export "$SECRET_NAME"="$SECRET_VALUE"
exec "$@"
```

---

## Security Controls

### Prevention

| Control | Implementation | Effectiveness |
|---------|------------------|---------------|
| Pre-commit hooks | gitleaks | High |
| File permissions | 600 on secrets.env | Medium |
| Environment isolation | Per-repo secrets | Medium |
| Naming convention | Prevents confusion | Low-Medium |

### Detection

| Control | Implementation | Effectiveness |
|---------|------------------|---------------|
| GitHub secret scanning | Built-in | Medium |
| gitleaks in CI | Weekly scan | Medium |
| Audit logging | Custom wrapper | Low |

### Response

| Scenario | Response |
|----------|----------|
| Secret committed | 1. Rotate immediately 2. Review access logs 3. Assess impact |
| Developer leaves | 1. Rotate all shared secrets 2. Remove GitHub access |
| Vendor breach | 1. Rotate affected secrets 2. Monitor for abuse |

---

## Migration Plan to Vault (Future)

### Timeline

| Quarter | Milestone |
|---------|-----------|
| Q2 2026 | Enhanced GitHub Secrets (Phase 1-3 complete) |
| Q3 2026 | Vault proof-of-concept |
| Q4 2026 | Vault production deployment (if needed) |
| Q1 2027 | GitHub Secrets deprecation |

### Vault Architecture (Future)

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    Future Vault Deployment                              │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │  Vault HA Cluster (3 nodes)                                      │   │
│  │  - Auto-unseal with cloud KMS                                   │   │
│  │  - Integrated storage (Raft)                                    │   │
│  │  - TLS everywhere                                               │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  Secret Engines:                                                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                     │
│  │ KV v2       │  │ GitHub      │  │ Database    │                     │
│  │ (static)    │  │ (dynamic)   │  │ (dynamic)   │                     │
│  │             │  │ tokens      │  │ creds       │                     │
│  └─────────────┘  └─────────────┘  └─────────────┘                     │
│                                                                         │
│  Auth Methods:                                                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                     │
│  │ GitHub      │  │ AppRole     │  │ OIDC        │                     │
│  │ (CI/CD)     │  │ (apps)      │  │ (humans)    │                     │
│  └─────────────┘  └─────────────┘  └─────────────┘                     │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Compliance Mapping

| Requirement | Current | Future (Vault) |
|-------------|---------|----------------|
| Encryption at rest | GitHub provides | AES-256-GCM |
| Encryption in transit | TLS 1.3 | TLS 1.3 |
| Access logging | Limited | Comprehensive |
| Rotation | Manual | Automatic (dynamic) |
| Audit trail | GitHub logs | Full audit log |
| Recovery | GitHub support | Backup/DR plan |

---

## References

1. [GitHub Secrets Documentation](https://docs.github.com/en/actions/security-guides/encrypted-secrets) - GitHub official docs
2. [HashiCorp Vault](https://www.vaultproject.io/) - Secret management platform
3. [Doppler](https://www.doppler.com/) - Secrets management SaaS
4. [OWASP Secrets Management](https://cheatsheetseries.owasp.org/cheatsheets/Secrets_Management_Cheat_Sheet.html) - Best practices
5. [gitleaks](https://github.com/gitleaks/gitleaks) - Secret detection

---

## Appendix: Secret Inventory

| Secret | Type | Rotation Frequency | Repositories | Environment |
|--------|------|-------------------|--------------|-------------|
| QODO_API_KEY | API Key | Quarterly | All AI testing repos | All |
| APPLITOOLS_API_KEY | API Key | Quarterly | AgilePlus | All |
| TESTRIGOR_API_KEY | API Key | Quarterly | AgilePlus | All |
| SLACK_WEBHOOK | URL | Annually | All | All |
| PAGERDUTY_KEY | API Key | Quarterly | Observability repos | Prod |
| PROMETHEUS_URL | URL | As needed | Observability repos | Per-env |

---

*Last Updated: 2026-04-04*  
*Supersedes: N/A*  
*Related: setup-ai-testing-secrets.sh, .pre-commit-config.yaml (gitleaks)*
