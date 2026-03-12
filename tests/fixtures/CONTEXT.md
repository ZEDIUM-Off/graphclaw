# Test Fixtures Context

## Scope

Static assets and canned traces consumed by automated tests.

## File Map

- `test_document.pdf` - document fixture for file/tool flows
- `traces/multi_tool_chain.json` - multi-step tool-call sample
- `traces/single_tool_echo.json` - minimal tool trace
- `traces/smoke_greeting.json` - simple conversational trace

## Routing

Test files reference these fixtures by path; this directory should stay data-only and should not grow helper logic.

## Current State

The fixture set is intentionally small and biased toward deterministic repository checks rather than broad media coverage.

## GraphClaw Relevance

These assets help preserve baseline behavior while GraphClaw documentation and context scaffolding evolve around the inherited runtime.

## Cautions

- Keep fixtures tiny, reproducible, and scrubbed of secrets.
- Avoid adding redundant near-duplicates when an existing trace or sample already covers the same surface.

## Agent Guidance

- Reuse an existing fixture before adding a new one.
- If a new fixture becomes necessary, document who consumes it and keep the filename descriptive and stable.
