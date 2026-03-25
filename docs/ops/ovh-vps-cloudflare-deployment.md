# OVH VPS Deployment with Cloudflare Tunnel

Last verified: **March 24, 2026**.

This guide documents a deployable current-state GraphClaw profile for `HIGHFINITY`.

## Scope

This guide is intentionally about the current inherited runtime:

- repository identity is `GraphClaw`;
- deployed binary and gateway remain the inherited `zeroclaw` runtime;
- deployed browser UI remains the inherited dashboard under `web/`;
- the graph-native context engine remains future architecture, not current runtime;
- the separate `ui/` subtree is not the production operator dashboard for this guide.

## Facts

- The gateway already refuses a non-local bind unless a tunnel is configured or `allow_public_bind = true`.
- The runtime exposes `/health`, `/metrics`, `/api/health`, pairing endpoints, and the embedded dashboard.
- The repository now includes a production-oriented compose profile at [`../../deploy/ovh-vps/`](../../deploy/ovh-vps/README.md).
- The Docker build now bundles the current `web/` dashboard into the production image instead of relying on a prebuilt `web/dist` checked into the repo.

## Deployment Choice

This guide uses:

- Docker Compose as the deployment primitive;
- PostgreSQL for persistent runtime memory in the first serious operator deployment;
- Cloudflare Tunnel as the remote access path;
- Cloudflare Access as the first identity gate;
- GraphClaw pairing as the second barrier inside the runtime.

## Security Posture

- The gateway is not published on a public host interface.
- Compose binds the gateway only to `127.0.0.1` on the VPS.
- Cloudflare Tunnel is the intended remote browser path.
- Pairing remains enabled by default.
- Secrets stay in `.env` or Cloudflare-managed tunnel state and are not committed.
- The runtime container runs non-root with a read-only root filesystem and dropped Linux capabilities.

## Why `allow_public_bind = true` Appears Here

Inside Docker, the runtime must listen on the private container network so that:

- `cloudflared` can reach it;
- local host-bound health checks can still work via `127.0.0.1:${HOST_GATEWAY_PORT}`.

That is why this deployment profile sets `allow_public_bind = true`.

This is acceptable in this profile because the compose stack does **not** publish the gateway on a public host interface. The risk boundary is Docker networking plus localhost publishing, not direct internet exposure.

## Prerequisites

- OVH VPS with Docker Engine and Compose plugin installed
- Cloudflare Zero Trust account
- Cloudflare Access application and policy for the dashboard hostname
- one provider API key for the runtime
- DNS for the private operator hostname

Minimum practical VPS envelope:

- 2 vCPU
- 4 GB RAM
- 40 GB SSD

## Files To Prepare

From the repository root:

```bash
cd deploy/ovh-vps
cp .env.example .env
cp config/config.toml.example config/config.toml
chmod +x preflight.sh smoke-test.sh
```

Then edit:

- `deploy/ovh-vps/.env`
- `deploy/ovh-vps/config/config.toml`

## Required Secret Inputs

`deploy/ovh-vps/.env`:

- `ZEROCLAW_API_KEY`
- `POSTGRES_PASSWORD`
- `CLOUDFLARE_TUNNEL_TOKEN`

Recommended non-secret edits:

- `ZEROCLAW_PROVIDER`
- `ZEROCLAW_MODEL`
- `GRAPHCLAW_IMAGE`
- `HOST_GATEWAY_PORT`

For this profile, the PostgreSQL connection URL is composed automatically from
`POSTGRES_USER`, `POSTGRES_PASSWORD`, and `POSTGRES_DB` inside
[`../../deploy/ovh-vps/compose.yaml`](../../deploy/ovh-vps/compose.yaml). Keep
those values coherent and do not maintain a separate
`ZEROCLAW_STORAGE_DB_URL` line in `deploy/ovh-vps/.env`.

## Cloudflare Setup

1. Create a tunnel in Cloudflare Zero Trust.
2. Route a hostname to that tunnel.
3. Create a Cloudflare Access application for that hostname.
4. Restrict allowed identities to Matisse, Evan, and Tom.
5. Copy the issued tunnel token into `deploy/ovh-vps/.env` as `CLOUDFLARE_TUNNEL_TOKEN`.

If you prefer the named-tunnel credentials-file path instead of token mode, start from [`../../deploy/ovh-vps/cloudflared/config.yml.example`](../../deploy/ovh-vps/cloudflared/config.yml.example).

## Deploy

Run:

