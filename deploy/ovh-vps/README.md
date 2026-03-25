# GraphClaw on OVH VPS with Cloudflare Tunnel

This deployment profile packages the current inherited runtime for a small trusted operator team.

It is designed for:

- `HIGHFINITY` internal use;
- an OVH VPS;
- browser access to the inherited dashboard under `web/`;
- Cloudflare Tunnel plus Cloudflare Access as the primary remote access barrier;
- no direct public exposure of the gateway port.

## Current-State Truth

- Repository identity: `GraphClaw`
- Deployable runtime: inherited `zeroclaw` binary and gateway
- Deployable browser UI for this profile: inherited dashboard under `web/`
- Out of scope for this deployment: the separate `ui/` subtree as a production operator surface

## Topology

```text
Matisse / Evan / Tom
  -> Cloudflare Access
  -> Cloudflare Tunnel
  -> OVH VPS
     -> GraphClaw runtime container
     -> PostgreSQL
     -> persistent GraphClaw data volume
```

The gateway is published only to `127.0.0.1` on the VPS for local verification and emergency SSH-only access. Remote browser access is expected to go through Cloudflare Access, not through a public gateway bind.

## Files

- `compose.yaml` - production-oriented compose stack
- `.env.example` - required environment and secret placeholders
- `config/config.toml.example` - production runtime config template
- `cloudflared/config.yml.example` - optional named-tunnel config template
- `preflight.sh` - local validation for files, env, and compose coherence
- `smoke-test.sh` - post-deploy health and gateway checks

## Quick Start

1. Copy the templates:

```bash
cd deploy/ovh-vps
cp .env.example .env
cp config/config.toml.example config/config.toml
```

2. Fill in the secrets in `.env`.

   The PostgreSQL connection URL is derived automatically by `compose.yaml` from
   `POSTGRES_USER`, `POSTGRES_PASSWORD`, and `POSTGRES_DB`. Do not maintain a
   separate `ZEROCLAW_STORAGE_DB_URL` value in `.env` for this profile.

3. Review `config/config.toml` and keep `gateway.require_pairing = true`.

4. Run the preflight:

```bash
./preflight.sh
```

5. Build and start the stack:

```bash
docker compose up -d --build
```

6. Run the smoke test:

```bash
./smoke-test.sh
```

## Cloudflare Access Posture

Recommended:

- create a dedicated Cloudflare Access application for the dashboard hostname;
- allow only the identities for Matisse, Evan, and Tom;
- keep GraphClaw gateway pairing enabled as a second barrier inside the app;
- do not publish the VPS gateway port on a public interface.

## Runtime and Storage Choice

This profile builds the current runtime with the optional `memory-postgres` feature enabled and uses PostgreSQL as the primary persistent memory backend.

Rationale:

- PostgreSQL is a cleaner first-production persistence target than container-local SQLite for an always-on VPS;
- the codebase already supports it behind the existing storage configuration;
- the deployment stays on the inherited runtime instead of inventing a new architecture.

If you need a lower-complexity fallback, you can switch back to SQLite by removing the Postgres environment overrides and service, but this profile intentionally optimizes for the more durable operator deployment.

## Operational Notes

- The app container runs as non-root from the release image.
- The container root filesystem is read-only; runtime state lives under `/zeroclaw-data`.
- `allow_public_bind = true` is set for the containerized profile because the app must listen on the private Docker network for `cloudflared` and localhost-published health checks. This does not mean direct internet exposure when the compose file is used as written.

## Related Docs

- `../../docs/ops/ovh-vps-cloudflare-deployment.md`
- `../../docs/ops/operations-runbook.md`
- `../../docs/ops/network-deployment.md`
