#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

echo "Checking required files..."
test -f .env || { echo "Missing deploy/ovh-vps/.env"; exit 1; }
test -f config/config.toml || { echo "Missing deploy/ovh-vps/config/config.toml"; exit 1; }

set -a
. ./.env
set +a

echo "Checking docker and compose availability..."
docker --version >/dev/null
docker compose version >/dev/null

echo "Checking required environment values..."
grep -Eq '^ZEROCLAW_API_KEY=.+$' .env || { echo "ZEROCLAW_API_KEY is missing in .env"; exit 1; }
grep -Eq '^POSTGRES_PASSWORD=.+$' .env || { echo "POSTGRES_PASSWORD is missing in .env"; exit 1; }
grep -Eq '^CLOUDFLARE_TUNNEL_TOKEN=.+$' .env || { echo "CLOUDFLARE_TUNNEL_TOKEN is missing in .env"; exit 1; }

echo "Validating compose config..."
docker compose config >/dev/null

echo "Preflight OK"
