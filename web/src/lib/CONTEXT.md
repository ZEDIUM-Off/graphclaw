# Web Lib Context

## Scope

Shared frontend infrastructure for transport, auth helpers, and locale support.

## File Map

- `api.ts` - HTTP client helpers
- `auth.ts` - auth-related utility logic
- `sse.ts` - server-sent event plumbing
- `ws.ts` - WebSocket utilities
- `i18n.ts` - locale helpers used by the app shell

## Routing

`web/src/hooks/` wraps these utilities for React usage, while `web/src/pages/` consumes the hook layer rather than reaching into transport helpers directly.

## Current State

This directory is the frontend infrastructure layer for the inherited dashboard and gateway protocols.

## GraphClaw Relevance

It will matter during migration because transport contracts often change last, but the docs here should still describe current baseline behavior instead of speculative GraphClaw-native APIs.

## Cautions

- Keep this subtree presentation-free.
- Do not invent new protocol names or rename inherited surfaces unless the task explicitly changes those contracts.

## Agent Guidance

- Shared browser-side transport code belongs here.
- When a change affects auth, SSE, or WebSocket behavior, verify the matching backend contract before editing.
