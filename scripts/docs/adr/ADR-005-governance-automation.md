# ADR-005: Governance Automation Evolution

**Status**: Proposed  
**Date**: 2026-04-04  
**Deciders**: Phenotype Architecture Team  

---

## Context

The Phenotype ecosystem has established initial governance controls through:

- **scaffold-smoke.sh**: Validates required scaffold files exist
- **traceability-check.py**: Validates FR/SPEC markers against traceability.json
- **validate-governance.sh**: Placeholder for future governance validation
- **pre-commit hooks**: Basic code quality and security checks

As the ecosystem scales (target: 50+ repositories, 20+ developers), manual governance approaches will become bottlenecks. Current gaps include:

1. **No automated policy enforcement** beyond file existence
2. **No cross-repository governance** aggregation
3. **No drift detection** from approved patterns
4. **No compliance reporting** for stakeholders
5. **Limited remediation guidance** when issues found

This ADR establishes the evolution path from simple file-check governance to comprehensive policy-as-code automation.

---

## Current State Analysis

### Existing Governance Tools

| Tool | Coverage | Automation | Enforcement | Reporting |
|------|----------|------------|-------------|-----------|
| scaffold-smoke.sh | File existence | Yes | Fail CI | Minimal |
| traceability-check.py | FR/SPEC markers | Yes | Fail CI | Per-repo |
| validate-governance.sh | Placeholder | No | No | No |
| pre-commit | Code quality | Yes | Block commit | None |

### Governance Gaps

| Gap | Impact | Current Workaround |
|-----|--------|-------------------|
| Policy definition | No formal rules | ADRs and conventions |
| Drift detection | No automatic detection | Manual review |
| Cross-repo view | Cannot see org-wide status | Manual aggregation |
| Remediation guidance | Generic error messages | Documentation |
| Compliance metrics | No dashboards | None |

---

## Decision Drivers

| Driver | Priority | Notes |
|--------|----------|-------|
| Scale | High | 50+ repos require automation |
| Consistency | High | All repos should follow patterns |
| Developer Experience | High | Governance should help, not hinder |
| Compliance | Medium | Future audit requirements |
| Cost | High | Prefer OSS/free solutions |
| Maintainability | High | Small team, limited ops bandwidth |

---

## Options Considered

### Option 1: Enhanced Scripts (Current + Extensions)

**Description**: Extend existing scripts with more sophisticated checks while maintaining the current approach.

**Additions**:
- `validate-governance.sh`: Implement comprehensive checks
- `governance-report.sh`: Aggregate cross-repo status
- `governance-fix.sh`: Auto-remediation for common issues

**Architecture**:
```
┌─────────────────────────────────────────────────────────────────────────┐
│                    Enhanced Scripts Approach                            │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │              governance.json (per-repo config)                   │   │
│  │                                                                  │   │
│  │  {                                                               │   │
│  │    "required_files": ["README.md", "LICENSE"],                 │   │
│  │    "required_markers": ["FR-"],                                 │   │
│  │    "code_quality": { "shellcheck": true, "ruff": true },       │   │
│  │    "dependencies": { "up_to_date": true }                       │   │
│  │  }                                                               │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │              validate-governance.sh                               │   │
│  │                                                                  │   │
│  │  - Check required files exist                                   │   │
│  │  - Check traceability markers                                   │   │
│  │  - Run linting tools                                            │   │
│  │  - Check dependency freshness                                   │   │
│  │  - Validate ADR coverage                                        │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │              governance-report.sh                                 │   │
│  │                                                                  │   │
│  │  Aggregate:                                                      │   │
│  │  - Compliance % across repos                                     │   │
│  │  - Common violations                                             │   │
│  │  - Trend over time                                               │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

**Pros**:
- Builds on existing tooling
- No new infrastructure
- Familiar patterns for team
- Low learning curve

**Cons**:
- Limited scalability
- No declarative policy definition
- Harder to maintain complex rules
- No ecosystem integration

---

### Option 2: Open Policy Agent (OPA) (Selected for Future)

**Description**: Implement policy-as-code using Open Policy Agent with Rego policies for comprehensive governance.

**Architecture**:
```
┌─────────────────────────────────────────────────────────────────────────┐
│                    Open Policy Agent Architecture                       │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Policy Definition (Rego)                                               │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │  package phenotype.governance                                     │   │
│  │                                                                  │   │
│  │  # Required files in every repository                            │   │
│  │  required_files := ["README.md", "LICENSE", "SPEC.md"]          │   │
│  │                                                                  │   │
│  │  deny[msg] {                                                     │   │
│  │    not input.files["README.md"]                                 │   │
│  │    msg := "Repository missing README.md"                        │   │
│  │  }                                                               │   │
│  │                                                                  │   │
│  │  # Traceability requirements                                     │   │
│  │  deny[msg] {                                                     │   │
│  │    count(input.traceability.markers) < 5                        │   │
│  │    msg := "Insufficient traceability markers"                   │   │
│  │  }                                                               │   │
│  │                                                                  │   │
│  │  # Code quality gates                                            │   │
│  │  deny[msg] {                                                     │   │
│  │    input.quality.shellcheck.violations > 0                      │   │
│  │    msg := sprintf("ShellCheck violations: %d",                  │   │
│  │           [input.quality.shellcheck.violations])                │   │
│  │  }                                                               │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  OPA Server / conftest (CI)                                             │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                                                                  │   │
│  │  Input (JSON):                                                   │   │
│  │  {                                                               │   │
│  │    "repository": "AgilePlus",                                   │   │
│  │    "files": { "README.md": true, ... },                         │   │
│  │    "traceability": { "markers": [...] },                          │   │
│  │    "quality": { "shellcheck": {...}, "ruff": {...} }           │   │
│  │  }                                                               │   │
│  │                                                                  │   │
│  │  Evaluation:                                                     │   │
│  │  - Load policies                                                 │   │
│  │  - Evaluate against input                                        │   │
│  │  - Return violations                                             │   │
│  │                                                                  │   │
│  │  Output:                                                         │   │
│  │  {                                                               │   │
│  │    "compliant": false,                                          │   │
│  │    "violations": [...],                                         │   │
│  │    "remediation": [...]                                          │   │
│  │  }                                                               │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  CI Integration                                                           │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │  - conftest test (local)                                         │   │
│  │  - OPA server (enterprise scale)                                 │   │
│  │  - Gatekeeper (Kubernetes, future)                               │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

