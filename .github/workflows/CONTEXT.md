# Workflows Context

## Local Purpose

This directory contains the runnable GitHub Actions workflow entry files and a small amount of workflow-local documentation. It defines CI and release behavior visible to contributors and maintainers.

## What Belongs Here

- root-level GitHub Actions workflow entry files;
- workflow-local explanatory docs that help maintainers understand branch or release flow.

## What Does Not Belong Here

- general contributor process docs duplicated from `docs/contributing/`;
- helper logic that belongs in scripts or reusable actions elsewhere;
- documentation-only rewrites that quietly change workflow semantics.

## File Map

- `checks-on-pr.yml` - pull request validation workflow
- `cross-platform-build-manual.yml` - manual build workflow
- `release-beta-on-push.yml` - beta release automation
- `release-stable-manual.yml` - manual stable release automation
- `README.md` - workflow-local guidance
- `master-branch-flow.md` - branch-flow explanation

## Routing

- workflow triggers, jobs, and release automation belong here
- contributor-facing explanation of CI policy belongs primarily in `docs/contributing/`
- shared script behavior should live outside this directory and be referenced from workflows

## References

- `.github/CONTEXT.md` - GitHub subtree boundary
- `docs/contributing/CONTEXT.md` - contributor-process boundary
- `scripts/ci/CONTEXT.md` - CI helper script boundary when workflows call out to scripts

## Current Inherited State

Workflow behavior is high-risk and still largely oriented around the inherited `zeroclaw` baseline, current release flow, and existing CI expectations. Those inherited names and assumptions remain valid when they describe live automation.

## GraphClaw Migration Relationship

GraphClaw migration may eventually change branch policy, release flow, or job naming, but not by implication. This directory should only evolve when actual automation behavior or its immediately supporting documentation changes.

## Cautions

- workflow edits can break CI, releases, or contributor expectations
- keep workflow entry files at this directory root unless GitHub behavior changes
- prefer clarifying docs or helper scripts first when the goal is understanding, not automation change

## Agent Workflow

1. Confirm whether the task is documentation scaffolding or real workflow behavior change.
2. Read the owning workflow and any called scripts before describing or modifying behavior.
3. Preserve inherited names while they still match active workflow files, jobs, or releases.
4. Treat every workflow edit as high risk and keep docs-only tasks semantically neutral.
