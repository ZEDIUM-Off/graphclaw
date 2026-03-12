# Docs Maintainers Context

## Local Purpose

This subtree contains maintainer-oriented repository maps, documentation inventories, governance notes, and cleanup snapshots used to steer GraphClaw's transition without rewriting implementation surfaces prematurely.

## What Belongs Here

- repository maps and structural inventories;
- documentation inventory and coverage tracking;
- triage snapshots, cleanup candidates, and trademark/governance notes for maintainers.

## What Does Not Belong Here

- end-user setup or runtime instructions;
- exact API, CLI, or SOP reference that belongs under `docs/reference/`;
- speculative migration execution written as if approved and complete.

## File Map

- `README.md` - maintainer docs entrypoint
- `repo-map.md` - repository navigation support
- `docs-inventory.md` - current documentation inventory
- `i18n-coverage.md` - localization coverage tracking
- `refactor-candidates.md` - candidate cleanup and refactor list
- `project-triage-snapshot-2026-02-18.md` - point-in-time repository assessment
- `structure-README.md` and `trademark.md` - structure and naming guidance

## Routing

- structural inventories and navigation aids belong here
- contributor process belongs in `docs/contributing/`
- implementation-level migration work belongs with source or product docs, not here
- repo-wide framing conflicts should be coordinated with top-level docs and root context

## References

- `docs/CONTEXT.md` - overall docs routing
- `CONTEXT.md` - repo-wide migration framing
- `AGENTS.md` - repo-wide agent expectations

## Current Inherited State

This area is especially useful during the GraphClaw transition because the repo still carries substantial inherited ZeroClaw structure. Maintainer docs can describe that reality directly without pretending migration is further along than it is.

## GraphClaw Migration Relationship

This subtree is where maintainers document the migration boundary, inventory inherited surfaces, and record cleanup candidates. It supports migration planning and navigation, but it should not be used to silently redefine product truth.

## Cautions

- separate inventory facts from proposed future changes
- keep snapshots date-bounded when they are time-sensitive
- avoid using maintainer notes as a substitute for updating canonical docs after real changes land

## Agent Workflow

1. Confirm the edit is maintainer-facing inventory or governance material.
2. Verify repository structure and file presence before updating maps or inventories.
3. Mark inherited `zeroclaw` surfaces plainly when they remain active.
4. Route user-facing documentation changes back to the appropriate public docs subtree.
