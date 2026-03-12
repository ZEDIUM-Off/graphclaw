# Manual Tests Context

## Scope

Human-run validation scripts and notes for flows that are awkward, expensive, or not yet worth automating.

## File Map

- `test_dockerignore.sh` - manual packaging check
- `telegram/testing-telegram.md` - operator notes
- `telegram/quick_test.sh`, `telegram/test_telegram_integration.sh` - manual Telegram helpers
- `telegram/generate_test_messages.py` - sample message generation support

## Routing

This subtree is procedure-oriented: markdown explains the flow, shell scripts execute steps, and small helper scripts prepare inputs.

## Current State

Manual coverage here supports inherited operational surfaces and messaging flows. It complements, but does not replace, automated tests.

## GraphClaw Relevance

During GraphClaw evolution, this directory helps document how existing user-facing paths are still verified before deeper migration work lands.

## Cautions

- Do not let repeatable procedures stay manual forever if they can be automated safely.
- Keep prerequisites, secrets, and environment assumptions obvious in the script or note that uses them.

## Agent Guidance

- Store human-run procedures here, not in `tests/integration/`.
- When adding a manual step, say why it is still manual and what automation layer would eventually own it.
