# Robot Kit Context

## Scope

Standalone workspace crate for robot/peripheral abstractions and hardware-facing behavior helpers.

## File Map

- `Cargo.toml` - crate manifest
- `robot.toml` - crate-local configuration
- `README.md`, `PI5_SETUP.md`, `SOUL.md` - crate references and operating notes
- `src/lib.rs` - crate root
- `src/traits.rs`, `src/safety.rs`, `src/config.rs` - shared boundaries and safety/config layers
- `src/drive.rs`, `src/look.rs`, `src/listen.rs`, `src/speak.rs`, `src/sense.rs`, `src/emote.rs` - capability modules
- `src/tests.rs` - crate-local tests

## Routing

Cargo enters through `src/lib.rs`, which fans into capability modules and shared boundary files such as `traits.rs` and `safety.rs`.

## Current State

`robot-kit` is inherited, domain-specific, and intentionally separate from the main GraphClaw application path.

## GraphClaw Relevance

This crate is adjacent to GraphClaw evolution rather than central to the current context-engine migration. Its main relevance is as a reminder not to blur hardware abstractions with repo-wide architectural experiments.

## Cautions

- Keep hardware safety and interface boundaries explicit.
- Do not use this crate as a landing zone for unrelated migration work.

## Agent Guidance

- Respect the crate's local domain vocabulary and safety concerns.
- If you change boundaries here, update the local references so future agents can navigate the crate correctly.
