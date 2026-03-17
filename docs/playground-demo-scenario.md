# GraphClaw Playground — Demo scenario v0

This document describes a minimal end-to-end scenario for the View System playground: create demo graph data, define views, resolve, and export for LLM.

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

Then read node ids (e.g. `MATCH (n:Concept) RETURN id(n), n.name`) to use as anchors in views.

## 2. Define view templates

In the Playground **View** tab:

1. **view_business_class**
   - Id: `view_business_class`
   - Name: `Business concepts only`
   - Kind: `boundary`
   - Description: `Concepts used for business reasoning (Pricing, Margin, Forecasting).`
   - Node ids: the comma-separated ids of Pricing, Margin, Forecasting (e.g. `0, 1, 2`).
   - Create view.

2. (Optional) **view_highfinity_core** and **view_mc_studio_core**: same pattern with different node id sets if you have more nodes.

## 3. Resolve a view

In the **Resolved** tab:

1. Template id: `view_business_class`.
2. Anchors: either leave empty (template’s selectors used) or JSON: `{"node_ids": [0, 1, 2]}` (replace with your ids).
3. Click **Resolve**. You should see the resolved nodes and edges (and composition trace).

## 4. Export for LLM

In the **Export LLM** tab:

1. Ensure a view is resolved (step 3) so that “Resolved view” state is set.
2. Choose format: `llm_compact` or `llm_explained`.
3. Click **Export**. The text appears below (YAML-like or Markdown).
4. Use **Copy to clipboard** to inject the result into an LLM context.

## Expected export shapes

- **llm_compact**: `view_id`, `view_kind`, `purpose`, `nodes`, `edges`, `constraints`, `usage_hint`.
- **llm_explained**: Markdown with Role, Included concepts, Excluded concepts, Relations.

## Success criteria

- Notion of View is clearer after using the playground.
- Composition (selectors + operations) is concretely manipulable.
- Export is usable as context for another agent or for documentation.

## References

- [View System Spec v0](../architecture/view-system-spec-v0.md)
- [Views and sets](../architecture/views-and-sets.md)
- [Memgraph backend](backends/memgraph.md)
