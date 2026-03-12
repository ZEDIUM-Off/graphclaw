# Cron Context

## Purpose

`src/cron/` handles scheduled tasks: schedule definitions, persistence, and execution timing.

## File / Folder Map

- `src/cron/mod.rs` - command handling and module entry
- `src/cron/scheduler.rs` - runtime scheduling engine
- `src/cron/store.rs` - persistence for jobs and run state
- `src/cron/schedule.rs` - schedule parsing and helpers
- `src/cron/types.rs` - shared cron task types

## Go Here For

- Scheduler timing or execution loop: `src/cron/scheduler.rs`
- Stored jobs or persistence format: `src/cron/store.rs`
- Expression parsing or schedule conversion: `src/cron/schedule.rs`
- Public cron task models: `src/cron/types.rs`

## Current State

This is inherited automation infrastructure used by the CLI and long-running runtime. It may later support richer workflow graphs, but it is still a conventional scheduler today.

## GraphClaw Evolution Note

Do not describe cron as already being a graph-native workflow engine. Any future integration should be framed as an extension of the existing scheduler.

## Constraints / Cautions

- Scheduling semantics are user-facing and easy to regress.
- Persistence compatibility matters for existing tasks.
- Timezone and one-shot vs recurring behavior need explicit handling.

## How Agents Should Work Here

Read the scheduler, store, and types together before changing behavior. Prefer tests-first changes for schedule semantics, keep persistence migrations explicit, and call out any timing behavior that users will notice.
