# Set System Spec v0.1

## Status

This document is the **first documentary pass** on the Set concept for GraphClaw. It defines the minimal Set lifecycle, composition algebra, and LLM export formats for the initial playground implementation. It is a deliberately bounded v0: it does not specify the full Context Engine, SessionWindow, ContextPack, or governance.

This spec supersedes the earlier `view-system-spec-v0.md` following the vocabulary pivot in Revision v0.1. The current playground code still uses `ViewTemplate` / `ResolvedView` naming; a future code migration will align those types with this specification.

For the broader operational semantics of Set and View (governed perimeter, packability, lazy vs materialized), see [views-and-sets.md](../concepts/views-and-sets.md). For terminology routing, see [glossary.md](../concepts/glossary.md).

## Why This Document Exists

GraphClaw's core is a **composable set system**, not just an agent wired to a graph DB. This spec stabilises, for the first implementation slice:

- what a Set is and is not;
- the lifecycle from definition to bound to resolved to export;
- the v0 composition algebra;
- the initial LLM export formats and their purpose.

The playground is the primary consumer: an atelier for composing and materialising Sets, not a generic graph visualiser or a full Context Engine UI.

---

## 1. Minimal Definition of Set

A **Set** is a **persisted, governed, declarative expression of an ensemble over the graph**.

It can be:

- **Extensional**: it explicitly enumerates certain nodes and/or relations.
- **Intensional**: it describes selection rules, filters, expansion, projection, or composition.

### What a Set Is Not

A Set is **not**:

- a raw permission;
- a plain Cypher query (though it may be resolved via Cypher);
- prompt text;
- a UI component;
- a rigid OO class hierarchy;
- a raw dump of query results;
- a runtime-only transient object (that is a `View`).

### What a Set Enables

A Set describes a **persisted logical surface** for access, exploration, grouping, or projection over a graph. It should support needs such as:

- "see only shared business logic";
- "exclude brand-specific concepts";
- "expand a neighbourhood along certain edge types";
- "project a subset into a compact form for an LLM".

### Set vs Security

A Set does not, by itself, carry the full security of the system. Real enforcement belongs to the engine. The Set is the **persisted governed surface** on which the engine relies to authorise or bound navigation.

---

## 2. Set Lifecycle (Minimal)

A Set is best thought of as a **cycle**, not a single object.

```mermaid
flowchart LR
    SetDef["Set (persisted)"]
    BoundSet[BoundSet]
    ResolvedSet[ResolvedSet]
    SetExport[SetExport]

    SetDef --> BoundSet
    BoundSet --> ResolvedSet
    ResolvedSet --> SetExport
```

### 2.1 Set (persisted definition)

Source definition: reusable, editable, composable, stored in the graph database.

**Minimal fields**:

- `id`
- `name`
- `kind` (see section 3)
- `description`
- `extends` (optional list of Set IDs for template inheritance)
- `operations` (composition/transformation steps)
- `selectors` (how to select initial nodes/relations)
- `filters` (predicates)
- `projections` (optional)
- `source_cypher` (optional: Cypher expression for source Sets)
- `additional_cypher` (optional: Cypher adjustment for composed Sets)
- metadata: default cost/limits

### 2.2 BoundSet

Local instantiation of a Set definition with:

- **Anchors**: e.g. node IDs or labels to bind to.
- **Parameters**: key-value overrides.
- **Resolution scope**: which graph or workspace (and optionally agent/instance) to resolve against.

Example: a generic Set `business_logic_shared` bound to workspaces `HIGHFINITY` and `MC_STUDIO`.

### 2.3 ResolvedSet

The result of actually materialising the Set on the graph.

**Minimal content**:

- nodes retained;
- relations retained;
- composition trace (how this result was built);
- completeness level;
- any degradations applied;
- estimated cost.

### 2.4 SetExport

A projection of a ResolvedSet into a form for external use. In v0, the primary consumer is:

- human inspection;
- explanation to a coding agent;
- injection into LLM context.

### 2.5 Resolution Invariants (v0)

For the playground slice, a `ResolvedSet` should be treated as a **materialised graph snapshot** derived from a Set resolution. It is still narrower than a future runtime `View` or `ContextPack`.

The minimum invariants are:

- **closed subgraph**: every retained relation must connect retained nodes;
- **bounded traversal**: expansion depth, relation families, and limits must stay explicit;
- **deterministic ordering**: nodes and relations should be emitted in a stable order whenever possible;
- **traceability**: the composition trace must explain the major resolution steps;
- **explicit degradation**: partial or conservative outcomes must be surfaced as degradations, not hidden behind silent behavior.

These invariants are the minimum needed to keep the result graph-theoretically sane while preserving the broader GraphClaw distinction:

1. the `Set` defines a governed, persisted surface;
2. the playground materialises one resolved result from that surface;
3. the resulting export is still **not** a full `ContextPack`.

---

## 3. Minimal Set Typology (v0)

Keep three categories; do not overload.

| Kind | Role | Examples |
|------|------|----------|
| **semantic** | Semantic or domain Set | Business logic, product concepts, regulatory categories |
| **boundary** | Logical boundary Set | Shared space HIGHFINITY + MC_STUDIO, "business only" subset |
| **projection** | Export/rendering oriented | Summary of retained concepts, compact LLM export, explanatory export |

A `runtime` kind may be introduced later; it is not in scope for this lot.

---

## 4. Composition Algebra (v0)

Composition is the heart of the playground. A Set must be composable from other Sets and set operations in a traceable, bounded way.

### 4.1 Operations Supported in v0

The set engine must support at least:

