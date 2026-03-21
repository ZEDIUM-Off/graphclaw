# StrategyResolver Interface

## Status

This document defines the target role of `StrategyResolver` as a migration-facing GraphClaw interface.

It does not claim that the inherited runtime already contains a dedicated strategy resolver in code. It describes the minimum boundary needed if GraphClaw is to govern not only context selection, but also the choice of reflection, exploration, packing, and orchestration strategies.

## Why This Interface Matters

The inherited runtime can already complete turns, call providers, use tools, and load memory. What it does not yet expose cleanly is the choice of *how* the turn should be conducted.

Without an explicit strategy-resolution boundary:

- reasoning style remains an implicit blend of prompt wording, habits, and local code paths;
- exploration style remains buried inside retrieval or dispatch behavior;
- packing style remains fused with prompt assembly;
- orchestration style remains close to the main-agent default path;
- fallbacks and degradations become difficult to audit.

`StrategyResolver` is the smallest interface that makes those choices explicit without freezing a final class layout too early.

## Reference Anchors

- graph theory reference: [`../../../.agents/skills/graphclaw/main_graphes/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/markdown.md)
- shortest paths and bounded graph exploration: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md)
- connectivity, components, and branch coherence: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-38/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-38/markdown.md)
- topological ordering and DAG-style dependency reading: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-46/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-46/markdown.md)
- ranking intuition through PageRank: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-87/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-87/markdown.md)
- local GoT reference: [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)
- GoT reasoning model: section 3.1 in [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)
- GoT transformations and scoring: section 3.2 and section 3.3 in [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)
- GoT execution decomposition: section 4.5 in [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)

## Interface Role

`StrategyResolver` resolves a coherent turn-time strategy set for a task under current runtime constraints.

Its role is to choose, at minimum conceptually:

- a reflection strategy;
- an exploration strategy;
- a packing strategy;
- an orchestration strategy.

It should sit after task-intent derivation and scope resolution, and before planning or execution artifacts are built.

## Inputs And Outputs

### Typical Inputs

- `TaskIntent` or an equivalent structured view of the task;
- current session state and visible scope hints;
- agent role, package-level defaults, or instance preferences;
- active policy, rights, and runtime budget constraints;
- provider capability profile and currently available action surface;
- success-history, fallback rules, or local preferences where those exist.

### Typical Outputs

- `StrategyResolution` as the coherent selected strategy set;
- explicit fallback or degradation notes when the preferred set cannot be fully satisfied;
- compatibility notes that planning or execution layers may consume;
- traceable rationale suitable for inclusion in `ResolutionTrace`.

## Minimum Responsibilities

The interface should eventually support at least:

- selecting a coherent strategy set instead of isolated independent picks;
- rejecting combinations that are mutually incompatible under current policy, budget, or capability constraints;
- degrading or substituting strategies when the preferred set is unavailable;
- remaining separate from the execution of the selected strategy;
- exposing enough structure that planning artifacts can be built without hidden prompt-only logic.

## What `StrategyResolver` Must Not Become

It should not become:

- the full `ThinkingContext`;
- the execution engine for plans;
- a provider-owned reasoning channel;
- a memory backend in disguise;
- a catch-all replacement for turn orchestration.

Its job is strategy choice and coherence checking, not the whole turn.

## Minimum Invariants

1. Strategy resolution happens before final planning and execution, even if the first implementation is very small.
2. The selected strategies must be treated as a coherent set rather than four unrelated toggles.
3. Strategy choice must respect policy, budget, and capability constraints.
4. Strategy definitions remain declarative; execution happens elsewhere.
5. Degradation and fallback must be explicit enough to trace later.
6. The inherited main-agent path may remain the default preset, but it should become a chosen or implied strategy set rather than the only invisible system mode.

## Candidate Strategy Families

This interface is expected to resolve families such as:

- direct-answer versus verification-heavy reflection;
- local versus broader graph exploration;
- evidence-first versus ultra-compact packing;
- single-agent versus delegated orchestration.

In the current GraphClaw reading, these families should be understood against:

- graph-theory distinctions between local neighborhood work, path-sensitive exploration, branch separation, and ordering;
- GoT distinctions between generating, refining, aggregating, and ranking thought branches before the next runtime step is chosen.

This document does not freeze the full taxonomy. It only establishes that strategy families should be selectable through an explicit boundary.

## Degradation And Fallback

The interface should make room for outcomes such as:

- prefer orchestration, but fall back to single-agent execution;
- prefer deeper exploration, but degrade to local exploration under budget;
- prefer structured verification, but degrade to simpler reflection if the provider or runtime surface is too limited;
- refuse a strategy set when policy or required capabilities are absent.

These are strategy outcomes, not execution results.

This is also the place where the architecture must stay sharper than a hidden prompt habit: fallback means changing the selected strategy regime, not merely writing a shorter or vaguer prompt.

## Compatibility With The Inherited Runtime

The first migration step should preserve the current turn path as the default behavior.

The compatibility rule is:

> the inherited main-agent, prompt-first path should survive as the first default strategy preset while a new resolver boundary is introduced around it.

That means the first implementation can stay conservative:

- derive a minimal `TaskIntent`;
- resolve to one default strategy bundle most of the time;
- record only coarse fallback reasons;
- leave existing prompt and dispatch code in place behind the selected preset.

## Likely Source-Area Consumers

### `src/agent/`

Likely role:

- primary caller that needs a resolved strategy set before planning turn work.

Likely seam files:

- `src/agent/classifier.rs`
- `src/agent/loop_.rs`
- `src/agent/dispatcher.rs`

### `src/providers/`

Likely role:

- expose provider capability profiles that constrain strategy choice.

Documentary caution:

- providers should constrain or support strategy resolution, not own it.

### `src/tools/`

Likely role:

- expose capabilities and action availability that may affect orchestration or exploration choices.

### `src/runtime/` and `src/security/`

Likely role:

- supply budget and policy constraints that can eliminate or degrade candidate strategies.

## Initial Files To Treat As Seams

- `src/agent/classifier.rs`
- `src/agent/loop_.rs`
- `src/agent/dispatcher.rs`
- `src/providers/traits.rs`
- `src/providers/router.rs`
- `src/tools/traits.rs`
- `src/security/policy.rs`

## Recommended Minimal Introduction Order

1. Introduce a narrow `TaskIntent`-like input for turn classification.
2. Resolve to a single default strategy preset with explicit naming.
3. Add capability-, budget-, or provider-driven fallback selection.
4. Emit coarse strategy-resolution data into `ResolutionTrace`.
5. Expand later into richer reflection, exploration, packing, and orchestration families.

## Slice JSON

This slice is an orientation artifact, not an implementation claim.

```json
{
  "nodes": [
    {
      "id": "n0",
      "position": { "x": -650, "y": -40 },
      "caption": "TaskIntent",
      "labels": ["TaskIntent"],
      "properties": {
        "file_origin": "src/agent/classifier.rs + future intent seam",
        "role": "Structured description of the current task"
      },
      "style": {}
    },
    {
      "id": "n1",
      "position": { "x": -300, "y": -180 },
      "caption": "Policy And Budget Constraints",
      "labels": ["PolicyDefinition"],
      "properties": {
        "file_origin": "src/security/ + src/runtime/",
        "role": "Constrain which strategy combinations are allowed"
      },
      "style": {}
    },
    {
      "id": "n2",
      "position": { "x": -300, "y": 80 },
      "caption": "Provider Capability Profile",
      "labels": ["Resource"],
      "properties": {
        "file_origin": "src/providers/",
        "role": "Declares provider-side execution constraints"
      },
      "style": {}
    },
    {
      "id": "n3",
      "position": { "x": 70, "y": -40 },
      "caption": "StrategyResolver",
      "labels": ["Action"],
      "properties": {
        "file_origin": "future strategy seam in src/agent/",
        "role": "Selects a coherent reflection, exploration, packing, and orchestration set"
      },
      "style": {}
    },
    {
      "id": "n4",
      "position": { "x": 430, "y": -40 },
      "caption": "StrategyResolution",
      "labels": ["StrategyResolution"],
      "properties": {
        "file_origin": "future explicit runtime artifact",
        "role": "Chosen strategy set plus fallback notes"
      },
      "style": {}
    },
    {
      "id": "n5",
      "position": { "x": 780, "y": -40 },
      "caption": "ReflectionPlan",
      "labels": ["ReflectionPlan"],
      "properties": {
        "file_origin": "future planning seam",
        "role": "Downstream plan built from the selected strategy set"
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
      "type": "PLAN_USES_STRATEGY",
      "style": {},
      "properties": {},
      "fromId": "n5",
      "toId": "n4"
    },
    {
      "id": "r4",
      "type": "FULFILLED_BY",
      "style": {},
      "properties": {},
      "fromId": "n4",
      "toId": "n3"
    }
  ]
}
```

## Related References

- [`../concepts/graph-context-engine.md`](../concepts/graph-context-engine.md) for the engine boundary
- [`../concepts/got.md`](../concepts/got.md) for graph-shaped thought evolution
- [`../concepts/agent-loop.md`](../concepts/agent-loop.md) for mono-agent loop placement
- [`../concepts/context-artifacts.md`](../concepts/context-artifacts.md) for downstream artifacts
- [`../runtime/turn-runtime-logic.md`](../runtime/turn-runtime-logic.md) for turn sequencing
- [`../migration/future-integration-seams.md`](../migration/future-integration-seams.md) for seam placement
