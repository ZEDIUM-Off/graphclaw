---
name: master-graphclaw-plan
overview: Master plan to evolve GraphClaw from the inherited ZeroClaw runtime toward the documented graph-native context engine through seam-first, testable phases. The plan is structured so each phase can later be turned into an independent sub-plan with clear scope and exit criteria.
todos:
  - id: phase-0-baseline
    content: Baseline current runtime seams, invariants, and test coverage before behavioral changes
    status: completed
  - id: phase-1-artifacts
    content: Define executable Rust artifact model for GraphClaw context resolution
    status: pending
  - id: phase-2-context-seam
    content: Introduce a top-level context creation seam in the agent loop
    status: pending
  - id: phase-3-view-sets
    content: Implement governed view and GraphSet operations independent of any backend
    status: pending
  - id: phase-4-budget-pack
    content: Add budget estimation, packable subgraph derivation, and final ContextPack assembly
    status: pending
  - id: phase-5-trace-mutation
    content: Implement ResolutionTrace and ContextMutationProposal handling
    status: pending
  - id: phase-6-backend-adapters
    content: Add graph-facing adapter traits and first Memgraph-oriented integration path
    status: pending
  - id: phase-7-tools-memory
    content: Convert memory recall and tool outputs into structured evidence inputs
    status: pending
  - id: phase-8-session-surface
    content: Thread new artifacts through gateway, channels, and session lifecycle
    status: pending
  - id: phase-9-packaging-rename
    content: Plan packaging layer and only then schedule naming migration
    status: pending
isProject: false
---

# GraphClaw Master Implementation Plan

## Goal

Evolve GraphClaw from the current inherited runtime into the documented Graph Context Engine incrementally, without pretending the target architecture already exists and without doing a rename-first rewrite.

Primary source anchors:

- [README.md](/home/zedium/graphclaw/README.md)
- [CONTEXT.md](/home/zedium/graphclaw/CONTEXT.md)
- [docs/architecture/concepts/graph-context-engine.md](/home/zedium/graphclaw/docs/architecture/concepts/graph-context-engine.md)
- [docs/architecture/migration/future-integration-seams.md](/home/zedium/graphclaw/docs/architecture/migration/future-integration-seams.md)
- [docs/architecture/runtime/turn-runtime-logic.md](/home/zedium/graphclaw/docs/architecture/runtime/turn-runtime-logic.md)
- [src/CONTEXT.md](/home/zedium/graphclaw/src/CONTEXT.md)

## Planning Principles

- Keep current `zeroclaw` technical surfaces operational until replacement seams are real.
- Implement semantics in code in the same order as the docs: artifacts first, orchestration seam second, backend mapping later.
- Treat memory and tools as inputs into context resolution, not as the owner of context semantics.
- Prefer coexistence between inherited and graph-native paths behind stable interfaces.
- Add tests and migration fixtures at each phase before broad adoption.

## Dependency Shape

```mermaid
flowchart LR
baseline[BaselineAndTests] --> artifacts[ArtifactModel]
artifacts --> seam[ContextCreationSeam]
seam --> viewsets[ViewAndGraphSet]
viewsets --> packing[BudgetAndContextPack]
packing --> trace[TraceAndMutation]
trace --> adapters[BackendAdapters]
adapters --> evidence[MemoryAndToolEvidence]
evidence --> sessions[GatewayAndSessionSurface]
sessions --> packaging[PackagingLayer]
packaging --> rename[NamingMigration]
```



## Phase 0: Baseline And Inventory (COMPLETED)

Purpose: freeze a trustworthy picture of the inherited turn pipeline before changing behavior.

### Finding 1: Two Independent Turn Pipelines

The codebase has two separate turn pipelines that share no code paths:


| Pipeline  | Entry                                     | System prompt                                    | Memory                                       | Tool loop              |
| --------- | ----------------------------------------- | ------------------------------------------------ | -------------------------------------------- | ---------------------- |
| **Agent** | `agent.rs:360` `Agent::turn`              | `SystemPromptBuilder` with `PromptSection` trait | `MemoryLoader` trait / `DefaultMemoryLoader` | `ToolDispatcher` trait |
| **Loop**  | `loop_.rs:3252` `process_message` / `run` | `channels::build_system_prompt_with_mode`        | `build_context()` / `build_memory_context()` | `run_tool_call_loop`   |


Both pipelines must be addressed by the ContextCreation seam.

