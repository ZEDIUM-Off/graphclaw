# Web Context

## Scope

Frontend dashboard application, TypeScript build configuration, and browser-facing runtime glue for the inherited gateway UI.

## File Map

- `package.json` - frontend package manifest
- `vite.config.ts`, `tsconfig.json`, `tsconfig.app.json`, `tsconfig.node.json` - build and type-check setup
- `src/` - application code

## Routing

Browser entry is `src/main.tsx`, app routing lives in `src/App.tsx`, and route/page composition is delegated to `src/pages/` under a shared `components/layout/` shell.

## Current State

This UI still presents inherited `zeroclaw` gateway surfaces, including pairing/auth and operational pages. It is not yet a separate GraphClaw-native frontend architecture.

## GraphClaw Relevance

The web app is part of the migration story because it exposes the runtime to operators, but it must describe current backend behavior truthfully while the context-engine direction is still emerging.

## Cautions

- Do not change docs here in a way that implies backend routes or product identity have already been migrated.
- Keep frontend structure aligned with real gateway contracts and current package/tooling layout.

## Agent Guidance

- Read `web/src/CONTEXT.md` and the nearest child context before editing a frontend slice.
- Treat this subtree as a compatibility UI over the current runtime, not as proof that migration is complete.
