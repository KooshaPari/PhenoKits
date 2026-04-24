# tooling Charter

## 1. Mission Statement

**tooling** is the central infrastructure for development tools, build systems, and productivity enhancements across the Phenotype ecosystem. The mission is to provide consistent, high-quality development environments, automate away repetitive tasks, and enable developers to focus on solving problems rather than wrestling with tools.

The project exists to be the force multiplier for engineering productivity—ensuring that every developer has access to the best tools, properly configured, with seamless integration into their daily workflow.

---

## 2. Tenets (Unless You Know Better Ones)

### Tenet 1: Developer Choice Within Standards

Developers choose their preferred tools from approved sets. Standardization happens at integration points, not at personal preference. VS Code or Neovim? Either works with our tooling.

### Tenet 2: Works Out of the Box

New developer checkout → build → test in <10 minutes. No manual configuration hunting. No "ask in Slack" setup steps. Batteries included, conventions over configuration.

### Tenet 3: Automation Over Documentation

If it's documented as a manual step, it should be automated. Documentation explains why; tooling handles how. The goal is eliminating manual steps, not just describing them.

### Tenet 4: Fast Feedback Loops

Tools optimize for developer wait time. Fast compilation, incremental builds, parallel testing. Every second of waiting is friction. Sub-second where possible.

### Tenet 5: Observable and Debuggable

When tools fail, they explain why clearly. Verbose modes available. Logs are structured and searchable. Debugging tool issues is straightforward, not arcane.

### Tenet 6: Incremental Adoption

New tooling can be adopted incrementally. No "big bang" migrations. Try it on one project, then expand. Escape hatches available. Gradual improvement beats delayed perfection.

### Tenet 7: Cross-Platform Consistency

Tools behave identically on macOS, Linux, and CI. Platform differences are handled transparently. Development environment matches CI environment.

---

## 3. Scope & Boundaries

### In Scope

**Build Systems:**
- Unified build orchestration
- Cross-compilation support
- Incremental build optimization
- Build caching strategies

**Development Environments:**
- Devcontainer configurations
- Nix/nix-shell environments
- Local development setup automation
- Editor/IDE integration

**Code Quality Tools:**
- Linting integration (clippy, ruff, eslint)
- Formatting tools (rustfmt, black, prettier)
- Static analysis integration
- Security scanning (semgrep, cargo-audit)

**CI/CD Tooling:**
- GitHub Actions reusable workflows
- Local CI simulation
- Pipeline optimization tools
- Artifact management

**Productivity Tools:**
- Git hooks and automation
- Task runners and build scripts
- Documentation generators
- Code generation tools

**Observability Tooling:**
- Local metrics collection
- Development logging
- Performance profiling tools
- Debug utilities

### Out of Scope

- Application business logic
- Production monitoring infrastructure
- User-facing features
- One-off personal scripts
- Experimental tools without maintenance plan

### Boundaries

- Tooling configures and orchestrates; applications do the work
- No vendor lock-in—tools use standard formats
- Open source preference for core tools
- Commercial tools allowed with clear value justification

---

## 4. Target Users & Personas

### Primary Persona: Developer Dana

**Role:** Software engineer building features
**Goals:** Fast builds, good error messages, working IDE
**Pain Points:** Slow compilation, broken IDE setup, unclear errors
**Needs:** Reliable tooling, fast feedback, minimal configuration
**Tech Comfort:** High, uses tools daily

### Secondary Persona: DevOps Derek

**Role:** Infrastructure engineer maintaining CI/CD
**Goals:** Reliable pipelines, fast CI, consistent environments
**Pain Points:** Slow CI, inconsistent local/CI behavior, tool drift
**Needs:** Fast, reliable CI, local simulation, version pinning
**Tech Comfort:** Very high, expert in build systems

### Tertiary Persona: New-Hire Nina

**Role:** Recently joined engineer
**Goals:** Quick setup, understanding conventions
**Pain Points:** Unclear setup steps, missing dependencies
**Needs:** One-command setup, clear documentation
**Tech Comfort:** Medium-High, experienced but new here

### Persona: Architect Avery

**Role:** System architect defining standards
**Goals:** Consistent tooling, maintainable infrastructure
**Pain Points:** Tool sprawl, inconsistent practices
**Needs:** Standardized tooling, governance, metrics
**Tech Comfort:** Very high, expert in developer experience

---

## 5. Success Criteria (Measurable)

### Setup Metrics

- **Onboarding Time:** New developer productive in <30 minutes
- **Setup Success Rate:** 95% successful first-time setup
- **Documentation Clarity:** 4.5/5 rating for setup docs

### Build Metrics

- **Build Speed:** Incremental builds <10 seconds
- **CI Speed:** CI pipeline completes in <10 minutes
- **Cache Hit Rate:** 80%+ build cache utilization
- **Build Reliability:** 99%+ successful builds

### Quality Metrics

- **Lint Pass Rate:** 95%+ of code passes linting
- **Format Compliance:** 100% of code auto-formatted
- **Security Scan Pass:** Zero high-severity findings in CI

### Developer Satisfaction

- **Tooling Satisfaction:** 4.5/5 average rating
- **Support Tickets:** <1 tooling ticket per developer per month
- **Adoption Rate:** 90%+ of developers using standard tooling

---

## 6. Governance Model

### Organization

```
tooling/
├── build/        # Build system configurations
├── ci/           # CI/CD workflows and tools
├── dev/          # Development environment setup
├── quality/      # Linting, formatting, static analysis
├── scripts/      # Shared automation scripts
├── docs/         # Tooling documentation
└── configs/      # Shared configuration files
```

### Tool Selection Process

1. **Identify Need:** What problem are we solving?
2. **Evaluate Options:** Open source preference, community adoption
3. **Prototype:** Trial on limited scope
4. **Decision:** Technical steering committee approval
5. **Rollout:** Incremental adoption with migration guide
6. **Review:** Regular usage and satisfaction review

### Maintenance

- Each tool has assigned owner
- Regular updates (security patches immediately)
- Quarterly tool review (usage, satisfaction, alternatives)
- Deprecation process for tools being replaced

---

## 7. Charter Compliance Checklist

### For New Tools

- [ ] Problem clearly defined
- [ ] Options evaluated and documented
- [ ] Cross-platform support verified
- [ ] Integration points defined
- [ ] Documentation created
- [ ] Migration path (if replacing existing tool)
- [ ] Owner assigned

### For Tool Updates

- [ ] Breaking changes documented
- [ ] Migration guide if needed
- [ ] Rollback plan if issues
- [ ] Stakeholders notified

### For Deprecations

- [ ] Alternative identified
- [ ] Deprecation timeline defined
- [ ] Migration guide created
- [ ] Users notified

---

## 8. Decision Authority Levels

### Level 1: Tool Owner Authority

**Scope:** Configuration updates, non-breaking changes
**Process:** Owner approval, code review

### Level 2: Tooling Team Authority

**Scope:** New tools within existing categories
**Process:** Team review, notify stakeholders

### Level 3: Technical Steering Authority

**Scope:** New tool categories, major changes
**Process:** Written proposal, steering approval

### Level 4: Executive Authority

**Scope:** Strategic tooling decisions, major investments
**Process:** Business case, executive approval

---

*This charter governs the development tooling that enables productivity across the Phenotype ecosystem. Great tooling makes great engineering possible.*

*Last Updated: April 2026*
*Next Review: July 2026*
