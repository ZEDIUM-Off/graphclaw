# Docs Architecture Context

## Local Purpose

This subtree holds the stable conceptual architecture for GraphClaw. It defines the target vocabulary, invariants, and logical runtime model that should remain meaningful even while the inherited implementation changes.

## What Belongs Here

- Graph Context Engine concepts and invariants;
- stable definitions for shared vocabulary;
- architecture-level references that explain the target model without claiming it is already implemented.

## What Does Not Belong Here

- source-level implementation notes that belong next to code;
- backend-specific operational detail that belongs in `docs/backends/`;
- speculative code plans written as if already approved runtime work.

## File Map

- `README.md` - entrypoint for conceptual architecture docs
- `graph-context-engine.md` - reference model for the Graph Context Engine
- `zero-to-graphclaw-transition.md` - migration framing from inherited runtime to interfacable context engine
- `views-and-sets.md` - operational semantics for `View`, `GraphSet`, and packability
- `context-artifacts.md` - distinctions between context artifacts and budget layers
- `turn-runtime-logic.md` - logical turn phases and current runtime mapping
- `future-integration-seams.md` - future interface families and likely seam placement
- `glossary.md` - shared terminology and concise definitions

## Routing

- concept definitions and invariants belong here
- transition-thesis and seam-framing docs belong here when they stay above source-level ownership
- artifact and turn-logic references belong here when they must remain stable even if code moves
- backend capability mapping belongs in `docs/backends/`
- repo and subtree ownership boundaries belong in the nearest `CONTEXT.md` files

## References

- `docs/README.md` - top-level docs routing
- `README.md` - repo identity and migration framing
- `AGENTS.md` - repo-wide rules and vocabulary expectations

## Current Inherited State

The current runtime still uses inherited `zeroclaw` technical surfaces. This subtree does not override that truth. It explains the target architecture the repo is moving toward.

## GraphClaw Migration Relationship

This area should make the migration legible before the runtime is fully reworked. It exists to stabilize meaning and reduce architecture drift across docs, code discussions, and future implementation seams.

## Cautions

- do not describe target concepts as if they already exist in runtime code
- do not let backend details redefine GraphClaw business concepts
- do not duplicate subtree boundary guidance that belongs in local `CONTEXT.md` files

## Agent Workflow

1. Read this file before editing conceptual architecture docs in this subtree.
2. Check whether the task is about stable meaning, transition framing, runtime logic, or backend support.
3. Keep definitions backend-agnostic unless a backend reference is explicitly required.
4. Update linked routing docs when a new architecture reference is added here.
