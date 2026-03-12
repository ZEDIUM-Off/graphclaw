# Python zeroclaw_tools Context

## Scope

Primary Python package for agent-side support code, CLI entrypoints, integrations, and tool implementations.

## File Map

- `__init__.py` - package surface
- `__main__.py` - CLI/module entrypoint
- `agent.py` - core Python agent support
- `integrations/` - external-service adapters
- `tools/` - Python tool implementations

## Routing

`python -m zeroclaw_tools` enters through `__main__.py`, shared runtime behavior lives in `agent.py`, and then branches into `integrations/` or `tools/` as needed.

## Current State

This package is still the inherited Python surface and intentionally keeps the `zeroclaw_tools` name for compatibility.

## GraphClaw Relevance

The package documents an important migration constraint: GraphClaw can evolve repo framing without breaking stable Python import paths prematurely.

## Cautions

- Do not rename the package or imply that a GraphClaw Python package already exists.
- Keep boundaries between entrypoint, integrations, and tool modules explicit.

## Agent Guidance

- Read the nearest child context before editing a subpackage.
- Favor compatibility-preserving documentation and changes unless the task explicitly targets Python-side API evolution.
