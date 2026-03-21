# Docs Maintainers Context

## Local Purpose

This subtree contains maintainer-oriented repository maps, documentation inventories, governance notes, and cleanup snapshots used to steer GraphClaw's transition without rewriting implementation surfaces prematurely.

## What Belongs Here

- repository maps and structural inventories;
- documentation inventory and coverage tracking;
- triage snapshots, cleanup candidates, and trademark/governance notes for maintainers.

## File Map

- `repo-map.md` - repository navigation support
- `docs-inventory.md` - documentation inventory snapshot
- `CONTEXT-CONVENTIONS.md` - canonical `CONTEXT.md` maintenance rules
- snapshots, structure, and trademark docs - maintainer governance material

## Routing

- repo maps and inventories stay here
- public docs changes should be routed back to the owning docs subtree
- `CONTEXT.md` maintenance rules come from `CONTEXT-CONVENTIONS.md`

## Agent Workflow

1. Confirm the edit is maintainer-facing inventory or governance material.
2. Verify repository structure and file presence before updating maps or inventories.
3. Mark inherited `zeroclaw` surfaces plainly when they remain active.
4. When folder boundaries or documentation routing change, update the affected `CONTEXT.md` files according to `CONTEXT-CONVENTIONS.md`.
5. Route user-facing documentation changes back to the appropriate public docs subtree.
