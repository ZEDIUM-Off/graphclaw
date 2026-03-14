# GraphClaw Architecture Docs

This subtree documents the stable concept model for GraphClaw.

Use it when the question is not "what does the inherited runtime do today?" but rather "what meanings and invariants are we trying to stabilize before implementation hardens?"

## Start Here

- Graph Context Engine reference: [`graph-context-engine.md`](graph-context-engine.md)
- transition framing from the inherited runtime: [`zero-to-graphclaw-transition.md`](zero-to-graphclaw-transition.md)
- views, set semantics, and packability: [`views-and-sets.md`](views-and-sets.md)
- context artifacts and budgeting: [`context-artifacts.md`](context-artifacts.md)
- logical turn phases and current insertion points: [`turn-runtime-logic.md`](turn-runtime-logic.md)
- future integration seams and interface families: [`future-integration-seams.md`](future-integration-seams.md)
- shared terminology: [`glossary.md`](glossary.md)

## Scope

This subtree is for:

- stable definitions such as `View`, `GraphSet`, `SessionWindow`, `ThinkingContext`, and `ContextPack`;
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
5. [`glossary.md`](glossary.md) for compact term definitions shared across the repo.

## Architecture Map

| Document | Primary question |
| --- | --- |
| `graph-context-engine.md` | what target model is GraphClaw trying to stabilize |
| `zero-to-graphclaw-transition.md` | how does the inherited runtime migrate without a rewrite-first strategy |
| `views-and-sets.md` | how should views, `GraphSet` objects, and packability work conceptually |
| `context-artifacts.md` | which context artifacts are distinct and how do budget concerns relate to them |
| `turn-runtime-logic.md` | how should a turn resolve logically and where does the current runtime fit |
| `future-integration-seams.md` | which interface families and runtime seams should emerge next |
| `glossary.md` | what concise definitions must stay stable across docs |
