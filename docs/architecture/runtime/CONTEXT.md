# Architecture Runtime Context

## Local Purpose

This directory contains runtime-facing architecture docs for turn sequencing, logical flow, and cross-cutting execution order.

## What Belongs Here

- turn-flow and phase-order documentation;
- runtime reading order across artifacts and decisions;
- routing from runtime logic to concept or interface docs when needed.

## File Map

- `turn-runtime-logic.md` - logical turn phases and inherited runtime mapping

## Routing

- use this directory when the question is how a turn should resolve logically;
- use `../concepts/agent-loop.md` and `../concepts/got.md` when the question is about mono-agent reasoning structure rather than runtime sequencing;
- use `../interfaces/CONTEXT.md` when the question is about seam boundaries rather than flow;
- keep the distinction between target logic and current inherited runtime explicit.
