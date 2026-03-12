# Python Context

## Local Purpose

Inherited Python companion package, packaging metadata, and its small test surface.

This subtree owns a compatibility-oriented Python surface for the repository. It remains adjacent to GraphClaw migration rather than a central implementation surface.

## What Belongs Here

- Python packaging metadata and package-level docs;
- the inherited `zeroclaw_tools` compatibility package;
- Python-side tests for that compatibility surface.

## What Does Not Belong Here

- premature rebranding of imports or package names;
- canonical GraphClaw concept definitions that belong in `docs/architecture/`;
- Rust runtime ownership that belongs under `src/`.

## File Map

- `pyproject.toml` - Python package metadata
- `README.md`, `README.vi.md` - package-facing docs
- `zeroclaw_tools/` - importable package and CLI entrypoint
- `tests/` - Python-side validation

## Routing

Packaging flows through `pyproject.toml`, runtime entrypoints route into `zeroclaw_tools/__main__.py` and `zeroclaw_tools/agent.py`, and behavior checks live in `tests/test_tools.py`.

- package behavior belongs in `zeroclaw_tools/`
- Python validation belongs in `python/tests/`
- migration framing belongs in root and docs architecture references

## Current State

This subtree is a compatibility layer around inherited `zeroclaw_tools` naming and APIs. It is intentionally not being rebranded in this documentation pass.

## Current Dependency Direction

- Routes packaging and Python entrypoints through `pyproject.toml` and `zeroclaw_tools/`.
- Depends on the inherited Python compatibility surface rather than the Rust GraphClaw migration seams.
- Stays downstream of repo-wide framing and should only adopt new terminology when it matches real package behavior.

## GraphClaw Relevance

Python remains adjacent to the GraphClaw migration rather than its center. The key job here is to document how the compatibility surface fits the current repo without pretending the package has already migrated.

Today, this subtree contributes by preserving stable compatibility while the main architectural reinterpretation happens elsewhere.

## References

- `python/zeroclaw_tools/CONTEXT.md` - package-level boundary
- `README.md` - repo identity and migration framing
- `docs/architecture/glossary.md` - stable vocabulary for cross-repo documentation

## Cautions

- Do not rename modules, import paths, or package metadata just to match GraphClaw branding.
- Keep documentation honest about the package's inherited identity and current scope.
- Do not imply that Python already exposes a GraphClaw-native package, context engine, or packaging protocol.

## Agent Guidance

- Read the nearest child `CONTEXT.md` before editing the package or its tests.
- Treat this subtree as a stable compatibility surface unless a task explicitly targets Python migration.
