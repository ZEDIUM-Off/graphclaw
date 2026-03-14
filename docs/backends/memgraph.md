# Memgraph Backend Reference

## Status

Memgraph is the reference graph backend for the first concrete GraphClaw backend axis.

This document explains how Memgraph can support the GraphClaw model. It does not redefine the GraphClaw model and it does not claim that every capability described here is already wired into the current runtime.

## Why Memgraph Appears Here

Memgraph is relevant because it provides a graph store, traversal capabilities, graph algorithms, and mutation utilities that are useful for a graph-native context engine.

That makes it a strong reference backend for early design and adapter work.

It does not make GraphClaw "a Memgraph project." GraphClaw remains defined by its own concept model: views, sets, session windows, thinking context, context packs, mutations, traces, and portable agent packaging.

For the concept model behind those terms, read:

- [`../architecture/graph-context-engine.md`](../architecture/graph-context-engine.md)
- [`../architecture/views-and-sets.md`](../architecture/views-and-sets.md)
- [`../architecture/context-artifacts.md`](../architecture/context-artifacts.md)
- [`../architecture/future-integration-seams.md`](../architecture/future-integration-seams.md)

## Documentation Rule

Backend reference pages must always move in this order:

1. GraphClaw concept
2. required capability family
3. Memgraph mechanism

Never document Memgraph first and then infer the GraphClaw model backward from an algorithm catalog.

## Layered Mapping

### Layer A: GraphClaw business concepts

Examples:

- `View`
- `GraphSet`
- `SessionWindow`
- `ThinkingContext`
- `ContextPack`
- `ContextMutationProposal`
- `ResolutionTrace`

### Layer B: Required capability families

Examples:

- expansion and neighborhood traversal;
- path search and connectivity checks;
- governed view-scoped filtering;
- ranking under budget;
- clustering, communities, and similarity;
- condensation and representative selection;
- lazy evaluation and optional materialization support;
- collection and property manipulation;
- optional dynamic update support.

### Layer C: Memgraph mechanisms

Examples include traversal procedures, path algorithms, ranking algorithms, community and similarity procedures, and graph mutation utilities exposed by the backend and its extension surface.

The GraphClaw architecture should start from Layer A, describe Layer B in backend-agnostic terms, and only then name Layer C.

## Minimum Backend Contract

For Memgraph to be a viable first reference backend, the documentation should assume these minimum backend responsibilities:

- persist and query a graph-shaped model of nodes, relations, and properties;
- support bounded neighborhood expansion and constrained traversal;
- support filtering by relation, type, and property;
- support view-scoped selection inputs that can preserve policy and packability constraints above the backend layer;
- support ranking inputs that can help order candidate nodes or sets;
- support explicit graph mutation so `ContextMutationProposal`, `ResolutionTrace`, and materialized helper subsets can be persisted through an adapter layer.

If one of these responsibilities cannot be met, Memgraph stops being a full reference backend for the GraphClaw target and becomes only a partial accelerator.

## Capability Tiers

Use this tiering when describing Memgraph support:

- required: graph persistence, constrained traversal, filtering, and graph mutation primitives;
- strongly helpful: ranking support, bounded path search, and materialization helpers;
- optional: community detection, similarity procedures, and dynamic update features that improve quality or efficiency without defining the core contract.

This distinction matters because GraphClaw must keep its concept contract stable even when optional backend features are unavailable.

## Capability Families

### Traversal

GraphClaw use:

- build or expand a `GraphSet`;
- walk outward from anchors inside a `View`;
- explore limited neighborhoods during `ThinkingContext`.

Memgraph role:

- graph traversal and path search provide the low-level substrate for these operations.
- this is a required capability tier for any serious GraphClaw backend.

### Structure

GraphClaw use:

- detect components, bridges, cycles, or structural boundaries;
- reason about which subgraphs should stay together or be split.

Memgraph role:

- structural graph procedures can support these decisions when they are useful for packing or navigation.
- this capability is strongly helpful, but not required for a minimal backend contract.

### Ranking

GraphClaw use:

- prioritize nodes or sets under cost constraints;
- pick representatives, entry points, or expansion candidates.

Memgraph role:

- ranking-oriented procedures can inform ordering, but GraphClaw still owns budget policy and final selection logic.
- ranking support is strongly helpful because it improves candidate ordering under budget.

