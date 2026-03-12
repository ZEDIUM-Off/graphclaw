# Gateway Context

## Purpose

`src/gateway/` is the HTTP boundary for the runtime: REST-like endpoints, SSE, WebSocket handling, and static asset serving.

## File / Folder Map

- `src/gateway/mod.rs` - gateway entry and shared glue
- `src/gateway/api.rs` - HTTP API handlers
- `src/gateway/sse.rs` - server-sent event streaming
- `src/gateway/ws.rs` - WebSocket handling
- `src/gateway/static_files.rs` - static file serving for the web surface

## Go Here For

- API request/response changes: `src/gateway/api.rs`
- Streaming event behavior: `src/gateway/sse.rs`
- WebSocket protocol behavior: `src/gateway/ws.rs`
- Web asset serving issues: `src/gateway/static_files.rs`

## Current State

This is a major inherited boundary between the runtime and external clients, including the web app and automation consumers.

## Current Dependency Direction

- Entered from runtime startup and command wiring in `src/main.rs` and daemon orchestration paths.
- Calls into channels, providers, memory, tools, security, and cost tracking from `src/gateway/mod.rs` and `src/gateway/api.rs`.
- Serves `web/` through `static_files.rs` and exposes streaming/session behavior through `sse.rs` and `ws.rs`.

## GraphClaw Evolution Note

Do not document the gateway as exposing a finished graph-native control plane. It currently serves inherited runtime APIs and transport mechanisms.

## Likely Migration Seams

1. `src/gateway/api.rs` is the seam for any future GraphClaw session, context, or package-management endpoints.
2. `src/gateway/sse.rs` and `src/gateway/ws.rs` are likely seams for streaming richer runtime artifacts such as resolution traces or context-pack summaries.
3. `src/gateway/static_files.rs` is strictly a delivery seam for the current web app and should not absorb backend architecture logic.

## What Must Stay Stable During Migration

- Existing HTTP, SSE, and WebSocket contracts unless the migration task explicitly changes them
- Authentication, pairing, and rate-limiting behavior
- Compatibility with the current web dashboard and automation clients

## Constraints / Cautions

- Protocol changes are compatibility-sensitive.
- Gateway behavior affects clients, docs, and deployment setups.
- Authentication, pairing, and streaming semantics must stay explicit.

## How Agents Should Work Here

Read the exact handler file plus `src/main.rs` or the caller path that wires it up. Treat API changes as contract changes, keep backward compatibility in mind, and document user-visible protocol shifts clearly.
