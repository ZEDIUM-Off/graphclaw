# Auth Context

## Purpose

`src/auth/` owns provider authentication flows, token/profile persistence, and shared OAuth helpers.

## File / Folder Map

- `src/auth/mod.rs` - module entry and auth service surface
- `src/auth/openai_oauth.rs` - OpenAI Codex OAuth flow helpers
- `src/auth/gemini_oauth.rs` - Gemini OAuth flow helpers
- `src/auth/anthropic_token.rs` - Anthropic token parsing and auth-kind handling
- `src/auth/oauth_common.rs` - shared OAuth utilities
- `src/auth/profiles.rs` - persisted auth profile data

## Go Here For

- OpenAI or Gemini login flow work: provider-specific OAuth files
- Shared PKCE, callback, or OAuth utility logic: `src/auth/oauth_common.rs`
- Stored auth profiles and active-profile state: `src/auth/profiles.rs`

## Current State

This folder supports inherited provider auth behavior used by the runtime and CLI. It is security-adjacent maintenance work, not a GraphClaw architecture layer.

## GraphClaw Evolution Note

Do not mix provider auth upkeep with graph-native migration claims. GraphClaw still relies on inherited provider credentials and flows here.

## Constraints / Cautions

- Treat tokens, refresh state, and stored profiles as sensitive data.
- Provider differences are real; do not over-generalize them away.
- Avoid renaming established provider/config surfaces unless migration is explicit.

## How Agents Should Work Here

Start with the provider-specific file plus `src/auth/profiles.rs` when behavior touches persistence. Keep secrets handling explicit, preserve backward compatibility for stored auth state, and verify any user-visible auth flow with the narrowest relevant tests.
