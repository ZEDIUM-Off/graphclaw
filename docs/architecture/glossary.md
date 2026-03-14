# GraphClaw Glossary

Use these definitions consistently across GraphClaw documentation.

These terms describe the target concept model. They do not imply that the current inherited runtime already implements every one of them.

For architecture work, this glossary and `docs/architecture/graph-context-engine.md` are the canonical sources of concept meaning. Other docs may summarize these terms, but they should not redefine them.

## Core Context Terms

### `View`

A governed projection over the graph that can expose one or more policy-constrained sets of nodes.

### `GraphSet`

A first-class logical set of node references that can be produced, filtered, combined, ranked, budgeted, summarized, or packed.

`GraphSet` may be lazy or materialized, and it can include or reference relations, provenance, construction metadata, and attached readable content.

### `Packable Subgraph`

A bounded projection derived from one or more `GraphSet` objects and prepared for possible inclusion in the final `ContextPack`.

### `Packability`

The conditional property that some graph material may be suitable for packing in a given turn, view, policy state, and budget. Packability is not an absolute property of a node or relation.

### `Bounded Complement`

The subset of nodes inside an explicitly bounded universe that are not in a given `GraphSet`. This is never an unbounded "everything else" operation over the whole graph.

### `Condensation`

A reduction operation that replaces a larger or denser `GraphSet` with a smaller representative structure while preserving the distinctions needed for the current decision, navigation, or packing step.

### `Projection Into A Packable Subgraph`

The operation that turns a logical `GraphSet` into a bounded subgraph candidate suitable for packing, including the selected nodes, the relations needed to keep them intelligible, and any required provenance or summary links.

### `SessionWindow`

The currently visible and mobilizable subgraph for a turn or a short sequence of turns. It is not the whole graph and not the complete conversation history.

### `ThinkingContext`

The temporary reflection context used to explore, compare, and arbitrate before final response packing.

This is a system phase, not merely an ordinary tool call.

### `ContextPack`

The final budgeted context artifact retained for response generation after selection, degradation, and traceable packing decisions.

### `ContextMutationProposal`

A structured proposal to change the visible or packable context, for example by adding, removing, pinning, compacting, or expanding material.

### `ResolutionTrace`

The explicit record of context-resolution decisions such as selection, rejection, degradation, summarization, and final packing.

## Agent Packaging Terms

### `AgentPackage`

A versioned portable unit of agent behavior, declared dependencies, and packaging metadata.

### `AgentInstance`

A concrete local installation or activation of an `AgentPackage`, with bindings and policy choices applied for a particular environment.

### `AgentSession`

The live runtime state attached to a conversation or execution horizon. A session is transient and operational, not a portable package.

### `Bindings`

Local attachments to providers, tools, stores, or environment-specific capabilities needed to run an instance. Bindings are not assumed to be portable.

## Packaging Scope Rule

The Graph Context Engine and the agent packaging model are adjacent but distinct layers:

- the Graph Context Engine governs views, sets, windows, reflection, mutations, and final packing for a turn;
- the packaging model governs how portable agent behavior, dependencies, and bindings are declared, installed, and instantiated.

Packaging may depend on graph-native concepts, but it should not be confused with the context engine itself.

## Framing Rules

- memory is not the same thing as context
- the graph backend is not the same thing as the Graph Context Engine
- a `View` is not an arbitrary query alias
- a `GraphSet` is not just a bag of prompt fragments
- a `GraphSet` is not the same thing as a packable subgraph
- a `ContextPack` is distinct from the `ThinkingContext` used to decide it
