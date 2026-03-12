# GraphClaw Documentation Hub

This is the main documentation entry point for GraphClaw.

Most of the documentation tree is still inherited from the ZeroClaw baseline. Many pages therefore describe the current implementation using `zeroclaw` commands, config keys, and runtime names. Those pages remain useful because they describe the system as it exists today.

Read the docs with the transition in mind:

- GraphClaw is the repository identity;
- the current runtime is still the inherited baseline;
- the future direction is a graph-native context engine introduced progressively, not all at once.

## Start Here

- repo identity and transition framing: [`../README.md`](../README.md)
- repo-wide agent rules: [`../AGENTS.md`](../AGENTS.md)
- root repository map: [`../CONTEXT.md`](../CONTEXT.md)
- Graph Context Engine reference: [`architecture/README.md`](architecture/README.md)
- backend references: [`backends/README.md`](backends/README.md)
- contributor workflow: [`../CONTRIBUTING.md`](../CONTRIBUTING.md)

## Docs Routing

Use this page to choose the right documentation branch before editing or relying on a section.

| If you need... | Read |
| --- | --- |
| GraphClaw concepts, glossary, or target runtime framing | [`architecture/README.md`](architecture/README.md) |
| backend reference mapping or Memgraph constraints | [`backends/README.md`](backends/README.md) |
| contributor and review process | [`contributing/README.md`](contributing/README.md) |
| setup and onboarding instructions | [`setup-guides/README.md`](setup-guides/README.md) |
| CLI, API, and SOP references | [`reference/README.md`](reference/README.md) |
| operations, deployment, or troubleshooting | [`ops/README.md`](ops/README.md) |
| security guidance | [`security/README.md`](security/README.md) |
| hardware and peripherals | [`hardware/README.md`](hardware/README.md) |
| maintainer responsibilities | [`maintainers/README.md`](maintainers/README.md) |
| localization and translations | [`i18n/README.md`](i18n/README.md), [`vi/README.md`](vi/README.md) |

## Main Documentation Areas

- conceptual architecture and glossary: [`architecture/README.md`](architecture/README.md)
- backend integration references: [`backends/README.md`](backends/README.md)
- getting started and setup: [`setup-guides/README.md`](setup-guides/README.md)
- CLI and API references: [`reference/README.md`](reference/README.md)
- operations and troubleshooting: [`ops/README.md`](ops/README.md)
- security guidance: [`security/README.md`](security/README.md)
- hardware and peripherals: [`hardware/README.md`](hardware/README.md)
- contributor and maintainer material: [`contributing/README.md`](contributing/README.md), [`maintainers/README.md`](maintainers/README.md)
- localization and translation surfaces: [`i18n/README.md`](i18n/README.md), [`vi/README.md`](vi/README.md)

## Docs Map

| Path | Purpose |
| --- | --- |
| `docs/architecture/` | stable GraphClaw concepts, glossary, and target runtime framing |
| `docs/backends/` | backend integration references and coupling guidance |
| `docs/contributing/` | contributor workflow, CI, review, and process references |
| `docs/setup-guides/` | setup and getting-started material |
| `docs/reference/` | CLI, API, and SOP references |
| `docs/ops/` | operations, troubleshooting, and runtime procedures |
| `docs/security/` | security guidance and policies |
| `docs/hardware/` | hardware, peripherals, and device notes |
| `docs/maintainers/` | maintainer-facing responsibilities and runbooks |
| `docs/i18n/` | localization guidance and translation structure |
| `docs/vi/` | Vietnamese documentation surfaces |

## Reading Guidance

- If a page still says ZeroClaw, interpret it as inherited implementation documentation unless the page explicitly states a new GraphClaw framing.
- If you are modifying a directory represented in docs, read that directory's `CONTEXT.md` first.
- Do not infer that the target GraphClaw architecture is already implemented just because the design direction exists.
- Use [`architecture/graph-context-engine.md`](architecture/graph-context-engine.md) as migration framing only; do not rewrite inherited documentation as if that target state already exists.
