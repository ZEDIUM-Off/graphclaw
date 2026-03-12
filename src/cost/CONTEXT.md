# Cost Context

## Purpose

`src/cost/` tracks usage accounting and cost-related types used by the runtime.

## File / Folder Map

- `src/cost/mod.rs` - module entry
- `src/cost/tracker.rs` - accounting logic and tracking flow
- `src/cost/types.rs` - shared cost data types

## Go Here For

- Usage accounting behavior: `src/cost/tracker.rs`
- Shared cost structs/enums: `src/cost/types.rs`

## Current State

This is a supporting inherited subsystem used by other runtime areas. It is operationally useful but not an architectural center of GraphClaw migration.

## GraphClaw Evolution Note

Future graph-oriented planning may depend on better cost modeling, but that capability is not already implemented here.

## Constraints / Cautions

- Silent accounting errors are hard to notice and hard to trust.
- Keep units and aggregation rules explicit.
- Avoid hiding billing logic inside unrelated provider code.

## How Agents Should Work Here

Follow the data flow from producer to tracker before editing. Keep calculations obvious, add focused tests for changed accounting rules, and note any user-visible quota or budget impact.
