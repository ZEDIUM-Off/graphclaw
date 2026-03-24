# GraphClaw Playground — Demo scenario v0

This document describes a minimal end-to-end scenario for the Set-first playground: create demo graph data, define sets, resolve them, and export for LLM use.

## Prerequisites

- Memgraph running: `make memgraph-up` (or `docker compose up -d` in `memgraph/`).
- Gateway running with playground enabled: `make run-gateway` (or `cargo run -- gateway`).
- Web dashboard: open `/playground` (e.g. http://127.0.0.1:8080/playground after pairing).

## 1. Create demo graph data

Either use the Playground UI (Graph tab) or run Cypher in Memgraph Lab (port 9001).

### Option A: Playground UI

1. **Create nodes** (Graph tab — Create node):
   - Labels: `Concept`, Name: `Pricing` → Create node (note the returned `id`, e.g. 0).
   - Labels: `Concept`, Name: `Margin` → Create node (id 1).
   - Labels: `Concept`, Name: `Forecasting` → Create node (id 2).
   - Labels: `Concept`, Name: `BrandIdentity` → Create node (id 3).

2. **Create edges** (Create edge):
   - From id: 0, To id: 1, Relation type: `RELATES_TO`.
   - From id: 2, To id: 0, Relation type: `DEPENDS_ON`.
   - (Optional) From 0 to 3: `RELATES_TO` to include a brand concept.

### Option B: Cypher (Memgraph Lab or mgconsole)

Run in Memgraph (e.g. Memgraph Lab → Query):

```cypher
CREATE (p:Concept {name: 'Pricing'});
CREATE (m:Concept {name: 'Margin'});
CREATE (f:Concept {name: 'Forecasting'});
CREATE (b:Concept {name: 'BrandIdentity'});
MATCH (p:Concept {name: 'Pricing'}), (m:Concept {name: 'Margin'}) CREATE (p)-[:RELATES_TO]->(m);
MATCH (f:Concept {name: 'Forecasting'}), (p:Concept {name: 'Pricing'}) CREATE (f)-[:DEPENDS_ON]->(p);
```

Then read node ids (e.g. `MATCH (n:Concept) RETURN id(n), n.name`) to use as anchors in sets.

## 2. Define sets

In the Playground `Graph` tab, create selections and turn them into sets. In the `Data > Sets` table, confirm that they were persisted.

1. **shared_business_class**
   - Id: `shared-business-class`
   - Name: `Business concepts only`
   - Kind: `boundary`
   - Description: `Concepts used for business reasoning (Pricing, Margin, Forecasting).`
   - Node ids: the comma-separated ids of Pricing, Margin, Forecasting (e.g. `0, 1, 2`).
   - Create set from selection.

2. (Optional) Create other sets the same way from alternative selections.

## 3. Resolve a set

In the `Sets` table:

1. Activate `shared-business-class`.
2. The graph map should narrow to the resolved subgraph for that set.
3. The right inspector should show the composition trace and metadata for the active set.

## 4. Export for LLM

Call the playground export route for the active resolved set:

1. Resolve the set first (step 3).
2. POST the resolved payload to `/api/playground/sets/export`.
3. Choose format: `llm_compact` or `llm_explained`.
4. Use the returned text as agent context or documentation material.

## Expected export shapes

- **llm_compact**: `set_id`, `set_kind`, `purpose`, `nodes`, `edges`, `constraints`, `usage_hint`.
- **llm_explained**: Markdown with Role, Included concepts, Excluded concepts, Relations.

## Success criteria

- Notion of Set is clearer after using the playground.
- Composition (selectors + operations) is concretely manipulable.
- Export is usable as context for another agent or for documentation.

## References

- [Set System Spec v0](../architecture/playground/set-system-spec-v0.md)
- [Set / View family](../architecture/concepts/views-and-sets.md)
- [Memgraph backend](backends/memgraph.md)
