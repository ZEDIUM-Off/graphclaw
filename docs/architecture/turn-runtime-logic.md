# Turn Runtime Logic

## Status

This document describes the logical turn phases GraphClaw is trying to stabilize and how they relate to the inherited runtime.

It is not a claim that the current runtime already exposes a complete graph-native turn pipeline in code.

## Why This Document Exists

The repo needs a shared way to talk about turn logic without confusing:

- the current inherited agent loop;
- the target Graph Context Engine;
- backend capabilities;
- source-level implementation details.

This document therefore describes the logical phases of a turn and then maps them to the current runtime seams that are most likely to host those concerns during migration.

## Logical Turn Phases

The stable logical sequence should be documented as:

1. determine the session-scoped constraints that bound what may be accessed;
2. resolve or refresh the relevant `View` scope;
3. build, expand, or refine candidate `GraphSet` objects;
4. enter `ThinkingContext` to compare candidate structures and possible mutations;
5. evaluate packability, policy, and budget;
6. derive a packable subgraph candidate;
7. build the final `ContextPack`;
8. record a `ResolutionTrace`;
9. hand the result into response generation and any post-turn persistence flow.

These are logical phases, not a fixed class diagram.

## Current Inherited Runtime Mapping

The current runtime is still organized primarily around `src/agent/`.

The important transition rule is:

> the inherited loop remains the operational path, but the docs should show where explicit context artifacts can later be inserted without rewriting the whole loop in one pass.

### `src/agent/prompt.rs`

Current role:

- assemble and format prompt-visible sections.

Future seam:

- consume a `ContextPack` or a more explicit context overlay rather than only implicit assembled fragments.

### `src/agent/memory_loader.rs`

Current role:

- preload memory or recall material before inference.

Future seam:

- obtain candidate context inputs from one or more providers, including legacy recall and future graph-backed resolution support.

This should become a seam for feeding context selection, not a place where the entire Graph Context Engine is conceptually absorbed.

### `src/agent/loop_.rs`

Current role:

- orchestrate the inherited turn loop and execution flow.

Future seam:

- own the handoff between turn orchestration and explicit context artifacts such as `SessionWindow`, `ContextPack`, and `ResolutionTrace`.

### `src/agent/dispatcher.rs`

Current role:

- handle dispatch decisions around providers, tool activity, and result flow.

Future seam:

- consume context artifacts produced upstream and emit execution outputs that can later contribute to trace or evidence handling.

## Reflection Is A System Phase

The reflective context phase should be documented as a system phase that precedes final response generation.

It may use:

- backend calls;
- retrieval operations;
- ranking;
- summarization;
- evidence gathering;
- tool-like internal operations.

But it should not be described as merely another ordinary tool alongside user-callable capabilities.

That distinction matters especially for `src/agent/` and `src/tools/` documentation.

## Budget In The Turn

The docs should keep these costs separate:

- broad exploration cost during `ThinkingContext`;
- cost of a packable subgraph candidate;
- final model-visible cost of the `ContextPack`.

The inherited runtime may still blur some of these today. The architecture docs should make the target distinction explicit so future implementation can align to it.

## Transition Discipline

When source-adjacent docs describe turn logic, they should:

- describe what the inherited runtime does today;
- name the likely future insertion points for explicit artifacts;
- avoid claiming those artifacts already exist if the code does not support them;
- preserve the distinction between orchestration, memory loading, tool execution, and context governance.

## Open Questions

Important unresolved questions to keep visible include:

- where the first alternative context-creation path should branch beside the inherited one;
- how much `ResolutionTrace` detail should be captured during routine turns;
- which parts of turn-time exploration may safely remain transient versus materialized;
- which current prompt sections should later become explicit `ContextPack` components.
