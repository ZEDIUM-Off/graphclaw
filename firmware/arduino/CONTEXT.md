# Arduino Firmware Context

## Scope

Minimal Arduino-side sketch support.

## File Map

- `arduino.ino` - single sketch entrypoint

## Routing

This directory routes directly into one sketch file and has no broader framework layer.

## Current State

The subtree is intentionally tiny and example-like.

## GraphClaw Relevance

Its relevance is mostly documentary: it shows that some inherited hardware support remains simple and separate from the main GraphClaw evolution track.

## Cautions

- Treat this as board-local code, not a template for all firmware work.
- Avoid adding repo-wide abstractions here.

## Agent Guidance

- Keep edits minimal and hardware-specific.
- Document any new board assumptions inline or in a nearby README if this subtree grows.
