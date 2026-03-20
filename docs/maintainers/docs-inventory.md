# GraphClaw Documentation Inventory

This inventory classifies the current documentation tree by intent so maintainers can distinguish runtime truth, conceptual architecture, process guidance, and time-bound material.

Last reviewed: **March 12, 2026**.

Current implementation note: many runtime-facing docs still describe inherited `zeroclaw` commands, payloads, and configuration because those remain the implemented interfaces today.

## Classification Legend

- **Current Guide/Reference**: intended to match current runtime behavior or repo structure
- **Architecture Reference**: target concept model or backend mapping for GraphClaw
- **Policy/Process**: collaboration or governance rules
- **Proposal/Roadmap**: design exploration that is not a strict runtime contract
- **Snapshot**: time-bound maintainer assessment

## Primary Entry Points

| Doc | Type | Audience |
| --- | --- | --- |
| `README.md` | Current Guide | all readers |
| `CONTEXT.md` | Current Guide | contributors and agents |
| `AGENTS.md` | Policy/Process | contributors and agents |
| `CONTRIBUTING.md` | Policy/Process | contributors |
| `docs/README.md` | Current Guide | all readers |
| `docs/CONTEXT.md` | Current Guide | contributors and agents |

## Architecture References

| Doc | Type | Audience |
| --- | --- | --- |
| `docs/architecture/README.md` | Architecture Reference | maintainers, contributors |
| `docs/architecture/concepts/definition-governance.md` | Policy/Process | maintainers, contributors |
| `docs/architecture/concepts/graph-context-engine.md` | Architecture Reference | maintainers, contributors |
| `docs/architecture/concepts/glossary.md` | Architecture Reference | maintainers, contributors |
| `docs/backends/README.md` | Architecture Reference | maintainers, contributors |
| `docs/backends/memgraph.md` | Architecture Reference | maintainers, contributors |

## Current Docs Hubs

| Doc | Type | Audience |
| --- | --- | --- |
| `docs/reference/README.md` | Current Guide | users, operators |
| `docs/ops/README.md` | Current Guide | operators |
| `docs/security/README.md` | Current Guide | operators, contributors |
| `docs/setup-guides/README.md` | Current Guide | users, operators |
| `docs/hardware/README.md` | Current Guide | hardware builders |
| `docs/contributing/README.md` | Current Guide | contributors, reviewers |
| `docs/maintainers/README.md` | Current Guide | maintainers |
| `docs/i18n/README.md` | Current Guide | translators, maintainers |

## Current Runtime-Facing References

Representative examples:

| Doc | Type | Audience |
| --- | --- | --- |
| `docs/reference/cli/commands-reference.md` | Current Reference | users, operators |
| `docs/reference/api/providers-reference.md` | Current Reference | users, operators |
| `docs/reference/api/channels-reference.md` | Current Reference | users, operators |
| `docs/reference/api/config-reference.md` | Current Reference | operators |
| `docs/reference/sop/README.md` | Current Reference | operators, contributors |
| `docs/ops/operations-runbook.md` | Current Guide | operators |
| `docs/ops/troubleshooting.md` | Current Guide | users, operators |
| `docs/ops/network-deployment.md` | Current Guide | operators |
| `docs/ops/resource-limits.md` | Proposal/Roadmap | operators, maintainers |

## Setup And Integration Guides

Representative examples:

| Doc | Type | Audience |
| --- | --- | --- |
| `docs/setup-guides/one-click-bootstrap.md` | Current Guide | users, operators |
| `docs/setup-guides/mattermost-setup.md` | Current Guide | operators |
| `docs/setup-guides/nextcloud-talk-setup.md` | Current Guide | operators |
| `docs/setup-guides/zai-glm-setup.md` | Current Guide | users, operators |
| `docs/contributing/custom-providers.md` | Current Integration Guide | integration developers |
| `docs/contributing/langgraph-integration.md` | Current Integration Guide | integration developers |

## Hardware And Security Material

Representative examples:

| Doc | Type | Audience |
| --- | --- | --- |
| `docs/hardware/arduino-uno-q-setup.md` | Current Guide | hardware builders |
| `docs/hardware/nucleo-setup.md` | Current Guide | hardware builders |
| `docs/hardware/hardware-peripherals-design.md` | Current Guide | hardware contributors |
| `docs/hardware/datasheets/esp32.md` | Current Reference | hardware builders |
| `docs/security/sandboxing.md` | Proposal/Roadmap | contributors, maintainers |
| `docs/security/audit-logging.md` | Proposal/Roadmap | contributors, maintainers |
| `docs/security/security-roadmap.md` | Proposal/Roadmap | maintainers |

## Contributor And Maintainer Process Docs

Representative examples:

| Doc | Type | Audience |
| --- | --- | --- |
| `docs/contributing/pr-workflow.md` | Policy/Process | contributors |
| `docs/contributing/reviewer-playbook.md` | Policy/Process | reviewers |
| `docs/contributing/ci-map.md` | Policy/Process | contributors, maintainers |
| `docs/contributing/docs-contract.md` | Policy/Process | contributors |
| `docs/contributing/context-template.md` | Policy/Process | contributors |
| `docs/maintainers/repo-map.md` | Current Guide | maintainers |
| `docs/maintainers/docs-inventory.md` | Current Guide | maintainers |

## Snapshot Docs

| Doc | Type |
| --- | --- |
| `docs/maintainers/project-triage-snapshot-2026-02-18.md` | Snapshot |

## Maintenance Recommendations

1. Update `docs/architecture/` whenever root framing or GraphClaw vocabulary changes materially.
2. Keep `docs/backends/memgraph.md` aligned with the real backend contract assumptions used in migration discussions.
3. Update `docs/reference/` when current CLI, API, or SOP surfaces change.
4. Update `docs/ops/` when operator-facing runtime behavior or deployment expectations change.
5. Keep proposal and roadmap docs explicitly marked so they are not mistaken for strict runtime contracts.
6. Keep localized docs and docs-hub routing aligned when adding new core documentation branches.
7. Re-review this inventory whenever major docs are added, moved, or reclassified.