**Pros**:
- Industry standard for policy-as-code
- Declarative, human-readable policies
- Extensive ecosystem
- Kubernetes integration (future)
- Strong open source community

**Cons**:
- Learning curve (Rego language)
- Additional infrastructure
- New tool for team to learn
- Overkill for current scale

---

### Option 3: Scorecard / Allstar

**Description**: Use OpenSSF Scorecard and Allstar for automated security posture management.

**Pros**:
- Purpose-built for GitHub security
- Automatic monitoring
- Security scorecard generation
- Free for open source

**Cons**:
- Security-focused only
- Limited customization
- GitHub-specific
- Less control over policies

---

### Option 4: Custom Platform

**Description**: Build a custom governance platform using Rust/Go with a web dashboard.

**Pros**:
- Complete control
- Tailored to exact needs
- Learning opportunity

**Cons**:
- Significant development effort
- Maintenance burden
- Reinventing existing solutions
- Not justified at current scale

---

## Decision

**Chosen Approach**: Hybrid Evolution

**Phase 1 (Immediate - Q2 2026)**: Enhanced Scripts
- Implement comprehensive `validate-governance.sh`
- Create `governance-report.sh` for cross-repo aggregation
- Add `governance.json` configuration files

**Phase 2 (Future - Q4 2026)**: OPA Migration
- Migrate to Open Policy Agent when scale requires it
- Maintain policy continuity through migration

**Rationale**:
1. **Current needs met**: Enhanced scripts sufficient for 5-10 repos
2. **Future-proof**: Clear migration path to OPA
3. **Cost**: No new infrastructure until needed
4. **Complexity**: Avoid learning curve until necessary

**Triggers for OPA Migration**:
- More than 15 repositories
- Complex conditional policies needed
- Kubernetes governance required
- Team comfortable with current approach

---

## Implementation Plan

### Phase 1: Enhanced Scripts (Q2 2026)

#### 1.1 governance.json Schema

```json
{
  "version": "1.0",
  "repository": {
    "name": "scripts",
    "type": "automation",
    "tier": "critical"
  },
  "requirements": {
    "files": {
      "required": [
        "README.md",
        "SPEC.md",
        "LICENSE"
      ],
      "recommended": [
        "CONTRIBUTING.md",
        "CHANGELOG.md"
      ]
    },
    "documentation": {
      "adr_required": true,
      "adr_minimum": 3,
      "spec_required": true,
      "readme_minimum_lines": 50
    },
    "traceability": {
      "markers_required": true,
      "minimum_markers": 5,
      "strict_mode": true
    },
    "code_quality": {
      "shellcheck": {
        "enabled": true,
        "severity": "warning"
      },
      "python_lint": {
        "enabled": true,
        "tool": "ruff"
      }
    },
    "security": {
      "gitleaks": true,
      "dependency_scan": true,
      "vulnerability_threshold": "high"
    }
  },
  "exemptions": []
}
```

#### 1.2 validate-governance.sh Implementation

