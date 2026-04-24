# APM Quick Reference

## Endpoints

| Surface | Default |
| --- | --- |
| App OTLP gRPC | `PHENO_OBSERVABILITY_OTLP_GRPC_ENDPOINT=127.0.0.1:4319` |
| Tempo OTLP gRPC | `127.0.0.1:4317` |
| Tempo OTLP HTTP | `http://127.0.0.1:4318` |
| Grafana | `PHENO_OBSERVABILITY_GRAFANA_URL=http://127.0.0.1:3000` |
| Tempo | `PHENO_OBSERVABILITY_TEMPO_URL=http://127.0.0.1:3200` |
| Repo-local Alloy readiness | `http://127.0.0.1:12345/-/ready` |

## Commands

```bash
bash scripts/shell/check-loki-installation.sh
bash scripts/shell/verify-apm-integration.sh
bash scripts/shell/tempo-if-not-running.sh
bash scripts/shell/alloy-if-not-running.sh
```

## Grafana Provisioning

- Tempo datasource: `deploy/monitoring/grafana/provisioning/datasources/tempo.yml`
- Loki datasource: `deploy/monitoring/grafana/provisioning/datasources/loki.yml`
- Prometheus datasource: `deploy/monitoring/grafana/provisioning/datasources/prometheus.yaml`
