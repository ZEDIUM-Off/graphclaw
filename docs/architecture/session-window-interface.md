# SessionWindow Interface

## Status

This document defines the target role of `SessionWindow` as a migration-facing runtime interface for GraphClaw.

It does not claim that the inherited runtime already exposes a concrete `SessionWindow` type in code. It defines the minimum boundary needed so visible context can become governable, inspectable, and packable without a rewrite-first migration.

## Why This Interface Matters

The inherited runtime still assembles turn context through prompt sections, memory hydration, tool results, and transient runtime state.

That is workable as baseline behavior, but it makes several questions hard to answer explicitly:

- what is currently visible to the agent;
- what has been pinned, excluded, or compacted;
- which view constraints are active;
- which mutations were accepted or rejected;
- what exact context was available before final packing.

`SessionWindow` is the smallest interface that turns "current visible context" into a governed runtime object rather than an implicit side effect of prompt assembly.

## Interface Role

`SessionWindow` is the session-scoped source of truth for the context currently visible and mobilizable for an agent turn or a short sequence of turns.

It should sit between:

- upstream context retrieval, exploration, and mutation planning;
- downstream packing, execution, trace emission, and optional inspection APIs.

It is not:

- the whole graph;
- the whole memory store;
- the final `ContextPack`;
- a provider payload;
- a hidden scratchpad for unrestricted reasoning.

## Inputs And Outputs

### Typical Inputs

- a governing `View` or view-derived visibility perimeter;
- candidate `GraphSet` or packable-subgraph results from retrieval or exploration;
- accepted `ContextMutationProposal` or `ContextEditPlan` operations;
- policy and budget constraints from runtime guards;
- turn-local evidence attachments or summaries.

### Typical Outputs

- the current visible node and relation set for packing;
- pinned, excluded, summarized, or degraded context state;
- compacted or replacement-ready material for `ContextPack` compilation;
- inspectable state for debugging, streaming, or UI surfaces;
- state transitions and decisions that can feed `ResolutionTrace`.

## Minimum Responsibilities

The interface should eventually support at least:

- holding the currently visible and mobilizable context projection;
- tracking view-bounded inclusion, exclusion, and pinning state;
- carrying summary replacements or compacted substitutes for heavier material;
- exposing enough structure for deterministic `ContextPack` derivation;
- accepting validated mutations rather than arbitrary direct edits;
- remaining inspectable without redefining context semantics inside transport or provider layers.

## Minimum Invariants

These invariants should remain stable even if the implementation changes:

1. `SessionWindow` is the runtime-visible context source of truth for the turn once context selection has occurred.
2. `SessionWindow` is governed by a `View` boundary and runtime policy constraints.
3. `SessionWindow` may contain summarized or replacement representations instead of only raw full-fidelity graph material.
4. `SessionWindow` is distinct from the final `ContextPack`; packing is a later compilation step.
5. `SessionWindow` changes must be traceable through accepted mutation paths rather than hidden prompt edits.
6. `SessionWindow` must remain narrower than the whole exploration space used during `ThinkingContext`.

## Allowed State Families

The interface should be able to represent at least these state families conceptually:

- included material that is directly visible;
- pinned material that should resist opportunistic pruning;
- excluded material that is known but intentionally hidden from current packing;
- summarized replacements that stand in for heavier structures;
- active view identifiers or equivalent governing scope references;
- budget-relevant metadata needed by later packing or mutation checks.

This document does not freeze a storage schema for that state.

## Mutation Boundary

`SessionWindow` should not be mutated freely by whichever module currently happens to assemble prompt text.

The intended boundary is:

1. retrieval or exploration produces candidates;
2. a plan or proposal requests explicit edits;
3. policy and budget guards validate those edits;
4. accepted edits update the `SessionWindow`;
5. downstream packing derives the final `ContextPack`.

That boundary matters because GraphClaw wants governable context evolution rather than prompt-local ad hoc rewriting.

## Errors, Rejection, And Degradation

The interface should make room for these outcomes:

- reject a mutation because it violates the active `View`;
- reject a mutation because it would exceed runtime budget constraints;
- degrade a mutation by replacing full content with a summary or excerpt;
- refuse an expansion when the required capability or rights are unavailable;
- preserve the prior `SessionWindow` when a requested edit cannot be applied safely.

Those outcomes should be visible to `ResolutionTrace` rather than disappearing inside prompt assembly side effects.

## Compatibility With The Inherited Runtime

The first migration step should not require the inherited agent loop to stop working.

The compatibility rule is:

> the inherited prompt and memory path may continue to assemble context, but that assembly should gradually feed and consume an explicit `SessionWindow` boundary instead of remaining the only implicit representation.

