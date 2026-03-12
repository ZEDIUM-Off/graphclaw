# Firmware Context

## Scope

Hardware-facing programs, board experiments, and bridge helpers that live beside the main runtime.

## File Map

- `arduino/` - minimal Arduino sketch
- `esp32/` - Rust ESP32 firmware and setup docs
- `esp32-ui/` - Rust plus Slint UI firmware
- `nucleo/` - Rust Nucleo board firmware
- `uno-q-bridge/` - hybrid sketch plus Python bridge workflow

## Routing

Each child directory is effectively its own hardware entrypoint with board-local build files, source, and setup instructions.

## Current State

This subtree remains inherited, specialized, and intentionally separate from the main GraphClaw code path. It is kept intact as hardware support rather than treated as a migration centerpiece.

## GraphClaw Relevance

Firmware is relevant mainly as an adjacent boundary: GraphClaw repo framing can evolve while hardware support remains stable and board-specific.

## Cautions

- Do not perform repo-wide naming or architecture cleanup here unless a hardware task explicitly needs it.
- Board setup is fragile; prefer small local changes over cross-tree abstraction.

## Agent Guidance

- Read the nearest board-level `CONTEXT.md` before editing a hardware subtree.
- Keep firmware work localized, explicit about toolchains, and separate from main runtime migration claims.
