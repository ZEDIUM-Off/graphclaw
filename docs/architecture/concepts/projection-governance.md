# Projection Governance

## Status

This document is cross-concept architecture framing.

It introduces `ProjectionRegistry` and `NLProjection` as target concepts, but it does not make them canonical concept sources yet.

## Purpose

GraphClaw needs an explicit place to describe:

- how nodes and relations become projectable into natural language;
- how projectability differs from simple membership in a [`View`](view.md);
- how projection remains governed by active scope and agent policy.

## Reference Anchors

- graph theory reference: [`../../../.agents/skills/graphclaw/main_graphes/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/markdown.md)
- graph as relation plus typed structure: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-5/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-5/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-6/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-6/markdown.md)
- paths and working-subgraph traversal context: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md)
- local GoT reference for graph-shaped runtime state and controlled transformations: [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)

## `ProjectionRegistry`

`ProjectionRegistry` is the inventory of natural-language projection templates and rules available by node type and relation type.

It should allow the architecture to declare:

- which node types have a projection rule;
- which more specific selections exist, such as `:Agent` or `:Agent:OtherLabel`;
- which relation types have a projection rule;
- which projection variants exist by runtime context;
- which projections remain allowed or forbidden inside an agent scope.

This is the architecture layer that lets GraphClaw say, concretely, "all nodes of type `:Agent` may use this NL template" or "only some relation types are verbalizable in the current scope."

## `NLProjection`

`NLProjection` is the definition of a projection from a [`View`](view.md) or a selected part of a `View` into a natural-language context.

At minimum it should fix:

- which node types to include;
- which relation types to include, flatten, or ignore;
- how each included type is rendered;
- what level of detail is preserved;
- what budget target applies;
- what runtime role the projection serves.

In practice, this is the layer that distinguishes:

- an exploration-side projection feeding `ThinkingContext`;
- a response-side projection feeding `ContextPack`.

## Stable Reading

The distinction to preserve is:

- belonging to a [`View`](view.md) is not enough;
- an element also needs a projection rule;
- and it must remain allowed inside the active scope.

This applies to:

- semantic nodes;
- runtime nodes;
- trace nodes;
- semantic relations;
- runtime relations;
- trace relations.
