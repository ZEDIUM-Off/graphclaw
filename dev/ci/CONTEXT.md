# Dev CI Context

## Scope

Container definition for local CI-parity workflows.

## File Map

- `Dockerfile` - local CI environment image definition

## Routing

`dev/docker-compose.ci.yml` and related local workflow scripts depend on this image definition for reproducible validation.

## Current State

This subtree is intentionally minimal because its job is parity, not feature growth.

## GraphClaw Relevance

Reliable CI parity is part of safe GraphClaw evolution; it lets documentation and migration work be checked against the same expectations contributors see locally.

## Cautions

- Keep changes tied to real CI dependency needs.
- Avoid adding convenience tools that make the image drift from actual validation requirements.

## Agent Guidance

- Touch this directory only when local CI behavior genuinely changes.
- If dependencies move here, explain which validation path now needs them.