```bash
cd deploy/ovh-vps
./preflight.sh
docker compose up -d --build
./smoke-test.sh
```

Useful verification:

```bash
docker compose ps
docker compose logs --tail=100 graphclaw
docker compose logs --tail=100 cloudflared
curl -fsS http://127.0.0.1:${HOST_GATEWAY_PORT:-42617}/health
```

Expected:

- `postgres` healthy
- `graphclaw` healthy
- `cloudflared` running
- `/health` returns `status = ok`
- `/health` reports `"require_pairing": true`

## First Access

1. Open the Cloudflare-protected hostname in the browser.
2. Authenticate through Cloudflare Access.
3. Reach the inherited dashboard served by the gateway.
4. Pair the device through the dashboard or pairing endpoint.

Do not disable pairing just because Cloudflare Access exists. Access protects the perimeter; pairing protects the runtime session boundary.

## Update Procedure

```bash
cd /path/to/graphclaw/deploy/ovh-vps
git pull
docker compose up -d --build
./smoke-test.sh
```

If you pin images in your own registry instead of building on-host, update the `GRAPHCLAW_IMAGE` value and pull before restarting.

## Logs and Health

```bash
cd deploy/ovh-vps
docker compose logs -f graphclaw
docker compose logs -f cloudflared
docker compose logs -f postgres
curl -fsS http://127.0.0.1:${HOST_GATEWAY_PORT:-42617}/health
curl -fsS http://127.0.0.1:${HOST_GATEWAY_PORT:-42617}/metrics
```

## Backup

Back up both:

- PostgreSQL data
- GraphClaw data volume at `/zeroclaw-data`

Recommended commands:

```bash
cd deploy/ovh-vps
mkdir -p backups
docker compose exec -T postgres pg_dump -U "${POSTGRES_USER}" "${POSTGRES_DB}" > "backups/postgres-$(date +%F-%H%M%S).sql"
docker run --rm \
  -v graphclaw-ovh_graphclaw-data:/from:ro \
  -v "$(pwd)/backups:/to" \
  alpine:3.22 \
  sh -c 'tar czf /to/graphclaw-data-$(date +%F-%H%M%S).tar.gz -C /from .'
```

If your compose project name changes, adjust the volume name accordingly.

## Restore

1. Stop the stack.
2. Restore the GraphClaw data volume from the tar archive.
3. Restore PostgreSQL from the SQL dump.
4. Start the stack again.
5. Run the smoke test.

Example restore flow:

```bash
cd deploy/ovh-vps
docker compose down
docker run --rm \
  -v graphclaw-ovh_graphclaw-data:/to \
  -v "$(pwd)/backups:/from:ro" \
  alpine:3.22 \
  sh -c 'cd /to && tar xzf /from/graphclaw-data-REPLACE_ME.tar.gz'
docker compose up -d postgres
cat backups/postgres-REPLACE_ME.sql | docker compose exec -T postgres psql -U "${POSTGRES_USER}" "${POSTGRES_DB}"
docker compose up -d
./smoke-test.sh
```

## Access Rotation

When team access changes:

1. update the Cloudflare Access policy first;
2. revoke GraphClaw paired devices or rotate tokens from the dashboard/API;
3. if the tunnel token is suspected compromised, rotate it in Cloudflare and update `.env`;
4. restart `cloudflared`.

## Validation Commands

Use these commands to prove the deployment definition is coherent before first rollout:

```bash
cd deploy/ovh-vps
./preflight.sh
docker compose config
docker compose build
docker compose up -d
./smoke-test.sh
```

Repo validation after documentation changes:

```bash
./scripts/ci/docs_quality_gate.sh
./scripts/ci/docs_links_gate.sh
./scripts/ci/docs_canonical_concepts_gate.sh
```

## Known Limitations

- The deployed runtime is still the inherited baseline, so many commands and technical interfaces remain named `zeroclaw`.
- The first-production profile depends on Cloudflare for remote browser access; it is not designed for direct public internet exposure.
- Pairing and auth remain inherited runtime behavior; this guide does not replace them with a new IAM model.
- The `ui/` subtree remains a separate migration surface and is not the operator dashboard in this deployment.

## Recommended Next Steps After First Deploy

- move the built image to a private registry if VPS-side build times become annoying;
- add off-host encrypted backups for both Postgres dumps and the GraphClaw data volume;
- wire host-level monitoring around container restarts and disk pressure;
- decide whether to keep PostgreSQL as the long-term current-state backend or deliberately standardize on persisted SQLite for a simpler single-node profile.
