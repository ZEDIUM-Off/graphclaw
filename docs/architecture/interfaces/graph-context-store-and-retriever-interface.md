# GraphContextStore And ContextRetriever Interfaces

## Status

This document defines a paired migration-facing boundary for `GraphContextStore` and `ContextRetriever`.

It does not claim that the inherited runtime already exposes these interfaces in code. It explains how GraphClaw can separate graph-aware and memory-aware context supply from higher-level context governance without collapsing the whole future engine into `src/memory/`.

## Why These Interfaces Matter

The inherited runtime already has multiple memory backends, retrieval helpers, embeddings, chunking, snapshots, and caches.

That is useful infrastructure, but it still leaves a key ambiguity:

- which layer stores or queries candidate context;
- which layer chooses retrieval strategy;
- which layer compacts, binds, or summarizes evidence;
- which layer owns final visibility, packability, and context policy.

GraphClaw needs a clean answer:

> storage and retrieval interfaces may supply context material, but they are not the whole Graph Context Engine.

`GraphContextStore` and `ContextRetriever` are the minimum paired boundary for stating that answer operationally.

## Interface Roles

### `GraphContextStore`

`GraphContextStore` is the graph-aware storage and query boundary for context-relevant structured knowledge.

It should eventually support operations such as:

- governed traversal inputs;
- candidate concept lookup;
- view-bounded graph expansion;
- optional materialization support for helper subsets or traces;
- retrieval of graph-linked supporting content.

It is a store and query boundary, not the owner of final context policy.

### `ContextRetriever`

`ContextRetriever` is the higher-level retrieval coordinator that gathers context candidates from one or more sources under strategy and policy constraints.

It should eventually coordinate:

- flat memory recall;
- graph-aware candidate retrieval;
- supporting resources or snapshots;
- ranking or pre-compaction inputs for later context work.

It is still not the owner of the final `SessionWindow`, `ContextPack`, or packing policy.

## Separation Rule

The minimum rule to keep stable is:

1. storage and query interfaces supply candidate material;
2. retrieval coordination gathers and normalizes that material;
3. compaction and evidence binding may refine the candidate set;
4. higher-level context governance decides what becomes visible and packable.

That rule prevents `src/memory/` from being redescribed as the whole context engine simply because graph-aware storage later arrives nearby.

## Inputs And Outputs

### `GraphContextStore` Inputs

- concept anchors, seeds, or graph identifiers;
- active view or scope constraints;
- relation, type, or property filters;
- traversal limits or materialization hints;
- policy-compatible query parameters.

### `GraphContextStore` Outputs

- graph candidates, subgraph fragments, or referenced content handles;
- lazy or materialized set-like results suitable for later retrieval coordination;
- optionally persisted helper subsets, traces, or summaries through adapter logic.

### `ContextRetriever` Inputs

- `TaskIntent` or equivalent task framing;
- selected strategy hints or retrieval profile inputs;
- current session scope or view constraints;
- memory-store handles, graph-store handles, and supporting resource access;
- ranking, budget, or fallback constraints.

### `ContextRetriever` Outputs

- normalized candidate evidence items or candidate `View` inputs;
- cross-source retrieval results suitable for compaction or planning;
- explicit notes about missing sources, degraded retrieval, or fallback paths.

## Minimum Responsibilities

### `GraphContextStore`

- expose graph-shaped retrieval operations without defining GraphClaw business meaning by itself;
- support constrained traversal and filtering;
- keep backend-specific details behind a stable adapter boundary;
- remain compatible with Memgraph as a reference backend without letting Memgraph define the concept model.

### `ContextRetriever`

- gather multi-source candidates under turn-specific constraints;
- keep retrieval separate from final packing and final visibility;
- support fallback when graph-aware or richer retrieval is unavailable;
- expose enough structure for later compaction and evidence binding.

## Minimum Invariants

1. `GraphContextStore` is a supply boundary, not the owner of `View` semantics or `ContextPack` policy.
2. `ContextRetriever` returns candidates, not final visible truth.
3. Retrieval and storage stay separable from final window mutation and packing.
4. Graph-aware retrieval may be one major source of knowledge, but not the sole possible source.
5. Backend-specific capabilities may improve retrieval quality without redefining the GraphClaw concept model.
6. `src/memory/` may host or expose parts of these seams early, but should not absorb the entire context-governance layer.

## Errors, Rejection, And Degradation

These interfaces should support outcomes such as:

- graph traversal unavailable, so fall back to flatter recall;
- graph query partially constrained by view or policy limits;
- ranking or compaction hints unavailable, so return broader raw candidates;
- supporting resources missing, so preserve candidate references without full attachment;
- backend-specific optional procedures absent, so degrade to simpler traversal or filtering.

Those outcomes should remain visible to later trace or planning layers.

## Compatibility With The Inherited Runtime

