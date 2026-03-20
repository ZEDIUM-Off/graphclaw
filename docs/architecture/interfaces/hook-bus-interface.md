# HookBus Interface

## Status

This document defines the target role of `HookBus` as a migration-facing GraphClaw interface.

It does not claim that the inherited runtime already exposes a dedicated hook bus in code. It defines the minimum boundary needed if orchestration, context packing, and related lifecycle events are to become observable and extensible without smuggling business logic into ad hoc callbacks.

## Why This Interface Matters

GraphClaw is moving toward explicit artifacts such as:

- `StrategyResolution`;
- `SessionWindow`;
- `ContextPack`;
- `OrchestrationPlan`;
- `ResolutionTrace`.

As those artifacts become more explicit, the runtime needs a clean way to observe important lifecycle moments without making each owner define its own incompatible event surface.

Without `HookBus`, it becomes difficult to:

- stream orchestration and packing events consistently;
- attach observability or debugging logic without rewriting core coordination paths;
- keep hooks separate from the policies or strategies they observe;
- decide which runtime events are worth promoting to trace or UI surfaces.

## Interface Role

`HookBus` is the bounded event seam for surfacing selected runtime lifecycle moments to observability, debugging, transport, or later extension layers.

It should sit adjacent to:

- orchestration;
- context mutation and packing;
- provider invocation;
- aggregation;
- degradation or error handling.

It is not:

- the owner of orchestration policy;
- the source of truth for `ResolutionTrace`;
- a replacement for explicit runtime artifacts;
- an excuse to hide business logic inside callbacks.

## Typical Hook Families

The stable first families should be:

- before route;
- after route;
- before spawn;
- after spawn;
- before context pack;
- after context pack;
- before aggregate;
- after aggregate;
- on error;
- on degrade.

These names are conceptual. This document does not freeze an enum or runtime API.

## Inputs And Outputs

### Typical Inputs

- hook events emitted by turn coordination, orchestration, packing, or provider-facing execution;
- references to explicit runtime artifacts relevant to the event;
- bounded metadata suitable for debugging, streaming, or observability.

### Typical Outputs

- event notifications to observers, streamers, or metrics layers;
- optional trace-enrichment inputs;
- optional transport-facing event payloads for SSE, WebSocket, or inspection views.

## Minimum Responsibilities

The interface should eventually support at least:

- publishing selected lifecycle events in a consistent shape;
- staying separate from the policy or strategy logic that produced those events;
- carrying references to explicit artifacts where possible instead of only raw text;
- remaining optional enough that hooks can be absent without changing core semantics.

## Minimum Invariants

1. Hooks observe or surface lifecycle moments; they do not own the corresponding policy.
2. Hook emission should not redefine the meaning of `ResolutionTrace`.
3. Hook payloads should prefer explicit artifact references or bounded metadata over opaque text blobs.
4. The runtime should be able to run without a rich hook surface; hooks are an adjunct seam, not the core engine.
5. Hook-driven observability should remain compatible with inherited single-agent behavior.

## Relationship To `ResolutionTrace`

`HookBus` and `ResolutionTrace` are adjacent but distinct.

The stable distinction should be:

- `HookBus` surfaces runtime lifecycle moments for observability or extension;
- `ResolutionTrace` records explicit context-resolution and strategy-resolution decisions that matter for audit and replay.

Some hook events may later contribute inputs to traces, but hook emission is not itself the trace model.

## Errors, Rejection, And Degradation

The interface should make room for:

- error events without forcing callers to hide the actual runtime error boundary;
- degrade events when strategy, packing, or orchestration falls back;
- hook absence without changing business behavior;
- bounded failure behavior if a hook consumer is unavailable.

## Compatibility With The Inherited Runtime

The first migration step should preserve current behavior.

The compatibility rule is:

> the inherited runtime may begin by emitting only a few coarse lifecycle events while remaining otherwise unchanged, so observability can grow without forcing a redesign-first orchestration layer.

That means the first implementation can stay conservative:

- emit a handful of coarse events around packing or delegation;
- keep event payloads small and artifact-oriented where possible;
- let SSE or WebSocket surfaces consume hook output later, not immediately.

## Likely Source-Area Consumers

### `src/agent/`

Likely role:

- emit high-level turn, orchestration, and packing lifecycle events.

Likely seam files:

- `src/agent/loop_.rs`
- `src/agent/dispatcher.rs`

### `src/gateway/`

Likely role:

- stream selected lifecycle events later through `sse.rs` or `ws.rs`.

### `src/runtime/` and `src/security/`

Likely role:

- emit or annotate bounded events related to execution or degraded behavior.

## Initial Files To Treat As Seams

- `src/agent/loop_.rs`
- `src/agent/dispatcher.rs`
- `src/gateway/sse.rs`
- `src/gateway/ws.rs`
- `src/runtime/traits.rs`
- `src/security/policy.rs`

## Recommended Minimal Introduction Order

1. Emit a small set of coarse lifecycle events around routing, packing, and degradation.
2. Keep hook output observational rather than policy-owning.
3. Connect selected events to streaming or inspection surfaces later.
4. Expand toward richer event families only after explicit artifacts are stable.

## Slice JSON

This slice is an orientation artifact, not an implementation claim.

```json
{
  "nodes": [
    {
      "id": "n0",
      "position": { "x": -650, "y": -50 },
      "caption": "OrchestrationPlan",
      "labels": ["OrchestrationPlan"],
      "properties": {
        "file_origin": "future explicit runtime artifact",
        "role": "Produces lifecycle moments worth observing"
      },
      "style": {}
    },
    {
      "id": "n1",
      "position": { "x": -300, "y": -180 },
      "caption": "ContextPack",
      "labels": ["ContextPack"],
      "properties": {
        "file_origin": "future pack compiler seam",
        "role": "Produces packing lifecycle events"
      },
      "style": {}
    },
    {
      "id": "n2",
      "position": { "x": -300, "y": 80 },
      "caption": "ResolutionTrace",
      "labels": ["ResolutionTrace"],
      "properties": {
        "file_origin": "future trace layer",
        "role": "Adjacent audit artifact that may consume some hook-derived inputs"
      },
      "style": {}
    },
    {
      "id": "n3",
      "position": { "x": 50, "y": -50 },
      "caption": "HookBus",
      "labels": ["Resource"],
      "properties": {
        "file_origin": "future lifecycle-event seam",
        "role": "Publishes bounded runtime lifecycle events"
      },
      "style": {}
    },
    {
      "id": "n4",
      "position": { "x": 430, "y": -180 },
      "caption": "SSE And WebSocket Streams",
      "labels": ["Resource"],
      "properties": {
        "file_origin": "src/gateway/sse.rs + src/gateway/ws.rs",
        "role": "Potential transport surfaces for selected hook output"
      },
      "style": {}
    },
    {
      "id": "n5",
      "position": { "x": 430, "y": 80 },
      "caption": "Debug And Observability Consumers",
      "labels": ["Resource"],
      "properties": {
        "file_origin": "future observability layer",
        "role": "Consumes bounded lifecycle notifications"
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
      "type": "HAS_TOPIC",
      "style": {},
      "properties": {},
      "fromId": "n3",
      "toId": "n4"
    },
    {
      "id": "r4",
      "type": "HAS_TOPIC",
      "style": {},
      "properties": {},
      "fromId": "n3",
      "toId": "n5"
    }
  ]
}
```

## Related References

- `orchestration-policies-interface.md`
- `context-pack-interface.md`
- `future-integration-seams.md`
- `context-artifacts.md`
