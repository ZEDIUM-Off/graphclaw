# Graph Of Thoughts

## Status

This document frames mono-agent `Graph of Thoughts` use in GraphClaw.

It is target-architecture documentation. It does not claim that the inherited runtime already executes this loop.

## Reference Anchor

- local GoT reference: [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)

The most relevant sections are:

- section 1, especially the paragraphs on arbitrary graph structure and graph-enabled transformations;
- section 3.1 for the directed graph model of reasoning;
- section 3.2 for `Aggregate`, `Refine`, and `Generate`;
- section 3.3 for scoring and ranking;
- section 4.5 for `Graph of Operations` versus `Graph Reasoning State`.

The graph-theory pages that best support this reading are:

- graph as a directed relation on a vertex set: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-5/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-5/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-6/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-6/markdown.md)
- topological ordering and acyclic dependency reading: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-46/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-46/markdown.md)
- ranking intuition: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-87/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-87/markdown.md)

## GraphClaw Reading

In GraphClaw, GoT should be read as a graph of runtime thoughts used to guide the next refinement of the active context state.

The thought graph stays distinct from:

- the persisted semantic graph;
- the active [`View`](view.md) ;
- the `ThinkingContext` projection;
- the final `ContextPack`.

## Useful GoT Operations

- `Generate` opens one or more candidate paths from a current reflection state;
- `Refine` loops on an existing thought;
- `Aggregate` merges several thoughts or branches;
- `Score` evaluates a thought or branch;
- `KeepBest` preserves prioritized thoughts;
- `Repeat` explores variants.

These operations map most directly to:

- graph-enabled transformations in section 3.2 of [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)
- scoring and ranking in section 3.3 of [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)

## Structural Principle

GoT should guide runtime reflection without redefining GraphClaw concepts.

The stable reading is:

- [`View`](view.md) provides the working subgraph;
- a projection derives `ThinkingContext`;
- GoT orchestrates thought generation, critique, aggregation, and ranking;
- the outputs of that reasoning help recompute the next `View`.
