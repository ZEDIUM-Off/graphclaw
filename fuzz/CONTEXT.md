# Fuzz Context

## Scope

Cargo-based fuzzing harnesses for robustness and adversarial input coverage.

## File Map

- `Cargo.toml` - fuzz workspace configuration
- `fuzz_targets/` - individual fuzz entrypoints

## Routing

This subtree routes through Cargo fuzz configuration at the root and then into focused targets under `fuzz_targets/`.

## Current State

Fuzz coverage is targeted at inherited parsing and input-handling risk surfaces rather than broad runtime behavior.

## GraphClaw Relevance

Fuzzing helps keep the current baseline safe while GraphClaw evolves; it should expand only where migration work touches real untrusted-input boundaries.

## Cautions

- Use this subtree for adversarial inputs, not generic regression coverage.
- Keep target scope concrete so failures remain actionable.

## Agent Guidance

- Read `fuzz_targets/CONTEXT.md` before editing a specific target.
- Add or update fuzzing only when a real parser, protocol, or validation risk justifies it.
