# Doctor Context

## Purpose

`src/doctor/` provides operator-facing diagnostics and status inspection for runtime health, trace data, and model availability.

## File / Folder Map

- `src/doctor/mod.rs` - doctor command entrypoints and diagnostic reporting

## Go Here For

- Default doctor output: `src/doctor/mod.rs`
- Model diagnostic flow: `src/doctor/mod.rs`
- Runtime trace inspection: `src/doctor/mod.rs` with `src/observability/runtime_trace.rs`

## Current State

This is an inherited diagnostics surface used to explain the current runtime honestly. It is not a substitute for broader observability architecture.

## GraphClaw Evolution Note

Keep diagnostics truthful about today's inherited behavior; do not describe future GraphClaw capabilities as already available.

## Constraints / Cautions

- Diagnostic output should reduce confusion, not add narrative guesswork.
- False positives and false reassurance are both harmful.
- Changes can affect support workflows and documentation.

## How Agents Should Work Here

Trace each doctor output back to its data source before editing. Prefer explicit, operator-readable reporting, and verify any new diagnostic path against the real subsystem it claims to inspect.
