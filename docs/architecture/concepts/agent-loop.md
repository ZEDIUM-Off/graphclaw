# Agent Loop

## Status

This document frames the mono-agent loop in the GraphClaw target architecture.

It is target-architecture documentation.

## Purpose

The aim is to replace a purely linear prompt-first reading with a governed loop based on:

- [`Set`](set.md) matter;
- runtime [`View`](view.md) composition;
- governed natural-language projection;
- GoT-style thought evolution;
- recomposition of the next working subgraph.

## Reference Anchors

- graph theory reference: [`../../../.agents/skills/graphclaw/main_graphes/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/markdown.md)
- paths and shortest paths for bounded exploration and path-sensitive refinement: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md)
- connectivity and strongly connected components for branch separation and working-zone coherence: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-38/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-38/markdown.md)
- cuts, articulation, and Menger for preserving critical structure during narrowing: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-44/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-44/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-46/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-46/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-49/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-49/markdown.md)
- local GoT reference: [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)
- GoT reasoning graph model: section 3.1 in [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)
- GoT transformations: section 3.2 in [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)
- GoT scoring and ranking: section 3.3 in [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)

## Loop

The target mono-agent loop can be read as:

1. derive a minimum task intent;
2. activate one or more starting [`Set`](set.md) objects;
3. compose an initial [`View`](view.md);
4. choose an exploration-side projection;
5. derive a `ThinkingContext`;
6. generate, critique, refine, or aggregate thoughts in a thought graph;
7. use those thoughts to recompute the next `View`;
8. iterate until one working branch is sufficient;
9. choose a final projection;
10. derive a `ContextPack`;
11. record the selection path in a `ResolutionTrace`.

## Reading Rule

This loop should be read with:

- [`got.md`](got.md) for thought-graph orchestration;
- [`projection-governance.md`](projection-governance.md) for projectability rules;
- [`context-artifacts.md`](context-artifacts.md) for artifact distinctions;
- [`../runtime/turn-runtime-logic.md`](../runtime/turn-runtime-logic.md) for broader turn sequencing.

This loop should be read as:

- graph-theory grounded in bounded subgraph work, paths, components, and cuts;
- GoT-grounded in graph-shaped thought evolution rather than a single reasoning chain;
- GraphClaw-grounded in recomputing the next [`View`](view.md) rather than merely appending more prompt text.
