# Providers Context

## Purpose

`src/providers/` contains model-provider integrations, provider routing, retries, compatibility adapters, and provider traits.

## File / Folder Map

- `src/providers/mod.rs` - provider registry and public entrypoints
- `src/providers/traits.rs` - core provider contracts
- `src/providers/router.rs` - provider selection and routing logic
- `src/providers/reliable.rs` - retry/reliability wrapper behavior
- `src/providers/compatible.rs` - compatibility adapter helpers
- `src/providers/openai.rs`, `anthropic.rs`, `gemini.rs`, `openrouter.rs`, `ollama.rs` - major provider implementations

## Go Here For

- Provider interface changes: `src/providers/traits.rs`
- Routing decisions: `src/providers/router.rs`
- Retry/fallback behavior: `src/providers/reliable.rs`
- Provider-specific API bugs: the matching provider file
- OpenAI-compatible endpoint handling: `src/providers/compatible.rs`

## Current State

This is a major inherited extension surface for model access. It remains provider-centric infrastructure rather than GraphClaw-specific reasoning machinery.

## GraphClaw Evolution Note

Future GraphClaw work may ask providers for richer context-aware behavior, but this folder currently implements ordinary provider adapters and routing.

## Constraints / Cautions

- Provider contracts are wide and user-visible.
- Retry/routing bugs can look like model or tool failures elsewhere.
- Keep provider maintenance separate from context-engine ambitions.

## How Agents Should Work Here

Read `traits.rs`, the concrete provider file, and any router/reliability wrapper involved in the path you are changing. Preserve compatibility, avoid cross-provider rewrites unless necessary, and document any config or API contract change.
