# UI Context

## Local Purpose

Future GraphClaw web interface built as a separate `Vue 3 + pnpm + Vite+` project.

This subtree is intentionally distinct from the inherited `web/` dashboard. In the current migration step it owns only the graph playground surface and the gateway communication needed to read that graph.

## What Belongs Here

- the future GraphClaw browser interface;
- Vue application bootstrap, routing, and UI composition for new surfaces;
- frontend transport helpers and types specific to the new UI;
- temporary playground-only pages that will later expand into the broader interface.

## File Map

- `package.json` - UI package manifest using `pnpm`
- `vite.config.ts` - Vite+/Vite configuration and gateway proxying
- `src/` - Vue application source tree

## Routing

The current route surface is intentionally narrow:

- `/playground` - graph playground backed by gateway snapshot routes
- `/` - redirect to `/playground`

## Current State

This subtree is a migration seam, not the already-complete GraphClaw web product. It should stay truthful about current gateway capabilities and remain scoped to the playground until explicit tasks broaden it.

## References

- `ui/src/CONTEXT.md` - source-tree routing and component boundaries
- `web/CONTEXT.md` - inherited dashboard boundary that still exists separately
- `src/gateway/CONTEXT.md` - backend transport boundary

## Cautions

- do not imply that the old `web/` subtree has already been replaced
- do not invent Graph Context Engine runtime artifacts in the UI before the backend exposes them
- keep the current migration step bounded to the playground unless the task explicitly broadens scope
