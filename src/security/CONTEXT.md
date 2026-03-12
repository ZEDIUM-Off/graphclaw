# Security Context

## Purpose

`src/security/` handles runtime safety controls: policy enforcement, secret handling, sandbox helpers, audit paths, prompt guardrails, pairing, OTP, and emergency stop controls.

## File / Folder Map

- `src/security/mod.rs` - module entry and shared exports
- `src/security/policy.rs` - policy decisions and enforcement helpers
- `src/security/secrets.rs` - secret storage and redaction helpers
- `src/security/audit.rs` - audit logging support
- `src/security/pairing.rs`, `otp.rs`, `estop.rs` - operator-control security flows
- `src/security/landlock.rs`, `bubblewrap.rs`, `firejail.rs`, `docker.rs` - sandbox/environment helpers
- `src/security/prompt_guard.rs`, `detect.rs`, `domain_matcher.rs`, `leak_detector.rs` - content and risk checks

## Go Here For

- Policy or permission logic: `src/security/policy.rs`
- Secret storage/redaction: `src/security/secrets.rs`
- Audit output: `src/security/audit.rs`
- Pairing, OTP, or estop behavior: matching control file
- Sandbox integration details: the relevant backend helper file

## Current State

This is one of the highest-risk inherited runtime areas. It contains real safety controls used by the current system, not just future-facing placeholders.

## GraphClaw Evolution Note

Do not describe this folder as if GraphClaw already ships a new graph-native policy engine. Current safety behavior is implemented through inherited runtime controls here.

## Constraints / Cautions

- Do not weaken protections casually.
- Security changes usually require tests, docs, and careful threat reasoning.
- Keep policy, storage, auditing, and sandbox concerns distinct.

## How Agents Should Work Here

Read the exact control path end to end before editing. Favor explicit reasoning, preserve secure defaults, add or update tests for behavior changes, and call out residual risk if a change cannot be fully verified.
