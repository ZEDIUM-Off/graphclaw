# AGENTS.md - GraphClaw

## Identity

This repository is GraphClaw.

It is currently built on an inherited ZeroClaw baseline. Many technical identifiers still use `zeroclaw` names across code, commands, tests, config, manifests, and firmware. Those names remain current implementation detail until an explicit migration task changes them.

GraphClaw should be treated as a transitional scaffold toward a graph-native context engine. Use target-architecture documents, especially [`graph-concept-ref.md`](graph-concept-ref.md), as migration framing only. Do not present that target state as already implemented.

## Required Reading

Read the nearest `CONTEXT.md` before changing any area.

If a local `CONTEXT.md` exists, treat it as the first source of truth for that directory. If not, move outward to the closest parent context file and then the root [`CONTEXT.md`](CONTEXT.md).

Use this reading order unless the task is explicitly narrower:

1. [`README.md`](README.md) for repo identity and current framing.
2. [`CONTEXT.md`](CONTEXT.md) for the root map and routing.
3. The nearest local `CONTEXT.md` for the area being changed.
4. Area-specific entry docs such as [`docs/README.md`](docs/README.md) or [`CONTRIBUTING.md`](CONTRIBUTING.md) when relevant.

## Task Routing

Use these anchors before editing:

| Area | Read first |
| --- | --- |
| root framing and navigation | [`README.md`](README.md), [`CONTEXT.md`](CONTEXT.md) |
| documentation | [`docs/README.md`](docs/README.md), [`docs/CONTEXT.md`](docs/CONTEXT.md) |
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

## Documentation Truthfulness

When editing root or docs-facing material:

- state GraphClaw as the repository identity;
- state inherited `zeroclaw` names as the current technical reality;
- distinguish current implementation, migration framing, and target architecture;
- prefer explicit references and local navigation over broad narrative claims.
