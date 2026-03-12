# Health Context

## Purpose

`src/health/` defines runtime health reporting primitives and lightweight health-state modeling.

## File / Folder Map

- `src/health/mod.rs` - health types and entrypoints

## Go Here For

- Shared health representations: `src/health/mod.rs`
- Inputs consumed by doctor or daemon health views: `src/health/mod.rs`

## Current State

This is a small inherited support layer that other subsystems use when they need to expose status.

## GraphClaw Evolution Note

Do not portray this as a full GraphClaw health graph. It remains a compact status abstraction for the inherited runtime.

## Constraints / Cautions

- Health state should be objective and low-noise.
- Avoid mixing diagnostic storytelling into core health primitives.
- Keep the interface small and reusable.

## How Agents Should Work Here

Read all current consumers before changing types or semantics. Prefer additive health signals over breaking changes, and keep the boundary between raw health data and higher-level diagnostics clear.
