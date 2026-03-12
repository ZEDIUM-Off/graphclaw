# Scripts Context

## Scope

Operational helper scripts for validation and release support.

## File Map

- `ci/` - validation and reporting gates
- `release/` - release-tag support

## Routing

Top-level script organization is by workflow: CI-related entrypoints live in `ci/`, release flow lives in `release/`, and callers should treat those subtrees as the source of truth for automation behavior.

## Current State

This directory supports inherited repo operations and should remain utility-oriented rather than absorbing product logic.

## GraphClaw Relevance

Scripts are part of migration discipline because they enforce how documentation and code are checked while the repo evolves.

## Cautions

- Do not add ad hoc one-off experiments here unless they are becoming reusable workflow tools.
- Keep script behavior explicit and discoverable; hidden automation is hard to trust during migration.

## Agent Guidance

- Read the nearest child context before changing CI or release tooling.
- If a script alters validation policy or release flow, update the relevant local docs in the same pass.
