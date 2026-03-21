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

