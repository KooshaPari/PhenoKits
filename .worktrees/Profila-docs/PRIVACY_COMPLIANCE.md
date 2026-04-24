# Privacy Compliance Guide

**Version:** 1.0.0
**Last Updated:** 2026-04-02

## Scope

This guide ensures Profila complies with:
- GDPR (European Union)
- CCPA (California)
- SOC 2 Type II requirements
- HIPAA (if handling health data)

## Data Classification

### Public
- Aggregate complexity metrics
- Anonymous usage statistics
- Open source profiling results

### Internal
- Repository-specific complexity scores
- Team performance metrics
- Build time statistics

### Confidential
- Full file paths
- Git author information
- Function name details

### Restricted
- Credentials in code
- API keys and tokens
- Personal identifying strings

## Technical Controls

### Access Control
```yaml
# Ensure output files have restricted permissions
file_permissions:
  output_dir: "0700"
  results_json: "0600"
  charts: "0644"
```

### Encryption
- All outputs encrypted at rest (AES-256)
- TLS 1.3 for data in transit
- No plaintext PII in logs

### Monitoring
```python
# Log only non-PII data
logger.info(f"Processed {len(files)} files in {duration}ms")
# NOT: logger.info(f"User {username} analyzed {filepath}")
```

## Audit Requirements

| Requirement | Implementation |
|-------------|----------------|
| Access Logging | All file reads logged with timestamp and actor |
| Data Exports | Audit trail for all data exports |
| Deletion | Cryptographic erasure confirmation |
| Breach Detection | Automated PII scanning in outputs |

## Review Schedule

- **Monthly** - Review access logs
- **Quarterly** - Update PII detection rules
- **Annually** - Complete privacy impact assessment

## Training

All engineers must complete:
1. Data classification training
2. PII identification workshop
3. Secure coding practices review
