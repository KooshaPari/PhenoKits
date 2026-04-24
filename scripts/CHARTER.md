# scripts Charter

## 1. Mission Statement

**scripts** serves as the central automation and tooling repository for the Phenotype ecosystem, providing standardized, reusable scripts that streamline development workflows, enforce quality gates, and automate repetitive tasks across all projects. The mission is to eliminate toil through intelligent automation, ensure consistency in development practices, and enable rapid project onboarding through battle-tested tooling.

The project exists to capture organizational knowledge in executable form—transforming manual processes into reliable, version-controlled automation that scales across teams, projects, and environments.

---

## 2. Tenets (Unless You Know Better Ones)

### Tenet 1: Automation Over Documentation

When a process is documented, it should also be automated. Documentation describes the "what" and "why"; scripts handle the "how". Every manual checklist should have a corresponding automated equivalent.

### Tenet 2: Composability

Scripts are small, focused, and composable. Complex workflows are built by chaining simple scripts rather than creating monolithic automation. Each script does one thing well and accepts well-defined inputs.

### Tenet 3: Environment Portability

Scripts run consistently across macOS, Linux, and CI environments. No "works on my machine"—all dependencies are explicit, and environment detection is automatic. Containerized execution is available for complex dependencies.

### Tenet 4: Fail Fast, Fail Loud

Scripts validate prerequisites before execution and exit immediately on errors. No partial state changes. All failures include actionable error messages with remediation steps. Dry-run mode available for destructive operations.

### Tenet 5: Observability

Every script execution produces structured output. Logs are timestamped, actions are auditable, and results are parseable. Silent failures are unacceptable—if something goes wrong, it's logged and reported.

### Tenet 6: Security by Default

Scripts handling credentials or sensitive data use secure handling patterns. No secrets in command history, environment variables, or logs. Least-privilege execution is enforced.

### Tenet 7: Discoverability

Scripts are documented, categorized, and discoverable. Self-documenting help is available via `--help`. Common workflows are discoverable through well-organized directory structure and clear naming conventions.

---

## 3. Scope & Boundaries

### In Scope

**Development Workflow Automation:**
- Repository setup and initialization scripts
- Dependency installation and updates
- Code quality enforcement (linting, formatting)
- Test execution orchestration
- Local development environment management

**Build & Release Automation:**
- Build pipeline scripts
- Version bumping and changelog generation
- Release packaging and artifact creation
- Deployment automation
- Rollback procedures

**Quality Gates:**
- Pre-commit hooks and validations
- CI/CD pipeline scripts
- Compliance checking automation
- Security scanning integration
- Performance benchmarking

**Maintenance & Operations:**
- Log aggregation and analysis
- Cleanup and housekeeping scripts
- Backup and restore procedures
- Health checks and monitoring setup

**Cross-Project Tooling:**
- Multi-repo operations
- Template synchronization
- Shared configuration management
- Common utility functions

### Out of Scope

- Long-running services or daemons (use appropriate service infrastructure)
- Complex application logic (belongs in application repositories)
- User-facing CLI tools (consider separate CLI project)
- One-off personal scripts (use personal dotfiles)
- Production data manipulation without audit trails
- Direct production database modifications (use proper migration tools)

### Boundaries

- Scripts are stateless where possible—state lives in proper databases or state stores
- No hardcoded credentials—use proper secret management
- Scripts are maintained, not abandoned—regular usage or deprecation
- Version compatibility is managed—breaking changes follow semver

---

## 4. Target Users & Personas

### Primary Persona: Developer Dana

**Role:** Software engineer across Phenotype ecosystem projects
**Goals:** Quick setup, consistent workflows, less time on tooling
**Pain Points:** Manual setup steps, inconsistent environments, forgotten commands
**Needs:** One-command setup, reliable automation, clear failure messages
**Tech Comfort:** High, writes code daily, comfortable with shell scripting

### Secondary Persona: DevOps Derek

**Role:** Infrastructure and CI/CD engineer
**Goals:** Reliable pipelines, consistent environments, automated operations
**Pain Points:** Brittle scripts, environment drift, hard-to-debug failures
**Needs:** Idempotent operations, comprehensive logging, exit code consistency
**Tech Comfort:** Very high, expert in automation and infrastructure tooling

### Tertiary Persona: New-Hire Nina

**Role:** Recently joined engineer, unfamiliar with ecosystem
**Goals:** Rapid onboarding, understanding project conventions
**Pain Points:** Unclear setup process, tribal knowledge gaps
**Needs:** Well-documented setup scripts, clear error messages, help availability
**Tech Comfort:** Medium-High, experienced developer but new to this codebase

### Persona: Release Manager Rick

**Role:** Coordinates releases across multiple projects
**Goals:** Consistent release process, reliable versioning, minimal human error
**Pain Points:** Manual release steps, inconsistent versioning, missed steps
**Needs:** Automated release workflows, version consistency, rollback capability
**Tech Comfort:** High, comfortable with automation tools and git workflows

### Persona: Security Sam

**Role:** Security engineer reviewing automation
**Goals:** Secure defaults, audit trails, no credential leaks
**Pain Points:** Scripts with hardcoded secrets, insufficient logging, privilege escalation
**Needs:** Security-first script patterns, comprehensive audit logs, least privilege
**Tech Comfort:** Very high, expert in security tooling and practices

---

## 5. Success Criteria (Measurable)

### Automation Coverage

- **Onboarding Time:** New developer fully set up in <30 minutes using provided scripts
- **Setup Success Rate:** 95% of new developers complete setup without manual intervention
- **Script Coverage:** 90% of documented manual processes have automated equivalents
- **Cross-Platform Success:** 100% of core scripts pass on macOS, Ubuntu, and CI environments

