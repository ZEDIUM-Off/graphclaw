# Memgraph crate context

## Purpose

Low-level Memgraph client for GraphClaw. This crate provides graph primitives only: connection, create node/edge, read subgraph, neighborhood, and selector-oriented fetches. It does **not** contain set-composition logic; that lives in the `sets` crate.

## What belongs here

- `MemgraphConfig`, `MemgraphClient`
- Types: `GraphNode`, `GraphEdge`, `Subgraph`
- Primitives: `create_node`, `create_edge`, `get_subgraph`, `select_nodes`, `neighborhood`, `inspect_node`, `run`
- Driver-facing safety for bounded graph reads and identifier validation

## What does not belong here

- `SetDefinition`, `BoundSet`, `ResolvedSet`, or any set composition logic
- Gateway or API layer

## Current Notes

- The crate now uses the official Memgraph Rust client family (`rsmgclient`) rather than `neo4rs`.
- Returned subgraphs should remain closed over retained nodes: edges whose endpoints are absent must not survive normalization.

## References

- `docs/backends/memgraph.md` — GraphClaw backend reference
- `docs/architecture/playground/set-system-spec-v0.md` — Set System v0 (consumes this crate via `sets`)
