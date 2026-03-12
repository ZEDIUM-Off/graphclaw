# Web Src Context

## Scope

Source tree for the dashboard UI: app bootstrapping, route composition, shared layout, frontend data hooks, transport helpers, and API-facing types.

## File Map

- `main.tsx` - browser bootstrap
- `App.tsx` - auth gate, locale context, and route table
- `index.css` - shared styling entry
- `components/` - layout and reusable UI structure
- `hooks/` - auth and transport-aware React hooks
- `lib/` - API, auth, SSE, WebSocket, and i18n utilities
- `pages/` - route-level screens
- `types/` - shared frontend contract types
- `vite-env.d.ts` - Vite typing glue

## Routing

`App.tsx` currently routes `/`, `/agent`, `/tools`, `/cron`, `/integrations`, `/memory`, `/config`, `/cost`, `/logs`, and `/doctor` through `Layout`, with unauthenticated users gated by the pairing dialog.

## Current State

The source tree is compact and still organized around inherited operational pages rather than a GraphClaw-specific information architecture.

## GraphClaw Relevance

This subtree is where future GraphClaw UX can evolve, but for now it should continue to mirror the baseline runtime honestly and preserve inherited naming where the code still depends on it.

## Cautions

- Keep page logic, shared hooks, and transport helpers separated; avoid moving everything into `App.tsx`.
- Do not encode speculative future routes or API contracts in context docs or types.

## Agent Guidance

- Move downward to the nearest child `CONTEXT.md` before editing a slice.
- When documenting or changing navigation, verify the real route table in `App.tsx` first.
