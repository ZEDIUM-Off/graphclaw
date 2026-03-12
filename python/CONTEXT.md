# Python Context

## Scope

Inherited Python companion package, packaging metadata, and its small test surface.

## File Map

- `pyproject.toml` - Python package metadata
- `README.md`, `README.vi.md` - package-facing docs
- `zeroclaw_tools/` - importable package and CLI entrypoint
- `tests/` - Python-side validation

## Routing

Packaging flows through `pyproject.toml`, runtime entrypoints route into `zeroclaw_tools/__main__.py` and `zeroclaw_tools/agent.py`, and behavior checks live in `tests/test_tools.py`.

## Current State

This subtree is a compatibility layer around inherited `zeroclaw_tools` naming and APIs. It is intentionally not being rebranded in this documentation pass.

## GraphClaw Relevance

Python remains adjacent to the GraphClaw migration rather than its center. The key job here is to document how the compatibility surface fits the current repo without pretending the package has already migrated.

## Cautions

- Do not rename modules, import paths, or package metadata just to match GraphClaw branding.
- Keep documentation honest about the package's inherited identity and current scope.

## Agent Guidance

- Read the nearest child `CONTEXT.md` before editing the package or its tests.
- Treat this subtree as a stable compatibility surface unless a task explicitly targets Python migration.
