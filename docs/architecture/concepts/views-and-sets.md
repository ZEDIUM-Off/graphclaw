# Sets, Views, And Packability

## Status

This file is now a routing hub for the `Set` / `View` family.

It does not carry the canonical definition of every concept in that family anymore. Canonical definitions now live in concept-specific files so the repository can keep one stable source per concept.

## Read Here First

- `Set`: [`set.md`](set.md)
- `ResolvedSet`: [`resolved-set.md`](resolved-set.md)
- `View`: [`view.md`](view.md)
- packable subgraph, packability, bounded complement, condensation, and projection into a packable subgraph: [`packability.md`](packability.md)

## Why This Hub Exists

`Set`, `ResolvedSet`, `View`, and packability remain tightly related.

This hub keeps the family legible without forcing every reader to start from a single oversized page. It should be used to route quickly, not to redefine the linked concepts.

## Reading Order

1. Read [`set.md`](set.md) for the base governed set abstraction and its persisted representation.
2. Read [`resolved-set.md`](resolved-set.md) for the derived artifact produced by resolution.
3. Read [`view.md`](view.md) for the runtime working-subgraph concept.
4. Read [`packability.md`](packability.md) for packable subgraphs, packability conditions, and the relation to `ContextPack`.

## Related Docs

- engine boundary and concept routing: [`graph-context-engine.md`](graph-context-engine.md)
- context artifacts and budget layers: [`context-artifacts.md`](context-artifacts.md)
- graph theory grounding, projection governance, GoT, and loop framing: [`graph-governed-agentics.md`](graph-governed-agentics.md), [`projection-governance.md`](projection-governance.md), [`got.md`](got.md), [`agent-loop.md`](agent-loop.md)
- playground slice: [`../playground/set-system-spec-v0.md`](../playground/set-system-spec-v0.md)
