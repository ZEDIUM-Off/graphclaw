# MutationGuard Interface

## Status

This document defines the target role of `MutationGuard` as a migration-facing GraphClaw interface.

It does not claim that the inherited runtime already exposes a dedicated mutation guard in code. It defines the minimum boundary needed if context edits to the active [`View`](../concepts/view.md) are to become validated, traceable, and policy-aware rather than being hidden inside prompt-local or retrieval-local behavior.

## Why This Interface Matters

GraphClaw now distinguishes:

- runtime graph work inside the active [`View`](../concepts/view.md);
- invocation-oriented projection inside [`ContextFrame`](../concepts/context-frame.md);
- final model-visible output inside `ContextPack`.

That distinction breaks down if any caller can freely mutate visible context without passing through an explicit validation boundary.

Without `MutationGuard`, it becomes difficult to answer:

- which proposed edit was accepted or rejected;
- whether a requested expansion violated the active `View`;
- whether a summary replacement was chosen because of budget;
- whether a mutation changed visible state or only a candidate plan;
- how to keep mutation outcomes visible in `ResolutionTrace`.

## Interface Role

`MutationGuard` validates, rejects, or degrades proposed changes to visible context before they modify the authoritative working [`View`](../concepts/view.md).

It should sit between:

- upstream `ContextEditPlan` or `ContextMutationProposal` requests;
- downstream window updates, pack recompilation, and trace emission.

It is not:

- the owner of `View` semantics;
- the whole policy engine;
- the pack compiler;
- a freeform prompt-edit helper.

## Inputs And Outputs

### Typical Inputs

- `ContextEditPlan` operations or `ContextMutationProposal` records;
- current [`View`](../concepts/view.md);
- active `View` constraints;
- runtime budget and policy limits;
- reference-integrity facts such as required relations, summaries, or evidence links;
- current strategy and packing implications where relevant.

### Typical Outputs

- accepted mutations ready to apply;
- rejected mutations with explicit reasons;
- degraded mutations that are allowed only in narrower or summarized form;
- mutation effects that require `ContextPack` recompilation;
- traceable mutation outcomes suitable for `ResolutionTrace`.

## Minimum Responsibilities

The interface should eventually support at least:

- validating whether a proposed mutation is legal under the active `View`;
- validating budget impact before visible state changes;
- validating integrity of summarized or replacement-ready forms;
- distinguishing reject, accept, and degrade outcomes explicitly;
- keeping mutation policy separate from storage and retrieval code.

## Minimum Invariants

1. `View` changes that affect later projections should pass through a validation boundary before becoming authoritative.
2. Mutation validation must consider at least view, budget, and policy constraints.
3. Rejection and degradation outcomes must be explicit enough to trace later.
4. Mutation validation is distinct from final packing, even when accepted mutations force repacking.
5. `MutationGuard` should consume policy inputs more often than define all policy meaning locally.

## Mutation Outcome Families

The stable outcome families should be:

- `accept`: the requested edit is valid as proposed;
- `reject`: the requested edit is not allowed under current constraints;
- `degrade`: the requested edit is allowed only in reduced or transformed form;
- `preserve`: keep prior visible state unchanged because the proposed edit cannot safely apply.

This document does not freeze an enum or storage format for those outcomes.

## Relationship To `View`

The stable relationship should be:

1. plans or proposals request changes to the working [`View`](../concepts/view.md);
2. `MutationGuard` validates those requests;
3. only accepted or degraded results modify the `View`;
4. final packing sees the updated working state rather than unvalidated requested state.

## Relationship To `ContextPack`

`MutationGuard` does not produce the final packed artifact by itself.

Its role is to decide whether the visible working state may change. After that:

- the [`View`](../concepts/view.md) is updated if appropriate;
- `ContextPack` may need to be recompiled from the new authoritative state;
- `ResolutionTrace` records what was accepted, rejected, or degraded.

## Errors, Rejection, And Degradation

Typical reasons to reject or degrade may include:

- requested expansion exceeds budget;
- requested node or relation is outside the active `View`;
- full form is not packable, but summary form is allowed;
- mutation breaks reference integrity needed for intelligible packing;
- the requested edit requires a capability or right not currently available.

## Compatibility With The Inherited Runtime

The first migration step should preserve the current turn path while making some decisions explicit.

The compatibility rule is:

> existing compaction, replacement, or visibility choices may continue to happen conservatively at first, but the system should progressively route them through an explicit mutation-validation boundary instead of leaving them implicit inside prompt or retrieval logic.