### Finding 2: Context Assembly Points (16 locations)

Context is assembled through string concatenation at 16+ locations across `agent/prompt.rs`, `agent/agent.rs`, `agent/memory_loader.rs`, `agent/dispatcher.rs`, `agent/loop_.rs`, and `channels/mod.rs`. Key duplication:

- **3 memory context builders**: `DefaultMemoryLoader::load_context` (memory_loader.rs:36), `build_context` (loop_.rs:241), `build_memory_context` (channels/mod.rs:1211)
- **2 system prompt builders**: `SystemPromptBuilder` (prompt.rs) vs `build_system_prompt_with_mode` (channels/mod.rs:2440)
- **2 tool instruction builders**: `XmlToolDispatcher::prompt_instructions` (dispatcher.rs:108) vs `build_tool_instructions` (loop_.rs:2727)

### Finding 3: Recommended Branch Point

The enriched-user-message construction point is the best place for the ContextCreation seam:

- **Agent pipeline**: `agent.rs:374-378` where `memory_loader.load_context()` output + timestamp + user message are combined
- **Loop pipeline**: `loop_.rs:3041-3060` where `build_context()` + `build_hardware_context()` + timestamp + user message are combined

Conceptual seam interface:

```rust
#[async_trait]
trait ContextCreation {
    async fn create_user_context(
        &self,
        user_message: &str,
        memory: &dyn Memory,
        options: ContextOptions,
    ) -> Result<String>;
}
```

The inherited implementation wraps current `memory_loader.load_context` / `build_context` + hardware RAG. A graph-native implementation would resolve `GraphSet`s and build a packable subgraph.

### Finding 4: Memory Contract Gaps

`MemoryEntry` has only: `id`, `key`, `content` (String), `category`, `timestamp` (String), `session_id`, `score`. Missing for GraphClaw: provenance, relations, node/edge types, token budget, View/policy ID, set semantics. The `Memory` trait exposes flat `store`/`recall`/`get`/`list`/`forget` with no graph, view, or budget concepts.

6 backends: SQLite (hybrid FTS+vector), Lucid, Postgres, Qdrant, Markdown, None. None has graph capabilities.

### Finding 5: Tool Contract Gaps

`ToolResult` has only: `success` (bool), `output` (String), `error` (Option). Missing: structured payloads, provenance, cost/budget annotations, tool name/call ID. Several tools (memory_recall, content_search, web_search, screenshot) produce internally structured data that gets flattened to text. ~28-35 tools registered via `Vec<Box<dyn Tool>>` with linear name lookup.

### Finding 6: Test Coverage Assessment


| Area                      | Coverage    | Gap Risk                                                           |
| ------------------------- | ----------- | ------------------------------------------------------------------ |
| Agent loop orchestration  | Good        | 60+ unit tests + integration                                       |
| Prompt section building   | Good (unit) | No integration-level prompt content assertion                      |
| Memory loading            | Good        | No integration test for DefaultMemoryLoader output shape in prompt |
| Tool dispatch             | Good        | Covered by integration + robustness tests                          |
| History compaction        | Partial     | Unit only, no integration/system test                              |
| Provider routing          | Partial     | No RouterProvider test                                             |
| Trace fixtures            | Dead        | TraceLlmProvider + verify_expects unused                           |
| Channel-agent integration | Missing     | No combined flow test                                              |


### Finding 7: Pre-Migration Regression Tests Needed

Before any Phase 1 work, these regression tests should be written:

1. Wire `TraceLlmProvider` + `verify_expects()` with existing JSON fixtures (`smoke_greeting.json`, `single_tool_echo.json`, `multi_tool_chain.json`)
2. Assert memory enrichment shapes the prompt section correctly (RecordingProvider + StaticMemoryLoader)
3. Assert history compaction reduces message count while preserving system prompt
4. Assert prompt section order is preserved through RecordingProvider

### Seam Map Summary


