# System Tests Context

## Scope

Broad end-to-end Rust tests for full-stack runtime behavior.

## File Map

- `mod.rs` - local suite router
- `full_stack.rs` - current system-level coverage

## Routing

`tests/test_system.rs` enters this subtree. The expectation here is whole-system confidence, not fine-grained diagnosis.

## Current State

System coverage is intentionally sparse and high-cost compared with component or integration tests.

## GraphClaw Relevance

As GraphClaw grows beyond the inherited baseline, this directory is where repo-wide behavior changes should eventually prove they still work together as one runtime.

## Cautions

- Do not use system tests as the first or only layer for routine changes.
- Keep scenarios few, representative, and stable enough to justify their runtime cost.

## Agent Guidance

- Add coverage here only when the risk is truly full-stack.
- Push smaller concerns down to `component/` or `integration/` so failures stay easier to localize.