That means the first implementation can be conservative:

- start with a small window object that wraps already selected context;
- keep current prompt composition behavior behind an adapter;
- record only coarse mutation or replacement decisions at first;
- let `prompt.rs` consume a window-derived pack or overlay instead of deciding all visibility itself.

## Likely Source-Area Consumers

### `src/agent/`

Likely role:

- primary runtime consumer during turn coordination and final prompt-visible preparation.

Documentary caution:

- `src/agent/` should consume `SessionWindow` more often than define its meaning.

### `src/memory/`

Likely role:

- produce retrieval candidates or helper summaries that may feed the window.

Documentary caution:

- `src/memory/` should not become the owner of final window governance.

### `src/runtime/` and `src/security/`

Likely role:

- enforce mutation, view, and budget constraints around window changes.

### `src/gateway/` and `web/`

Likely role:

- expose inspection, streaming, or operator views of current window state later in migration.

## Initial Files To Treat As Seams

- `src/agent/loop_.rs`
- `src/agent/prompt.rs`
- `src/agent/memory_loader.rs`
- `src/runtime/traits.rs`
- `src/security/policy.rs`
- `src/gateway/api.rs`
- `src/gateway/sse.rs`

## Recommended Minimal Introduction Order

1. Introduce a narrow runtime object that can hold the currently selected visible context.
2. Make prompt assembly consume a window-derived view instead of only implicit fragments.
3. Route accepted compaction or replacement decisions through explicit window updates.
4. Add coarse trace hooks for accepted, rejected, or degraded mutations.
5. Expose read-only inspection later through gateway or web surfaces.

## Slice JSON

This slice is an orientation artifact, not an implementation claim.

```json
{
  "nodes": [
    {
      "id": "n0",
      "position": { "x": -620, "y": -60 },
      "caption": "SessionWindow",
      "labels": ["SessionWindow"],
      "properties": {
        "file_origin": "future explicit runtime state",
        "role": "Active visible context source of truth"
      },
      "style": {}
    },
    {
      "id": "n1",
      "position": { "x": -280, "y": -180 },
      "caption": "ContextEditPlan",
      "labels": ["ContextEditPlan"],
      "properties": {
        "file_origin": "future context planning seam",
        "role": "Requests explicit window edits"
      },
      "style": {}
    },
    {
      "id": "n2",
      "position": { "x": -280, "y": 60 },
      "caption": "MutationGuard",
      "labels": ["Action"],
      "properties": {
        "file_origin": "src/runtime/ + src/security/",
        "role": "Validates, rejects, or degrades requested edits"
      },
      "style": {}
    },
    {
      "id": "n3",
      "position": { "x": 80, "y": -60 },
      "caption": "ViewDefinition",
      "labels": ["ViewDefinition"],
      "properties": {
        "file_origin": "docs/architecture/ + future context seam",
        "role": "Governs what the window may expose"
      },
      "style": {}
    },
    {
      "id": "n4",
      "position": { "x": 430, "y": -60 },
      "caption": "ContextPack",
      "labels": ["ContextPack"],
      "properties": {
        "file_origin": "future pack compiler",
        "role": "Final packed artifact derived from the window"
      },
      "style": {}
    },
    {
      "id": "n5",
      "position": { "x": 760, "y": -60 },
      "caption": "ResolutionTrace",
      "labels": ["ResolutionTrace"],
      "properties": {
        "file_origin": "future trace layer",
        "role": "Records accepted, rejected, or degraded window changes"
      },
      "style": {}
    }
  ],
  "relationships": [
    {
      "id": "r0",
      "type": "PLAN_REQUESTS_CONTEXT_EDIT",
      "style": {},
      "properties": {},
      "fromId": "n1",
      "toId": "n0"
    },
    {
      "id": "r1",
      "type": "RELATED_TO",
      "style": {},
      "properties": {},
      "fromId": "n2",
      "toId": "n1"
    },
    {
      "id": "r2",
      "type": "RELATED_TO",
      "style": {},
      "properties": {},
      "fromId": "n2",
      "toId": "n3"
    },
    {
      "id": "r3",
      "type": "CONTEXT_PACK_DERIVED_FROM",
      "style": {},
      "properties": {},
      "fromId": "n4",
      "toId": "n0"
    },
    {
      "id": "r4",
      "type": "TRACE_OBSERVED_MUTATION",
      "style": {},
      "properties": {},
      "fromId": "n5",
      "toId": "n1"
    }
  ]
}
```

## Related References

- `context-artifacts.md`
- `turn-runtime-logic.md`
- `future-integration-seams.md`
- `views-and-sets.md`