### Materialization And Projection

GraphClaw use:

- evaluate lazy `GraphSet` definitions into repeatable working results;
- derive a packable subgraph from broader working sets;
- persist selected helper structures or traces when needed for audit or reuse.

Memgraph role:

- query execution and mutation support can help materialize derived subsets or trace-linked helper structures through an adapter layer.
- this capability is strongly helpful because GraphClaw may need both lazy and materialized forms, but the backend should not define when materialization is conceptually required.

### Similarity And Communities

GraphClaw use:

- condense related material;
- choose representatives;
- navigate semantically or structurally adjacent zones of the graph.

Memgraph role:

- similarity and community procedures can help produce candidate clusters, but the meaning of those clusters remains a GraphClaw concern.
- these capabilities are optional accelerators, not part of the minimum backend contract.

### Dynamic Support

GraphClaw use:

- keep selected graph-derived artifacts fresh when local graph state changes;
- reduce re-computation cost for some update patterns.

Memgraph role:

- dynamic or edition-dependent capabilities may help here, but they should be treated as optional enhancements rather than core architectural assumptions.

### Utility Operations

GraphClaw use:

- create, merge, refactor, or annotate graph structures backing package imports, `ContextMutationProposal` records, `ResolutionTrace` records, and materialized subsets.

Memgraph role:

- mutation and utility procedures are useful implementation tools, but they are never the center of the GraphClaw business model.
- graph mutation support is required; higher-level utility procedures are optional convenience.

## Mapping Concepts To Backend Needs

| GraphClaw concept | Capability need | Memgraph role |
| --- | --- | --- |
| `View` | governed projection and constrained expansion | query and traversal substrate |
| `GraphSet` | reusable set construction, filtering, ranking, and composition | query results, set-like operations, graph algorithms |
| `SessionWindow` | bounded visible subgraph | bounded traversal, filtering, and materialization helpers |
| `ThinkingContext` | temporary exploration and comparison | query execution and algorithm support for candidate evaluation |
| packable subgraph | bounded projection close to final context | filtering, traversal, ranking inputs, optional materialization |
| `ContextPack` | final budgeted subset | ranked selection, summarization inputs, optional materialization support |
| `ContextMutationProposal` | structured context edits | graph update primitives and utility procedures |
| `ResolutionTrace` | explicit decision record | persisted nodes, edges, or event records via adapter logic |

## Concrete Backend Assumptions

This reference assumes the Memgraph adapter can rely on:

- graph-shaped persistence as the primary storage model rather than a side index;
- query-time traversal over nodes and edges, not only flat recall lookups;
- enough mutation support to create or update trace, summary, or materialized helper structures through an adapter;
- adapter-owned fallback behavior when an optional procedure or edition-specific feature is unavailable.

This reference does not assume:

- every advanced algorithm family is always present;
- optional or edition-dependent procedures define the GraphClaw contract;
- Memgraph alone decides cost semantics, packing policy, or concept meaning.

## Cost And Budget Notes

Memgraph can help produce and rank candidates, but GraphClaw must still own cost semantics.

The backend reference should therefore distinguish:

- logical relevance from packability;
- backend score from final selection decision;
- lazy set definition from materialized subset state;
- raw expansion cost from summarized or condensed alternatives;
- optional backend accelerators from required system behavior.

## Coupling Risks

The main risks to avoid are:

- documenting GraphClaw as a wrapper around backend procedures;
- leaking backend-specific terminology into stable business concepts;
- assuming edition-specific or optional backend features are always available;
- letting algorithm availability dictate the GraphClaw runtime contract.

## Fallback Guidance

When a Memgraph capability is missing, optional, slow, or too coupled to a specific edition, documentation should state the fallback explicitly:

- degrade to simpler traversal or filtering;
- replace structural precision with conservative subset selection;
- summarize instead of expanding;
- materialize less and compute more lazily;
- keep the GraphClaw concept contract stable even if backend quality drops.

## Relationship To Current Implementation

This document is a backend integration reference for future adapter boundaries and conceptual alignment.

The current inherited runtime may still rely on existing memory and recall surfaces that are not yet modeled as a full GraphClaw graph backend. That gap should be described honestly in source-adjacent docs and migration work.