| Current Owner      | Key Function                                  | Future Seam                                      | Priority |
| ------------------ | --------------------------------------------- | ------------------------------------------------ | -------- |
| `agent.rs`         | `Agent::turn` (lines 360+)                    | Consume ContextPack instead of raw memory+prompt | Phase 2  |
| `agent.rs`         | `Agent::build_system_prompt` (lines 375-387)  | Accept graph-native context overlay              | Phase 2  |
| `memory_loader.rs` | `DefaultMemoryLoader::load_context` (line 36) | Replace with evidence provider                   | Phase 7  |
| `prompt.rs`        | `SystemPromptBuilder::build` (lines 54-61)    | Pluggable graph-view sections                    | Phase 2  |
| `loop_.rs`         | `build_context` (line 241)                    | Unified ContextCreation strategy                 | Phase 2  |
| `loop_.rs`         | `build_hardware_context` (line 290)           | Unified ContextCreation strategy                 | Phase 2  |
| `loop_.rs`         | `process_message` (line 3252)                 | Route through ContextCreation seam               | Phase 2  |
| `channels/mod.rs`  | `build_system_prompt_with_mode` (line 2440)   | Consume ContextPack sections                     | Phase 8  |
| `channels/mod.rs`  | `build_memory_context` (line 1211)            | Unified ContextCreation strategy                 | Phase 2  |
| `memory/traits.rs` | `Memory` trait (line 56)                      | Add graph/evidence methods                       | Phase 7  |
| `tools/traits.rs`  | `Tool` trait / `ToolResult` (line 5)          | Add structured evidence payload                  | Phase 7  |


### Key Types Reference


| Type                  | File                        | Role                                                                                      |
| --------------------- | --------------------------- | ----------------------------------------------------------------------------------------- |
| `Agent`               | `agent/agent.rs:19`         | Top-level agent; holds provider, tools, memory, prompt_builder, dispatcher, memory_loader |
| `PromptContext`       | `agent/prompt.rs:13`        | Input for PromptSection::build                                                            |
| `PromptSection`       | `agent/prompt.rs:23`        | Trait for prompt sections                                                                 |
| `SystemPromptBuilder` | `agent/prompt.rs:28`        | Aggregates sections into system prompt                                                    |
| `MemoryLoader`        | `agent/memory_loader.rs:6`  | Trait for memory to context string                                                        |
| `DefaultMemoryLoader` | `agent/memory_loader.rs:12` | Uses memory.recall, formats as list                                                       |
| `ToolDispatcher`      | `agent/dispatcher.rs:22`    | Parse response, format results, prompt instructions                                       |
| `MemoryEntry`         | `memory/traits.rs:5`        | id, key, content, category, timestamp, session_id, score                                  |
| `MemoryCategory`      | `memory/traits.rs:29`       | Core, Daily, Conversation, Custom                                                         |
| `Memory`              | `memory/traits.rs:56`       | Core memory trait: store, recall, get, list, forget                                       |
| `ToolResult`          | `tools/traits.rs:5`         | success, output, error                                                                    |
| `ToolSpec`            | `tools/traits.rs:13`        | name, description, parameters                                                             |
| `Tool`                | `tools/traits.rs:21`        | Core tool trait: name, description, parameters_schema, execute                            |


## Phase 1: Executable Artifact Model

Purpose: turn the architecture vocabulary into real Rust types without changing the whole turn engine yet.

Scope:

- Create `src/context/` module (not inside `memory` or `tools`) with these types:
  - `SessionWindow` -- current visibility scope for a turn
  - `GraphSet` -- lazy/materialized logical set with provenance, construction rules, node refs
  - `PackableSubgraph` -- bounded projection derived from GraphSet(s)
  - `ContextPack` -- final budgeted artifact for response generation
  - `ResolutionTrace` -- record of selection/rejection/degradation/packing decisions
  - `ContextMutationProposal` -- structured change request (add, remove, pin, compact, expand, switch-view)
  - `View` -- governed projection descriptor (exposed nodes, hidden, policies, degradation rules)
  - `BudgetEstimate` -- exploration cost, packable cost, final cost
- Each type: `#[derive(Debug, Clone, Serialize, Deserialize)]`, builder/constructor, validation
- Register `pub mod context;` in `src/lib.rs`
- Unit tests covering construction, serialization round-trip, and basic invariants

Files to create:

- `src/context/mod.rs` -- module root
- `src/context/session_window.rs`
- `src/context/graph_set.rs`
- `src/context/packable_subgraph.rs`
- `src/context/context_pack.rs`
- `src/context/resolution_trace.rs`
- `src/context/mutation.rs`
- `src/context/view.rs`
- `src/context/budget.rs`

Files to modify:

- `src/lib.rs` -- add `pub mod context;`

Exit criteria:

- `cargo test -p zeroclaw --lib` passes with new artifact tests
- artifact types are not consumed by agent/memory/tools yet (pure addition)
- docs/architecture glossary terms map 1:1 to code types

