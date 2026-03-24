#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

set -a
. ./.env
set +a

HEALTH_URL="${HEALTH_URL:-http://127.0.0.1:${HOST_GATEWAY_PORT:-42617}/health}"
TMP_HEALTH_FILE="$(mktemp)"

trap 'rm -f "$TMP_HEALTH_FILE"' EXIT

echo "Waiting for gateway health at ${HEALTH_URL}..."
for _ in $(seq 1 30); do
  if curl -fsS "$HEALTH_URL" >"$TMP_HEALTH_FILE" 2>/dev/null; then
    break
  fi
  sleep 2
done

test -s "$TMP_HEALTH_FILE" || {
  echo "Gateway health endpoint did not become ready"
  exit 1
}

echo "Checking health payload..."
grep -q '"status":"ok"' "$TMP_HEALTH_FILE" || {
  echo "Health payload does not report status=ok"
  cat "$TMP_HEALTH_FILE"
  exit 1
}

grep -q '"require_pairing":true' "$TMP_HEALTH_FILE" || {
  echo "Gateway is not enforcing pairing"
  cat "$TMP_HEALTH_FILE"
  exit 1
}

echo "Smoke test OK"