```bash
#!/usr/bin/env bash
# validate-governance.sh - Comprehensive governance validation

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONFIG_FILE="${REPO_ROOT}/governance.json"
RESULTS=()
VIOLATIONS=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $*"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*"; }

check_required_files() {
    local config="$1"
    log_info "Checking required files..."
    
    local files
    files=$(echo "$config" | jq -r '.requirements.files.required[]')
    
    for file in $files; do
        if [ -f "${REPO_ROOT}/${file}" ]; then
            RESULTS+=("✓ $file exists")
        else
            RESULTS+=("✗ $file missing")
            ((VIOLATIONS++))
        fi
    done
}

check_traceability() {
    log_info "Checking traceability markers..."
    
    if [ -f "${REPO_ROOT}/traceability.json" ]; then
        local count
        count=$(find "${REPO_ROOT}" -type f \( -name "*.rs" -o -name "*.py" -o -name "*.ts" \) -exec grep -h "FR-" {} + | wc -l)
        
        if [ "$count" -ge 5 ]; then
            RESULTS+=("✓ Traceability: $count markers found")
        else
            RESULTS+=("✗ Traceability: only $count markers (min 5)")
            ((VIOLATIONS++))
        fi
    else
        RESULTS+=("✗ traceability.json not found")
        ((VIOLATIONS++))
    fi
}

check_code_quality() {
    log_info "Checking code quality..."
    
    # Shellcheck
    if command -v shellcheck &> /dev/null; then
        local shell_files
        shell_files=$(find "${REPO_ROOT}" -name "*.sh" -type f)
        
        for file in $shell_files; do
            if shellcheck -S warning "$file" 2>/dev/null; then
                RESULTS+=("✓ shellcheck passed: $(basename "$file")")
            else
                RESULTS+=("✗ shellcheck failed: $(basename "$file")")
                ((VIOLATIONS++))
            fi
        done
    else
        log_warn "shellcheck not installed, skipping"
    fi
}

generate_report() {
    echo ""
    echo "========================================"
    echo "     GOVERNANCE VALIDATION REPORT       "
    echo "========================================"
    echo ""
    
    for result in "${RESULTS[@]}"; do
        echo "  $result"
    done
    
    echo ""
    echo "========================================"
    echo "  Total violations: $VIOLATIONS"
    echo "  Status: $([ "$VIOLATIONS" -eq 0 ] && echo "✓ PASS" || echo "✗ FAIL")"
    echo "========================================"
    
    return "$VIOLATIONS"
}

main() {
    if [ ! -f "$CONFIG_FILE" ]; then
        log_error "governance.json not found"
        exit 1
    fi
    
    local config
    config=$(cat "$CONFIG_FILE")
    
    check_required_files "$config"
    check_traceability
    check_code_quality
    
    generate_report
}

main "$@"
```

#### 1.3 governance-report.sh (Cross-Repo)

```bash
#!/usr/bin/env bash
# governance-report.sh - Aggregate governance status across repos

REPOS_FILE="${1:-repos.txt}"
REPORT_FILE="/tmp/governance-report-$(date +%Y%m%d).json"

declare -A REPO_STATUS
declare -A REPO_VIOLATIONS

echo "Generating governance report..."

while IFS= read -r repo_path; do
    [ -z "$repo_path" ] && continue
    [ ! -d "$repo_path" ] && continue
    
    repo_name=$(basename "$repo_path")
    
    if [ -f "${repo_path}/scripts/validate-governance.sh" ]; then
        cd "$repo_path"
        if ./scripts/validate-governance.sh > /dev/null 2>&1; then
            REPO_STATUS[$repo_name]="PASS"
            REPO_VIOLATIONS[$repo_name]=0
        else
            REPO_STATUS[$repo_name]="FAIL"
            REPO_VIOLATIONS[$repo_name]=$?
        fi
    else
        REPO_STATUS[$repo_name]="NO_GOV"
        REPO_VIOLATIONS[$repo_name]=0
    fi
done < "$REPOS_FILE"

# Generate JSON report
cat > "$REPORT_FILE" << EOF
{
  "generated_at": "$(date -Iseconds)",
  "repositories": [
$(for repo in "${!REPO_STATUS[@]}"; do
    echo "    {"
    echo "      \"name\": \"$repo\","
    echo "      \"status\": \"${REPO_STATUS[$repo]}\","
    echo "      \"violations\": ${REPO_VIOLATIONS[$repo]}"
    echo "    },"
done | sed '$ s/,$//')
  ],
  "summary": {
    "total": ${#REPO_STATUS[@]},
    "pass": $(printf '%s\n' "${REPO_STATUS[@]}" | grep -c "PASS"),
    "fail": $(printf '%s\n' "${REPO_STATUS[@]}" | grep -c "FAIL"),
    "no_governance": $(printf '%s\n' "${REPO_STATUS[@]}" | grep -c "NO_GOV")
  }
}
EOF

echo "Report saved to: $REPORT_FILE"
cat "$REPORT_FILE"
```

