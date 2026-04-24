# Troubleshooting Guide

Common issues and resolutions.

## Installation Issues

### ImportError for psutil

```bash
pip install psutil
pip install -e ".[dev]"
```

### Permission denied on scripts

```bash
chmod +x bin/*.sh
# Or use bash explicitly:
bash bin/profiler.sh
```

## Runtime Issues

### High CPU overhead

1. Reduce sampling rate:
   ```bash
   PROFILA_SAMPLE_RATE=500 ./bin/profiler.sh
   ```

2. Exclude large directories:
   ```bash
   ./bin/complexity_analyzer.py -d ./src --exclude ./src/vendor
   ```

### Out of memory

```bash
# Process in batches
find ./src -name "*.py" | head -100 | xargs ./bin/analyze.py
```

## Escalation

If issues persist:
1. Document with full error and steps to reproduce
2. Create GitHub issue with `bug` label
3. Escalate to #engineering-help if urgent
