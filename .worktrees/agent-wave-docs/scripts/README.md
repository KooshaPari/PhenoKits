# CI Gate Scripts

This directory contains scripts that enforce quality, policy, security, and merge gates for the agent-wave project.

## Scripts

### quality-gate.sh

Runs quality checks to ensure code meets minimum standards.

**Usage:**
```bash
./scripts/quality-gate.sh verify
```

**Checks:**
- YAML linting via `yamllint` (if available)
- Pre-commit checks via `pre-commit` (if available)

**Exit Code:**
- `0` - All quality checks passed
- `1` - Quality checks failed

---

### policy-gate.sh

Enforces engineering policies including PR labels and commit message format.

**Usage:**
```bash
./scripts/policy-gate.sh
```

**Checks:**
- PR labels validation (advisory)
- Conventional commit format validation (advisory)

**Commit Format:**
Commits should follow the conventional commit format:
```
type(scope): description

Optional body with more details.

Optional footer.
```

Allowed types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `ci`, `build`, `revert`

**Exit Code:**
- `0` - All policies enforced successfully
- `1` - Policy enforcement failed

---

### security-guard.sh

Scans for security issues including hardcoded secrets and vulnerabilities.

**Usage:**
```bash
./scripts/security-guard.sh audit
```

**Checks:**
- Hardcoded secrets pattern detection
- `gitleaks` scan (if available)

**Patterns Detected:**
- Password assignments
- API key assignments
- Secret assignments
- Token assignments
- AWS credentials
- Private key content

**Exit Code:**
- `0` - No security issues found
- `1` - Security issues detected

---

### self-merge-gate.sh

Prevents self-merges and verifies review approvals.

**Usage:**
```bash
./scripts/self-merge-gate.sh
```

**Checks:**
- Self-merge prevention
- Review approval verification

**Exit Code:**
- `0` - Merge eligibility check passed
- `1` - Merge eligibility check failed

---

## Running Scripts Locally

All scripts can be run locally during development:

```bash
# Make scripts executable
chmod +x scripts/*.sh

# Run quality checks
./scripts/quality-gate.sh verify

# Enforce policies
./scripts/policy-gate.sh

# Run security audit
./scripts/security-guard.sh audit

# Check merge eligibility
./scripts/self-merge-gate.sh
```

## CI Integration

These scripts are automatically invoked by GitHub Actions workflows:

- `.github/workflows/quality-gate.yml` - Runs on pull requests
- `.github/workflows/policy-gate.yml` - Runs on pull requests
- `.github/workflows/security-guard.yml` - Runs on push and pull requests
- `.github/workflows/self-merge-gate.yml` - Runs on pull request approvals

## Dependencies

Some scripts depend on external tools:

- `yamllint` - YAML file linting (optional)
- `pre-commit` - Pre-commit framework (optional)
- `gitleaks` - Secret detection (optional)
- `git` - Git command-line tool (required)

Install optional dependencies:

```bash
# Install yamllint
pip install yamllint

# Install pre-commit
pip install pre-commit
pre-commit install

# Install gitleaks
# See: https://github.com/gitleaks/gitleaks
```

## Notes

- All scripts use POSIX shell conventions for maximum compatibility
- Scripts include color output for better readability in CI logs
- Non-critical checks are advisory (warnings) rather than blocking
- All scripts exit with code 0 on success, 1 on failure
