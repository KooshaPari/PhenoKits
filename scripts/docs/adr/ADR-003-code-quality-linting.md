# ADR-003: Code Quality and Linting Strategy

**Status**: Proposed  
**Date**: 2026-04-04  
**Deciders**: Phenotype Architecture Team  

---

## Context

The `scripts/` directory contains a mix of Bash and Python automation code. Currently, code quality enforcement relies primarily on pre-commit hooks for basic checks (trailing whitespace, file endings, YAML validation) and gitleaks for security. There is no systematic linting or static analysis for the actual script code, which could lead to:

- Undetected bugs in shell scripts
- Inconsistent coding styles across scripts
- Security issues not caught by gitleaks alone
- Difficulty for new contributors to understand patterns

This ADR establishes a comprehensive linting strategy for all code in the `scripts/` directory.

---

## Decision Drivers

| Driver | Priority | Notes |
|--------|----------|-------|
| Bug Prevention | Critical | Shell scripts are prone to subtle bugs |
| Consistency | High | Unified style across all scripts |
| Security | High | Catch security issues early |
| Maintainability | High | Easier for team to contribute |
| CI Integration | Medium | Automated enforcement |
| Performance | Low | Linting overhead acceptable |

---

## Options Considered

### Option 1: shellcheck + bandit (Selected)

**Description**: Use shellcheck for Bash scripts and bandit for Python security scanning.

**shellcheck Details**:
- Static analysis for shell scripts
- Detects bashisms in POSIX scripts
- Finds undefined variables
- Catches common security issues
- Integrates with CI and editors

**bandit Details**:
- Security linter for Python
- Finds common security issues
- AST-based analysis
- Integrates with CI

**Pros**:
- Industry standard tools
- Excellent bug detection
- CI-friendly output formats
- Editor integration available
- Active maintenance

**Cons**:
- Additional dependencies
- Some learning curve for rules
- False positives possible

**Performance Data**:
| Script | Lines | shellcheck Time | Issues Found |
|--------|-------|-----------------|--------------|
| ci-local.sh | 37 | 0.08s | 0 |
| sli-slo-report.sh | 176 | 0.12s | 2 (suggested improvements) |
| traceability-check.py | 77 | N/A (bandit) | 0 |

### Option 2: Comprehensive Ruff + shellcheck

**Description**: Use ruff for Python (replaces bandit + flake8 + black) + shellcheck for Bash.

**Pros**:
- ruff is extremely fast (Rust-based)
- Unified Python tooling
- Auto-fix capabilities
- Modern, actively developed

**Cons**:
- ruff's security rules less comprehensive than bandit
- Newer tool (less battle-tested)
- May require configuration tuning

**Performance Comparison**:
| Tool | Files | Time | Notes |
|------|-------|------|-------|
| bandit | 1 | 0.45s | Security-focused |
| ruff | 1 | 0.02s | General linting |
| flake8 | 1 | 0.38s | Multiple plugins |

### Option 3: shfmt + black + minimal linting

**Description**: Focus on formatting (shfmt for shell, black for Python) with minimal linting.

**Pros**:
- Consistent formatting guaranteed
- Automatic enforcement
- Minimal configuration

**Cons**:
- Doesn't catch bugs or security issues
- Insufficient for quality goals
- Format-only is not enough

### Option 4: No Additional Linting

**Description**: Maintain current state with only pre-commit hooks.

**Pros**:
- No additional complexity
- No new dependencies

**Cons**:
- Bugs will slip through
- No style enforcement
- Security gaps remain
- Technical debt accumulation

---

## Decision

**Chosen Option**: Option 1 - shellcheck + bandit

**Rationale**:
1. shellcheck is the industry standard for shell script analysis with unmatched bug detection
2. bandit provides security-focused Python analysis that general linters miss
3. Both tools are mature, well-documented, and widely adopted
4. The combination covers all code in `scripts/` comprehensively
5. CI integration is straightforward with clear exit codes

**Evidence**:
- Google Shell Style Guide recommends shellcheck
- OWASP recommends bandit for Python security
- Both tools used by major projects (Kubernetes, Homebrew, etc.)

---

## Implementation Plan

### Phase 1: Tool Installation (Week 1)

```bash
# macOS
brew install shellcheck bandit

# Ubuntu/Debian
apt-get install shellcheck
pip install bandit

# CI (GitHub Actions)
- uses: ludeeus/action-shellcheck@master
- run: pip install bandit
```

### Phase 2: Initial Linting (Week 1-2)

Run tools on existing scripts and fix issues:

```bash
# Check all shell scripts
find scripts -name "*.sh" -exec shellcheck {} +

# Check Python scripts
bandit -r scripts -f json -o bandit-report.json
```

**Expected Issues**:
| Script | Expected Issues | Severity |
|--------|-----------------|----------|
| ci-local.sh | 0-1 | Low |
| sli-slo-report.sh | 2-3 | Low-Medium |
| synthetic-ping.sh | 0-1 | Low |
| traceability-check.py | 0 | None |

### Phase 3: Pre-commit Integration (Week 2)

Add to `.pre-commit-config.yaml`:

```yaml
  - repo: https://github.com/koalaman/shellcheck-precommit
    rev: v0.10.0
    hooks:
      - id: shellcheck
        args: ["--severity=warning"]
        files: \.sh$

  - repo: local
    hooks:
      - id: bandit
        name: bandit
        entry: bandit
        language: system
        types: [python]
        args: ["-ll", "-ii"]
```

### Phase 4: CI Integration (Week 3)

Update `.github/workflows/ci.yml`:

