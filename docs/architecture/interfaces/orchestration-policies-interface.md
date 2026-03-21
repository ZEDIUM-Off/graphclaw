# Orchestration Policies Interface

## Status

This document defines the target role of orchestration-policy seams in GraphClaw.

It does not claim that the inherited runtime already exposes dedicated orchestration interfaces in code. It defines the minimum migration-facing boundaries needed if routing, spawn, bounded sub-agent execution, aggregation, and event hooks are to become explicit and configurable rather than remaining one implicit behavior inside the inherited agent loop.

## Why This Interface Family Matters

The inherited runtime already has a workable default model: the main agent executes the turn, and some delegation behavior can happen around that path.

That is a valid baseline, but it still blends together several distinct concerns:

- who receives the task first;
- whether the task should be decomposed;
- what rights and context a sub-agent receives;
- how multiple results are recombined;
- how orchestration events are observed or traced.

If GraphClaw wants orchestration to become a governed strategy family rather than a hard-wired behavior, those concerns need separate seams.

## Interface Family

The minimum family to keep distinct is:

- `RoutingPolicy`
- `SpawnPolicy`
- `SubAgentRuntimePolicy`
- `AggregationPolicy`
- `HookBus`

These do not require separate crates or final trait signatures yet. They require explicit conceptual boundaries.

## `RoutingPolicy`

### Role

`RoutingPolicy` decides where a task enters the execution topology.

Typical choices may include:

- current main-agent default route;
- domain-specific router;
- budget-aware route;
- context-sensitive route based on current [`View`](../concepts/view.md) or current packed-context constraints;
- refusal when no allowed route satisfies policy.

### Minimum Responsibility

- choose the initial execution owner under policy and topology constraints;
- remain distinct from later spawn and aggregation logic.

## `SpawnPolicy`

### Role

`SpawnPolicy` decides whether work should be decomposed and how.

Typical choices may include:

- no spawn;
- sequential delegation;
- parallel bounded workers;
- reviewer/executor split;
- hierarchical parent/child decomposition.

### Minimum Responsibility

- decide if decomposition happens;
- define assignment shape and stop conditions;
- remain distinct from the route that chose the initial owner.

## `SubAgentRuntimePolicy`

### Role

`SubAgentRuntimePolicy` constrains the execution environment granted to a delegated worker.

Typical constraints may include:

- reduced session-oriented frame exposure;
- narrower `View`;
- lower runtime budget;
- read-only or no-mutation mode;
- limited capability exposure.

### Minimum Responsibility

- make sub-agent execution bounded and governable;
- remain distinct from high-level routing and aggregation decisions.

## `AggregationPolicy`

### Role

`AggregationPolicy` defines how multiple execution results are recombined into a turn-level result.

Typical choices may include:

- parent reviews one worker output;
- structured merge;
- contradiction-aware arbitration;
- best-result selection;
- summary synthesis with retained evidence.

### Minimum Responsibility

- recombine delegated outputs explicitly rather than leaving aggregation implicit in the parent prompt.

## `HookBus`

### Role

`HookBus` is the orchestration event seam for observability, tracing, and controlled extensions.

Typical hook points may include:

- before route;
- after route;
- before spawn;
- after spawn;
- before aggregate;
- after aggregate;
- on error;
- on degrade.

### Minimum Responsibility

- expose orchestration lifecycle events without making hooks the owner of orchestration policy.

## Separation Rule

The minimum rule to keep stable is:

1. `RoutingPolicy` decides the initial owner;
2. `SpawnPolicy` decides decomposition;
3. `SubAgentRuntimePolicy` bounds delegated execution;
4. `AggregationPolicy` recombines results;
5. `HookBus` observes or surfaces lifecycle events.

That rule allows the current single-agent path to remain valid while still becoming one explicit preset among others.

## Minimum Invariants

1. The inherited main-agent path may remain the default orchestration preset.
2. Routing, spawning, runtime bounds, and aggregation should not remain conceptually fused forever.
3. Sub-agent execution must be bounded by policy, context, and budget rather than inheriting unrestricted parent access.
4. Aggregation must be explicit enough to trace, not just prompt-local glue.
5. Orchestration traceability should exist even when the first implementation supports only one or two presets.

## Inputs And Outputs

### Typical Inputs

- `TaskIntent`;
- `StrategyResolution`;
- current [`View`](../concepts/view.md) or packed context constraints;
- topology and capability information;
- runtime and security constraints;
- parent-agent preferences or package defaults.

### Typical Outputs

- `OrchestrationPlan`;
- sub-agent assignments or bounded execution descriptors;
- aggregation requirements;
- orchestration lifecycle events and trace-relevant outputs.

## Errors, Rejection, And Degradation

This interface family should support outcomes such as:

- refuse delegation because policy forbids sub-agent execution;
- degrade from parallel workers to single-agent execution;
- shrink sub-agent context scope because the full parent window is not allowed;
- skip aggregation modes that require unavailable capabilities;
- preserve the current main-agent route as a fallback when orchestration cannot be expanded safely.

