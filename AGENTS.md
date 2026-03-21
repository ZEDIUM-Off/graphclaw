# AGENTS.md - GraphClaw

## Identity
Agis comme mon mentor stratégique et partenaire intellectuel d’élite. Ton but est d’élever durablement mon niveau de pensée, de compréhension, d’apprentissage, de décision, de création, d’écriture, de stratégie et d’exécution. Recherche la vérité, pas le confort. Ne valide jamais mes idées par défaut : challenge hypothèses, logique, angles morts, raccourcis, faux dilemmes et conclusions hâtives. Si je me trompe, dis-le clairement et explique pourquoi. Même si mon input est bref, flou ou incomplet, réponds à intelligence maximale : reconstruis l’intention probable, explicite tes hypothèses, structure mieux que moi le problème, transforme le flou en options, décisions et actions, et pose seulement les questions qui augmentent vraiment la qualité du raisonnement. Sépare faits, hypothèses, inconnues et recommandations. Donne le meilleur contre-argument possible. Ramène toujours au concret : définitions, exemples, critères de réussite, priorités, arbitrages, risques, effets de second ordre, coûts d’opportunité. Sois créatif avec rigueur. Quand le sujet touche au business, raisonne comme un CEO. Replace toujours l’utilisateur final au centre : compréhension, ressenti, friction, confiance, valeur perçue. N’invente pas. Ne flatte pas. Ne remplis pas l’espace. Chaque réponse doit soit améliorer ma réflexion, soit ma compréhension, soit mon niveau intellectuel.

This repository is GraphClaw.

It is currently built on an inherited ZeroClaw baseline. Many technical identifiers still use `zeroclaw` names across code, commands, tests, config, manifests, and firmware. Those names remain current implementation detail until an explicit migration task changes them.

GraphClaw should be treated as a transitional scaffold toward a graph-native context engine. Use target-architecture documents, especially [`docs/architecture/concepts/definition-governance.md`](docs/architecture/concepts/definition-governance.md) and [`docs/architecture/concepts/graph-context-engine.md`](docs/architecture/concepts/graph-context-engine.md), as migration framing only. Do not present that target state as already implemented.

## Canonical Concept Rule

GraphClaw concepts follow a single-definition rule.

- each concept has exactly one canonical definition;
- canonical definitions live under `docs/architecture/`;
- every non-canonical mention must reference the canonical source with a Markdown link;
- `AGENTS.md` and `CONTEXT.md` may contextualize local implementation or runtime behavior, but they must not redefine concepts.

The governing policy lives in [`docs/architecture/concepts/definition-governance.md`](docs/architecture/concepts/definition-governance.md).

## Required Reading

Read the nearest `CONTEXT.md` before changing any area.

If a local `CONTEXT.md` exists, treat it as the first source of truth for that directory. If not, move outward to the closest parent context file and then the root [`CONTEXT.md`](CONTEXT.md).

Use this reading order unless the task is explicitly narrower:

1. [`README.md`](README.md) for repo identity and current framing.
2. [`CONTEXT.md`](CONTEXT.md) for the root map and routing.
3. The nearest local `CONTEXT.md` for the area being changed.
4. Area-specific entry docs such as [`docs/README.md`](docs/README.md) or [`CONTRIBUTING.md`](CONTRIBUTING.md) when relevant.
5. [`docs/architecture/concepts/definition-governance.md`](docs/architecture/concepts/definition-governance.md) and [`docs/architecture/concepts/graph-context-engine.md`](docs/architecture/concepts/graph-context-engine.md) when the task touches GraphClaw concepts, target runtime invariants, or context-engine terminology.

## Task Routing

Use these anchors before editing:

| Area | Read first |
| --- | --- |
| root framing and navigation | [`README.md`](README.md), [`CONTEXT.md`](CONTEXT.md) |
| documentation | [`docs/README.md`](docs/README.md), [`docs/CONTEXT.md`](docs/CONTEXT.md) |
| concept model and stable vocabulary | [`docs/architecture/README.md`](docs/architecture/README.md), [`docs/architecture/concepts/definition-governance.md`](docs/architecture/concepts/definition-governance.md), [`docs/architecture/concepts/graph-context-engine.md`](docs/architecture/concepts/graph-context-engine.md), [`docs/architecture/concepts/glossary.md`](docs/architecture/concepts/glossary.md) |
| migration thesis and future seams | [`docs/architecture/migration/zero-to-graphclaw-transition.md`](docs/architecture/migration/zero-to-graphclaw-transition.md), [`docs/architecture/migration/future-integration-seams.md`](docs/architecture/migration/future-integration-seams.md) |
| views, sets, packability, artifacts, and turn logic | [`docs/architecture/concepts/views-and-sets.md`](docs/architecture/concepts/views-and-sets.md), [`docs/architecture/concepts/set.md`](docs/architecture/concepts/set.md), [`docs/architecture/concepts/view.md`](docs/architecture/concepts/view.md), [`docs/architecture/concepts/packability.md`](docs/architecture/concepts/packability.md), [`docs/architecture/concepts/context-artifacts.md`](docs/architecture/concepts/context-artifacts.md), [`docs/architecture/runtime/turn-runtime-logic.md`](docs/architecture/runtime/turn-runtime-logic.md) |
| backend capability mapping | [`docs/backends/README.md`](docs/backends/README.md), [`docs/backends/memgraph.md`](docs/backends/memgraph.md) |
| contributor workflow | [`CONTRIBUTING.md`](CONTRIBUTING.md) |
| Rust runtime | [`src/CONTEXT.md`](src/CONTEXT.md) |
| crates | [`crates/CONTEXT.md`](crates/CONTEXT.md) |
| web | [`web/CONTEXT.md`](web/CONTEXT.md) |
| ui | [`ui/CONTEXT.md`](ui/CONTEXT.md) |
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
./scripts/ci/docs_canonical_concepts_gate.sh
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

## Canonical Concept Routing

Use canonical concept sources instead of redefining terms locally:

- `Set`: [`docs/architecture/concepts/set.md`](docs/architecture/concepts/set.md)
- `ResolvedSet`: [`docs/architecture/concepts/resolved-set.md`](docs/architecture/concepts/resolved-set.md)
- `View`: [`docs/architecture/concepts/view.md`](docs/architecture/concepts/view.md)
- `ContextFrame`: [`docs/architecture/concepts/context-frame.md`](docs/architecture/concepts/context-frame.md)
- `SessionFrame`: [`docs/architecture/concepts/session-frame.md`](docs/architecture/concepts/session-frame.md)
- `ContextPack`: [`docs/architecture/interfaces/context-pack-interface.md`](docs/architecture/interfaces/context-pack-interface.md)
- `StrategyResolution`: [`docs/architecture/interfaces/strategy-resolver-interface.md`](docs/architecture/interfaces/strategy-resolver-interface.md)
- `ContextMutationProposal`, `ResolutionTrace`, `TaskIntent`, `AgentPackage`: [`docs/architecture/concepts/graph-context-engine.md`](docs/architecture/concepts/graph-context-engine.md)
- routing policy and canonical registry: [`docs/architecture/concepts/definition-governance.md`](docs/architecture/concepts/definition-governance.md)

When an agent document needs to mention one of these concepts, prefer:

- a link to the canonical source;
- a local explanation of current code representation or runtime use;
- an explicit note when inherited implementation still diverges from the target concept.

## Documentation Truthfulness

When editing root or docs-facing material:

- state GraphClaw as the repository identity;
- state inherited `zeroclaw` names as the current technical reality;
- distinguish current implementation, migration framing, and target architecture;
- prefer explicit references and local navigation over broad narrative claims.
