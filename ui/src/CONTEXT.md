# UI Src Context

## Local Purpose

Vue source tree for the future GraphClaw interface.

For the current migration step, this source tree owns only the bounded graph playground: route bootstrapping, gateway graph fetches, Sigma rendering, and graph-element inspection.

## File Map

- `main.ts` - Vue bootstrap
- `App.vue` - application shell
- `router/` - route table
- `views/` - route-level pages
- `components/` - shared UI widgets
- `composables/` - reusable gateway and state logic
- `lib/` - API transport helpers and shared types

## Routing

- route composition belongs in `views/`
- reusable graph UI belongs in `components/`
- gateway fetch and inspection flow belongs in `composables/` and `lib/`

## Current State

This tree is intentionally small and playground-focused. It is the seed of the future UI, not yet a replacement for the inherited dashboard in `web/`.
