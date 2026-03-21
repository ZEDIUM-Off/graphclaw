# Docs Context

## Local Purpose

This subtree is the primary human-facing documentation surface for GraphClaw. It covers the current inherited `zeroclaw` runtime, operator and contributor workflows, and the migration framing that explains how the repository is evolving toward a graph-native context engine.

## What Belongs Here

- documentation for users, operators, contributors, and maintainers;
- reference material for current CLI, API, security, setup, and SOP surfaces;
- explicit migration framing that distinguishes inherited behavior from target direction;
- stable conceptual architecture and backend reference pages under dedicated docs branches;
- documentation that frames variable reflection, exploration, packing, and orchestration strategies as target-architecture concepts rather than current runtime facts;
- localized documentation and translation governance.

## File Map

- `README.md` - docs landing page
- `architecture/` - concepts, migration, interfaces, runtime, and playground docs
- `backends/` - backend capability mapping docs
- `contributing/` - contributor and docs-process docs
- `maintainers/` - repo maps, inventories, and governance support
- `ops/`, `reference/`, `security/`, `setup-guides/` - current-state operational and reference docs
- `i18n/`, `vi/` - translation and inherited localized docs

## Routing

- concept or migration docs: go to `architecture/CONTEXT.md`
- backend mapping docs: go to `backends/CONTEXT.md`
- contributor process docs: go to `contributing/CONTEXT.md`
- operational docs: go to `ops/CONTEXT.md`, `security/CONTEXT.md`, or `setup-guides/CONTEXT.md`
- localized docs: go to `i18n/CONTEXT.md` or `vi/CONTEXT.md`

## Agent Workflow

1. Read this file, then the nearest deeper `CONTEXT.md` before editing a docs area.
2. Identify whether the target page is current-state reference, migration framing, or roadmap material.
3. Make the smallest truthful change that improves clarity.
4. Preserve inherited `zeroclaw` terminology where it still matches the implementation.
5. Run documentation validation appropriate to the scope of the change before finishing.
