# Learnings

## [LRN-20260321-001] best_practice

**Logged**: 2026-03-21T00:00:00+01:00
**Priority**: high
**Status**: pending
**Area**: docs

### Summary
When a documentation task changes routing or concept boundaries, update the affected `CONTEXT.md` files as part of the same task.

### Details
During the GraphClaw documentation restructuring, concept files were split into finer canonical sources and hubs were reduced to routing roles. The follow-up correction was that this kind of work must also maintain local and parent `CONTEXT.md` files according to the canonical conventions in `docs/maintainers/CONTEXT-CONVENTIONS.md`, otherwise repository routing drifts behind the actual documentation structure.

### Suggested Action
Whenever a task moves, splits, or re-routes canonical docs, update the nearest `CONTEXT.md` files in the same change and keep them as routing-only documents.

### Metadata
- Source: user_feedback
- Related Files: .agents/skills/self-improvement/SKILL.md, docs/maintainers/CONTEXT.md, docs/maintainers/CONTEXT-CONVENTIONS.md
- Tags: context, docs, routing, best-practice
- Pattern-Key: docs.context-routing-sync
- Recurrence-Count: 1
- First-Seen: 2026-03-21
- Last-Seen: 2026-03-21

---

## [LRN-20260321-002] correction

**Logged**: 2026-03-21T00:00:00+01:00
**Priority**: high
**Status**: pending
**Area**: config

### Summary
When importing upstream frontend or CI files into GraphClaw, preserve the repo package-manager convention and do not reintroduce `npm`/`package-lock` flows if the repo standard is `pnpm`.

### Details
During selective upstream integration of the inherited `web/` dashboard, upstream files brought in `package-lock.json`, `npm ci`, and `npm run build` references. The user clarified that GraphClaw uses `pnpm` as its package manager. The correct integration pattern is to adapt imported frontend and workflow files to `pnpm` immediately rather than carrying upstream package-manager assumptions forward.

### Suggested Action
For future upstream imports touching frontend or CI files, check the local package-manager convention first and normalize lockfiles, workflow commands, and build comments in the same pass.

### Metadata
- Source: user_feedback
- Related Files: web/package.json, web/pnpm-lock.yaml, .github/workflows/checks-on-pr.yml, .github/workflows/ci-run.yml, .github/workflows/cross-platform-build-manual.yml, Makefile
- Tags: correction, pnpm, ci, frontend, upstream-sync
- Pattern-Key: integration.package-manager-alignment
- Recurrence-Count: 1
- First-Seen: 2026-03-21
- Last-Seen: 2026-03-21

---
