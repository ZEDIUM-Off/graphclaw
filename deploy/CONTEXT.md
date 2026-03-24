# Deploy Context

## Purpose

This subtree contains deployable current-state runtime assets for GraphClaw.

The focus here is operational packaging of the inherited `zeroclaw` runtime: compose stacks, environment templates, tunnel/access scaffolding, and operator helpers that make the existing system deployable without pretending the target graph-native architecture already exists.

## What Belongs Here

- deployment-specific compose definitions and environment templates;
- operator bootstrap, preflight, smoke-test, backup, and restore helpers;
- access-layer scaffolding such as Cloudflare Tunnel templates;
- deployment notes that are tighter and more execution-oriented than the broader operations docs in `docs/ops/`.

## Routing

- OVH VPS deployment assets: `deploy/ovh-vps/`
- broader operator procedures and troubleshooting: `docs/ops/`
- runtime config semantics: `src/config/`
- gateway exposure behavior: `src/gateway/` and `src/tunnel/`

## Current State

This area packages the inherited runtime for real deployment. It does not redefine product behavior or rename current technical surfaces.

## Cautions

- Keep deployment assets truthful to the current runtime and dashboard under `web/`.
- Preserve inherited `zeroclaw` identifiers where they are actual runtime interfaces.
- Prefer secure-by-default exposure patterns that avoid direct public gateway binds.

## Agent Guidance

Read the nearest child docs before editing. When a deployment asset changes operational behavior, update the corresponding operator documentation in `docs/ops/` and the root routing docs in the same pass.
