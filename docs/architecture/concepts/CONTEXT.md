# Architecture Concepts Context

## Scope

Canonical concept sources, concept-governance rules, terminology routing, and concept-maturity tracking for GraphClaw.

## What Belongs Here

- canonical definitions;
- concept invariants;
- terminology routing;
- concept maturity and stability tracking;
- cross-concept framing that remains above source-level implementation.

## File Map

- `definition-governance.md` - single-definition policy and canonical registry
- `graph-context-engine.md` - top-level engine framing
- `views-and-sets.md` - routing hub for the `Set` / `View` / packability family
- `set.md` - canonical `Set` semantics
- `resolved-set.md` - canonical `ResolvedSet` semantics
- `view.md` - canonical `View` semantics
- `session-frame.md` - canonical `SessionFrame` semantics
- `packability.md` - canonical packability family semantics
- `graph-governed-agentics.md` - routing hub for graph theory, GoT, projection governance, and mono-agent loop framing
- `projection-governance.md` - `ProjectionRegistry`, `NLProjection`, and projectability rules
- `context-frame.md` - canonical `ContextFrame` semantics for invocation-oriented context composition
- `got.md` - mono-agent Graph-of-Thought framing
- `agent-loop.md` - mono-agent loop framing
- `context-artifacts.md` - artifact and planning distinctions
- `glossary.md` - terminology routing index
- `conceptual-maturity-tracker.md` - maturity map

## Routing

- if the task is about what a concept means, start here;
- if the task is about where a concept should live canonically, start with `definition-governance.md`;
- if the task is about Sets, Views, or packability, start with `views-and-sets.md`, then move to `set.md`, `view.md`, or `packability.md`.
- if the task is about graph-theory grounding, `ProjectionRegistry`, `ContextFrame`, GoT, or mono-agent loop orchestration, start with `graph-governed-agentics.md`, then move to `projection-governance.md`, `context-frame.md`, `got.md`, or `agent-loop.md`.
