# Views crate context

## Purpose

View composition and resolution for GraphClaw. Holds ViewTemplate, BoundView, ResolvedView; resolves through a bounded graph-backend interface; exports to LLM (`llm_compact`, `llm_explained`).

## What belongs here

- Types: ViewTemplate, BoundView, ResolvedView, ResolvedViewExport, ViewKind, ViewOperation, Selectors
- ViewsService: in-memory template store, bind, resolve (via graph backend trait + Memgraph adapter)
- Export: export_llm_compact, export_llm_explained, export_for_llm
- Internal materialized working set for bounded set algebra and closed-subgraph invariants

## What does not belong here

- Gateway or HTTP layer
- Full Context Engine (ContextPack, SessionWindow, etc.)

## Current Notes

- This crate should keep GraphClaw view semantics primary and backend mechanics secondary.
- `ResolvedView` in this slice is a materialized result for the playground, not the full GraphClaw `ContextPack`.

## References

- `docs/architecture/view-system-spec-v0.md` — View System v0
- `crates/memgraph` — graph backend used for resolution
