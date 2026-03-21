# Root Context

## Purpose

This repository is GraphClaw: a transitional fork baseline being evolved from ZeroClaw toward a graph-native context engine.

At the root level, documentation is expected to fix meaning before it fixes implementation details.

Reference documentation under `docs/` defines concepts. `AGENTS.md` and `CONTEXT.md` route readers and contextualize how those concepts appear in the codebase.

## What Belongs Here

- repo-wide narrative and agent rules;
- top-level entry documents and workspace metadata;
- framing that applies across multiple subtrees;
- routing into the conceptual architecture and backend reference branches under `docs/`.
- repo-wide rules that separate canonical concept reference from local codebase contextualization.

## Key Files

- `README.md` - repository identity and top-level navigation
- `AGENTS.md` - repo-wide agent contract and reading order
- `docs/README.md` - documentation entrypoint
- `docs/architecture/` - canonical concept and architecture docs
- `docs/backends/` - backend capability mapping docs
- `src/`, `tests/`, `web/`, `python/`, `firmware/` - primary implementation subtrees

## Task Routing

- repo framing or root docs: read `README.md`, `AGENTS.md`, then this file
- architecture concepts or migration docs: read `docs/CONTEXT.md`, then `docs/architecture/CONTEXT.md`
- implementation work: move to the nearest subtree `CONTEXT.md`
- backend stack or Memgraph setup: read `docs/backends/CONTEXT.md` or `memgraph/CONTEXT.md`

## How Agents Should Work Here

Use this file to orient yourself, then move to the nearest local `CONTEXT.md` before changing a specific area.
