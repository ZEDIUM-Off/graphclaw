# Crates Context

## Scope

Workspace member crates that live outside the main `src/` tree.

## File Map

- `robot-kit/` - current standalone workspace crate

## Routing

This directory is a crate registry boundary for the workspace: each child directory should be an intentional crate with its own docs, manifest, and local context.

## Current State

The workspace is still small and currently contains one clearly separate crate rather than a large multi-crate architecture.

## GraphClaw Relevance

This subtree matters because crate boundaries are architectural commitments. GraphClaw migration should grow them only when the boundary is real, not as cleanup theater.

## Cautions

- Do not move code out of `src/` just to make the workspace look more modular.
- Keep each crate justified by a real ownership or dependency boundary.

## Agent Guidance

- Read the local crate context before editing a member crate.
- Document why a crate exists if its role changes materially.
