# GraphClaw Repository Map

GraphClaw is the repository identity. The current implementation is still the inherited `zeroclaw` runtime baseline, so many commands, package names, module names, and user-facing compatibility surfaces still use `zeroclaw`.

Use this file as a maintainer-facing exploration map:

- to understand how the current runtime is stitched together;
- to identify the first safe seams for GraphClaw migration;
- to avoid rewriting or renaming the repository before those seams exist.

## Current Runtime Flow

```text
message transport / CLI / gateway
        │
        ▼
channels/ or gateway/
        │
        ▼
agent/
  ├─ prompt.rs          static system-prompt assembly
  ├─ memory_loader.rs   recall and context hydration
  ├─ dispatcher.rs      provider/tool dispatch
  └─ loop_.rs           multi-turn tool-calling loop
        │
        ├── providers/       model inference
        ├── tools/           runtime capabilities
        ├── memory/          persistence and retrieval
        ├── security/        policy and approval controls
        ├── observability/   traces and metrics
        └── runtime/         execution adapters
```

## Top-Level Layout

| Path | Role |
| --- | --- |
| `src/` | inherited Rust runtime and the main migration surface |
| `tests/` | component, integration, live, and system validation |
| `web/` | dashboard UI for current gateway/runtime surfaces |
| `python/` | inherited Python tooling and compatibility utilities |
| `firmware/` | board-specific firmware and hardware-side support |
| `crates/robot-kit/` | separate Rust crate for robot/peripheral abstractions |
| `docs/` | public docs, references, ops, contributor material, and migration framing |
| `dev/`, `scripts/`, `.github/` | local dev, CI, release, and automation surfaces |
| `docs/architecture/concepts/graph-context-engine.md` | target GraphClaw context-engine framing, not proof of implementation |

## Runtime Areas That Matter First

| Area | Why it matters | Read first |
| --- | --- | --- |
| `src/agent/` | current turn loop, prompt assembly, dispatch, memory hydration | `src/agent/CONTEXT.md` |
| `src/memory/` | current persistence and retrieval layer, likely graph-adapter seam | `src/memory/CONTEXT.md` |
| `src/tools/` | capability registry and execution surface exposed to the model | `src/tools/CONTEXT.md` |
| `src/runtime/` | execution adapters and capability reporting | `src/runtime/CONTEXT.md` |
| `src/gateway/` | current external API/session boundary for web and automation clients | `src/gateway/CONTEXT.md` |
| `src/config/` | compatibility-sensitive config schema for nearly every subsystem | `src/config/CONTEXT.md` |
| `src/security/` | approval, isolation, and policy constraints that migration must respect | `src/security/CONTEXT.md` |

## Current Dependency Direction

```text
main.rs
  ├─▶ channels/ ─▶ agent/
  ├─▶ gateway/  ─▶ agent/
  ├─▶ daemon/   ─▶ gateway/ + channels/ + cron/ + heartbeat/
  └─▶ service/  ─▶ daemon/runtime lifecycle

agent/
  ├─▶ providers/
  ├─▶ tools/
  ├─▶ memory/
  ├─▶ security/
  ├─▶ observability/
  ├─▶ runtime/
  └─▶ config/
```

The important maintainer reading is not “what can be renamed?” but “where does context currently get assembled, selected, and fed back into the loop?”

## First GraphClaw Migration Seams

These are the safest first seams to explore:

1. `src/agent/prompt.rs`
   Separate static instruction assembly from future dynamic context-pack overlays.
2. `src/agent/memory_loader.rs`
   Replace flat recall hydration gradually with explicit context selection inputs.
3. `src/agent/loop_.rs`
   Introduce first-class runtime artifacts such as `SessionWindow`, `ContextPack`, or `ResolutionTrace` records without replacing the whole loop.
4. `src/memory/traits.rs` and `src/memory/backend.rs`
   Add a graph-facing storage boundary behind traits instead of deleting existing backends.
5. `src/tools/traits.rs` and `src/tools/mod.rs`
   Evolve capability metadata and structured execution output without breaking current tool contracts.
6. `src/gateway/api.rs`, `sse.rs`, and `ws.rs`
   Add future GraphClaw-facing runtime/session endpoints only after internal seams exist.
7. `src/config/schema.rs`
   Additive GraphClaw config should be layered in without renaming current `zeroclaw` compatibility surfaces first.

## What Must Stay Stable Early

- current `zeroclaw` CLI command tree and active config keys
- provider, tool, and gateway contracts used by existing clients
- current persistence and recall behavior unless migration work explicitly changes it
- security, approval, and sandbox expectations
- existing web dashboard compatibility with the gateway

## Recommended Exploration Order

For implementation planning, the shortest useful path is:

1. `README.md`
2. `CONTEXT.md`
3. `docs/architecture/concepts/graph-context-engine.md`
4. `src/CONTEXT.md`
5. `src/agent/CONTEXT.md`
6. `src/memory/CONTEXT.md`
7. `src/tools/CONTEXT.md`
8. `src/runtime/CONTEXT.md`
9. `src/gateway/CONTEXT.md`
10. `src/config/CONTEXT.md`

Then read the owning code files named by those local contexts.

## Current Technical Reality

The repository still exposes inherited technical surfaces such as:

- `Cargo.toml` package name `zeroclaw`
- CLI commands under `zeroclaw ...`
- Python package `zeroclaw-tools` / `zeroclaw_tools`
- user/workspace/config paths and environment variables under inherited names

These are intentionally left in place until migration is grounded in stable boundaries rather than branding-only changes.

## Set/View Vocabulary Pivot (Revision v0.1)

The architecture documentation now distinguishes:

- **`Set`**: persisted, governed, composable object stored in the graph database;
- **`View`**: runtime-only, transient exploration surface;
- **`ResolvedSet`**: derived resolution artifact, not the canonical definition.

This vocabulary pivot is documented in [`docs/architecture/concepts/views-and-sets.md`](../architecture/views-and-sets.md), [`docs/architecture/playground/set-system-spec-v0.md`](../architecture/set-system-spec-v0.md), and routed through [`docs/architecture/concepts/glossary.md`](../architecture/glossary.md).

The current code (`crates/views/`, `src/gateway/playground.rs`, `web/src/`) still uses the pre-revision `ViewTemplate` / `ResolvedView` / `ViewsService` naming. A future code migration plan will align those types. Until then, treat the code-level `View*` names as implementation aliases for the `Set` concepts defined in the architecture docs.

Key implementation seams this revision will eventually touch:

- `crates/views/` -- type renames and composition invariant enforcement;
- `src/gateway/playground.rs` -- route and DTO renames;
- `web/src/` -- TypeScript type and component renames;
- `memgraph/` -- schema additions for `(:Set)` nodes and composition relations;
- `src/agent/` and `src/memory/` -- future graph adapter seams that will consume `Set` definitions.

None of these code changes are part of the documentation revision itself.

## Maintainer Rule

If a document, PR, or implementation change claims that GraphClaw already has a graph-native context engine, a package protocol, or a graph-native control plane, verify the actual owning code path first. In this repository phase, navigation and seam definition should advance before broad renames or architectural claims.
