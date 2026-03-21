# GraphClaw Backend References

This subtree explains how concrete backends can support the GraphClaw model.

Use it after reading the conceptual architecture docs. Backend pages should inherit GraphClaw semantics from `docs/architecture/` rather than define them locally.

## Start Here

- Graph Context Engine reference: [`../architecture/graph-context-engine.md`](../architecture/graph-context-engine.md)
- transition framing and seams: [`../architecture/zero-to-graphclaw-transition.md`](../architecture/zero-to-graphclaw-transition.md)
- set/view/packability semantics: [`../architecture/concepts/views-and-sets.md`](../architecture/concepts/views-and-sets.md), [`../architecture/concepts/set.md`](../architecture/concepts/set.md), [`../architecture/concepts/view.md`](../architecture/concepts/view.md), [`../architecture/concepts/packability.md`](../architecture/concepts/packability.md)
- context artifacts and budget layers: [`../architecture/context-artifacts.md`](../architecture/context-artifacts.md)
- Memgraph backend reference: [`memgraph.md`](memgraph.md)

## Scope

This subtree is for:

- capability mapping between GraphClaw concepts and backend primitives;
- fallback and degradation rules;
- backend-specific coupling warnings.

This subtree is not for:

- redefining `View`, `ThinkingContext`, or `ContextPack`;
- implementation walkthroughs of source adapters;
- general operator runbooks.