## Phase 2: Context Creation Seam

Purpose: separate turn orchestration from context-resolution strategy so inherited and graph-native paths can coexist.

Scope:

- Define `ContextCreation` trait in `src/context/` accepting user message, memory ref, and options; returning enriched context (initially String, later ContextPack)
- Implement `LegacyContextCreation` wrapping current behavior:
  - Agent pipeline: wraps `DefaultMemoryLoader::load_context` (memory_loader.rs:36)
  - Loop pipeline: wraps `build_context` (loop_.rs:241) + `build_hardware_context` (loop_.rs:290)
- Wire `Agent::turn` (agent.rs:374-378) through the seam
- Wire `process_message` (loop_.rs:3041-3060) through the seam
- Wire `run` interactive path (loop_.rs:3162-3180) through the seam
- Consolidate the 3 memory context builders behind the seam
- Add integration tests proving strategy swap (mock ContextCreation vs LegacyContextCreation)

Primary integration points (with line references from audit):

- `agent.rs:374-378` -- enriched user message construction
- `loop_.rs:3041-3060` -- process_message context assembly
- `loop_.rs:3162-3180` -- interactive run context assembly
- `channels/mod.rs:1211-1261` -- build_memory_context (can be deferred to Phase 8)

Exit criteria:

- all existing tests still pass (no behavior change)
- new integration test swaps `LegacyContextCreation` for a mock and verifies the agent loop accepts it
- the 3 memory context builders are behind one trait boundary (channels path may remain separate until Phase 8)

## Phase 3: Views And GraphSets

Purpose: implement governed navigation and selection as first-class runtime behavior.

Scope:

- Add `View` resolution contracts and `GraphSet` operations matching [docs/architecture/concepts/views-and-sets.md](/home/zedium/graphclaw/docs/architecture/concepts/views-and-sets.md).
- Start with backend-agnostic operations: union, intersection, difference, bounded complement, expansion, filtering, ranking, condensation.
- Distinguish lazy from materialized sets.

Exit criteria:

- GraphSet operations are executable and testable without Memgraph
- at least one composed-view path works through the seam

## Phase 4: Budgeting And Final Packing

Purpose: make packability and model-visible context explicit.

Scope:

- Implement cost estimation layers described in [docs/architecture/concepts/context-artifacts.md](/home/zedium/graphclaw/docs/architecture/concepts/context-artifacts.md): exploration cost, packable-subgraph cost, final context cost.
- Build projection from `GraphSet` to packable subgraph to `ContextPack`.
- Replace prompt-string-only assembly with `ContextPack` consumption in [src/agent/prompt.rs](/home/zedium/graphclaw/src/agent/prompt.rs).

Exit criteria:

- final response generation consumes `ContextPack`
- degradation and summarization paths are test-covered

## Phase 5: Trace And Mutation

Purpose: make context decisions explicit and auditable.

Scope:

- Implement `ResolutionTrace` generation for selection, rejection, degradation, and packing.
- Add `ContextMutationProposal` flows for pin, remove, compact, summarize, expand, and switch-view operations.
- Reuse observability precursors where possible instead of duplicating runtime event plumbing.

Likely reuse target:

- [src/observability/runtime_trace.rs](/home/zedium/graphclaw/src/observability/runtime_trace.rs)

Exit criteria:

- routine turns emit trace artifacts
- mutation proposals can be represented and validated even if persistence remains partial

## Phase 6: Backend Adapter Layer

Purpose: add graph-native capability without letting a backend define the domain model.

Scope:

- Define graph-facing traits after artifact and set semantics are stable.
- Add the first Memgraph-oriented adapter following [docs/backends/memgraph.md](/home/zedium/graphclaw/docs/backends/memgraph.md).
- Keep existing memory backends working as legacy providers during transition.

Exit criteria:

- graph-facing traits can power view/set operations behind the same seam
- Memgraph integration remains optional and capability-driven

## Phase 7: Memory And Tool Evidence

Purpose: convert retrieval and tool execution into structured evidence for context resolution.

Scope:

- Evolve memory recall outputs from flat prompt text toward candidate evidence objects.
- Evolve tool outputs from plain text return values toward structured evidence where appropriate.
- Keep user-visible tool behavior stable while adding richer internal representations.

Primary integration points:

- [src/memory/traits.rs](/home/zedium/graphclaw/src/memory/traits.rs)
- [src/agent/memory_loader.rs](/home/zedium/graphclaw/src/agent/memory_loader.rs)
- [src/tools/traits.rs](/home/zedium/graphclaw/src/tools/traits.rs)

