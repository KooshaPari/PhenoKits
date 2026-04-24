# APM Integration Guide

Tracera emits traces through the shared Phenotype observability contract.

## Local Defaults

```bash
TRACING_ENABLED=true
PHENO_OBSERVABILITY_OTLP_GRPC_ENDPOINT=127.0.0.1:4319
PHENO_OBSERVABILITY_GRAFANA_URL=http://127.0.0.1:3000
PHENO_OBSERVABILITY_TEMPO_URL=http://127.0.0.1:3200
```

## Runtime Path

1. Applications export OTLP traces to the repo-local Alloy receiver on `127.0.0.1:4319`.
2. Grafana Alloy exports batched traces to Tempo on `127.0.0.1:4317`.
3. Tempo stores traces and serves queries at `http://127.0.0.1:3200`.
4. Grafana uses the `shared-traces` datasource to query Tempo and correlate logs.

## Verify

```bash
bash scripts/shell/verify-apm-integration.sh
```

The verifier checks tracing modules, collector config, Tempo datasource provisioning,
dashboards, and optional running service health.
