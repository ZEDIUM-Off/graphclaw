# GraphClaw Architecture Docs

This subtree documents the stable concept model for GraphClaw.

Use it when the question is not "what does the inherited runtime do today?" but rather "what meanings and invariants are we trying to stabilize before implementation hardens?"

## Start Here

- Graph Context Engine reference: [`graph-context-engine.md`](graph-context-engine.md)
- transition framing from the inherited runtime: [`zero-to-graphclaw-transition.md`](zero-to-graphclaw-transition.md)
- views, set semantics, and packability: [`views-and-sets.md`](views-and-sets.md)
- View System v0 (lifecycle, algebra, LLM export) for the playground: [`view-system-spec-v0.md`](view-system-spec-v0.md)
- context artifacts, planning artifacts, and budgeting: [`context-artifacts.md`](context-artifacts.md)
- logical turn phases, strategy resolution, current insertion points, and cross-cutting sequential paths (current vs future): [`turn-runtime-logic.md`](turn-runtime-logic.md)
- future integration seams, strategy seams, and interface families: [`future-integration-seams.md`](future-integration-seams.md)
- interface fiches for likely first seams: [`session-window-interface.md`](session-window-interface.md), [`context-pack-interface.md`](context-pack-interface.md), [`strategy-resolver-interface.md`](strategy-resolver-interface.md), [`graph-context-store-and-retriever-interface.md`](graph-context-store-and-retriever-interface.md), [`mutation-guard-interface.md`](mutation-guard-interface.md), [`orchestration-policies-interface.md`](orchestration-policies-interface.md), [`hook-bus-interface.md`](hook-bus-interface.md)
- shared terminology: [`glossary.md`](glossary.md)

## Mermaid Convention

Architecture diagrams in this subtree are orientation aids, not implementation claims.

- use one Mermaid diagram per question or document purpose;
- label nodes as `current inherited runtime`, `target concept`, or `future seam` when that distinction matters;
- use solid arrows for reading order, conceptual structure, or current routing;
- use dotted arrows for migration adjacency, future seam placement, or coexistence targets that are not yet implemented.

If a diagram could be misread as a code-level runtime graph, add a short sentence that restates its status.

## Orientation Diagram

Use this map to choose the right architecture reference before reading in detail.

```mermaid
flowchart TD
    A["graph-context-engine.md - Target concept model"]
    B["zero-to-graphclaw-transition.md - Seam-first migration"]
    C["views-and-sets.md - View and GraphSet semantics"]
    D["context-artifacts.md - Artifact boundaries and budgets"]
    E["turn-runtime-logic.md - Logical turn phases and inherited runtime mapping"]
    F["future-integration-seams.md - Future interface families"]
    G["session-window-interface.md - Window source-of-truth seam"]
    H["context-pack-interface.md - Final packed context seam"]
    I["strategy-resolver-interface.md - Strategy selection seam"]
    J["graph-context-store-and-retriever-interface.md - Context supply seam"]
    K["mutation-guard-interface.md - Context edit validation seam"]
    L["orchestration-policies-interface.md - Routing and spawn seams"]
    M["hook-bus-interface.md - Lifecycle event seam"]
    N["glossary.md - Stable terminology"]

    A --> B
    A --> C
    A --> D
    B --> E
    D --> E
    B --> F
    D --> G
    D --> H
    E --> I
    F --> I
    F --> J
    D --> J
    G --> K
    H --> K
    F --> L
    I --> L
    L --> M
    H --> M
    A --> N
```

## Scope

This subtree is for:

- stable definitions such as `View`, `GraphSet`, `SessionWindow`, `ThinkingContext`, and `ContextPack`;
- strategy families for reflection, exploration, packing, and orchestration as target-architecture concepts;
- explicit planning and trace artifacts such as `TaskIntent`, `StrategyResolution`, `ReflectionPlan`, `ExplorationPlan`, and `OrchestrationPlan`;
- framing for the Graph Engine as governed context resolution plus strategy resolution, rather than only retrieval;
- global invariants for context resolution;
- migration framing that explains how the inherited runtime can gain graph-native seams progressively;
- logical turn-phase descriptions that stay useful even if the code moves;
- documentation of future interface families without freezing class signatures too early.

This subtree is not for:

- source-level implementation walkthroughs;
- backend-specific procedure catalogs;
- process or contributor workflow guidance.

## Recommended Reading Order

1. [`graph-context-engine.md`](graph-context-engine.md) for the top-level reference frame.
2. [`zero-to-graphclaw-transition.md`](zero-to-graphclaw-transition.md) for the migration thesis and coexistence model.
3. [`views-and-sets.md`](views-and-sets.md) and [`context-artifacts.md`](context-artifacts.md) for operational concept detail.
4. [`turn-runtime-logic.md`](turn-runtime-logic.md) and [`future-integration-seams.md`](future-integration-seams.md) when the task touches runtime boundaries or future interface placement.
5. The interface fiches when the task needs a concrete first seam: [`session-window-interface.md`](session-window-interface.md), [`context-pack-interface.md`](context-pack-interface.md), [`strategy-resolver-interface.md`](strategy-resolver-interface.md), [`graph-context-store-and-retriever-interface.md`](graph-context-store-and-retriever-interface.md), [`mutation-guard-interface.md`](mutation-guard-interface.md), [`orchestration-policies-interface.md`](orchestration-policies-interface.md), and [`hook-bus-interface.md`](hook-bus-interface.md).
6. [`glossary.md`](glossary.md) for compact term definitions shared across the repo.

## Architecture Map

| Document | Primary question |
| --- | --- |
| `graph-context-engine.md` | what target model is GraphClaw trying to stabilize, including governed strategy choice |
| `zero-to-graphclaw-transition.md` | how does the inherited runtime migrate without a rewrite-first strategy |
| `views-and-sets.md` | how should views, `GraphSet` objects, and packability work conceptually |
| `view-system-spec-v0.md` | View System v0: lifecycle, composition algebra, LLM export (playground slice) |
| `context-artifacts.md` | which context and planning artifacts are distinct and how do budget concerns relate to them |
| `turn-runtime-logic.md` | how should a turn resolve logically, including strategy resolution, where does the current runtime fit, and how do gateway/agent/memory/tools/providers/runtime/security articulate in current vs future paths |
| `future-integration-seams.md` | which interface families, orchestration seams, and runtime seams should emerge next |
| `session-window-interface.md` | what should become the governed runtime source of truth for visible context |
| `context-pack-interface.md` | what should become the final model-visible packed context artifact |
| `strategy-resolver-interface.md` | how should turn-time strategy choice become an explicit runtime seam |
| `graph-context-store-and-retriever-interface.md` | how should graph-aware and memory-aware context supply be separated from higher-level context governance |
| `mutation-guard-interface.md` | how should visible-context edits be validated, rejected, or degraded before state changes |
| `orchestration-policies-interface.md` | how should routing, spawn, bounded sub-agent runtime, aggregation, and hooks become explicit seams |
| `hook-bus-interface.md` | how should lifecycle events become observable without owning orchestration or packing policy |
| `glossary.md` | what concise definitions must stay stable across docs |
