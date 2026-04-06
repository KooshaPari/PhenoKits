# Phenotype Dependency Guard Specification

> Dependency Guard Toolchain and Audit Utility

**Version**: 1.0 | **Status**: Draft | **Last Updated**: 2026-04-02

## Overview

Phenotype-dep-guard provides comprehensive dependency management, security auditing, and license compliance for the Phenotype ecosystem.

### Key Features

- **Vulnerability Scanning**: CVE detection, severity assessment
- **License Compliance**: SPDX analysis, policy enforcement
- **Supply Chain Security**: SLSA compliance, SBOM generation
- **Dependency Graph**: Visualize and analyze dependency trees
- **Update Management**: Automated update suggestions, breaking change detection
- **Policy Enforcement**: Custom rules, gate CI/CD pipelines

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     Phenotype Dependency Guard                            │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    Scanning Engine                                     │   │
│  │                                                                        │   │
│  │   ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │   │
│  │   │ Vuln Scanner │  │ License      │  │ Supply Chain │             │   │
│  │   │ (OSV/Advisory│  │ Analyzer     │  │ Validator    │             │   │
│  │   └──────────────┘  └──────────────┘  └──────────────┘             │   │
│  │                                                                        │   │
│  └────────────────────────────────────────────────────────────────────────┘   │
│                                   │                                         │
│                                   ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    Dependency Graph Analyzer                           │   │
│  │                                                                        │   │
│  │   Parse manifest → Build graph → Detect cycles → Find duplicates      │   │
│  │                                                                        │   │
│  └────────────────────────────────────────────────────────────────────────┘   │
│                                   │                                         │
│                                   ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    Policy Engine                                       │   │
│  │                                                                        │   │
│  │   Custom Rules → Severity Thresholds → CI/CD Gates → Notifications    │   │
│  │                                                                        │   │
│  └────────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Supported Ecosystems

| Language | Tool | File | Status |
|----------|------|------|--------|
| **Rust** | Cargo | Cargo.toml, Cargo.lock | ✅ |
| **Python** | pip, poetry, uv | requirements.txt, pyproject.toml | ✅ |
| **Go** | Go modules | go.mod, go.sum | ✅ |
| **Node.js** | npm, yarn, pnpm | package.json, package-lock.json | ✅ |
| **Java** | Maven, Gradle | pom.xml, build.gradle | ⚠️ |
| **Ruby** | Bundler | Gemfile, Gemfile.lock | ⚠️ |
| **C/C++** | Conan, vcpkg | conanfile.txt | 📋 |

## Vulnerability Scanning

```bash
# Scan project
dep-guard scan

# Scan with specific ecosystem
dep-guard scan --ecosystem rust,python

# Output formats
dep-guard scan --format json --output report.json
dep-guard scan --format sarif --output report.sarif
dep-guard scan --format html --output report.html

# Fail on severity
dep-guard scan --fail-on critical,high

# Ignore specific CVEs
dep-guard scan --ignore CVE-2024-1234,CVE-2024-5678
```

## License Compliance

```bash
# Analyze licenses
dep-guard license --check

# SPDX compliance
dep-guard license --spdx

# Generate SBOM
dep-guard license --sbom --format spdx-json
dep-guard license --sbom --format cyclonedx-json

# Policy check
dep-guard license --policy policy.yaml
```

### License Policy Example

```yaml
# dep-guard-policy.yaml
allowed:
  - MIT
  - Apache-2.0
  - BSD-3-Clause
  - BSD-2-Clause
  
restricted:
  - GPL-2.0
  - GPL-3.0
  - AGPL-3.0
  
forbidden:
  - proprietary
  - unknown
  
exceptions:
  - package: openssl
    license: OpenSSL
    reason: "Industry standard crypto library"
```

## Dependency Graph

```bash
# Generate dependency tree
dep-guard graph --format tree
dep-guard graph --format dot --output deps.dot
dep-guard graph --format mermaid

# Find duplicates
dep-guard graph --duplicates

# Find unused
dep-guard graph --unused

# Size analysis
dep-guard graph --size-analysis
```

## Update Management

```bash
# Check for updates
dep-guard update --check

# Update suggestions with breaking change detection
dep-guard update --suggest

# Apply safe updates
dep-guard update --apply --safe-only

# Lock file maintenance
dep-guard update --lock
```

## CI/CD Integration

### GitHub Actions

```yaml
- name: Dependency Guard
  uses: phenotype/dep-guard-action@v1
  with:
    scan-vulnerabilities: true
    scan-licenses: true
    fail-on: critical,high
    generate-sbom: true
```

### GitLab CI

```yaml
dependency-guard:
  image: phenotype/dep-guard:latest
  script:
    - dep-guard scan --fail-on critical
    - dep-guard license --check
  artifacts:
    reports:
      dependency_scanning: report.json
```

## Configuration

```yaml
# dep-guard.yaml
version: 1

ecosystems:
  - rust
  - python
  - node

scan:
  vulnerability:
    enabled: true
    sources:
      - osv
      - github-advisory
      - snyk
    
  license:
    enabled: true
    policy: .dep-guard-license-policy.yaml
    
  supply_chain:
    enabled: true
    slsa_level: 3
    
graph:
  show_transitive: true
  max_depth: 10
  
update:
  check_interval: 24h
  auto_apply_safe: false
  
notifications:
  slack:
    webhook: ${SLACK_WEBHOOK}
  email:
    to: security@phenotype.io
```

## Performance Targets

| Operation | P50 | P99 |
|-----------|-----|-----|
| Manifest parse | 10ms | 50ms |
| Vulnerability lookup | 100ms | 500ms |
| Graph build | 50ms | 200ms |
| Full scan | 1s | 5s |

## References

- [OSV - Open Source Vulnerabilities](https://osv.dev/)
- [SLSA - Supply Chain Levels](https://slsa.dev/)
- [SPDX - Software Package Data Exchange](https://spdx.dev/)
- [CycloneDX - Bill of Materials](https://cyclonedx.org/)
- [OWASP Dependency-Check](https://owasp.org/www-project-dependency-check/)