The first migration step should preserve existing memory backends and recall behavior.

The compatibility rule is:

> the inherited memory system remains operational, while new graph-aware supply boundaries are introduced as additional providers of candidate context rather than as proof that memory and context governance have merged.

That means the first implementation can stay conservative:

- keep existing memory loading paths working;
- introduce graph-aware candidate retrieval as one more source;
- normalize outputs into a retrieval result shape before any final packing step;
- keep `prompt.rs` and turn orchestration consuming downstream packed or windowed results rather than raw store logic.

## Likely Source-Area Consumers

### `src/memory/`

Likely role:

- earliest home for storage and retrieval adapters, including graph-facing backends or bridge code.

Documentary caution:

- this area should remain a supplier of context material and storage support, not the owner of final visibility or packability policy.

### `src/agent/`

Likely role:

- consume normalized retrieval outputs through a context coordination seam instead of direct memory hydration assumptions.

### `docs/backends/`

Likely role:

- describe backend capability mapping after the GraphClaw interface meaning is fixed here.

## Initial Files To Treat As Seams

- `src/memory/traits.rs`
- `src/memory/backend.rs`
- `src/memory/embeddings.rs`
- `src/memory/vector.rs`
- `src/memory/chunker.rs`
- `src/memory/snapshot.rs`
- `src/agent/memory_loader.rs`
- `docs/backends/memgraph.md`

## Recommended Minimal Introduction Order

1. Normalize existing memory recall behind a retrieval result boundary.
2. Add a graph-aware store adapter boundary without removing current backends.
3. Introduce a retriever that can combine flat recall and graph candidates.
4. Add coarse fallback and degradation reporting.
5. Introduce later compaction and evidence-binding seams once candidate supply is stable.

## Slice JSON

This slice is an orientation artifact, not an implementation claim.

```json
{
  "nodes": [
    {
      "id": "n0",
      "position": { "x": -680, "y": -50 },
      "caption": "MemoryStore",
      "labels": ["Resource"],
      "properties": {
        "file_origin": "src/memory/backend.rs + concrete backends",
        "role": "Supplies flat recall, facts, snapshots, or cache-backed material"
      },
      "style": {}
    },
    {
      "id": "n1",
      "position": { "x": -330, "y": -180 },
      "caption": "GraphContextStore",
      "labels": ["Resource"],
      "properties": {
        "file_origin": "future graph-aware store seam in src/memory/",
        "role": "Supplies graph-structured candidates under constraints"
      },
      "style": {}
    },
    {
      "id": "n2",
      "position": { "x": -330, "y": 80 },
      "caption": "TaskIntent",
      "labels": ["TaskIntent"],
      "properties": {
        "file_origin": "src/agent/classifier.rs + future intent seam",
        "role": "Frames the retrieval need"
      },
      "style": {}
    },
    {
      "id": "n3",
      "position": { "x": 50, "y": -50 },
      "caption": "ContextRetriever",
      "labels": ["Action"],
      "properties": {
        "file_origin": "src/agent/memory_loader.rs + future context seam",
        "role": "Combines multi-source retrieval into normalized candidates"
      },
      "style": {}
    },
    {
      "id": "n4",
      "position": { "x": 420, "y": -50 },
      "caption": "Candidate Evidence Set",
      "labels": ["View"],
      "properties": {
        "file_origin": "future normalized retrieval artifact",
        "role": "Candidate material before final visibility and packing"
      },
      "style": {}
    },
    {
      "id": "n5",
      "position": { "x": 790, "y": -50 },
      "caption": "SessionWindow",
      "labels": ["SessionWindow"],
      "properties": {
        "file_origin": "future explicit runtime object",
        "role": "Consumes validated candidate material later in the turn"
      },
      "style": {}
    }
  ],
  "relationships": [
    {
      "id": "r0",
      "type": "HAS_TOPIC",
      "style": {},
      "properties": {},
      "fromId": "n0",
      "toId": "n3"
    },
    {
      "id": "r1",
      "type": "HAS_TOPIC",
      "style": {},
      "properties": {},
      "fromId": "n1",
      "toId": "n3"
    },
    {
      "id": "r2",
      "type": "RELATED_TO",
      "style": {},
      "properties": {},
      "fromId": "n3",
      "toId": "n2"
    },
    {
      "id": "r3",
      "type": "FULFILLED_BY",
      "style": {},
      "properties": {},
      "fromId": "n4",
      "toId": "n3"
    },
    {
      "id": "r4",
      "type": "SESSION_WINDOW_CONTAINS",
      "style": {},
      "properties": {},
      "fromId": "n5",
      "toId": "n4"
    }
  ]
}
```

## Related References

- `views-and-sets.md`
- `context-artifacts.md`
- `future-integration-seams.md`
- `../backends/memgraph.md`
