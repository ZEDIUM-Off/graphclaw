# GraphClaw

GraphClaw is the repository identity.

This repository currently runs on an inherited ZeroClaw baseline. Many technical surfaces still use `zeroclaw` names across crates, binaries, CLI commands, configuration, tests, manifests, and firmware. Those names remain the current implementation reality until an explicit migration task changes them.

GraphClaw should therefore be read as a transitional scaffold: a working inherited runtime being reorganized and documented so it can evolve toward a graph-native context engine without a rename-first rewrite.

[`graph-concept-ref.md`](graph-concept-ref.md) is the migration framing document for that direction. It describes the target architecture and vocabulary to steer toward. It does not mean that the target architecture is already implemented in this repository.

## Read First

Use this order before making changes:

1. [`AGENTS.md`](AGENTS.md) for repository-wide rules.
2. [`CONTEXT.md`](CONTEXT.md) for the root map and task routing.
3. The nearest local `CONTEXT.md` for the area you will edit.
4. [`docs/README.md`](docs/README.md) or [`CONTRIBUTING.md`](CONTRIBUTING.md) if the task is documentation, review, or process related.

## Task Routing

Use these entry points to orient work quickly:

| If you are working on... | Read first |
| --- | --- |
| repo identity, root framing, agent rules | [`README.md`](README.md), [`AGENTS.md`](AGENTS.md), [`CONTEXT.md`](CONTEXT.md) |
| Rust runtime behavior | [`src/CONTEXT.md`](src/CONTEXT.md) |
| workspace crates | [`crates/CONTEXT.md`](crates/CONTEXT.md) |
| web UI | [`web/CONTEXT.md`](web/CONTEXT.md) |
| Python tooling | [`python/CONTEXT.md`](python/CONTEXT.md) |
| firmware or board-side code | [`firmware/CONTEXT.md`](firmware/CONTEXT.md) |
| tests | [`tests/CONTEXT.md`](tests/CONTEXT.md) |
| scripts or CI helpers | [`scripts/CONTEXT.md`](scripts/CONTEXT.md), [`dev/CONTEXT.md`](dev/CONTEXT.md) |
| documentation trees | [`docs/README.md`](docs/README.md), [`docs/CONTEXT.md`](docs/CONTEXT.md) |

Always move from the root framing to the closest local `CONTEXT.md` before editing a specific area.

## Repository Map

| Path | Purpose |
| --- | --- |
| `src/` | inherited Rust runtime, services, integrations, and subsystem modules |
| `crates/` | workspace member crates and supporting libraries |
| `web/` | frontend and dashboard surfaces |
| `python/` | inherited Python tooling and integrations |
| `firmware/` | hardware-side experiments and board support |
| `tests/` | component, integration, live, manual, and system test areas |
| `docs/` | documentation hub, references, contributor guides, and operations material |
| `scripts/` | CI, release, and maintenance scripts |
| `dev/` | local development and CI helper surfaces |
| `.github/` | workflow automation and repository policy metadata |
| `graph-concept-ref.md` | migration framing for the graph-native context-engine direction |

## Current Technical Reality

At the time of this cleanup, the working baseline still exposes:

- Cargo packages such as `zeroclaw`
- CLI commands such as `zeroclaw ...`
- inherited config keys, paths, and environment variable names
- modules, fixtures, and firmware assets that still use inherited naming

Treat those identifiers as present-day technical interfaces, not as the long-term GraphClaw naming outcome.

## Progressive Migration Track

GraphClaw is not migrating by renaming the whole repository first. The safer path is to insert explicit seams into the inherited runtime and move responsibility gradually.

The intended order is:

1. Context seam: introduce a first-class context boundary inside the Rust runtime so turn assembly is no longer scattered across prompt construction, memory recall, and tool orchestration.
2. Runtime artifacts: make concepts such as `SessionWindow`, `ContextPack`, and resolution traces explicit runtime objects rather than implicit text concatenation.
3. Graph adapter boundary: add a graph-facing storage interface behind traits so graph-native retrieval and topology-aware context selection can be introduced without replacing every inherited backend at once.
4. Package and binding layer: define how agents, skills, bundles, and policies become portable installation units instead of only repo-local files and conventions.
5. Naming migration: only rename inherited `zeroclaw` surfaces when the underlying runtime boundaries are stable enough that the rename reflects real architecture rather than wishful branding.

`graph-concept-ref.md` describes the target model in more detail. The current repository should be read as being between steps 0 and 1: documentation and navigation are being prepared first so later runtime changes can happen deliberately.

## Working Rules

- Preserve buildability unless a task explicitly permits breakage.
- Evolve the repository progressively instead of through sweeping rewrites.
- Keep inherited behavior, migration framing, and target architecture clearly separated in documentation.
- Do not rename crates, binaries, commands, runtime identifiers, or config keys unless the task is explicitly about migration.
- Update local `CONTEXT.md` files when a change alters area boundaries, expectations, or navigation.

## Validation

For documentation-only work:

```bash
./scripts/ci/docs_quality_gate.sh
./scripts/ci/docs_links_gate.sh
```

For code changes on the inherited baseline:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```
