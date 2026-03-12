# Nucleo Firmware Context

## Scope

Rust firmware for the Nucleo board target.

## File Map

- `Cargo.toml`, `Cargo.lock` - board-local crate metadata
- `src/main.rs` - firmware entrypoint

## Routing

This subtree is a small Cargo-driven firmware crate that routes directly into `src/main.rs`.

## Current State

The area is compact but tightly coupled to real hardware behavior and flashing workflows.

## GraphClaw Relevance

Nucleo support is peripheral to GraphClaw's current migration work and should remain described as such.

## Cautions

- Treat changes here as hardware work first.
- Avoid broad documentation or naming cleanup that is unrelated to the board.

## Agent Guidance

- Keep edits small and explicit.
- If hardware assumptions change, document them locally rather than in repo-wide docs.
