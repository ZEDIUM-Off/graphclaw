# Docs Context

## Local Purpose

This subtree is the primary human-facing documentation surface for GraphClaw. It covers the current inherited `zeroclaw` runtime, operator and contributor workflows, and the migration framing that explains how the repository is evolving toward a graph-native context engine.

## What Belongs Here

- documentation for users, operators, contributors, and maintainers;
- reference material for current CLI, API, security, setup, and SOP surfaces;
- explicit migration framing that distinguishes inherited behavior from target direction;
- localized documentation and translation governance.

## What Does Not Belong Here

- source-level implementation notes that belong next to code;
- speculative architecture written as if the migration is already complete;
- workflow or CI behavior changes hidden inside documentation-only edits.

## File Map

- `README.md` - primary docs entrypoint and navigation hub
- `contributing/` - contributor process, CI, review, release, and docs workflow material
- `hardware/` - board, peripheral, and device setup/design notes
- `i18n/` - translation structure and localized documentation governance
- `maintainers/` - repo maps, inventories, and transition-oriented maintainer notes
- `ops/` - operational runbooks and troubleshooting for the current runtime
- `reference/` - exact CLI, API, and SOP lookup material
- `security/` - security guidance, hardening notes, and roadmap material
- `setup-guides/` - installation and service-specific setup guides
- `vi/` - inherited Vietnamese documentation subtree retained from the baseline

## Routing

- contributor workflow or review policy changes belong in `docs/contributing/`
- operator runbooks and runtime troubleshooting belong in `docs/ops/`
- exact command, config, or interface references belong in `docs/reference/`
- security posture and roadmap material belong in `docs/security/`
- localized doc governance belongs in `docs/i18n/`
- translation-heavy legacy Vietnamese content belongs in `docs/vi/`

## References

- `CONTEXT.md` - repo-wide migration and documentation truthfulness baseline
- `README.md` - canonical docs landing page
- `docs/maintainers/repo-map.md` - repository navigation support for maintainers
- `docs/maintainers/docs-inventory.md` - documentation inventory snapshot

## Current Inherited State

Most content in this tree is still inherited from ZeroClaw-era documentation. Many pages accurately describe `zeroclaw` names, commands, and workflows because those surfaces still exist in the current implementation. That is not stale branding by itself; it is current repository truth until the underlying product behavior changes.

## GraphClaw Migration Relationship

This tree should make the migration legible without pretending it is finished. Documentation may introduce GraphClaw framing, explain the graph-native direction, and call out inherited terminology, but it must not silently rewrite current commands, config keys, APIs, or operating procedures into future-state names.

## Cautions

- keep current behavior and roadmap statements clearly separated
- avoid taxonomy-wide rewrites unless the task is explicitly broad
- do not remove localized or inherited docs just because the root framing changed
- if `docs/README.md` conflicts with local truth, resolve that deliberately rather than by distorting subtree context

## Agent Workflow

1. Read this file, then the nearest deeper `CONTEXT.md` before editing a docs area.
2. Identify whether the target page is current-state reference, migration framing, or roadmap material.
3. Make the smallest truthful change that improves clarity.
4. Preserve inherited `zeroclaw` terminology where it still matches the implementation.
5. Run documentation validation appropriate to the scope of the change before finishing.