| Operation | Arity | Description |
|-----------|-------|-------------|
| `union(a, b, ...)` | n-ary (N ≥ 2) | A ∪ B ∪ C ∪ ... -- associative, commutative, idempotent |
| `intersection(a, b, ...)` | n-ary (N ≥ 2) | A ∩ B ∩ C ∩ ... -- associative, commutative, idempotent |
| `difference(a, b)` | binary, ordered | A \ B -- relative complement, non-commutative |
| `symmetric_difference(a, b)` | binary | A △ B = (A \ B) ∪ (B \ A) -- commutative |
| `expand(set, relation, depth)` | unary | Expand from a Set along a relation type to a given depth |
| `filter_nodes(set, predicate)` | unary | Restrict nodes by predicate |
| `filter_edges(set, predicate)` | unary | Restrict edges by predicate |
| `project(set, mode)` | unary | Project Set (e.g. nodes only, or summarised) |
| `slice(set, limit, order)` | unary | Limit and order (e.g. top-k) |

### 4.2 Composition Invariant

For persisted Set composition via graph relations:

- A Set has either 0 or N (N ≥ 2) outgoing composition relations.
- All composition relations on a single Set must be of the same type (homogeneous).
- `UNION_OF` and `INTERSECTION_OF` are n-ary (N ≥ 2): associative, commutative, idempotent.
- `DIFF_OF` is binary (N = 2) and ordered: operand role (`base` / `subtracted`) must be explicit on the relation.
- `SYMMETRIC_DIFF_OF` is binary (N = 2).
- 1 relation or mixed types are structurally invalid.

### 4.3 Constraints

This algebra must remain:

- explicit;
- bounded;
- serialisable;
- traceable;
- as deterministic as possible.

Do not introduce an arbitrary DSL at this stage.

### 4.4 Inheritance / Extension

Avoid a heavy OO hierarchy. Prefer declarative composition:

- `extends: []` for light inheritance (e.g. extend a list of Set IDs);
- `compose:` for union / intersection / difference;
- `operations:` for transform steps (expand, filter, project, slice).

---

## 5. LLM Export: Purpose and Form

### 5.1 Purpose

The goal is not yet to produce the final full prompt. It is to produce a **textual and structured artifact** that allows an LLM to understand:

- what the Set represents;
- what it contains;
- how it was built;
- what it excludes;
- how to use it as context.

### 5.2 Two Export Formats

#### A. `llm_compact`

Dense, short, suitable for injection.

Example target shape:

```yaml
set_id: business_logic_shared
set_kind: projection
purpose: Shared business logic across bound workspaces
nodes:
  - Pricing
  - Margin
  - Forecasting
edges:
  - Pricing RELATES_TO Margin
  - Forecasting DEPENDS_ON Pricing
constraints:
  - business-only
  - no brand-specific nodes
usage_hint: Use for business reasoning only.
```

#### B. `llm_explained`

Narrative, more readable, for documentation and discussion with an agent.

Example target shape:

```md
# Set Export — business_logic_shared

## Role
Shared business logic between bound workspaces.

## Included concepts
- Pricing
- Margin
- Forecasting

## Excluded concepts
- Brand-specific product identity
- Activity-specific operational details

## Relations
- Pricing RELATES_TO Margin
- Forecasting DEPENDS_ON Pricing
```

### 5.3 Rule

LLM export must **not** be a raw DB dump. It must be:

- compact;
- stable;
- traceable;
- understandable;
- usage-oriented.

---

## 6. Explicit v0 Limits

This lot does **not** fix or implement:

- full **ContextPack**;
- **SessionWindow**;
- **ContextMutationProposal**;
- full **PromptProjection**;
- the complete Context Engine.

Those can follow once the Set system is clarified in practice.

`SetExport` in this document is therefore a **playground-facing external artifact**, not the final model-visible packed context of the future engine.

---

## 7. Concrete Composition Examples

### 7.1 Example: Three Base Sets

- **set_highfinity_core**: semantic Set over HIGHFINITY concepts (e.g. nodes with label `Concept` and property `workspace: "HIGHFINITY"`).
- **set_mc_studio_core**: semantic Set over MC_STUDIO concepts.
- **set_business_class**: boundary Set that selects only "business" concepts (e.g. by label or tag), excluding brand-specific or activity-specific nodes.

### 7.2 Example: Composed Set

1. **Union**: `union(set_highfinity_core, set_mc_studio_core)` -- all concepts from both workspaces.
2. **Projection**: apply "business only" -- restrict to nodes that belong to the business_class (intersection with set_business_class or filter_nodes by predicate).
3. **Exclusion**: remove brand-specific nodes (e.g. `difference(..., set_brand_specific)` or filter_nodes excluding certain labels).
4. **Resolve** the composed Set on the current graph -- ResolvedSet.
5. **Export** as `llm_compact` and `llm_explained` for use as LLM context.

This is the kind of flow the playground must support end-to-end.

---

## 8. Code Migration Note

The current playground implementation (`crates/views/`, `src/gateway/playground.rs`, `web/src/`) still uses the `ViewTemplate` / `BoundView` / `ResolvedView` / `ViewsService` naming from the pre-revision vocabulary. A future code scaffold plan will rename these types to align with this specification. Until that migration, treat the code-level `View*` names as implementation aliases for the `Set` concepts defined here.

---

## References

- [views-and-sets.md](../concepts/views-and-sets.md) -- operational semantics of Set, View, packability, lazy vs materialised.
- [glossary.md](../concepts/glossary.md) -- terminology routing index.
- [graph-context-engine.md](../concepts/graph-context-engine.md) -- target concept model (Set System v0 is a subset for the playground, not the full engine).
