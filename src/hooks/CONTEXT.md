# Hooks Context

## Purpose

`src/hooks/` defines the hook system: hook traits, runner logic, and the boundary between runtime events and hook implementations.

## File / Folder Map

- `src/hooks/mod.rs` - hook module entry
- `src/hooks/traits.rs` - hook contracts and shared interfaces
- `src/hooks/runner.rs` - hook execution and coordination
- `src/hooks/builtin/` - built-in hook implementations shipped with the runtime

## Go Here For

- Hook interface changes: `src/hooks/traits.rs`
- Hook execution ordering or dispatch: `src/hooks/runner.rs`
- Built-in shipped hooks: `src/hooks/builtin/`

## Current State

This is inherited extension plumbing for the runtime. It is useful for controlled extensibility but not a replacement for missing architecture.

## GraphClaw Evolution Note

If GraphClaw grows new context seams here later, document them as seams. Do not pretend hooks already implement the graph-native engine.

## Constraints / Cautions

- Keep contracts explicit so hook behavior stays auditable.
- Avoid turning hooks into a catch-all for unrelated features.
- Hook ordering and failure handling are compatibility-sensitive.

## How Agents Should Work Here

Read `traits.rs`, `runner.rs`, and the affected implementation together. Preserve clear boundaries, prefer small composable hooks, and document event timing when it matters to operators or tool authors.