Exit criteria:

- memory and selected tools can feed the context engine without direct prompt concatenation
- compatibility adapters exist for older call sites

## Phase 8: Session, Gateway, And Channel Surface

Purpose: thread artifact-aware context behavior through long-running runtime surfaces.

Scope:

- Propagate session identifiers, `SessionWindow`, traces, and packed-context lifecycle through gateway and channel entrypoints.
- Expose only the minimum externally visible metadata required for debugging and governance.

Primary integration points:

- [src/gateway/](/home/zedium/graphclaw/src/gateway)
- [src/channels/](/home/zedium/graphclaw/src/channels)

Exit criteria:

- context artifacts participate in multi-turn runtime flow
- system and integration tests cover persisted session behavior

## Phase 9: Packaging Layer And Naming Migration

Purpose: finish the adjacent architecture only after the context engine is real enough to support it.

Scope:

- Define `AgentPackage`, `AgentInstance`, `AgentSession`, and `Bindings` using the already-stable context-engine boundaries.
- Plan technical rename work only after runtime seams, tests, docs, and compatibility strategy are mature.

Exit criteria:

- packaging model is separated cleanly from context governance
- rename work is scheduled as a later migration program, not mixed into core engine delivery

## Suggested Subplans

This master plan should be split into these subplans:

- **Subplan A**: regression test harness (pre-migration safety net) -- see detailed breakdown below
- **Subplan B**: artifact model (`src/context/` module with all GraphClaw types)
- **Subplan C**: ContextCreation seam (trait, LegacyContextCreation, agent+loop wiring)
- **Subplan D**: view resolution and GraphSet operations (backend-agnostic)
- **Subplan E**: budget estimation, packable subgraph, ContextPack assembly
- **Subplan F**: ResolutionTrace and ContextMutationProposal handling
- **Subplan G**: graph adapter layer with first Memgraph path
- **Subplan H**: memory/tool evidence integration
- **Subplan I**: gateway/session lifecycle integration
- **Subplan J**: packaging model and late rename migration

## Subplan A: Pre-Migration Regression Harness (detailed)

This subplan must be completed before Phase 1 work begins.

### A.1 Wire trace fixture replay tests

- Load `tests/fixtures/traces/smoke_greeting.json` via `TraceLlmProvider`
- Load `tests/fixtures/traces/single_tool_echo.json`
- Load `tests/fixtures/traces/multi_tool_chain.json`
- Call `verify_expects()` for each
- File: `tests/integration/trace_fixtures.rs` (new)
- Wire into `test_integration.rs`

### A.2 Memory enrichment prompt shape test

- Use `RecordingProvider` + `StaticMemoryLoader` with known context
- Run `Agent::turn` with a user message
- Assert provider received user message starting with `[Memory context]` and containing key-value pairs
- Assert timestamp format is present
- File: `tests/integration/agent.rs` (add test)

### A.3 Default memory loader integration format test

- Use `build_agent_with_sqlite_memory()` helper
- Store known entries, then run a turn
- Assert user message in provider request matches expected `- {key}: {content}` format
- File: `tests/integration/agent.rs` (add test)

### A.4 History compaction integration test

- Use `RecordingProvider` with many scripted turns
- Trigger `auto_compact_history` or `trim_history`
- Assert history length reduced; system prompt preserved; recent messages retained
- File: `tests/integration/agent.rs` or `tests/system/` (add test)

### A.5 Prompt section order test

- Use `RecordingProvider`
- Run a turn, capture provider request
- Assert system message contains sections in expected order: identity, tools, skills, workspace, runtime, datetime
- File: `tests/integration/agent.rs` (add test)

### Exit criteria for Subplan A

- All 5 test groups pass
- `cargo test` continues to pass with no regressions
- Dead trace infrastructure (`TraceLlmProvider`, `verify_expects()`) is exercised

## Validation Strategy

- Unit tests for artifact invariants and set operations.
- Component and integration tests for agent-loop seam behavior.
- System tests for multi-turn session flow and backward compatibility.
- Trace fixture replay as regression anchors through all phases.
- Docs remain aligned with runtime reality throughout; target concepts must never be described as already complete before the code lands.

## Immediate Next Step

Execute Subplan A (regression test harness) to establish the safety net, then proceed to Subplan B (artifact model in `src/context/`).