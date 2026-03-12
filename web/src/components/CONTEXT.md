# Web Components Context

## Scope

Shared frontend structure components, currently centered on the dashboard layout shell.

## File Map

- `layout/Layout.tsx` - top-level route wrapper
- `layout/Header.tsx` - shared header chrome
- `layout/Sidebar.tsx` - primary navigation shell

## Routing

`web/src/App.tsx` mounts `Layout` around the page routes, and `Layout` composes `Header` and `Sidebar` to give all route-level pages a consistent frame.

## Current State

This directory is intentionally small and mostly layout-oriented. Route-specific behavior still lives in `web/src/pages/`.

## GraphClaw Relevance

Layout components shape how operators experience GraphClaw's transition, but they should stay grounded in the current runtime pages rather than advertise a future UX that does not exist yet.

## Cautions

- Keep page data fetching and transport logic out of layout components.
- Avoid burying navigation assumptions here that really belong in the route table.

## Agent Guidance

- Put shared structural UI here; keep feature-specific composition in `pages/`.
- If a component becomes reusable beyond layout, document that boundary clearly before growing this subtree.
