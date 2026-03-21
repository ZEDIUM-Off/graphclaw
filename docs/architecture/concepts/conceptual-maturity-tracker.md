# GraphClaw Conceptual Maturity Tracker

## Status

This document tracks the maturity of GraphClaw's conceptual architecture.

It is a documentation governance artifact. It does not describe implementation status and it does not replace the canonical concept definitions listed in [`definition-governance.md`](definition-governance.md).

## Why This Document Exists

The architecture subtree already defines a substantial part of the target concept model, but that material is spread across multiple references, interface fiches, and open-question sections.

GraphClaw now needs an explicit place to track:

- what is already conceptually established;
- what is mostly defined but still needs sharper contracts;
- what remains materially open or fragile;
- which document is currently the source of truth for each area.

This keeps conceptualization reviewable instead of letting maturity signals remain implicit.

## Reading Rule

Use this document to assess maturity, not to redefine terms.

When a concept is marked as established here, the canonical meaning still lives in the linked architecture documents. When a concept is marked as open here, that means the repo should avoid speaking as if the missing detail were already settled.

## Maturity Scale

| Level | Meaning |
| --- | --- |
| `Established` | the repo has a stable conceptual distinction, purpose, and framing rule that should now be treated as baseline |
| `Needs Precision` | the concept family is real and useful, but important invariants, edge conditions, or operating rules still need sharper definition |
| `Open / Fragile` | the repo has identified the question, but the current answer is intentionally incomplete, contested, or still too loose to treat as stable |

## Current Snapshot

As of March 21, 2026, the architecture docs show a strong conceptual baseline around vocabulary, artifact separation, and migration discipline.

The least mature areas are not the top-level ideas themselves, but the transition from concept to governable runtime contract:

- exact trace granularity;
- exact persistence versus transient rules;
- exact rights and policy composition across context layers;
- exact first runtime insertion point beside the inherited path;
- exact representation rules for packable content and attached readable material.

## Maturity Matrix

