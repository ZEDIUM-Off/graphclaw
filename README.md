# GraphClaw

GraphClaw is a documentation-led fork of an inherited ZeroClaw baseline.

The repository identity is `GraphClaw`, but the current implementation still exposes many inherited `zeroclaw` technical surfaces across crates, binaries, CLI commands, configuration, tests, manifests, and firmware. Those names remain the current implementation reality until an explicit migration task changes them.

GraphClaw should therefore be read as a transitional scaffold: a working inherited runtime being reorganized so it can evolve toward a graph-native context engine without a rename-first rewrite.

## What GraphClaw Is Trying To Become

The target is not a renamed ZeroClaw and not a thin graph wrapper around an existing prompt stack.

The target is a `Graph Context Engine` where:

- context is a governed artifact rather than an implicit byproduct of prompts, history, and recall;
- views and sets are first-class objects instead of ad hoc query results;
- context selection is budgeted, policy-aware, and traceable;
- reflective context resolution happens before final response packing;
- portable agent behavior is modeled as packages, instances, bindings, and sessions rather than just repo-local files.

The context engine and the packaging model are adjacent but not identical layers. The engine governs context resolution for turns; packaging governs portability and installation of agent behavior.

That target is documented first and implemented progressively afterward.

## Current Status

This repository is still in the documentation-and-boundary phase of migration.

- the inherited runtime is still the source of operational truth;
- documentation now carries the reference vocabulary and conceptual boundaries the target system is trying to stabilize;
- documentation is also being used to mark which runtime processes should later become interfacable seams rather than implicit inherited pipelines;
- Memgraph is the reference backend for the first graph-native architecture axis, but it is not the business model of the project.

Do not read target-architecture documents as proof that the current runtime already contains those components.

## Read First

Use this order before making changes:

1. [`AGENTS.md`](AGENTS.md) for repository-wide rules.
2. [`CONTEXT.md`](CONTEXT.md) for the root map and task routing.
3. The nearest local `CONTEXT.md` for the area you will edit.
4. [`docs/README.md`](docs/README.md) or [`CONTRIBUTING.md`](CONTRIBUTING.md) if the task is documentation, review, or process related.
5. [`docs/architecture/concepts/graph-context-engine.md`](docs/architecture/concepts/graph-context-engine.md) when the task touches GraphClaw concepts, invariants, or target runtime seams.

## Documentation Architecture

GraphClaw documentation is intentionally layered:

| Level | Primary question | Typical locations |
| --- | --- | --- |
| intent | why does GraphClaw exist | `README.md`, strategy docs |
| conceptual architecture | what are the reference concepts and strategy families being stabilized | `docs/architecture/concepts/definition-governance.md`, `docs/architecture/concepts/graph-context-engine.md`, concept-specific docs under `docs/architecture/` |
| transition architecture | how does the inherited runtime gain seams without a rewrite-first migration | `docs/architecture/migration/zero-to-graphclaw-transition.md`, `docs/architecture/migration/future-integration-seams.md` |
| context semantics | how should views, sets, packability, artifacts, and budgets work | `docs/architecture/concepts/views-and-sets.md`, `docs/architecture/concepts/set.md`, `docs/architecture/concepts/view.md`, `docs/architecture/concepts/packability.md`, `docs/architecture/concepts/context-artifacts.md`, `docs/architecture/runtime/turn-runtime-logic.md` |
| project architecture | how is the repo divided | `CONTEXT.md`, local `CONTEXT.md` files |
| runtime logic | how should a turn resolve logically, including strategy resolution and bounded orchestration | `docs/architecture/runtime/turn-runtime-logic.md`, runtime-area docs |
| backend integration | how does a concrete graph backend support the model | `docs/backends/memgraph.md` |
| implementation | what code exists today | source-adjacent docs and code |

The intended order is: semantics first, boundaries second, mechanisms third, implementation last.

## Task Routing

Use these entry points to orient work quickly:

