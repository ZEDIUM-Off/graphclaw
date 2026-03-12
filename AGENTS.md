# AGENTS.md - GraphClaw

## Identity

This repository is GraphClaw.

It is currently built on an inherited ZeroClaw baseline. Many technical identifiers still use `zeroclaw` names across code, commands, tests, config, manifests, and firmware. Those names remain current implementation detail until an explicit migration task changes them.

GraphClaw should be treated as a transitional scaffold toward a graph-native context engine. Use target-architecture documents, especially [`docs/architecture/graph-context-engine.md`](docs/architecture/graph-context-engine.md) and [`docs/architecture/glossary.md`](docs/architecture/glossary.md), as migration framing only. Do not present that target state as already implemented.

## Required Reading

Read the nearest `CONTEXT.md` before changing any area.

If a local `CONTEXT.md` exists, treat it as the first source of truth for that directory. If not, move outward to the closest parent context file and then the root [`CONTEXT.md`](CONTEXT.md).

Use this reading order unless the task is explicitly narrower:

1. [`README.md`](README.md) for repo identity and current framing.
2. [`CONTEXT.md`](CONTEXT.md) for the root map and routing.
3. The nearest local `CONTEXT.md` for the area being changed.
4. Area-specific entry docs such as [`docs/README.md`](docs/README.md) or [`CONTRIBUTING.md`](CONTRIBUTING.md) when relevant.
5. [`docs/architecture/graph-context-engine.md`](docs/architecture/graph-context-engine.md) when the task touches GraphClaw concepts, target runtime invariants, or context-engine terminology.

## Task Routing

Use these anchors before editing:

| Area | Read first |
| --- | --- |
| root framing and navigation | [`README.md`](README.md), [`CONTEXT.md`](CONTEXT.md) |
| documentation | [`docs/README.md`](docs/README.md), [`docs/CONTEXT.md`](docs/CONTEXT.md) |
| concept model and stable vocabulary | [`docs/architecture/README.md`](docs/architecture/README.md), [`docs/architecture/graph-context-engine.md`](docs/architecture/graph-context-engine.md), [`docs/architecture/glossary.md`](docs/architecture/glossary.md) |
| backend capability mapping | [`docs/backends/README.md`](docs/backends/README.md), [`docs/backends/memgraph.md`](docs/backends/memgraph.md) |
| contributor workflow | [`CONTRIBUTING.md`](CONTRIBUTING.md) |
| Rust runtime | [`src/CONTEXT.md`](src/CONTEXT.md) |
| crates | [`crates/CONTEXT.md`](crates/CONTEXT.md) |
| web | [`web/CONTEXT.md`](web/CONTEXT.md) |
| Python | [`python/CONTEXT.md`](python/CONTEXT.md) |
| firmware | [`firmware/CONTEXT.md`](firmware/CONTEXT.md) |
| tests | [`tests/CONTEXT.md`](tests/CONTEXT.md) |
| CI and scripts | [`scripts/CONTEXT.md`](scripts/CONTEXT.md), [`dev/CONTEXT.md`](dev/CONTEXT.md), [`.github/CONTEXT.md`](.github/CONTEXT.md) |

## Default Development Rules

- evolve the repo progressively, not through sweeping rewrites;
- preserve buildability unless a task explicitly permits breakage;
- distinguish inherited behavior, target architecture, and the current migration step;
- document structural decisions when they materially change repository behavior or navigation;
- treat documentation and context scaffolding as architectural work, not decoration.

## TDD Rule

GraphClaw evolves in TDD by default.

For behavioral changes:

1. write or update the test before implementing the real behavior;
2. confirm the test captures the intended change;
3. implement the behavior;
4. run the relevant tests before delivery;
5. add coverage for every new feature or materially changed behavior.

Do not implement real behavior first and retrofit tests afterward.

## Documentation-Only Rule

This cleanup is a documentation task. For documentation-only work, the TDD equivalent is audit validation rather than product-test creation.

That means:

- do not add Rust or Python product tests when behavior did not change;
- do run markdown and link audits;
- do verify that repo framing is consistent and truthful.
- do keep folder maps, references, and routing guidance aligned with the current tree.

## Validation Defaults

For documentation-only work:

```bash
./scripts/ci/docs_quality_gate.sh
./scripts/ci/docs_links_gate.sh
```

For code changes, run the narrowest relevant inherited checks first, usually including:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Migration Discipline

The current repository tree is a transitional scaffold for a graph-native context-engine direction.

Therefore:

- do not mass-rename crates, binaries, modules, commands, runtime identifiers, or config keys unless the task is explicitly about migration;
- do not pretend the target architecture already exists;
- do not rewrite unrelated areas opportunistically;
- update local `CONTEXT.md` files when a change alters area boundaries or expectations.

## Reference Vocabulary

Use the following terms consistently in documentation and migration-oriented design work.

This section is a quick-reference summary for agents. The canonical definitions live in `docs/architecture/graph-context-engine.md` and `docs/architecture/glossary.md`.

- `AgentPackage`: a versioned portable unit of agent behavior, declared dependencies, and packaging metadata.
- `AgentInstance`: a concrete installation or activation of a package in a local environment, with local bindings and policy choices applied.
- `AgentSession`: the live runtime state for a conversation or short execution horizon; sessions are not packages.
- `Bindings`: local provider, tool, storage, or environment attachments required to run an instance; bindings are not inherently portable.
- `View`: a governed projection over the graph that can produce one or more policy-constrained sets.
- `GraphSet`: a first-class logical set of node references, with provenance, combination rules, and budget implications.
- `SessionWindow`: the currently visible and mobilizable subgraph for a turn or short sequence of turns; it is not the whole graph or the full conversation history.
- `ThinkingContext`: the temporary reflection context used to explore or evaluate before final response packing.
- `ContextPack`: the final budgeted context artifact retained for response generation.
- `ContextMutationProposal`: a structured proposal to change visible or packable context.
- `ResolutionTrace`: the explicit record of context-resolution decisions such as selection, degradation, rejection, summarization, and final packing.

Agents should consume `View`, `GraphSet`, `SessionWindow`, `ThinkingContext`, `ContextPack`, and `ResolutionTrace` artifacts. They should not be documented as the owners of the core context-resolution model itself.

## Documentation Truthfulness

When editing root or docs-facing material:

- state GraphClaw as the repository identity;
- state inherited `zeroclaw` names as the current technical reality;
- distinguish current implementation, migration framing, and target architecture;
- prefer explicit references and local navigation over broad narrative claims.
