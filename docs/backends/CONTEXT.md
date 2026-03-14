# Docs Backends Context

## Local Purpose

This subtree documents how concrete backends support, constrain, or degrade the GraphClaw concept model. It exists to map stable GraphClaw semantics onto backend capabilities without letting backend procedure catalogs redefine the architecture.

## What Belongs Here

- backend reference pages for graph storage and query engines;
- capability mapping between GraphClaw concepts and backend primitives;
- coupling, fallback, and degradation guidance for backend integrations.

## What Does Not Belong Here

- backend-agnostic concept definitions that belong in `docs/architecture/`;
- source-level adapter implementation notes that belong next to code;
- operational runbooks for the current product that belong in `docs/ops/`.

## File Map

- `README.md` - entrypoint for backend references
- `memgraph.md` - reference mapping for the Memgraph backend

## Routing

- backend capability mapping belongs here
- stable concept definitions belong in `docs/architecture/`
- runtime ownership boundaries belong in the nearest local `CONTEXT.md` files

## References

- `docs/architecture/graph-context-engine.md` - source of stable GraphClaw concepts
- `docs/architecture/views-and-sets.md` - source of `View`, `GraphSet`, and packability semantics
- `docs/architecture/context-artifacts.md` - source of artifact and budget distinctions
- `docs/architecture/future-integration-seams.md` - source of interface-family framing
- `docs/README.md` - docs-tree routing
- `README.md` - top-level migration framing

## Current Inherited State

The current repository does not yet expose a complete GraphClaw graph backend layer. This subtree documents the intended integration direction without implying that the adapter boundary is already fully implemented.

## GraphClaw Migration Relationship

This subtree supports the first concrete backend axis of migration. It should help maintainers reason about capabilities, limits, and fallbacks before backend-facing code becomes a hard dependency.

## Cautions

- do not let backend mechanism names replace GraphClaw business concepts
- do not imply that a reference backend is the only possible backend
- do not write operational deployment docs here unless the task is explicitly backend-ops focused

## Agent Workflow

1. Read this file before editing backend reference pages in this subtree.
2. Start from the GraphClaw concept model, then list capability needs, then map backend mechanisms.
3. Mark edition-dependent or optional backend features explicitly.
4. Call out fallback paths and coupling risks whenever backend support is incomplete.
