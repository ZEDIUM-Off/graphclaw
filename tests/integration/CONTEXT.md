# Integration Tests Context

## Scope

Cross-module Rust tests for behaviors that span agent flow, routing, hooks, and memory lifecycle concerns.

## File Map

- `mod.rs` - local suite router
- `agent.rs`, `agent_robustness.rs` - multi-part agent behavior coverage
- `channel_routing.rs` - channel dispatch behavior
- `hooks.rs` - hook pipeline interactions
- `memory_restart.rs`, `memory_comparison.rs` - persistence and memory semantics
- `telegram_attachment_fallback.rs` - integration-specific fallback scenario

## Routing

`tests/test_integration.rs` enters this subtree, then `mod.rs` exposes the focused integration scenarios above.

## Current State

This layer validates how inherited subsystems cooperate today, including behavior that would be awkward or misleading to express as a unit-style test.

## GraphClaw Relevance

As GraphClaw evolves toward a graph-native context engine, this directory is where boundary changes should prove that current runtime wiring still behaves truthfully during the transition.

## Cautions

- Keep scenarios integration-sized, not end-to-end replicas of the whole stack.
- If a case depends on real providers or credentials, it belongs in `tests/live/`, not here.

## Agent Guidance

- Reach for this layer when component tests cannot express the real contract between modules.
- Keep setups explicit so future migration work can see which boundaries are still inherited.
