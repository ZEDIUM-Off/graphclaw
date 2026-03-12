# Dev Sandbox Context

## Scope

Container definition for the local sandboxed user environment.

## File Map

- `Dockerfile` - sandbox image definition

## Routing

Local sandbox workflows depend on this Dockerfile to create a predictable user-like environment for agent/runtime testing.

## Current State

This directory is infrastructure for testing behavior safely, not a product feature area.

## GraphClaw Relevance

The sandbox supports migration work by making current behavior reproducible while GraphClaw scaffolding is still changing around it.

## Cautions

- Keep the environment predictable and intentionally small.
- Avoid adding tools that are convenient for one task but unrepresentative of real workflows.

## Agent Guidance

- Change this only when sandbox behavior truly needs to shift.
- Document new environment assumptions locally so future tests remain explainable.
