# Memory Context

## Purpose

`src/memory/` contains the runtime memory subsystem: storage backends, embeddings, snapshots, chunking, recall support, and memory CLI helpers.

## File / Folder Map

- `src/memory/mod.rs` - module entry and shared wiring
- `src/memory/traits.rs` - backend and memory contracts
- `src/memory/backend.rs` - backend selection and orchestration
- `src/memory/sqlite.rs`, `postgres.rs`, `qdrant.rs`, `none.rs` - backend implementations
- `src/memory/vector.rs`, `embeddings.rs`, `chunker.rs` - vector and chunking support
- `src/memory/snapshot.rs`, `response_cache.rs`, `hygiene.rs` - supporting memory maintenance paths
- `src/memory/cli.rs` - CLI-facing memory commands

## Go Here For

- Backend selection or memory plumbing: `src/memory/backend.rs`
- Backend-specific bugs: the matching backend file
- Embedding or vector retrieval flow: `src/memory/embeddings.rs` and `src/memory/vector.rs`
- Chunking behavior: `src/memory/chunker.rs`
- Memory CLI actions: `src/memory/cli.rs`

## Current State

This is one of the most important future seam areas for GraphClaw, but today it is still an inherited memory system with multiple backends and retrieval utilities, not a graph-native context engine.

## Current Dependency Direction

- Read by the agent loop through `src/agent/memory_loader.rs`, by gateway/webhook flows that persist message state, and by CLI-facing commands through `src/memory/cli.rs`.
- Routes through `src/memory/backend.rs` and `src/memory/traits.rs` before reaching concrete backends such as `sqlite.rs`, `postgres.rs`, `qdrant.rs`, `markdown.rs`, and `none.rs`.
- Depends on embedding and chunking helpers in `embeddings.rs`, `vector.rs`, and `chunker.rs` to turn raw text into retrievable artifacts.

## GraphClaw Evolution Note

Be precise: storage, recall, and future context resolution are related but not identical. Do not describe current memory code as if graph-native context traversal already exists.

## Likely Migration Seams

1. `src/memory/traits.rs` is the seam for introducing a graph-facing storage abstraction without forcing every existing backend to disappear immediately.
2. `src/memory/backend.rs` is the seam for selecting between legacy memory backends and future graph-capable context stores.
3. `src/memory/vector.rs` and `src/memory/embeddings.rs` are where semantic recall can later become one input into context resolution rather than the whole mechanism.
4. `src/memory/chunker.rs` and category metadata are likely precursors to future graph nodes, evidence items, or packed context sections.
5. `src/memory/snapshot.rs` and `response_cache.rs` should remain operational concerns, not become dumping grounds for context-engine logic.

## What Must Stay Stable During Migration

- Existing memory backend compatibility and persistence semantics
- CLI and operator expectations around memory commands
- Current recall behavior unless a migration task explicitly changes relevance strategy
- Clear separation between persistence, ranking, chunking, and maintenance code paths

## Constraints / Cautions

- Backend compatibility and stored data formats matter.
- Retrieval quality changes can look like model regressions.
- Keep CLI, backend, and ranking concerns separated.

## How Agents Should Work Here

Identify whether the task is about persistence, embeddings, retrieval, or operator tooling before editing.

Recommended exploration order:

1. `src/memory/traits.rs`
2. `src/memory/backend.rs`
3. `src/memory/embeddings.rs` and `src/memory/vector.rs`
4. the concrete backend file you need

Write tests for behavior changes, and document new seams rather than collapsing everything into `mod.rs`.
