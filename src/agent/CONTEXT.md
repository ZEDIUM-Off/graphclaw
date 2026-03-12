# Agent Context

## Purpose

`src/agent/` holds the core chat-loop machinery: prompt assembly, dispatch, classification, and loading memory into each run.

## File / Folder Map

- `src/agent/mod.rs` - module entry and public surface
- `src/agent/loop_.rs` - main agent loop orchestration
- `src/agent/prompt.rs` - prompt construction and formatting
- `src/agent/dispatcher.rs` - model/tool dispatch path
- `src/agent/memory_loader.rs` - memory retrieval injected into runs
- `src/agent/classifier.rs` - request classification helpers
- `src/agent/tests.rs` - focused tests for this subsystem

## Go Here For

- Prompt wording or prompt structure: `src/agent/prompt.rs`
- Turn loop behavior: `src/agent/loop_.rs`
- Dispatch decisions: `src/agent/dispatcher.rs`
- Memory hydration before inference: `src/agent/memory_loader.rs`

## Current State

This is still the inherited agent runtime path. It is one of the most likely future insertion points for richer GraphClaw context resolution, but the area today is still prompt-and-dispatch centric.

## Current Dependency Direction

- Usually entered from `src/main.rs` command paths, channel routing in `src/channels/`, and gateway-driven runs in `src/gateway/`.
- Calls outward into `src/providers/` for model inference, `src/tools/` for capability execution, `src/memory/` for recall and autosave, `src/security/` for policy checks, and `src/observability/` for trace/log emission.
- Uses `src/agent/prompt.rs` for system prompt assembly and `src/agent/memory_loader.rs` for pre-run context hydration.

## GraphClaw Evolution Note

Do not document this folder as if GraphClaw already has a graph-native runtime planner here. Any future graph-aware work must layer onto the existing loop carefully.

## Likely Migration Seams

1. `src/agent/prompt.rs` is the clearest seam for separating static system prompt sections from future dynamic context-pack overlays.
2. `src/agent/memory_loader.rs` is the seam where flat recall can eventually give way to richer context selection and graph-backed evidence assembly.
3. `src/agent/loop_.rs` is the seam for introducing explicit turn artifacts such as `SessionWindow`, `ContextPack`, and resolution traces without rewriting the whole loop at once.
4. `src/agent/dispatcher.rs` is where tool-call planning and provider response handling may later consume structured context artifacts instead of only prompt text.

## What Must Stay Stable During Migration

- Multi-turn tool execution semantics in `loop_.rs`
- Current provider and tool dispatch contracts
- Existing progress streaming, compaction, and approval behavior unless migration work explicitly changes them
- User-visible behavior for current `zeroclaw` command paths until compatibility plans exist

## Constraints / Cautions

- Small changes can affect every user interaction.
- Prompt, dispatch, and memory loading are separate concerns; keep them separate.
- Preserve inherited `zeroclaw` names unless the task is explicitly a rename migration.

## How Agents Should Work Here

Read the exact file that owns the behavior before editing.

Recommended exploration order:

1. `src/agent/prompt.rs`
2. `src/agent/memory_loader.rs`
3. `src/agent/loop_.rs`
4. `src/agent/dispatcher.rs`

Prefer tests-first changes for runtime behavior, keep cross-module changes explicit, and document seams instead of smuggling new architecture into generic helpers.
