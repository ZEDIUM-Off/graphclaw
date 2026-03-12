# Tests Context

## Scope

Top-level Rust test harness layout for the inherited runtime: thin entry files at this level route into the layer-specific subtrees below.

## File Map

- `test_component.rs`, `test_integration.rs`, `test_live.rs`, `test_system.rs` - top-level suite entrypoints
- `component/`, `integration/`, `live/`, `system/` - behavior layers
- `support/` - shared mocks, helpers, and assertions
- `fixtures/` - static test assets and trace samples
- `manual/` - human-run validation scripts and notes

## Routing

Top-level `test_*.rs` files dispatch into `mod.rs` files inside each layer, then into focused case files such as `component/gateway.rs` or `integration/channel_routing.rs`.

## Current State

This tree still validates inherited `zeroclaw` behavior and naming. It is the main place where GraphClaw's TDD discipline should be expressed when runtime behavior changes.

## GraphClaw Relevance

The GraphClaw migration is not a rename-first effort. Tests here should clarify current guarantees while the context-engine direction is still being scaffolded.

## Cautions

- Do not rename inherited test modules just to match GraphClaw branding.
- Keep deterministic layers ahead of `live/` and `manual/` whenever the same risk can be covered more cheaply.

## Agent Guidance

- Read the nearest child `CONTEXT.md` before editing a specific layer.
- For behavioral work, decide which test layer should fail first; for documentation-only work, run doc audits instead of adding product tests.
