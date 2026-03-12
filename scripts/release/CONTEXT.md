# Release Scripts Context

## Scope

Release-flow automation for the current repository lifecycle.

## File Map

- `cut_release_tag.sh` - current release-tag helper

## Routing

This subtree is intentionally narrow: release operations enter through `cut_release_tag.sh` and should stay easy to audit.

## Current State

Release flow still follows inherited runtime naming and lifecycle assumptions.

## GraphClaw Relevance

Release automation should trail verified migration work, not announce it early. This directory therefore reflects current baseline behavior rather than future branding.

## Cautions

- Keep changes conservative.
- Do not update release semantics here just because higher-level docs are evolving.

## Agent Guidance

- Touch this subtree only when release behavior truly changes.
- When you do change it, document the operational reason, not just the script delta.
