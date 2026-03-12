# ESP32 Firmware Context

## Scope

Rust-based ESP32 firmware plus its board-specific setup and toolchain configuration.

## File Map

- `Cargo.toml`, `Cargo.lock` - firmware crate metadata
- `src/main.rs` - runtime entrypoint
- `build.rs` - build-time glue
- `README.md`, `SETUP.md` - operator and setup references
- `rust-toolchain.toml`, `.cargo/config.toml` - toolchain routing

## Routing

Build and flash flow starts with Cargo and the ESP32 toolchain files, then enters `src/main.rs` as the firmware entrypoint.

## Current State

This is a self-contained inherited hardware workspace with real setup friction and strong board assumptions.

## GraphClaw Relevance

ESP32 support is adjacent to GraphClaw's evolution, but it should not be used to imply that hardware integration is part of the current context-engine migration step.

## Cautions

- Toolchain and flashing setup are fragile; avoid casual cleanup.
- Keep documentation and code aligned with the actual ESP32 workflow rather than repo-wide narratives.

## Agent Guidance

- Treat this as a board-local Rust project.
- If a change touches setup, update the local docs and keep validation tied to real hardware expectations.
