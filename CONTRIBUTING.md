# Contributing to GraphClaw

GraphClaw is currently maintained as a progressive fork of the inherited ZeroClaw codebase.

That means contribution work has two simultaneous constraints:

- keep the inherited runtime functioning as it does today;
- improve the repository so future migration toward a graph-native context engine becomes clearer and safer.

## Start Here

Before changing anything substantial:

1. read [`AGENTS.md`](AGENTS.md);
2. read [`CONTEXT.md`](CONTEXT.md);
3. read the nearest local `CONTEXT.md`;
4. use [`docs/README.md`](docs/README.md) if the task is docs- or process-related;
5. confirm whether you are changing inherited behavior, repository framing, or both.

## Task Routing

Choose the nearest guidance before editing:

| Change type | Read first |
| --- | --- |
| root docs, repo framing, or navigation | [`README.md`](README.md), [`CONTEXT.md`](CONTEXT.md) |
| contributor workflow or review policy | [`docs/contributing/README.md`](docs/contributing/README.md) |
| runtime behavior | [`src/CONTEXT.md`](src/CONTEXT.md) |
| crates | [`crates/CONTEXT.md`](crates/CONTEXT.md) |
| web | [`web/CONTEXT.md`](web/CONTEXT.md) |
| Python | [`python/CONTEXT.md`](python/CONTEXT.md) |
| firmware | [`firmware/CONTEXT.md`](firmware/CONTEXT.md) |
| tests | [`tests/CONTEXT.md`](tests/CONTEXT.md) |
| scripts or CI | [`scripts/CONTEXT.md`](scripts/CONTEXT.md), [`dev/CONTEXT.md`](dev/CONTEXT.md), [`.github/CONTEXT.md`](.github/CONTEXT.md) |

## Scope Control

- prefer incremental changes over large rewrites;
- preserve buildability unless a task explicitly allows breakage;
- do not mass-rename `zeroclaw` technical identifiers unless the task is explicitly about migration;
- do not present the graph-native context-engine target as already implemented;
- update local `CONTEXT.md` files when area boundaries or expectations change.

## Migration Framing

Use [`docs/architecture/graph-context-engine.md`](docs/architecture/graph-context-engine.md) as a design reference for where GraphClaw is intended to move. Treat it as migration framing, not as proof that the runtime already has those components.

Current implementation reality still includes inherited `zeroclaw` names across crates, binaries, commands, configuration, tests, and firmware. Keep documentation and change descriptions truthful about that distinction.

## Validation Policy

For documentation-only work, validate the documentation surface rather than adding product tests:

```bash
./scripts/ci/docs_quality_gate.sh
./scripts/ci/docs_links_gate.sh
```

For behavioral changes on the inherited baseline, follow TDD and run the narrowest relevant checks before delivery:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

For fuller local parity:

```bash
./dev/ci.sh all
```

## TDD

GraphClaw evolves in TDD by default.

For behavioral changes:

1. write the test first;
2. confirm the test captures the intended change;
3. implement the real behavior second;
4. run the relevant tests before delivery;
5. add tests for every new feature or materially changed behavior.

## Contribution Workflow

```bash
git clone <your-graphclaw-fork-url>
cd graphclaw
git config core.hooksPath .githooks
```

Then make the smallest defensible change, validate at the right risk level, and describe the result precisely in review:

- what changed;
- what remains inherited;
- what was intentionally left for later migration.

## References

- repo identity and root map: [`README.md`](README.md), [`CONTEXT.md`](CONTEXT.md)
- repo-wide agent rules: [`AGENTS.md`](AGENTS.md)
- documentation hub: [`docs/README.md`](docs/README.md)
- architecture references: [`docs/architecture/README.md`](docs/architecture/README.md)
- contributor references: [`docs/contributing/README.md`](docs/contributing/README.md)
