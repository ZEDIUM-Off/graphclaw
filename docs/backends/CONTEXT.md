# Docs Backends Context

## Local Purpose

This subtree documents how concrete backends support, constrain, or degrade the GraphClaw concept model. It exists to map stable GraphClaw semantics onto backend capabilities without letting backend procedure catalogs redefine the architecture.

## What Belongs Here

- backend reference pages for graph storage and query engines;
- capability mapping between GraphClaw concepts and backend primitives;
- coupling, fallback, and degradation guidance for backend integrations.

## File Map

- `README.md` - backend docs entrypoint
- `memgraph.md` - Memgraph capability mapping and backend notes

## Routing

- concept meaning lives upstream in `docs/architecture/CONTEXT.md`
- backend-specific mapping work belongs in this directory
- concrete stack operation for the local Memgraph environment belongs in `../memgraph/CONTEXT.md`

## Agent Workflow

1. Read this file before editing backend reference pages in this subtree.
2. Start from the GraphClaw concept model, then list capability needs, then map backend mechanisms.
3. Mark edition-dependent or optional backend features explicitly.
4. Call out fallback paths and coupling risks whenever backend support is incomplete.
