# Web Types Context

## Scope

Shared TypeScript contract definitions for frontend API usage.

## File Map

- `api.ts` - typed request/response shapes used across pages, hooks, and transport helpers

## Routing

This subtree feeds types outward into `lib/`, `hooks/`, and page components. It should stay dependency-light and contract-focused.

## Current State

The type surface is small and tracks the inherited gateway API rather than a GraphClaw-native contract layer.

## GraphClaw Relevance

Typed contracts are one of the clearest migration seams, so accuracy matters more than branding here while the repo still depends on inherited backend names and shapes.

## Cautions

- Do not add speculative future API types just to prepare for migration.
- Keep types as the source of truth for shared contracts, not a dumping ground for view-only aliases.

## Agent Guidance

- Update these types when the real backend contract changes.
- Preserve strict alignment with current runtime payloads and naming until an explicit migration task says otherwise.
