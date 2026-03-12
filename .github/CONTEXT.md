# GitHub Context

## Local Purpose

This subtree holds GitHub-hosted repository scaffolding: automation entrypoints, issue intake forms, code scanning configuration, review metadata, and policy files that shape contribution and release behavior.

## What Belongs Here

- GitHub Actions workflow entry files and related workflow docs;
- issue templates, pull request templates, and issue configuration;
- repository metadata such as `CODEOWNERS`, Dependabot, labeling, and action lint policy;
- code scanning configuration used by GitHub automation.

## What Does Not Belong Here

- product documentation that belongs in `docs/`;
- implementation changes disguised as metadata cleanup;
- casual workflow rewrites during documentation-only work.

## File Map

- `workflows/` - runnable GitHub Actions entry files and workflow notes
- `ISSUE_TEMPLATE/` - issue forms and issue template configuration
- `codeql/` - CodeQL scanning configuration
- `pull_request_template.md` - PR template
- `CODEOWNERS`, `dependabot.yml`, `labeler.yml`, `label-policy.json`, `actionlint.yaml` - repository automation and policy files

## Routing

- workflow execution behavior belongs in `.github/workflows/`
- issue intake and contributor form design belongs in `.github/ISSUE_TEMPLATE/`
- code scanning scope belongs in `.github/codeql/`
- contributor-facing narrative about these systems belongs in `docs/contributing/`

## References

- `CONTEXT.md` - repo-wide migration framing
- `docs/contributing/CONTEXT.md` - contributor process boundary
- `.github/workflows/CONTEXT.md` - workflow-specific cautions

## Current Inherited State

This area still reflects the inherited CI, release, and repository automation model built around the current `zeroclaw` baseline. Names, workflow triggers, and release assumptions may therefore still use inherited terminology because the automation still serves that implementation.

## GraphClaw Migration Relationship

GraphClaw migration can affect repository framing and future automation shape, but this subtree should only change when GitHub-hosted behavior, metadata, or process truly changes. Documentation-only cleanup must not quietly alter CI, release, or intake behavior.

## Cautions

- workflow and policy edits are high-risk because they affect contributors and release flow
- keep documentation framing separate from automation semantics
- do not rebrand inherited job names or release assumptions unless the automation itself changed

## Agent Workflow

1. Read this file and then the nearest deeper `.github/**/CONTEXT.md` before editing.
2. Confirm whether the task is documentation scaffolding or actual automation behavior.
3. For docs-only changes, avoid modifying YAML semantics, triggers, or policy behavior.
4. Preserve inherited names where they still match active jobs, workflows, or release surfaces.
