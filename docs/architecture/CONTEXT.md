# Docs Architecture Context

## Local Purpose

This subtree holds the stable conceptual architecture for GraphClaw. It defines the target vocabulary, invariants, and logical runtime model that should remain meaningful even while the inherited implementation changes.

## What Belongs Here

- Graph Context Engine concepts and invariants;
- canonical-definition governance and concept-source routing;
- Graph Engine framing as governed context resolution plus strategy resolution;
- canonical concept sources and shared terminology routing;
- variable strategy families for reflection, exploration, packing, and orchestration;
- explicit runtime planning and trace artifacts such as task intent, strategy resolution, and bounded execution plans;
- architecture-level references that explain the target model without claiming it is already implemented.

## What Does Not Belong Here

- source-level implementation notes that belong next to code;
- backend-specific operational detail that belongs in `docs/backends/`;
- speculative code plans written as if already approved runtime work.

## File Map

- `README.md` - entrypoint for conceptual architecture docs
- `concepts/` - canonical concept sources, governance, terminology routing, and maturity tracking
- `migration/` - transition framing and future seam placement
- `interfaces/` - migration-facing interface fiches
- `runtime/` - logical runtime and turn-phase references
- `playground/` - bounded playground specifications and redirects
- `snapshots/` - dated architecture alignment material

## Routing

- concept definitions, terminology routing, and maturity tracking belong in `concepts/`
- transition-thesis and seam-framing docs belong in `migration/`
- interface fiches for first migration seams belong in `interfaces/`
- logical runtime and turn-flow references belong in `runtime/`
- bounded playground specifications belong in `playground/`
- dated alignment notes belong in `snapshots/`
- backend capability mapping belongs in `docs/backends/`
- repo and subtree ownership boundaries belong in the nearest `CONTEXT.md` files

## Mermaid Convention

Architecture docs in this subtree use Mermaid only to clarify routing, concept boundaries, or migration seams.

- keep each diagram scoped to one purpose;
- use solid arrows for reading routes, conceptual dependency, or current documentation structure;
- use dotted arrows for future seam placement or coexistence targets that are not yet implemented;
- include explicit `current`, `target`, or `future` wording where omission could blur repository truth.

These diagrams must not imply that the target Graph Context Engine already exists in runtime code.

## Routing Diagram

Use this map to pick the next document from an architecture question.

```mermaid
flowchart LR
    Q[Architecture question]
    A["concepts/ - Canonical concepts and routing"]
    B["migration/ - Seam-first migration"]
    C["runtime/ - Turn logic"]
    D["interfaces/ - Interface fiches"]
    E["playground/ - Bounded v0 specs"]
    F["snapshots/ - Dated notes"]

    Q --> A
    Q --> B
    Q --> C
    Q --> D
    Q --> E
    Q --> F
    A --> B
    A --> C
    A --> D
    A --> E
    B --> C
    B --> D
```

## References

- `docs/README.md` - top-level docs routing
- `README.md` - repo identity and migration framing
- `AGENTS.md` - repo-wide rules and vocabulary expectations

## Current Inherited State

The current runtime still uses inherited `zeroclaw` technical surfaces. This subtree does not override that truth. It explains the target architecture the repo is moving toward, including variable strategy selection and modular orchestration as migration-facing concepts rather than code facts.

## GraphClaw Migration Relationship

This area should make the migration legible before the runtime is fully reworked. It exists to stabilize meaning and reduce architecture drift across docs, code discussions, and future implementation seams.

## Cautions

- do not describe target concepts as if they already exist in runtime code
- do not let backend details redefine GraphClaw business concepts
- do not collapse declarative strategy definitions into hidden runtime behavior when documenting target seams
- do not duplicate subtree boundary guidance that belongs in local `CONTEXT.md` files

## Agent Workflow

1. Read this file before editing conceptual architecture docs in this subtree.
2. Move to the nearest child `CONTEXT.md` before editing inside `concepts/`, `migration/`, `interfaces/`, `runtime/`, `playground/`, or `snapshots/`.
3. Keep definitions backend-agnostic unless a backend reference is explicitly required.
4. Update linked routing docs when a new architecture reference is added here or when new first-class concept families change the expected reading path.
