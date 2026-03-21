# Architecture Interfaces Context

## Local Purpose

This directory contains architecture interface fiches. Each file describes one migration-facing seam and the documentation question that seam must answer.

## What Belongs Here

- interface fiches that define what a seam isolates, consumes, or produces;
- interface-specific terminology and artifact boundaries;
- routing to the concept or runtime docs that a fiche depends on.

## File Map

- `session-window-interface.md` - retired migration note redirecting to `View`, `SessionFrame`, and `ContextPack`
- `context-pack-interface.md` - final packed-context seam
- `strategy-resolver-interface.md` - turn-time strategy selection seam
- `graph-context-store-and-retriever-interface.md` - graph-aware context supply seam
- `mutation-guard-interface.md` - visible-context mutation validation seam
- `orchestration-policies-interface.md` - routing, spawn, and aggregation policy seam
- `hook-bus-interface.md` - lifecycle-event publication seam

## Routing

- use this directory when the question is what a seam must isolate, consume, or produce;
- read `../concepts/CONTEXT.md` first if the question is really about concept meaning;
- read `../runtime/CONTEXT.md` if the question is really about turn sequencing rather than seam boundaries;
- do not turn these docs into code-signature dumps.
