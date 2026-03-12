# Component Tests Context

## Scope

Fast Rust tests for narrow runtime behavior, config parsing, schema checks, security rules, and regressions that do not need multi-subsystem orchestration.

## File Map

- `mod.rs` - local module router
- `config_schema.rs`, `config_persistence.rs` - config shape and persistence checks
- `provider_schema.rs`, `provider_resolution.rs` - provider contract coverage
- `security.rs`, `whatsapp_webhook_security.rs` - security-focused cases
- `gateway.rs`, `dockerignore_test.rs` - focused gateway and packaging regressions
- `reply_target_field_regression.rs`, `otel_dependency_feature_regression.rs` - narrow regression guards

## Routing

`tests/test_component.rs` enters this layer, `mod.rs` wires the suite, and each leaf file owns one small concern.

## Current State

This directory is a regression net around inherited runtime details. Several files are intentionally named after past bugs or edge cases rather than broad features.

## GraphClaw Relevance

Use this layer to keep migration-adjacent refactors honest without implying that GraphClaw architecture is already replacing the baseline runtime.

## Cautions

- Keep cases isolated and cheap; if setup becomes orchestration-heavy, move the scenario to `tests/integration/`.
- Do not fold unrelated regressions into a generic test file just to reduce file count.

## Agent Guidance

- Prefer adding or updating a narrow case here before escalating to broader test layers.
- Preserve existing inherited names in assertions, fixtures, and snapshots unless the task explicitly changes those contracts.
