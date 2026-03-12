# RAG Context

## Purpose

`src/rag/` is a small runtime entrypoint for retrieval-augmented support behavior.

## File / Folder Map

- `src/rag/mod.rs` - current RAG-facing surface

## Go Here For

- Current RAG entrypoints: `src/rag/mod.rs`
- Related retrieval/storage behavior: also inspect `src/memory/`

## Current State

This area is intentionally thin. It should not be mistaken for the future GraphClaw context engine or for the full retrieval stack, which largely lives elsewhere.

## GraphClaw Evolution Note

Do not claim a graph-native retrieval layer already exists here. If this area grows, describe the seam precisely.

## Constraints / Cautions

- Avoid turning this module into a vague grab-bag.
- Keep ownership boundaries clear with `src/memory/` and `src/agent/`.
- Favor explicit interfaces over placeholder abstractions.

## How Agents Should Work Here

Read the whole module and identify whether the real behavior belongs here or in `src/memory/` first. Expand scope only with a clear boundary and update this context when the folder stops being a thin facade.
