# Views And Sets

## Status

This document refines the operational meaning of `View`, `GraphSet`, packability, and related set semantics for GraphClaw.

It is still target-architecture documentation. It does not claim that the inherited runtime already exposes these objects explicitly in code.

## Why This Document Exists

`View` and `GraphSet` are too central to remain only brief glossary entries. They need operational detail so the repository can reason clearly about navigation, selection, packability, and future integration seams.

## `View`

A `View` is a governed projection over the graph that bounds what can be seen, traversed, compared, and potentially packed for a given actor and situation.

The key rule is:

> a `View` defines the governed universe inside which `GraphSet` operations make sense.

### Maximum Agent View

For a given agent, there can be a maximum active `View` representing the broadest governed space that agent may navigate at that moment.

That maximum view can bound:

- visible node families;
- visible relation families;
- policy rules and redactions;
- accessible concepts and attached content;
- operations allowed during navigation or packing.

This maximum view is not automatically the set that will be packed. It is the outer governed perimeter.

### Intermediate Or Composed Views

GraphClaw should also expect narrower or composed views inside that perimeter.

Examples include:

- focus on one concept neighborhood;
- focus on skills attached to a concept;
- focus on documents or notes associated with selected nodes;
- a composed working view assembled during `ThinkingContext`.

These are not separate worlds. They are derived projections inside the currently allowed maximum view.

### View Responsibilities

When a view is documented, the docs should be able to answer:

- what becomes visible;
- what remains hidden;
- which policies and rights constrain it;
- which degradations are allowed;
- whether it is persistent, session-scoped, or temporary;
- which `GraphSet` families are meaningful inside it.

## `GraphSet`

A `GraphSet` is a first-class logical set manipulated inside a `View`.

It is not just a list of nodes and not just an export bundle.

Depending on the stage, a `GraphSet` may contain or reference:

- nodes;
- relations;
- provenance;
- construction rules;
- ranking or filtering metadata;
- references to attached documents or fragments;
- selection notes needed for later packing or audit.

Its primary role is contextual navigation and selection. Export or serialization is secondary.

### Interpreting A `GraphSet`

A documented `GraphSet` should usually state:

- the view it is relative to;
- its seeds or anchors;
- the construction rule;
- allowed relations and filters;
- whether it is lazy or materialized;
- whether it is navigational, analytical, packable, or mixed;
- what budget concerns apply.

### Lazy Versus Materialized

GraphClaw should document the difference between a lazy and a materialized `GraphSet`.

#### Lazy `GraphSet`

A lazy `GraphSet` is defined by a seed, rule, expression, view, or filter pipeline without requiring immediate full evaluation or persistence.

This is useful for:

- navigation;
- deferred expansion;
- policy-aware exploration;
- cheap recomputation when a user focus changes.

#### Materialized `GraphSet`

A materialized `GraphSet` has been evaluated, fixed, or persisted as an explicit result for a given moment or audit horizon.

This is useful for:

- traceability;
- repeatable packing;
- budget inspection;
- backend materialization or caching.

GraphClaw may need both forms. The docs should avoid treating definition and snapshot as the same thing.

## Referenced Content

Nodes in a `GraphSet` may refer to content that is not reducible to the node identifier itself.

Examples include:

- markdown documents;
- notes;
- summaries;
- attachments;
- derived fragments or excerpts.

The documentation should therefore allow for a `GraphSet` to reference not only graph structure, but also the conditions under which attached content becomes readable, summarizable, or packable.

## Packable Subgraph

A packable subgraph is not the same thing as a `GraphSet`.

The useful distinction is:

> a `GraphSet` is a governed logical working set; a packable subgraph is a derived bounded projection prepared for possible inclusion in the final `ContextPack`.

This means:

- a `GraphSet` can be broader and more exploratory;
- a packable subgraph is closer to the final model-visible artifact;
- not every `GraphSet` becomes packable;
- some nodes or relations may remain useful for navigation but never appear in the final packed context.

## Packability

GraphClaw should not document packability as an intrinsic property of a node or relation.

Packability is conditional. It depends on at least:

- the active `View`;
- the agent's rights and policy constraints;
- turn-specific needs;
- budget;
- whether summary forms are allowed;
- whether attached content is readable in full, in excerpted form, or only through derived summary.

The same node can therefore be:

- navigable but not packable;
- packable in one view but not another;
- packable only through summary;
- packable only when accompanying relation context is preserved.

## Relationship Between `View`, `GraphSet`, And Packability

The sequence to keep stable in docs is:

1. the `View` defines the governed perimeter;
2. one or more `GraphSet` objects are manipulated inside that perimeter;
3. a packable subgraph is derived from some of those sets;
4. the final `ContextPack` retains only the budgeted result of that derivation.

## Set Operations

The following operations should remain conceptually stable even if the backend changes:

- union;
- intersection;
- difference;
- bounded complement;
- expansion;
- filtering by relation, type, and property;
- ranking;
- condensation;
- projection into a packable subgraph.

Those are GraphClaw operations first. Backend procedures may support them, but should not define their meaning.

## Open Questions

The docs should keep these questions visible rather than hiding them behind vague language:

- which relation families are needed only for navigation and never for final packing;
- when a materialized `GraphSet` should be persisted versus recomputed;
- how attached content should be represented when a node is packable only through excerpts or summaries;
- how rights and policy checks interact with composed views and derived sets.
