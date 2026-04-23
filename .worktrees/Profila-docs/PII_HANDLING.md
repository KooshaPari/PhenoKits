# PII Handling Policy

**Version:** 1.0.0
**Last Updated:** 2026-04-02

## Overview

This document outlines how Profila handles Personally Identifiable Information (PII) to ensure compliance with data protection regulations including GDPR, CCPA, and SOC 2.

## What Profila Collects

Profila is a code profiling tool that collects:

| Data Type | Examples | PII Status |
|-----------|----------|------------|
| System Metrics | CPU%, Memory%, Disk usage | Not PII |
| Code Complexity | Cyclomatic complexity, line counts | Not PII |
| Function Names | `calculate_total()`, `get_user()` | Potentially PII |
| File Paths | `/home/user/project/file.py` | Potentially PII |
| Git History | Commit messages, author info | Potentially PII |

## PII Risk Assessment

### Low Risk
- Aggregate metrics without identifiers
- Anonymized complexity scores
- System-level statistics

### Medium Risk (Requires Mitigation)
- File paths containing usernames
- Function names that may identify users
- Git commit metadata

### High Risk (Must Avoid)
- User credentials or tokens
- Personal identifying strings in code
- API keys or secrets

## Mitigation Strategies

### 1. Path Anonymization

```python
import os
import hashlib

def anonymize_path(path: str) -> str:
    """Replace user directory with generic placeholder."""
    home = os.path.expanduser("~")
    if path.startswith(home):
        return path.replace(home, "/home/user", 1)
    return path
```

### 2. Function Name Sanitization

```python
import re

def sanitize_function_name(name: str) -> str:
    """Remove potential PII from function names."""
    # Keep only alphanumeric and underscores
    return re.sub(r'[^a-zA-Z0-9_]', '_', name)
```

### 3. Git Author Anonymization

```python
def anonymize_git_author(author: str) -> str:
    """Hash author names to prevent identification."""
    return hashlib.sha256(author.encode()).hexdigest()[:12]
```

## Data Retention

| Data Type | Retention | Disposal |
|-----------|-----------|----------|
| Profiling Results | 30 days | Secure delete |
| Metrics Snapshots | 90 days | Secure delete |
| Debug Logs | 7 days | Auto-expiry |
| Audit Logs | 1 year | Archival then delete |

## Compliance Checklist

- [ ] No credentials or secrets in profiling output
- [ ] File paths are anonymized by default
- [ ] User directory references are masked
- [ ] Git author information is hashed
- [ ] Function names are sanitized
- [ ] Output files have appropriate access controls
- [ ] Data retention policies are enforced

## Incident Response

If PII is discovered in Profila output:

1. **Contain** - Stop the profiling session
2. **Assess** - Identify what PII was exposed
3. **Notify** - Alert the affected user within 72 hours
4. **Remediate** - Update anonymization logic
5. **Document** - Record the incident and resolution

## Contact

For PII-related concerns:
- **Email:** privacy@phenotype.dev
- **Security:** security@phenotype.dev
