# Changelog

## v0.1.9 - 2026-03-21

GraphClaw alignment release built on a selective uptake of ZeroClaw `v0.5.5` stable.

### Highlights

- align inherited platform surfaces with recent ZeroClaw runtime maturity while keeping GraphClaw as a separate product direction;
- replace the inherited `web/` dashboard baseline with the newer upstream dashboard and keep `ui/` as the dedicated GraphClaw playground surface;
- import stronger CI and release scaffolding, now aligned with `pnpm` for frontend workflows;
- harden gateway, security, runtime, channels, and provider behavior with targeted upstream fixes rather than a full product merge;
- preserve GraphClaw architecture, graph-context, and migration documentation as the canonical framing for the fork.

### Alignment Scope

Based on upstream `zeroclaw` release [`v0.5.5`](https://github.com/zeroclaw-labs/zeroclaw/releases/tag/v0.5.5), with selective GraphClaw-side adoption of:

- CI/CD workflow maturity;
- gateway and WebSocket robustness, including `path_prefix` support and persisted dashboard chat sessions;
- runtime policy and pairing improvements;
- channel improvements, broader transport coverage, and webhook support;
- provider reliability, multimodal support, and stream handling;
- updated inherited dashboard assets under `web/`.

### Functional Baseline Added

- restore the `v0.5.5` gateway session API surface for listing and deleting persisted dashboard sessions;
- restore richer pairing and node-management runtime wiring where it materially contributes to the inherited ZeroClaw operational baseline;
- keep additional provider sources available, including CLI-backed providers, without treating them as native GraphClaw product primitives;
- keep `ui/` as the sovereign GraphClaw playground while the inherited `web/` dashboard remains the operational control surface.

### GraphClaw-Specific Surfaces Preserved

- graph-native architecture and migration documentation under `docs/architecture/`;
- Vue playground application under `ui/`;
- Memgraph-oriented crates and local workflow targets;
- inherited `zeroclaw` identifiers kept where they remain active implementation surfaces.