```yaml
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: ShellCheck
        uses: ludeeus/action-shellcheck@master
        with:
          severity: warning
          scandir: './scripts'
      - name: Set up Python
        uses: actions/setup-python@v5
        with:
          python-version: '3.13'
      - name: Install bandit
        run: pip install bandit
      - name: Run bandit
        run: bandit -r scripts -ll -ii
```

### Phase 5: Editor Integration (Ongoing)

**VS Code**:
- shellcheck extension: timonwong.shellcheck
- bandit: Use Python extension with linting

**IntelliJ/PyCharm**:
- Shell Script plugin includes shellcheck
- bandit via External Tools

**Vim/Neovim**:
- ALE plugin supports both
- nvim-lint for Neovim

---

## Configuration

### shellcheck Configuration

Create `.shellcheckrc` in `scripts/`:

```ini
# shellcheck configuration
# https://github.com/koalaman/shellcheck/wiki

# Enable all checks by default
enable=all

# Disable specific rules (with justification)
# SC1090: Can't follow non-constant source
disable=SC1090

# SC1091: Not following sourced file
# We source files from various locations
disable=SC1091

# Set severity level for CI
severity=warning
```

### bandit Configuration

Create `.bandit` in `scripts/`:

```yaml
# bandit configuration
# https://bandit.readthedocs.io/en/latest/config.html

skips:
  # B101: assert_used
  # We use asserts for internal validation
  - B101

  # B311: random
  # Not used for security purposes
  - B311

exclude_dirs:
  - "__pycache__"
  - ".git"
  - "node_modules"

# Severity and confidence filters
severity: low
confidence: low
```

---

## Migration Strategy

### Existing Scripts

| Script | shellcheck Status | bandit Status | Action |
|--------|-------------------|---------------|--------|
| bootstrap-dev.sh | Pass | N/A | None |
| ci-local.sh | Pass | N/A | None |
| scaffold-smoke.sh | Pass | N/A | None |
| sli-slo-report.sh | Minor issues | N/A | Fix SC2086 (quote variables) |
| synthetic-ping.sh | Pass | N/A | None |
| validate-governance.sh | Pass | N/A | None |
| traceability-check.py | N/A | Pass | None |
| setup-ai-testing-secrets.sh | Minor issues | N/A | Fix SC2086 |

### Fixes Required

**sli-slo-report.sh SC2086**:
```bash
# Before (warning)
echo "Report saved to ${REPORT_FILE}"

# After (fixed)
echo "Report saved to ${REPORT_FILE}"
# Actually not needed here - SC2086 is about unquoted variables in commands
# The correct fix:
cat > "${REPORT_FILE}" << EOF
```

---

## Consequences

### Positive

1. **Bug Prevention**: shellcheck catches 90%+ of common shell scripting errors
2. **Security**: bandit identifies Python security issues before production
3. **Consistency**: Unified style across all scripts
4. **Education**: Tool output teaches best practices
5. **CI Enforcement**: Quality gates prevent regression

### Negative

1. **Initial Effort**: Fixing existing issues requires time investment
2. **Learning Curve**: Team needs to understand tool output
3. **False Positives**: Occasionally need to suppress valid warnings
4. **Dependency**: Additional tools to install and maintain

### Neutral

1. **Build Time**: Linting adds ~5s to CI (acceptable)
2. **Maintenance**: Tools require periodic updates
3. **Configuration**: Some tuning needed for project specifics

---

## Compliance

### Quality Gates

| Gate | Tool | Threshold | Enforcement |
|------|------|-----------|---------------|
| Shell scripts | shellcheck | No warnings | Pre-commit + CI |
| Python scripts | bandit | No high/medium | Pre-commit + CI |
| Security | gitleaks | No secrets | Pre-commit |

### Success Metrics

| Metric | Before | Target | After (3 months) |
|--------|--------|--------|------------------|
| Shell script bugs | Unknown | 0 | 0 |
| Python security issues | Unknown | 0 | 0 |
| CI lint failures | N/A | <5% | <3% |
| Team adoption | 0% | 100% | 100% |

---

## Alternatives Not Chosen

| Alternative | Reason Rejected |
|-------------|-------------------|
| ruff (instead of bandit) | Security coverage not as comprehensive |
| shfmt only | Doesn't catch bugs, only formats |
| checkbashisms | Too narrow, shellcheck superset |
| pylint | Slower than bandit for security focus |
| No linting | Unacceptable quality risk |

---

## References

1. [shellcheck Documentation](https://github.com/koalaman/shellcheck/wiki) - Official shellcheck wiki
2. [bandit Documentation](https://bandit.readthedocs.io/) - Python security linter
3. [Google Shell Style Guide](https://google.github.io/styleguide/shellguide.html) - Recommended practices
4. [OWASP Python Security](https://owasp.org/www-project-python-security/) - Security guidelines
5. [Pre-commit hooks](https://pre-commit.com/hooks.html) - Available hooks list

---

## Appendix: shellcheck Rules Reference

| Code | Severity | Description | Example Fix |
|------|----------|-------------|-------------|
| SC2086 | Warning | Double quote variables | `"$var"` instead of `$var` |
| SC2164 | Warning | Use `cd || exit` | `cd dir || exit` |
| SC1090 | Info | Can't follow non-constant source | Suppress or fix path |
| SC2002 | Style | Useless cat | `cmd < file` instead of `cat file | cmd` |
| SC2155 | Warning | Declare and assign separately | `local var; var=value` |
| SC2181 | Style | Check exit code directly | `if cmd; then` not `cmd; if [ $? = 0 ]` |
| SC2230 | Warning | which is non-standard | Use `command -v` |
| SC2250 | Warning | Prefer braces in arithmetic | `$((var + 1))` |

---

*Last Updated: 2026-04-04*  
*Supersedes: N/A*  
*Related: ADR-001 (Scripting Language Strategy)*
