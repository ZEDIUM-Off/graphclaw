# RAG Context

## Local Purpose

`src/rag/` is a small runtime entrypoint for retrieval-augmented support behavior.

This area is intentionally narrow. It should document a retrieval seam, not masquerade as the full GraphClaw context layer.

## What Belongs Here

- narrowly scoped RAG-facing entrypoints;
- thin glue that clearly belongs neither in `src/agent/` nor in `src/memory/`.

## What Does Not Belong Here

- a generic dumping ground for context-engine work;
- backend adapter documentation;
- broad memory or agent-loop ownership that belongs elsewhere.

## File / Folder Map

- `src/rag/mod.rs` - current RAG-facing surface

## Go Here For

- Current RAG entrypoints: `src/rag/mod.rs`
- Related retrieval/storage behavior: also inspect `src/memory/`

## Current State

This area is intentionally thin. It should not be mistaken for the future GraphClaw context engine or for the full retrieval stack, which largely lives elsewhere.

If it grows, the documentation should explain why that growth belongs here instead of in `src/memory/` or a future dedicated context-layer seam.

## Routing

- retrieval implementation detail usually belongs in `src/memory/`
- turn orchestration belongs in `src/agent/`
- stable GraphClaw concept framing belongs in `docs/architecture/`

## GraphClaw Evolution Note

Do not claim a graph-native retrieval layer already exists here. If this area grows, describe the seam precisely.

## Constraints / Cautions

- Avoid turning this module into a vague grab-bag.
- Keep ownership boundaries clear with `src/memory/` and `src/agent/`.
- Favor explicit interfaces over placeholder abstractions.

## References

- `src/memory/CONTEXT.md` - main retrieval and storage boundary
- `src/agent/CONTEXT.md` - agent-loop consumer boundary
- `docs/architecture/graph-context-engine.md` - distinction between retrieval support and context-engine logic

## How Agents Should Work Here

Read the whole module and identify whether the real behavior belongs here or in `src/memory/` first. Expand scope only with a clear boundary and update this context when the folder stops being a thin facade.
