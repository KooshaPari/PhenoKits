# Profila Operational Runbooks

Quick reference for common Profila tasks.

## Emergency Contacts

| Team | Contact |
|------|---------|
| Platform | #platform-support |
| Security | #security-incident |
| Engineering | #engineering-help |

## Key Thresholds

| Metric | Warning | Critical |
|--------|---------|----------|
| Profiler Overhead | > 3% | > 5% |
| Memory Usage | > 400MB | > 512MB |
| Analysis Time | > 30s | > 60s |

## Common Commands

```bash
# Run all tests
pytest tests/ -v

# Profile a directory
./bin/profiler.sh -d ./src -o ./output

# Check system metrics
python3 bin/system_metrics.py
```