### Reliability Metrics

- **Script Success Rate:** 99% of script executions complete successfully in CI
- **Idempotency:** All setup scripts can be safely re-run without side effects
- **Failure Recovery:** 80% of script failures include automated or documented recovery steps
- **Rollback Success:** 100% of destructive operations have verified rollback procedures

### Developer Experience

- **Command Discovery:** New developer can find relevant script within 5 minutes
- **Help Quality:** All scripts provide helpful `--help` output
- **Error Clarity:** 90% of error messages include specific remediation steps
- **Execution Time:** Common scripts complete in <5 seconds (excluding network operations)

### Maintenance Metrics

- **Test Coverage:** 70% of complex scripts have automated tests
- **Documentation:** 100% of scripts have usage documentation
- **Deprecation Rate:** <5% of scripts deprecated per quarter (indicates churn)
- **Active Usage:** 80% of scripts executed at least monthly

### Security Metrics

- **Secret Scanning:** 100% of scripts pass automated secret detection
- **Privilege Audit:** All privilege escalation is documented and justified
- **Security Review:** New scripts handling credentials undergo security review

---

## 6. Governance Model

### Repository Organization

**Directory Structure by Purpose:**
```
scripts/
├── setup/          # Environment and project setup
├── build/          # Build and compilation
├── test/           # Test execution and coverage
├── release/        # Release management
├── quality/        # Quality gates and linting
├── ops/            # Operations and maintenance
├── utils/          # Shared utility functions
└── ci/             # CI/CD pipeline scripts
```

### Contribution Standards

**Script Requirements:**
- All scripts must include header documentation
- Shebang line must be explicit (#!/bin/bash, #!/usr/bin/env python3, etc.)
- Scripts must handle errors explicitly (`set -euo pipefail` for bash)
- All scripts must support `--help` or equivalent
- Dependencies must be documented and validated

**Review Process:**
- All new scripts require code review
- Scripts handling credentials require security review
- Complex scripts (>100 lines) require testing
- Breaking changes must be announced with migration guide

### Maintenance Ownership

- Each script directory has assigned maintainer
- Unused scripts (no execution in 6 months) are candidates for deprecation
- Bug fixes prioritized by script criticality and user impact
- Quarterly script inventory and cleanup

### Change Control

**Minor Changes:**
- Bug fixes, output formatting, help text updates
- Single reviewer approval

**Moderate Changes:**
- New utility functions, non-breaking additions
- Changes to existing script behavior
- Two reviewer approval

**Major Changes:**
- New script categories
- Changes to cross-cutting patterns
- Deprecation of widely-used scripts
- Technical steering committee approval

---

## 7. Charter Compliance Checklist

### For New Scripts

- [ ] Script follows established patterns for language (bash, python, etc.)
- [ ] Header documentation includes purpose, usage, and dependencies
- [ ] `--help` or equivalent self-documentation implemented
- [ ] Error handling follows fail-fast principle
- [ ] No hardcoded credentials or secrets
- [ ] Exit codes follow conventions (0=success, 1=error, specific codes for known states)
- [ ] Tested on target platforms (macOS, Linux, CI)
- [ ] Added to appropriate directory with consistent naming
- [ ] README or documentation updated

### For Script Modifications

- [ ] Backwards compatibility maintained or proper deprecation path
- [ ] Documentation updated to reflect changes
- [ ] Test cases updated or added
- [ ] Breaking changes announced to stakeholders
- [ ] Rollback procedure updated if applicable

### For Deprecations

- [ ] Usage audit confirms script can be safely deprecated
- [ ] Deprecation notice added to script output
- [ ] Migration path documented for alternative solutions
- [ ] Removal scheduled with sufficient notice (minimum 1 quarter)

### Periodic Reviews

- **Monthly:** Script execution metrics review
- **Quarterly:** Unused script identification and cleanup
- **Annually:** Full charter review and governance assessment

---

## 8. Decision Authority Levels

### Level 1: Script Author Authority

**Scope:** Script implementation details, documentation, minor fixes
**Examples:**
- Script logic and implementation
- Help text and documentation
- Error message improvements
- Test case additions

**Process:** Standard code review, no additional approval needed

### Level 2: Directory Maintainer Authority

**Scope:** New scripts in assigned domain, script modifications within domain
**Examples:**
- New build automation scripts
- New test execution helpers
- Modifications to existing setup scripts
- Utility function additions

**Process:** Review by directory maintainer, notify team channel

### Level 3: Technical Steering Authority

**Scope:** Cross-cutting changes, new categories, architectural patterns
**Examples:**
- New script language adoption (e.g., adding Rust scripts)
- Changes to shared utility patterns
- New quality gate automation
- CI/CD pipeline fundamental changes

**Process:** Written proposal, 48-hour review window, steering committee approval

### Level 4: Executive Authority

**Scope:** Strategic tooling direction, major automation investments
**Examples:**
- Migration to new automation platform
- Significant CI/CD infrastructure changes
- Security policy changes affecting scripts
- Resource allocation for tooling team

**Process:** Formal proposal with cost/benefit analysis, stakeholder alignment

### Emergency Decisions

- **Security Vulnerabilities:** Immediate fix authorized by security lead
- **CI/CD Outages:** DevOps lead can implement emergency workarounds
- **Hotfixes:** Release manager can authorize expedited script changes

All emergency decisions require post-hoc review within 48 hours.

---

*This charter governs all automation and scripting within the Phenotype ecosystem. Updates require technical steering committee approval.*

*Last Updated: April 2026*
*Next Review: July 2026*
