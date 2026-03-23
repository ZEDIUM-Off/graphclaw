# GraphClaw Glossary Index

This file is a routing index for GraphClaw terminology.

It is not a second canonical-definition surface. Canonical definitions live in the concept-specific architecture documents listed below.

Use this index to find the right source quickly, then read the linked document instead of relying on a duplicate local summary.

## Core Context Terms


| Term                                  | Canonical source                                                                                     |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `Set`                                 | `[set.md](set.md)`                                                                                   |
| `ResolvedSet`                         | `[resolved-set.md](resolved-set.md)`                                                                 |
| `View`                                | `[view.md](view.md)`                                                                                 |
| `Graph of Thoughts` (`GoT`)           | `[got.md](got.md)`                                                                                   |
| `Graph of Operations` (`GoO`)         | `[goo.md](goo.md)`                                                                                   |
| `GoTState`                            | `[got.md#gotstate](got.md#gotstate)`                                                                 |
| `Reusable GoO`                        | `[goo.md#goo-persiste-et-reutilisation](goo.md#goo-persiste-et-reutilisation)`                      |
| `Packable Subgraph`                   | `[packability.md](packability.md)`                                                                   |
| `Packability`                         | `[packability.md](packability.md)`                                                                   |
| `Bounded Complement`                  | `[packability.md](packability.md)`                                                                   |
| `Condensation`                        | `[packability.md](packability.md)`                                                                   |
| `Projection Into A Packable Subgraph` | `[packability.md](packability.md)`                                                                   |
| `SessionFrame`                        | `[session-frame.md](session-frame.md)`                                                               |
| `ContextFrame`                        | `[context-frame.md](context-frame.md)`                                                               |
| `ContextPack`                         | `[context-pack-interface.md](../interfaces/context-pack-interface.md)`                               |
| `ContextMutationProposal`             | `[graph-context-engine.md#contextmutationproposal](graph-context-engine.md#contextmutationproposal)` |
| `ResolutionTrace`                     | `[graph-context-engine.md#resolutiontrace](graph-context-engine.md#resolutiontrace)`                 |
| `TaskIntent`                          | `[graph-context-engine.md#taskintent](graph-context-engine.md#taskintent)`                           |
| `StrategyResolution`                  | `[strategy-resolver-interface.md](../interfaces/strategy-resolver-interface.md)`                     |
| `ReflectionPlan`                      | `[context-artifacts.md](context-artifacts.md)`                                                       |
| `ExplorationPlan`                     | `[context-artifacts.md](context-artifacts.md)`                                                       |
| `ContextEditPlan`                     | `[context-artifacts.md](context-artifacts.md)`                                                       |
| `MutationGuard`                       | `[mutation-guard-interface.md](../interfaces/mutation-guard-interface.md)`                           |
| `OrchestrationPlan`                   | `[context-artifacts.md](context-artifacts.md)`                                                       |


## Strategy Terms


| Term                              | Canonical source                                                                           |
| --------------------------------- | ------------------------------------------------------------------------------------------ |
| `ReflectionStrategyDefinition`    | `[graph-context-engine.md#strategy-families](graph-context-engine.md#strategy-families)`   |
| `ExplorationStrategyDefinition`   | `[graph-context-engine.md#strategy-families](graph-context-engine.md#strategy-families)`   |
| `PackingStrategyDefinition`       | `[graph-context-engine.md#strategy-families](graph-context-engine.md#strategy-families)`   |
| `OrchestrationStrategyDefinition` | `[graph-context-engine.md#strategy-families](graph-context-engine.md#strategy-families)`   |
| `RoutingPolicy`                   | `[orchestration-policies-interface.md](../interfaces/orchestration-policies-interface.md)` |
| `SpawnPolicy`                     | `[orchestration-policies-interface.md](../interfaces/orchestration-policies-interface.md)` |
| `SubAgentRuntimePolicy`           | `[orchestration-policies-interface.md](../interfaces/orchestration-policies-interface.md)` |
| `AggregationPolicy`               | `[orchestration-policies-interface.md](../interfaces/orchestration-policies-interface.md)` |
| `HookBus`                         | `[hook-bus-interface.md](../interfaces/hook-bus-interface.md)`                             |


## Agent Packaging Terms


| Term            | Canonical source                                                               |
| --------------- | ------------------------------------------------------------------------------ |
| `AgentPackage`  | `[graph-context-engine.md#agentpackage](graph-context-engine.md#agentpackage)` |
| `AgentInstance` | `[graph-context-engine.md#agentpackage](graph-context-engine.md#agentpackage)` |
| `AgentSession`  | `[graph-context-engine.md#agentpackage](graph-context-engine.md#agentpackage)` |
| `Bindings`      | `[graph-context-engine.md#agentpackage](graph-context-engine.md#agentpackage)` |


## Framing Rules

- use this file as an index, not as a definition source;
- if a term needs a new canonical home, update `[definition-governance.md](definition-governance.md)` first;
- if a non-canonical doc mentions a term, link the canonical source rather than redefining it.
