# GraphClaw Backend References

This subtree explains how concrete backends can support the GraphClaw model.

Use it after reading the conceptual architecture docs. Backend pages should inherit GraphClaw semantics from `docs/architecture/` rather than define them locally.

## Start Here

- Memgraph backend reference: [`memgraph.md`](memgraph.md)

## Scope

This subtree is for:

- capability mapping between GraphClaw concepts and backend primitives;
- fallback and degradation rules;
- backend-specific coupling warnings.

This subtree is not for:

- redefining `View`, `GraphSet`, `ThinkingContext`, or `ContextPack`;
- implementation walkthroughs of source adapters;
- general operator runbooks.