### Phase 2: OPA Migration (Future)

#### 2.1 Rego Policy Example

```rego
# phenotype_governance.rego
package phenotype.governance

import future.keywords.if
import future.keywords.in

# Default allow
default allow := false

# Repository must have README
allow if {
    input.files["README.md"]
}

deny contains msg if {
    not input.files["README.md"]
    msg := "Repository must have a README.md file"
}

deny contains msg if {
    not input.files["LICENSE"]
    msg := "Repository must have a LICENSE file"
}

deny contains msg if {
    not input.files["SPEC.md"]
    msg := "Repository must have a SPEC.md file"
}

# Code quality checks
deny contains msg if {
    input.quality.shellcheck.violations > 0
    msg := sprintf("ShellCheck found %d violations", [input.quality.shellcheck.violations])
}

# Traceability requirements
deny contains msg if {
    count(input.traceability.markers) < 5
    msg := sprintf("Insufficient traceability markers: %d (min 5)", [count(input.traceability.markers)])
}

# Compliance calculation
compliant := count(deny) == 0

# Remediation hints
remediation := [
    "Run: ./scripts/validate-governance.sh",
    "Check: governance.json configuration",
    "Review: ADR-005 for requirements"
] if {
    not compliant
}
```

#### 2.2 CI Integration

```yaml
# .github/workflows/governance.yml
name: Governance

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  schedule:
    - cron: '0 9 * * 1'  # Weekly on Monday

jobs:
  governance:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Run governance validation
        run: ./scripts/validate-governance.sh
      
      - name: Upload governance report
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: governance-report
          path: governance-report.json

  cross-repo:
    runs-on: ubuntu-latest
    if: github.event_name == 'schedule'
    steps:
      - uses: actions/checkout@v4
      
      - name: Generate cross-repo report
        run: ./scripts/governance-report.sh repos.txt
      
      - name: Post to Slack
        run: |
          curl -X POST "${SLACK_WEBHOOK}" \
            -H "Content-Type: application/json" \
            -d @governance-summary.json
```

---

## Success Metrics

| Metric | Current | Target (6 months) | Measurement |
|--------|---------|-------------------|-------------|
| Repo compliance rate | 40% | 95% | governance-report.sh |
| Avg violations per repo | 8 | 2 | validate-governance.sh |
| Time to detect drift | Manual | <24 hours | CI schedule |
| Remediation guidance | None | Automated | Tool output |
| Developer satisfaction | N/A | >80% | Survey |

---

## Consequences

### Positive

1. **Consistency**: All repositories follow unified standards
2. **Early Detection**: Issues caught before they propagate
3. **Reduced Toil**: Automated checking reduces manual review
4. **Compliance**: Clear audit trail for stakeholders
5. **Developer Guidance**: Helpful error messages with fixes

### Negative

1. **Initial Friction**: Repositories need governance.json
2. **Maintenance**: Rules need updating as standards evolve
3. **False Positives**: Some valid exceptions may need exemptions
4. **Learning Curve**: Team needs to understand governance model

### Neutral

1. **CI Time**: Additional ~10s per build
2. **File Count**: One governance.json per repo
3. **Documentation**: Ongoing maintenance of governance docs

---

## References

1. [Open Policy Agent](https://www.openpolicyagent.org/) - Policy-as-code engine
2. [conftest](https://github.com/open-policy-agent/conftest) - OPA for CI
3. [OPA Gatekeeper](https://open-policy-agent.github.io/gatekeeper/) - Kubernetes integration
4. [OpenSSF Scorecard](https://github.com/ossf/scorecard) - Security scoring
5. [Allstar](https://github.com/ossf/allstar) - GitHub security monitoring
6. [Rego Language](https://www.openpolicyagent.org/docs/latest/policy-language/) - Policy syntax

---

## Appendix: Governance Checklist

### Repository Setup

- [ ] Create `governance.json`
- [ ] Ensure required files exist
- [ ] Set up traceability markers
- [ ] Configure pre-commit hooks
- [ ] Enable CI governance checks
- [ ] Document exemptions (if any)

### Ongoing Maintenance

- [ ] Review governance.json quarterly
- [ ] Update policies as standards evolve
- [ ] Monitor compliance dashboard
- [ ] Address violations within 1 week
- [ ] Document policy changes in ADRs

---

*Last Updated: 2026-04-04*  
*Supersedes: N/A*  
*Related: validate-governance.sh, scaffold-smoke.sh, traceability-check.py, ADR-003*
