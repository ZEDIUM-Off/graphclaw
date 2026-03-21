# Architecture Migration Context

## Local Purpose

This directory contains migration-framing docs for architecture evolution, seam placement, and transition sequencing inside the documentation set.

## What Belongs Here

- migration thesis and transition framing;
- future seam placement and architecture-evolution notes;
- routing from migration docs to concept, runtime, or interface docs.

## File Map

- `zero-to-graphclaw-transition.md` - migration thesis
- `future-integration-seams.md` - future seam families

## Routing

- use this directory when the question is how GraphClaw evolves from inherited runtime to explicit architecture;
- use `../concepts/CONTEXT.md` when the question is what a concept means before asking how it migrates;
- use `../interfaces/CONTEXT.md` when the question is which seam contract should exist;
- keep these docs above source-level ownership and below pure concept-definition work.
