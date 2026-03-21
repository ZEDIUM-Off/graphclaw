# View

## Status

This file is the canonical definition source for `View`.

It is target-architecture documentation.

## Reference Anchors

- graph theory reference: [`../../../.agents/skills/graphclaw/main_graphes/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/markdown.md)
- directed graph, neighbors, predecessors, successors: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-6/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-6/markdown.md)
- paths and shortest paths: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md)
- connectivity and strongly connected components: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-38/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-38/markdown.md)
- `Graph of Thoughts` reference: [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)

## Definition

Une `View` est un [`Set`](set.md) runtime, transitoire, de premiere classe, utilise comme sous-graphe de travail pour l'exploration, la navigation, le filtrage et la composition temporaire a partir d'un ou plusieurs [`Set`](set.md).

## Est

Une `View` est :

- un `Set` runtime actif ;
- un sous-graphe complet avec noeuds et relations ;
- un support de navigation et de selection ;
- porteuse de provenance, de regles de construction, et d'implications de budget ;
- potentiellement lazy ou materialisee ;
- derivee, et non gouvernee comme un objet persiste.

## N'est pas

Une `View` n'est pas :

- un [`Set`](set.md) persiste ;
- un sous-graphe packable final ;
- un `ContextPack` ;
- une simple liste de fragments de prompt ;
- une identite du graphe tout entier.

## Invariants

- une `View` est definie comme sous-graphe de travail complet, pas par sa seule projetabilite NL ;
- une `View` peut contenir des noeuds et relations semantiques, runtime, ou de trace, si ces elements restent dans la portee active autorisee ;
- une `View` peut etre recomposee a chaque etape de reflexion ;
- une `View` n'est pas egale a sa projection en langage naturel.

## What A View Does

During a turn, an agent may:

- start from its bounding Set;
- reference several visible sub-Sets;
- perform temporary composition operations;
- produce multiple intermediate Views;
- persist none of these Views in the database.

## What A View Contains

Depending on the stage, a `View` may contain or reference:

- nodes;
- relations;
- provenance;
- construction rules;
- ranking or filtering metadata;
- references to attached documents or fragments;
- selection notes needed for later packing or audit;
- budget implications.

## `View` And Projectability

A `View` is not defined by text projectability alone.

Whether some part of a `View` can be rendered into natural language is a separate question from whether it belongs to the working subgraph.

For the current cross-concept framing around `ProjectionRegistry` and `NLProjection`, see [`projection-governance.md`](projection-governance.md).

The distinction to preserve is:

- `View` = complete working subgraph;
- NL projection = governed rendering of some authorized part of that subgraph.

## Maximum Agent View

For a given agent, there can be a maximum active `View` representing the broadest runtime governed space that agent may navigate at that moment.

This is derived from the agent's bounding Set(s) and their resolution.

## Intermediate Views

Narrower or composed views inside the maximum perimeter:

- focus on one concept neighborhood;
- focus on skills attached to a concept;
- focus on documents or notes associated with selected nodes;
- focus on a composed working view assembled during reflection.

These are derived working subgraphs, not separate persisted worlds.

## Lazy Versus Materialized

### Lazy `View`

Defined by a seed, rule, expression, or filter pipeline without immediate full evaluation.

Useful for navigation, deferred expansion, policy-aware exploration, and cheap recomputation when focus changes.

### Materialized `View`

Evaluated and fixed as an explicit result for a given moment or audit horizon.

Useful for traceability, repeatable packing, budget inspection, and backend caching.

GraphClaw may need both forms.
