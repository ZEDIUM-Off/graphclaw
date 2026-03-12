# Tunnel Context

## Purpose

`src/tunnel/` contains tunnel-provider integrations used to expose local runtime services externally.

## File / Folder Map

- `src/tunnel/mod.rs` - tunnel selection and shared entrypoints
- `src/tunnel/cloudflare.rs` - Cloudflare tunnel support
- `src/tunnel/ngrok.rs` - ngrok support
- `src/tunnel/tailscale.rs` - Tailscale support
- `src/tunnel/custom.rs` - custom tunnel behavior
- `src/tunnel/none.rs` - disabled/no-tunnel behavior

## Go Here For

- Tunnel selection or shared behavior: `src/tunnel/mod.rs`
- Provider-specific tunnel setup: the matching provider file
- No-tunnel fallback behavior: `src/tunnel/none.rs`

## Current State

This is inherited operational infrastructure that sits near gateway and deployment behavior.

## GraphClaw Evolution Note

Do not present tunnels as part of an already-built graph-native runtime layer. They remain conventional provider integrations for exposing services.

## Constraints / Cautions

- Provider assumptions and auth/setup details vary.
- User-visible network exposure behavior must stay explicit.
- Keep gateway concerns and tunnel-provider specifics separated.

## How Agents Should Work Here

Read `src/tunnel/mod.rs` and the concrete provider file together. Keep provider-specific logic isolated, document exposure assumptions clearly, and verify downstream effects on gateway startup and operator workflows.
