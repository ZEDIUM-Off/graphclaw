# Python Integrations Context

## Scope

Python adapters for external services that sit on top of the core package behavior.

## File Map

- `__init__.py` - subpackage surface
- `discord_bot.py` - current Discord integration entry

## Routing

Integration-specific code lives here and should depend on shared package behavior from the parent `zeroclaw_tools` package rather than owning core agent logic itself.

## Dependency Map

```mermaid
flowchart LR
    Parent[parent package runtime] --> Surface[__init__.py]
    Surface --> Discord[discord_bot.py]
    Discord --> Service[external service]
```

## Current State

This subtree is small and currently centered on a single integration path.

## GraphClaw Relevance

It shows how inherited ecosystem integrations are preserved while GraphClaw repo framing changes around them.

## Cautions

- Keep service-specific glue local to this directory.
- Do not move generic helpers here just because an integration happens to use them first.

## Agent Guidance

- Add new external adapters here, not in `tools/`.
- Document service assumptions and dependencies explicitly when this subtree grows.
