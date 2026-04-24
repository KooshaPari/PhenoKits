#!/usr/bin/env bash
# Start repo-local Grafana Tempo unless a Tempo-compatible service is already ready.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
TEMPO_HTTP_PORT="${TEMPO_HTTP_PORT:-3200}"
TEMPO_OTLP_GRPC_PORT="${TEMPO_OTLP_GRPC_PORT:-4317}"
TEMPO_CONFIG="${TEMPO_CONFIG:-deploy/monitoring/tempo-local.yaml}"
TEMPO_IMAGE="${TEMPO_IMAGE:-grafana/tempo:2.7.1}"

if curl -fsS "http://127.0.0.1:${TEMPO_HTTP_PORT}/ready" >/dev/null 2>&1; then
  echo "Grafana Tempo already ready on port ${TEMPO_HTTP_PORT}."
  exec tail -f /dev/null
fi

cd "$ROOT"
mkdir -p .tempo/wal .tempo/blocks .process-compose/logs

if [ ! -f "$TEMPO_CONFIG" ]; then
  echo "Tempo config not found: $TEMPO_CONFIG" >&2
  exit 1
fi

if command -v tempo >/dev/null 2>&1; then
  echo "Starting Grafana Tempo on port ${TEMPO_HTTP_PORT}..."
  exec tempo -config.file="$TEMPO_CONFIG"
fi

if command -v docker >/dev/null 2>&1 && docker info >/dev/null 2>&1; then
  container_name="${TEMPO_CONTAINER_NAME:-tracera-tempo-local}"
  docker_config=".tempo/tempo-docker.yaml"
  sed 's/127\.0\.0\.1/0.0.0.0/g' "$TEMPO_CONFIG" > "$docker_config"
  if docker ps --format '{{.Names}}' | grep -qx "$container_name"; then
    echo "Tempo container ${container_name} is already running."
    exec tail -f /dev/null
  fi
  if docker ps -a --format '{{.Names}}' | grep -qx "$container_name"; then
    docker rm "$container_name" >/dev/null
  fi
  echo "Starting Grafana Tempo container ${container_name}..."
  exec docker run --rm --name "$container_name" \
    -w /tmp \
    -p "127.0.0.1:${TEMPO_HTTP_PORT}:3200" \
    -p "127.0.0.1:${TEMPO_OTLP_GRPC_PORT}:4317" \
    -p "127.0.0.1:4318:4318" \
    -v "$ROOT/$docker_config:/etc/tempo.yaml:ro" \
    -v "$ROOT/.tempo:/tmp/tempo" \
    "$TEMPO_IMAGE" -config.file=/etc/tempo.yaml
fi

echo "Grafana Tempo binary not found and Docker is unavailable." >&2
echo "Install Tempo or start an external Tempo at http://127.0.0.1:${TEMPO_HTTP_PORT}." >&2
exit 1