| If you are working on... | Read first |
| --- | --- |
| repo identity, root framing, agent rules | [`README.md`](README.md), [`AGENTS.md`](AGENTS.md), [`CONTEXT.md`](CONTEXT.md) |
| Graph Context Engine concepts, strategy families, or invariants | [`docs/architecture/README.md`](docs/architecture/README.md), [`docs/architecture/concepts/graph-context-engine.md`](docs/architecture/concepts/graph-context-engine.md) |
| migration seams, transition thesis, orchestration framing, or future interface families | [`docs/architecture/migration/zero-to-graphclaw-transition.md`](docs/architecture/migration/zero-to-graphclaw-transition.md), [`docs/architecture/migration/future-integration-seams.md`](docs/architecture/migration/future-integration-seams.md) |
| view, set, packability, artifact, planning-artifact, or turn-logic semantics | [`docs/architecture/concepts/views-and-sets.md`](docs/architecture/concepts/views-and-sets.md), [`docs/architecture/concepts/set.md`](docs/architecture/concepts/set.md), [`docs/architecture/concepts/view.md`](docs/architecture/concepts/view.md), [`docs/architecture/concepts/packability.md`](docs/architecture/concepts/packability.md), [`docs/architecture/concepts/context-artifacts.md`](docs/architecture/concepts/context-artifacts.md), [`docs/architecture/runtime/turn-runtime-logic.md`](docs/architecture/runtime/turn-runtime-logic.md) |
| backend mapping or Memgraph constraints | [`docs/backends/README.md`](docs/backends/README.md), [`docs/backends/memgraph.md`](docs/backends/memgraph.md) |
| Rust runtime behavior | [`src/CONTEXT.md`](src/CONTEXT.md) |
| workspace crates | [`crates/CONTEXT.md`](crates/CONTEXT.md) |
| web UI | [`web/CONTEXT.md`](web/CONTEXT.md) |
| future Vue UI | [`ui/CONTEXT.md`](ui/CONTEXT.md) |
| deployable OVH VPS runtime assets | [`deploy/CONTEXT.md`](deploy/CONTEXT.md), [`deploy/ovh-vps/README.md`](deploy/ovh-vps/README.md) |
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
| `ui/` | future Vue-based GraphClaw interface, currently scoped to the playground |
| `python/` | inherited Python tooling and integrations |
| `firmware/` | hardware-side experiments and board support |
| `tests/` | component, integration, live, manual, and system test areas |
| `docs/` | documentation hub, architecture references, backend references, contributor guides, and operations material |
| `deploy/` | production-oriented deployment assets for current-state runtime profiles |
| `scripts/` | CI, release, and maintenance scripts |
| `dev/` | local development and CI helper surfaces |
| `.github/` | workflow automation and repository policy metadata |

## Deployable Current State

GraphClaw now includes a production-oriented current-state deployment profile for an OVH VPS behind Cloudflare Tunnel and Cloudflare Access.

- the deployed runtime remains the inherited `zeroclaw` gateway and runtime;
- the deployed browser UI for this profile remains `web/`;
- the separate `ui/` subtree remains out of scope for the first internal operator deployment;
- deploy assets live under [`deploy/ovh-vps/`](deploy/ovh-vps/README.md);
- operator guidance lives under [`docs/ops/ovh-vps-cloudflare-deployment.md`](docs/ops/ovh-vps-cloudflare-deployment.md).

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

1. Documentation and canonical concept sources: stabilize the meaning of `Set`, `View`, `ResolvedSet`, `ContextFrame`, `SessionFrame`, `ContextPack`, `ContextMutationProposal`, `ResolutionTrace`, and portable agent packaging boundaries.
2. Boundary docs: make repository and subsystem responsibilities explicit through `CONTEXT.md` files and architecture references.
3. Runtime artifacts: make context-resolution objects explicit instead of relying on implicit prompt concatenation.
4. Graph adapter boundary: add a graph-facing storage interface behind traits so topology-aware context selection can arrive without replacing every inherited backend at once.
5. Package and binding layer: define how agents, skills, bundles, and policies become portable installation units instead of only repo-local files and conventions.
6. Naming migration: rename inherited `zeroclaw` surfaces only when the underlying runtime boundaries are stable enough that the rename reflects real architecture rather than wishful branding.

The current repository should be read as being between steps 1 and 2: documentation and navigation are being hardened first so later runtime changes can happen deliberately.

## Migration By Seams

The main migration claim is not "the engine already exists." It is that some current runtime processes should be prepared to branch behind cleaner conceptual boundaries.

The main process families to document this way are:

- context creation for a turn;
- memory loading and recall as one input into context selection;
- `Set` resolution and runtime `View` derivation and manipulation;
- budget estimation for packable candidates and final context;
- final `ContextPack` assembly;
- trace and mutation persistence where those artifacts become explicit.

The intended end state is coexistence through interfaces: the inherited pipeline and a future Graph Context Engine should be able to sit behind the same high-level boundary for a transition period.

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