| Concept area | Maturity | What is clearly established | What still needs work | Primary references |
| --- | --- | --- | --- | --- |
| repo identity and migration truthfulness | `Established` | GraphClaw is the repo identity; inherited `zeroclaw` surfaces remain current implementation truth; target architecture must not be presented as already implemented | maintain discipline as docs expand | [`../../README.md`](../../README.md), [`../../AGENTS.md`](../../AGENTS.md), [`graph-context-engine.md`](graph-context-engine.md) |
| seam-first migration thesis | `Established` | migration proceeds through vocabulary, boundaries, artifacts, and seams rather than rename-first rewrite | decide which runtime seam is introduced first in code | [`zero-to-graphclaw-transition.md`](../migration/zero-to-graphclaw-transition.md), [`future-integration-seams.md`](../migration/future-integration-seams.md) |
| Graph Context Engine boundary | `Established` | engine means governed context resolution plus turn-time strategy resolution; it is distinct from backend, packaging, and whole runtime | keep boundary intact when future code work starts | [`graph-context-engine.md`](graph-context-engine.md), [`definition-governance.md`](definition-governance.md) |
| `Set` / `ResolvedSet` / `View` distinction | `Established` | governed `Set`, derived `ResolvedSet`, and runtime `View` are now clearly separated | finalize some operating details around validation, relation names, and runtime representation | [`views-and-sets.md`](views-and-sets.md), [`set.md`](set.md), [`resolved-set.md`](resolved-set.md), [`view.md`](view.md), [`definition-governance.md`](definition-governance.md) |
| artifact chain (`TaskIntent` to `ResolutionTrace`) | `Established` | the repo has a coherent target artifact chain and distinguishes planning artifacts from context artifacts | specify which artifacts must materialize and when | [`context-artifacts.md`](context-artifacts.md), [`turn-runtime-logic.md`](../runtime/turn-runtime-logic.md) |
| strategy-family framing | `Established` | reflection, exploration, packing, and orchestration are explicit strategy families, not hidden habits | sharpen concrete strategy taxonomy later without collapsing the abstraction | [`graph-context-engine.md`](graph-context-engine.md), [`strategy-resolver-interface.md`](../interfaces/strategy-resolver-interface.md) |
| frame-oriented context composition | `Established` | `ContextFrame` is the invocation-oriented distillation layer between governed graph state and provider-visible packed context | stabilize reusable frame metadata without reopening the concept boundary | [`context-frame.md`](context-frame.md), [`context-pack-interface.md`](../interfaces/context-pack-interface.md), [`context-artifacts.md`](context-artifacts.md) |
| `SessionFrame` role | `Needs Precision` | `SessionFrame` is the session-oriented `ContextFrame` derived from the active `View` | sharpen exact projection contract and minimal session metadata vocabulary | [`session-frame.md`](session-frame.md), [`context-frame.md`](context-frame.md), [`context-pack-interface.md`](../interfaces/context-pack-interface.md) |
| `ContextPack` role | `Needs Precision` | there is a clear target role: final model-visible, budgeted artifact derived from narrower governed state and composed from ordered `ContextFrame` objects | sharpen exact representation contract, provider-facing ordering rules, and provenance expectations | [`context-pack-interface.md`](../interfaces/context-pack-interface.md), [`context-frame.md`](context-frame.md), [`context-artifacts.md`](context-artifacts.md) |
| `StrategyResolution` and `StrategyResolver` boundary | `Needs Precision` | the repo clearly distinguishes strategy choice from downstream execution | sharpen minimal input schema, coherence rules, and degradation vocabulary | [`strategy-resolver-interface.md`](../interfaces/strategy-resolver-interface.md), [`turn-runtime-logic.md`](../runtime/turn-runtime-logic.md) |
| budget model | `Needs Precision` | navigation cost, packable-subgraph cost, and final context cost are conceptually separated | define what must be measured, estimated, or enforced at each layer | [`context-artifacts.md`](context-artifacts.md), [`turn-runtime-logic.md`](../runtime/turn-runtime-logic.md) |
| set validation contract | `Needs Precision` | Sets should be schema-validatable and cost-aware | define the minimum schema contract and failure semantics | [`set.md`](set.md) |
| view materialization model | `Needs Precision` | lazy versus materialized `View` is a meaningful distinction | define whether runtime views stay Rust-only, traceable artifacts, or mixed forms | [`view.md`](view.md) |
| navigation versus definition relations | `Needs Precision` | the distinction is mandatory and structurally important | finalize the navigation/exposition relation family names | [`set.md`](set.md), [`graph-context-engine.md`](graph-context-engine.md) |
| context mutation governance | `Needs Precision` | mutations should be explicit, validated, and traceable rather than prompt-local edits | define durable mutation classes and which ones remain turn-local only | [`context-artifacts.md`](context-artifacts.md), [`view.md`](view.md), [`mutation-guard-interface.md`](../interfaces/mutation-guard-interface.md) |
| trace granularity | `Open / Fragile` | `ResolutionTrace` is a required concept and should record selection, degradation, and packing decisions | the required detail level for routine turns is still unresolved | [`context-artifacts.md`](context-artifacts.md), [`turn-runtime-logic.md`](../runtime/turn-runtime-logic.md), [`zero-to-graphclaw-transition.md`](../migration/zero-to-graphclaw-transition.md) |
| persistence versus transient policy | `Open / Fragile` | the repo already distinguishes persisted definitions from transient runtime objects | exact persistability rules for views, packable subgraphs, and traces remain open | [`set.md`](set.md), [`view.md`](view.md), [`packability.md`](packability.md), [`context-artifacts.md`](context-artifacts.md), [`future-integration-seams.md`](../migration/future-integration-seams.md) |
| rights and policy composition across layers | `Open / Fragile` | policy and rights are known to constrain views, mutations, packing, and exposure | the composition model across `Set`, `View`, `ContextFrame`, `SessionFrame`, and `ContextPack` is not yet stabilized | [`set.md`](set.md), [`view.md`](view.md), [`context-frame.md`](context-frame.md), [`session-frame.md`](session-frame.md), [`context-pack-interface.md`](../interfaces/context-pack-interface.md) |
| first runtime insertion point | `Open / Fragile` | the docs agree that the inherited loop stays operational while a new seam appears beside it | the first concrete alternative context-creation branch is still undecided | [`zero-to-graphclaw-transition.md`](../migration/zero-to-graphclaw-transition.md), [`turn-runtime-logic.md`](../runtime/turn-runtime-logic.md), [`future-integration-seams.md`](../migration/future-integration-seams.md) |
| packable content and attached readable material | `Open / Fragile` | the docs already recognize that some content is only packable through excerpts, summaries, or replacements | the exact representation rules for attached content are still weakly specified | [`packability.md`](packability.md), [`context-artifacts.md`](context-artifacts.md), [`context-pack-interface.md`](../interfaces/context-pack-interface.md) |

## What GraphClaw Has Already Settled Well

The repo is no longer conceptually vague about its top-level direction.

The clearest established points are:

- GraphClaw is not "renamed ZeroClaw"; it is a transitional scaffold toward a graph-native context engine.
- The conceptual boundary between engine, backend, runtime, and packaging is now explicit.
- `Set`, `ResolvedSet`, and `View` are no longer conflated.
- context selection, runtime graph work, frame derivation, and final packed model context are no longer treated as the same thing.
- migration is framed as seam introduction, not mass renaming.

Those points should now be treated as baseline doctrine unless an explicit design change revisits them.

## What Should Be Tightened Next

The next conceptual work should bias toward contracts, not new slogans.

The highest-value clarification areas are:

1. define the minimum schema and governance metadata contract for `ContextFrame`;
2. define the minimum projection contract for `SessionFrame`;
3. define the minimum representational contract for `ContextPack`;
4. define the minimum granularity and lifecycle of `ResolutionTrace`;
5. define which artifacts are mandatory transient objects versus optional persisted artifacts;
6. define how rights and policy checks compose across `Set`, `View`, `ContextFrame`, `SessionFrame`, and `ContextPack`.

## Working Rule For Future Architecture Edits

When adding or revising a concept in `docs/architecture/`, update this tracker if the change does one of the following:

- stabilizes a concept enough to move it from `Needs Precision` to `Established`;
- reveals a new unresolved dependency that should be marked `Open / Fragile`;
- changes the primary source-of-truth document for a concept family.

Do not use this tracker to invent new definitions that are absent from the canonical architecture references.
