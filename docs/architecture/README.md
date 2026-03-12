# GraphClaw Architecture Docs

This subtree documents the stable concept model for GraphClaw.

Use it when the question is not "what does the inherited runtime do today?" but rather "what meanings and invariants are we trying to stabilize before implementation hardens?"

## Start Here

- Graph Context Engine reference: [`graph-context-engine.md`](graph-context-engine.md)
- shared terminology: [`glossary.md`](glossary.md)

## Scope

This subtree is for:

- stable definitions such as `View`, `GraphSet`, `SessionWindow`, `ThinkingContext`, and `ContextPack`;
- global invariants for context resolution;
- logical turn-phase descriptions that stay useful even if the code moves.

This subtree is not for:

- source-level implementation walkthroughs;
- backend-specific procedure catalogs;
- process or contributor workflow guidance.