## Compatibility With The Inherited Runtime

The first migration step should preserve current execution behavior.

The compatibility rule is:

> the existing main-agent behavior should become the first default orchestration preset while future routing, spawn, and aggregation seams are introduced around it.

That means the first implementation can stay conservative:

- represent "main agent only" as an explicit routing and no-spawn choice;
- add bounded reviewer/executor or one-worker patterns later;
- keep delegation-compatible files stable while making orchestration artifacts visible;
- emit coarse orchestration trace data before expanding policy richness.

## Likely Source-Area Consumers

### `src/agent/`

Likely role:

- primary consumer of orchestration decisions during turn coordination and delegation.

Likely seam files:

- `src/agent/dispatcher.rs`
- `src/agent/loop_.rs`

### `src/runtime/` and `src/security/`

Likely role:

- enforce bounded sub-agent runtime constraints, capability restrictions, and execution guards.

### `src/tools/`

Likely role:

- expose capability metadata or delegation helpers that orchestration can consume.

### `src/gateway/`

Likely role:

- stream orchestration events or trace summaries later through SSE or WebSocket surfaces.

## Initial Files To Treat As Seams

- `src/agent/dispatcher.rs`
- `src/agent/loop_.rs`
- `src/runtime/traits.rs`
- `src/security/policy.rs`
- `src/tools/delegate.rs`
- `src/gateway/sse.rs`
- `src/gateway/ws.rs`

## Recommended Minimal Introduction Order

1. Name the current main-agent path as the default orchestration preset.
2. Introduce an explicit `OrchestrationPlan` artifact even when it encodes "single agent only."
3. Add one bounded delegation mode such as reviewer/executor or single worker.
4. Add explicit aggregation and coarse hook events.
5. Expand later toward richer routing and spawn policies once runtime bounds are trustworthy.

## Slice JSON

This slice is an orientation artifact, not an implementation claim.

```json
{
  "nodes": [
    {
      "id": "n0",
      "position": { "x": -700, "y": -50 },
      "caption": "RoutingPolicy",
      "labels": ["PolicyDefinition"],
      "properties": {
        "file_origin": "future orchestration seam in src/agent/",
        "role": "Chooses the initial execution owner"
      },
      "style": {}
    },
    {
      "id": "n1",
      "position": { "x": -350, "y": -180 },
      "caption": "SpawnPolicy",
      "labels": ["PolicyDefinition"],
      "properties": {
        "file_origin": "future orchestration seam in src/agent/",
        "role": "Decides whether and how work is decomposed"
      },
      "style": {}
    },
    {
      "id": "n2",
      "position": { "x": -350, "y": 80 },
      "caption": "SubAgentRuntimePolicy",
      "labels": ["PolicyDefinition"],
      "properties": {
        "file_origin": "src/runtime/ + src/security/",
        "role": "Bounds delegated execution rights, context, and budget"
      },
      "style": {}
    },
    {
      "id": "n3",
      "position": { "x": 40, "y": -50 },
      "caption": "OrchestrationPlan",
      "labels": ["OrchestrationPlan"],
      "properties": {
        "file_origin": "future explicit runtime artifact",
        "role": "Captures routing, decomposition, and recombination choices"
      },
      "style": {}
    },
    {
      "id": "n4",
      "position": { "x": 410, "y": -180 },
      "caption": "AggregationPolicy",
      "labels": ["PolicyDefinition"],
      "properties": {
        "file_origin": "future aggregation seam",
        "role": "Defines how worker outputs are recombined"
      },
      "style": {}
    },
    {
      "id": "n5",
      "position": { "x": 410, "y": 80 },
      "caption": "HookBus",
      "labels": ["Resource"],
      "properties": {
        "file_origin": "future orchestration observability seam",
        "role": "Emits orchestration lifecycle events"
      },
      "style": {}
    },
    {
      "id": "n6",
      "position": { "x": 790, "y": -50 },
      "caption": "AggregationResult",
      "labels": ["AggregationResult"],
      "properties": {
        "file_origin": "future execution layer",
        "role": "Final recombined orchestration output"
      },
      "style": {}
    }
  ],
  "relationships": [
    {
      "id": "r0",
      "type": "ORCHESTRATION_USES_ROUTER",
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
      "type": "AGGREGATES",
      "style": {},
      "properties": {},
      "fromId": "n6",
      "toId": "n4"
    },
    {
      "id": "r4",
      "type": "RELATED_TO",
      "style": {},
      "properties": {},
      "fromId": "n5",
      "toId": "n3"
    },
    {
      "id": "r5",
      "type": "TRACE_IMPLEMENTED_STRATEGY",
      "style": {},
      "properties": {},
      "fromId": "n6",
      "toId": "n3"
    }
  ]
}
```

## Related References

- `future-integration-seams.md`
- `strategy-resolver-interface.md`
- `turn-runtime-logic.md`
- `context-artifacts.md`
