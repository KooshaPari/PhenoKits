#!/usr/bin/env bash
# Start redis_exporter for the Redis-compatible endpoint only if port 9121 is not in use.

set -e
REDIS_EXPORTER_PORT="${REDIS_EXPORTER_PORT:-9121}"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

if ! command -v redis_exporter >/dev/null 2>&1; then
  echo "redis_exporter not found; skipping optional Redis-compatible exporter."
  exec tail -f /dev/null
fi

bash "$ROOT/scripts/shell/guard-port.sh" "redis_exporter" "$REDIS_EXPORTER_PORT" "redis_exporter"

exec redis_exporter --redis.addr=redis://localhost:6379
