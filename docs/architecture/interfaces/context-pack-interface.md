# ContextPack Interface

## Status

This document defines the target role of `ContextPack` as a migration-facing GraphClaw interface.

It does not claim that the inherited runtime already exposes a concrete `ContextPack` type in code. It defines the minimum boundary needed so final model-visible context can become explicit, budgeted, inspectable, and separable from broader exploration state.

## Why This Interface Matters

The inherited runtime can already send prompt-visible context to providers, but that context is still often assembled through a mixture of prompt sections, memory hydration, and transient runtime conventions.

That leaves several questions under-specified:

- what exact context was finally exposed to the model;
- how budget and summarization shaped the result;
- which visible material stayed exploratory and never became model-visible;
- how the final packed result differs from the broader `SessionWindow`;
- what trace should exist for the final packing decision.

`ContextPack` is the smallest interface that makes "what the model actually receives" a governed artifact rather than an implementation side effect.

## Reference Anchors

- graph theory reference: [`../../../.agents/skills/graphclaw/main_graphes/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/markdown.md)
- paths and shortest paths for keeping only intelligible minimal relation structure: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md)
- connectivity, cuts, articulation, and Menger for preserving critical structure while narrowing the final pack: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-44/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-44/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-46/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-46/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-49/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-49/markdown.md)
- ranking intuition through PageRank: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-87/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-87/markdown.md)
- local GoT reference: [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)
- GoT scoring and ranking for branch selection before final packing: section 3.3 in [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)

## Interface Role

`ContextPack` is the final budgeted, model-visible context artifact compiled from narrower runtime state such as `SessionWindow`, packable-subgraph candidates, summaries, and policy-constrained selections.

It should sit between:

- upstream context selection, mutation validation, and packing decisions;
- downstream provider invocation, response generation, and trace or inspection surfaces.

It is not:

- the whole `ThinkingContext`;
- the whole `SessionWindow`;
- the entire conversation history;
- a raw retrieval result;
- an unrestricted dump of all available graph material.

## Inputs And Outputs

### Typical Inputs

- the currently accepted `SessionWindow`;
- packable-subgraph candidates or equivalent bounded projections;
- active packing strategy or packing policy inputs;
- summary, compaction, or replacement results;
- rights, budget, and redaction constraints;
- evidence or provenance links needed to keep packed material intelligible.

### Typical Outputs

- final model-visible context sections or equivalent packed representation;
- ordering, grouping, or summary choices suitable for provider consumption;
- budget-relevant metadata about what survived packing;
- traceable packing decisions suitable for `ResolutionTrace`;
- optional inspection-friendly structure for gateway or UI surfaces.

## Minimum Responsibilities

The interface should eventually support at least:

- representing the final bounded context actually passed to model execution;
- preserving the distinction between visible runtime state and model-visible packed output;
- reflecting summary and degradation choices explicitly rather than only through prompt text;
- exposing enough structure that provider calls are downstream consumers rather than hidden owners of context shape;
- remaining stable enough to inspect, diff, or trace even if formatting strategies change later.

## Minimum Invariants

1. `ContextPack` is the final model-visible context artifact for the turn.
2. `ContextPack` is derived from narrower governed runtime state rather than directly from unbounded retrieval.
3. `ContextPack` is budgeted and policy-constrained.
4. `ContextPack` is distinct from `SessionWindow`, `ThinkingContext`, and raw retrieval candidates.
5. `ContextPack` may contain summaries, excerpts, or replacements instead of only full-fidelity source material.
6. `ContextPack` creation should be traceable through explicit packing or degradation decisions.

## Relationship To `SessionWindow`

The stable relationship should be:

1. `SessionWindow` holds current visible and mobilizable context state;
2. packing logic chooses what subset or representation becomes model-visible;
3. `ContextPack` is the compiled result of that choice.

This distinction matters because GraphClaw wants runtime-visible governable context without assuming every visible item must be packed in full.

In the current concept split, this should be read together with:

- [`../concepts/view.md`](../concepts/view.md) for the working subgraph itself;
- [`../concepts/packability.md`](../concepts/packability.md) for packable-subgraph conditions;
- [`../concepts/projection-governance.md`](../concepts/projection-governance.md) for the NL projection step that produces the final artifact.

## Relationship To Providers

Providers should consume a `ContextPack` or a representation derived from it.

Providers should not define:

- what counts as packable;
- how summaries are chosen;
- which visible material is excluded;
- how budget or policy governs the final packed result.

Provider-specific formatting or capability constraints may shape the final representation, but the conceptual meaning of `ContextPack` belongs above provider adapters.

## Errors, Rejection, And Degradation

The interface should make room for outcomes such as:

- exclude heavy material because final budget is too small;
- replace full content with summary or excerpt form;
- redact portions because rights or policy forbid full exposure;
- preserve provenance while degrading detail;
- refuse a requested packed expansion that violates the active view or current policy.

These are packing outcomes, not merely prompt-formatting side effects.

This is where graph-theory ideas such as cuts, articulation, and preserving enough path structure matter most: the final pack should shrink aggressively without making the surviving context unintelligible.

## Compatibility With The Inherited Runtime

The first migration step should preserve the current provider-facing prompt path.

The compatibility rule is:

> the inherited prompt assembly may continue to shape provider payloads initially, but it should progressively consume a `ContextPack` boundary instead of being the only place where final context exists.

That means the first implementation can stay conservative:

- start with a small struct or equivalent wrapper around already assembled final context;
- preserve current prompt formatting while naming and tracing the packed result explicitly;
- keep provider adapters unchanged except for consuming a pack-derived representation;
- refine later toward richer ordering, provenance, and summary metadata.

## Likely Source-Area Consumers

### `src/agent/`

Likely role:

- primary consumer for final prompt-visible or provider-visible turn input.

Likely seam files:

- `src/agent/prompt.rs`
- `src/agent/loop_.rs`
- `src/agent/dispatcher.rs`

### `src/providers/`

Likely role:

- consume the packed result through a provider-facing invocation boundary.

Documentary caution:

- provider adapters should consume the pack, not define its semantics.

### `src/gateway/` and `web/`

Likely role:

- later expose packed output, summaries, or trace-linked inspection surfaces.

## Initial Files To Treat As Seams

- `src/agent/prompt.rs`
- `src/agent/dispatcher.rs`
- `src/agent/loop_.rs`
- `src/providers/traits.rs`
- `src/providers/router.rs`
- `src/gateway/api.rs`
- `src/gateway/sse.rs`

## Recommended Minimal Introduction Order

1. Wrap current final context assembly in an explicit pack boundary.
2. Keep prompt formatting behavior stable while making the packed artifact inspectable.
3. Emit coarse pack-level trace information.
4. Introduce richer pack metadata such as summaries, exclusions, and provenance later.
5. Expose read-only packed output through transport or UI surfaces only after the boundary is stable.

## Slice JSON

This slice is an orientation artifact, not an implementation claim.

```json
{
  "nodes": [
    {
      "id": "n0",
      "position": { "x": -650, "y": -50 },
      "caption": "SessionWindow",
      "labels": ["SessionWindow"],
      "properties": {
        "file_origin": "future explicit runtime state",
        "role": "Visible context state before final packing"
      },
      "style": {}
    },
    {
      "id": "n1",
      "position": { "x": -300, "y": -180 },
      "caption": "Packing Strategy",
      "labels": ["PackingStrategyDefinition"],
      "properties": {
        "file_origin": "future strategy seam",
        "role": "Shapes what representation survives into the final pack"
      },
      "style": {}
    },
    {
      "id": "n2",
      "position": { "x": -300, "y": 80 },
      "caption": "Budget And Policy Constraints",
      "labels": ["PolicyDefinition"],
      "properties": {
        "file_origin": "src/runtime/ + src/security/",
        "role": "Constrain the final model-visible result"
      },
      "style": {}
    },
    {
      "id": "n3",
      "position": { "x": 70, "y": -50 },
      "caption": "ContextPack",
      "labels": ["ContextPack"],
      "properties": {
        "file_origin": "future pack compiler seam",
        "role": "Final packed context artifact for provider consumption"
      },
      "style": {}
    },
    {
      "id": "n4",
      "position": { "x": 430, "y": -50 },
      "caption": "Provider Invocation",
      "labels": ["Resource"],
      "properties": {
        "file_origin": "src/providers/",
        "role": "Consumes the packed context under a uniform provider contract"
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
        "role": "Records packing, degradation, and exclusion decisions"
      },
      "style": {}
    }
  ],
  "relationships": [
    {
      "id": "r0",
      "type": "CONTEXT_PACK_DERIVED_FROM",
      "style": {},
      "properties": {},
      "fromId": "n3",
      "toId": "n0"
    },
    {
      "id": "r1",
      "type": "PLAN_USES_STRATEGY",
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
      "type": "HAS_TOPIC",
      "style": {},
      "properties": {},
      "fromId": "n3",
      "toId": "n4"
    },
    {
      "id": "r4",
      "type": "TRACE_IMPLEMENTED_STRATEGY",
      "style": {},
      "properties": {},
      "fromId": "n5",
      "toId": "n3"
    }
  ]
}
```

## Related References

- [`../concepts/context-artifacts.md`](../concepts/context-artifacts.md) for artifact boundaries
- [`../concepts/view.md`](../concepts/view.md) for the runtime working subgraph
- [`../concepts/packability.md`](../concepts/packability.md) for packable-subgraph conditions
- [`../concepts/projection-governance.md`](../concepts/projection-governance.md) for the projection step into model-visible form
- [`../runtime/turn-runtime-logic.md`](../runtime/turn-runtime-logic.md) for broader turn sequencing
- [`../migration/future-integration-seams.md`](../migration/future-integration-seams.md) for seam placement
- [`session-window-interface.md`](session-window-interface.md) for the upstream visible-state boundary
