# Peripherals Context

## Purpose

`src/peripherals/` manages attached boards and device-specific helpers for flashing, serial access, GPIO-style capabilities, and peripheral registration.

## File / Folder Map

- `src/peripherals/mod.rs` - peripheral command handling and module entry
- `src/peripherals/traits.rs` - peripheral contracts
- `src/peripherals/serial.rs` - serial transport helpers
- `src/peripherals/rpi.rs` - Raspberry Pi support
- `src/peripherals/nucleo_flash.rs`, `arduino_flash.rs`, `arduino_upload.rs` - flashing flows
- `src/peripherals/uno_q_bridge.rs`, `uno_q_setup.rs` - Uno Q support
- `src/peripherals/capabilities_tool.rs` - capability exposure helpers

## Go Here For

- Peripheral lifecycle or command handling: `src/peripherals/mod.rs`
- Shared peripheral interface changes: `src/peripherals/traits.rs`
- Serial transport behavior: `src/peripherals/serial.rs`
- Flashing workflows: the matching flash/upload file
- Uno Q support: `src/peripherals/uno_q_bridge.rs` or `src/peripherals/uno_q_setup.rs`

## Current State

This is inherited runtime-to-device plumbing for supported hardware boards and their capabilities.

## GraphClaw Evolution Note

Do not present peripherals as already being modeled as a graph-native hardware layer. The current code is concrete device integration and transport management.

## Constraints / Cautions

- Hardware failures are expensive to debug and can be device-specific.
- Keep transport assumptions local to the implementation that needs them.
- User-visible flashing/setup flows must remain clear and safe.

## How Agents Should Work Here

Identify the exact board, transport, or flashing path first. Read the relevant implementation plus `traits.rs`, keep hardware-specific code localized, and avoid broad refactors unless multiple devices truly share the same contract.
