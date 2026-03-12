# Root Context

## Purpose

This repository is GraphClaw: a transitional fork baseline being evolved from ZeroClaw toward a graph-native context engine.

## What Belongs Here

- repo-wide narrative and agent rules;
- top-level entry documents and workspace metadata;
- framing that applies across multiple subtrees.

## What Does Not Belong Here

- low-level subsystem instructions that belong in local `CONTEXT.md` files;
- speculative architecture written as if already implemented.

## Key Files

- `README.md` - canonical GraphClaw root README
- `AGENTS.md` - repo-wide agent contract
- `graph-concept-ref.md` - target architecture framing
- `src/`, `docs/`, `tests/`, `web/`, `python/`, `firmware/` - main working areas

## Current State

The repo still builds and runs through inherited `zeroclaw` technical surfaces. The new context layer exists to make the transition explicit and navigable without forcing a risky rename-first migration.

Do not interpret target-architecture references as proof of implementation. [`graph-concept-ref.md`](graph-concept-ref.md) is a design reference for migration direction, not a statement that the graph-native context engine already exists in the current runtime.

## Root File Map

| File or path | Role |
| --- | --- |
| `README.md` | repo identity, current state, top-level map, and validation entry point |
| `AGENTS.md` | repo-wide agent operating rules and reading order |
| `CONTEXT.md` | root routing map for where to read before changing areas |
| `CONTRIBUTING.md` | contributor workflow, scope control, and validation expectations |
| `docs/README.md` | documentation hub and docs-specific routing |
| `graph-concept-ref.md` | migration framing for the target graph-native context-engine direction |
| `src/`, `crates/`, `web/`, `python/`, `firmware/`, `tests/` | primary implementation surfaces |
| `scripts/`, `dev/`, `.github/` | automation, CI, and repository maintenance surfaces |

## Task Routing

Start at the root, then move inward:

1. Read `README.md` and this file for repository-wide framing.
2. Read the closest local `CONTEXT.md` before changing a subtree.
3. Use the nearest area entry document if the subtree has one.

Use these routes:

| Task | Read next |
| --- | --- |
| root documentation or repo framing | `README.md`, `AGENTS.md`, `CONTRIBUTING.md` |
| documentation trees | `docs/README.md`, `docs/CONTEXT.md` |
| Rust runtime behavior | `src/CONTEXT.md` |
| crate-level work | `crates/CONTEXT.md` |
| web work | `web/CONTEXT.md` |
| Python tooling | `python/CONTEXT.md` |
| firmware | `firmware/CONTEXT.md` |
| tests | `tests/CONTEXT.md` |
| CI, release, or helper scripts | `scripts/CONTEXT.md`, `dev/CONTEXT.md`, `.github/CONTEXT.md` |

## References

- `README.md` for the canonical top-level repository narrative
- `docs/README.md` for documentation navigation
- `CONTRIBUTING.md` for contribution workflow and validation
- `graph-concept-ref.md` for migration framing
- local `CONTEXT.md` files for area-specific expectations

## Migration Track

Use the repository as a progressive migration scaffold, not as a finished graph-native implementation.

The practical sequence is:

1. clarify area boundaries and navigation through local `CONTEXT.md` files;
2. isolate where turn context is assembled in the inherited runtime;
3. introduce explicit runtime artifacts for context selection, packing, and traceability;
4. add graph-facing interfaces behind stable Rust traits;
5. migrate packaging, binding, and portable knowledge surfaces after those seams exist;
6. rename inherited `zeroclaw` technical surfaces only when the implementation has truly crossed the boundary.

When a task touches migration design, read both this file and `graph-concept-ref.md`, then move to the nearest runtime-area `CONTEXT.md`.

## How Agents Should Work Here

Use this file to orient yourself, then move to the nearest local `CONTEXT.md` before changing a specific area.