That means the first implementation can stay conservative:

- validate only a small set of mutation kinds first, such as add, remove, pin, and replace-with-summary;
- keep current prompt behavior stable after accepted changes;
- emit coarse accept/reject/degrade outcomes before deeper mutation semantics exist.

## Likely Source-Area Consumers

### `src/runtime/` and `src/security/`

Likely role:

- primary support area for budget, execution, and policy constraints around mutation validation.

Likely seam files:

- `src/runtime/traits.rs`
- `src/security/policy.rs`

### `src/agent/`

Likely role:

- submit proposed context edits and consume accepted visible-state changes.

Likely seam files:

- `src/agent/loop_.rs`
- `src/agent/prompt.rs`

## Initial Files To Treat As Seams

- `src/runtime/traits.rs`
- `src/security/policy.rs`
- `src/agent/loop_.rs`
- `src/agent/prompt.rs`
- `src/gateway/sse.rs`

## Recommended Minimal Introduction Order

1. Route a small set of visible-context edits through an explicit validation step.
2. Record accept, reject, or degrade outcomes in coarse trace form.
3. Recompute `ContextPack` from accepted visible state rather than requested edits.
4. Expand later into richer integrity checks, view-aware degradation, and finer-grained mutation kinds.

## Slice JSON

This slice is an orientation artifact, not an implementation claim.

```json
{
  "nodes": [
    {
      "id": "n0",
      "position": { "x": -650, "y": -50 },
      "caption": "ContextEditPlan",
      "labels": ["ContextEditPlan"],
      "properties": {
        "file_origin": "future planning seam",
        "role": "Requests explicit visible-context edits"
      },
      "style": {}
    },
    {
      "id": "n1",
      "position": { "x": -300, "y": -180 },
      "caption": "ViewDefinition",
      "labels": ["ViewDefinition"],
      "properties": {
        "file_origin": "docs/architecture/ + future context seam",
        "role": "Constrains which edits are legal"
      },
      "style": {}
    },
    {
      "id": "n2",
      "position": { "x": -300, "y": 80 },
      "caption": "Runtime Budget And Policy",
      "labels": ["PolicyDefinition"],
      "properties": {
        "file_origin": "src/runtime/ + src/security/",
        "role": "Constrains budget, rights, and mutation safety"
      },
      "style": {}
    },
    {
      "id": "n3",
      "position": { "x": 50, "y": -50 },
      "caption": "MutationGuard",
      "labels": ["Action"],
      "properties": {
        "file_origin": "future mutation-validation seam",
        "role": "Accepts, rejects, or degrades proposed context edits"
      },
      "style": {}
    },
    {
      "id": "n4",
      "position": { "x": 430, "y": -50 },
      "caption": "View",
      "labels": ["View"],
      "properties": {
        "file_origin": "future explicit runtime state",
        "role": "Working graph state updated only after validated edits"
      },
      "style": {}
    },
    {
      "id": "n5",
      "position": { "x": 790, "y": -50 },
      "caption": "ResolutionTrace",
      "labels": ["ResolutionTrace"],
      "properties": {
        "file_origin": "future trace layer",
        "role": "Records mutation acceptance, rejection, and degradation outcomes"
      },
      "style": {}
    }
  ],
  "relationships": [
    {
      "id": "r0",
      "type": "RELATED_TO",
      "style": {},
      "properties": {},
      "fromId": "n3",
      "toId": "n0"
    },
    {
      "id": "r1",
      "type": "RELATED_TO",
      "style": {},
      "properties": {},
      "fromId": "n3",
      "toId": "n1"
    },
    {
      "id": "r2",
      "type": "RELATED_TO",
      "style": {},
      "properties": {},
      "fromId": "n3",
      "toId": "n2"
    },
    {
      "id": "r3",
      "type": "MUTATION_TARGETS",
      "style": {},
      "properties": {},
      "fromId": "n0",
      "toId": "n4"
    },
    {
      "id": "r4",
      "type": "TRACE_OBSERVED_MUTATION",
      "style": {},
      "properties": {},
      "fromId": "n5",
      "toId": "n0"
    }
  ]
}
```

## Related References

- [`../concepts/view.md`](../concepts/view.md)
- [`context-pack-interface.md`](context-pack-interface.md)
- [`../migration/future-integration-seams.md`](../migration/future-integration-seams.md)
- [`../concepts/context-artifacts.md`](../concepts/context-artifacts.md)
