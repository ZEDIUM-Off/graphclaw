# Python zeroclaw_tools Context

## Local Purpose

Primary Python package for agent-side support code, CLI entrypoints, integrations, and tool implementations.

This subtree owns the inherited Python package surface. It is maintained for compatibility and support, not as the leading edge of the GraphClaw architecture.

## What Belongs Here

- Python entrypoints and package wiring;
- Python-specific integrations and tool implementations;
- compatibility-preserving package documentation.

## What Does Not Belong Here

- premature package renaming;
- canonical GraphClaw concept definitions;
- Rust runtime ownership or transport contracts.

## File Map

- `__init__.py` - package surface
- `__main__.py` - CLI/module entrypoint
- `agent.py` - core Python agent support
- `integrations/` - external-service adapters
- `tools/` - Python tool implementations

## Routing

`python -m zeroclaw_tools` enters through `__main__.py`, shared runtime behavior lives in `agent.py`, and then branches into `integrations/` or `tools/` as needed.

- integration-specific work belongs in `integrations/`
- Python tool behavior belongs in `tools/`
- repo-wide GraphClaw concept framing belongs in `docs/architecture/`

## Current State

This package is still the inherited Python surface and intentionally keeps the `zeroclaw_tools` name for compatibility.

## GraphClaw Relevance

The package documents an important migration constraint: GraphClaw can evolve repo framing without breaking stable Python import paths prematurely.

## References

- `python/CONTEXT.md` - parent compatibility boundary
- `docs/architecture/graph-context-engine.md` - target architecture framing kept separate from Python compatibility reality

## Cautions

- Do not rename the package or imply that a GraphClaw Python package already exists.
- Keep boundaries between entrypoint, integrations, and tool modules explicit.
- Do not document Python helper behavior as if it already implements GraphClaw `AgentPackage` or context-engine contracts unless that is explicitly built.

## Agent Guidance

- Read the nearest child context before editing a subpackage.
- Favor compatibility-preserving documentation and changes unless the task explicitly targets Python-side API evolution.
