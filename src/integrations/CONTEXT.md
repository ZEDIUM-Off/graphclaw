# Integrations Context

## Purpose

`src/integrations/` is the lightweight registry layer for optional runtime integrations.

## File / Folder Map

- `src/integrations/mod.rs` - command handling and integration entrypoints
- `src/integrations/registry.rs` - integration registry data and lookup

## Go Here For

- Integration lookup or metadata: `src/integrations/registry.rs`
- CLI/runtime entrypoints for integration info: `src/integrations/mod.rs`

## Current State

This folder is still a thin inherited registry boundary, not a general home for every external connection.

## GraphClaw Evolution Note

If GraphClaw later gains richer integration metadata, add it deliberately. Do not treat this small registry as a completed graph-native catalog.

## Constraints / Cautions

- Resist turning this into a catch-all module.
- Keep registry data authoritative and easy to inspect.
- Separate listing/metadata concerns from runtime behavior.

## How Agents Should Work Here

Keep the boundary narrow. Update registry data and callers together, avoid hiding integration-specific logic here, and document any new routing expectations in the local context if the scope grows.
