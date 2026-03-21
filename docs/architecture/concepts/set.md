# Set

## Status

This file is the canonical definition source for `Set`.

It is target-architecture documentation. It does not claim that the inherited runtime already exposes every form of `Set` described here.

## Reference Anchors

- graph theory reference: [`../../../.agents/skills/graphclaw/main_graphes/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/markdown.md)
- graph as a relation on a set of vertices: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-5/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-5/markdown.md)
- directed graph, neighbors, predecessors, successors: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-6/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-6/markdown.md)
- subgraphs and bounded graph work: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md)

## Definition

Un `Set` est l'abstraction d'ensemble graphe gouverne, composable et validable qui sert de brique de base pour les perimetres de contexte, les frontieres de navigation, et la composition d'ensembles dans GraphClaw.

## Est

Un `Set` est :

- un ensemble gouverne sur le graphe ;
- listable et referencable ;
- composable avec d'autres `Set` par relations de definition ;
- exploitable comme frontiere gouvernee ;
- validable par schema ;
- source d'un travail runtime ulterieur via des [`View`](view.md).

## N'est pas

Un `Set` n'est pas :

- un simple alias de requete ad hoc ;
- necessairement un objet persiste ;
- necessairement un objet runtime transitoire ;
- identique a une [`View`](view.md) ;
- un [`ResolvedSet`](resolved-set.md) ;
- un `ContextPack`.

## Invariants

- le concept canonique est l'abstraction `Set` elle-meme ;
- les objets `Set` persistes restent une forme primaire et attendue de cette abstraction ;
- la persistance ne doit pas etre confondue avec le concept ;
- un `Set` ne sert pas seulement de frontiere statique d'agent ; il peut aussi servir d'operande gouverne dans des compositions de travail.

## Representation Actuelle

Dans la representation graphe cible, un `Set` persiste peut etre represente par un noeud `(:Set)` avec une definition source ou composee.

Cette representation persiste est importante, mais elle ne doit pas etre prise pour l'unique lecture du concept.

## Persisted Definition Forms

### Source Set

A persisted Set with no composition relations is defined by a source Cypher expression carried as a property (`source_cypher`) on the `(:Set)` node itself.

Examples:

- `MATCH (n:Agent) RETURN n`
- `MATCH (n:Concept {type: "Finance"})<-[r]-(m) RETURN n, r, m`

These expressions must be read-only, bounded, documented, and validated.

### Composed Set

A persisted Set defined by composition has one or more outgoing composition relations, all of the same type, pointing to operand Sets.

| Relation type | Set-algebraic operation | Arity | Associative | Commutative | Idempotent |
| --- | --- | --- | --- | --- | --- |
| `UNION_OF` | A ∪ B ∪ C ∪ ... | n-ary (N ≥ 2) | yes | yes | yes |
| `INTERSECTION_OF` | A ∩ B ∩ C ∩ ... | n-ary (N ≥ 2) | yes | yes | yes |
| `DIFF_OF` | A \ B | binary (N = 2, ordered) | no | no | no |
| `SYMMETRIC_DIFF_OF` | A △ B | binary (N = 2) | yes | yes | no |

Because `UNION_OF` and `INTERSECTION_OF` are associative and commutative, a single Set node can reference N ≥ 2 operands directly.

Because `DIFF_OF` is neither associative nor commutative, it is strictly binary and ordered.

Because n-ary symmetric difference has non-obvious semantics, `SYMMETRIC_DIFF_OF` remains restricted to binary form here.

A composed Set may also carry an `additional_cypher` property that applies further adjustments to the composed result.

## Composition Invariant

A persisted `Set` must have either 0 or N (N ≥ 2) outgoing composition relations:

- 0 relations: the Set is a source Set, defined by `source_cypher`;
- N ≥ 2 relations of the same type: the Set is a composed Set.

Structural validity rules:

- a Set with exactly 1 composition relation is invalid;
- all composition relations on a single Set must be of the same type;
- for `DIFF_OF` and `SYMMETRIC_DIFF_OF`, N must be exactly 2;
- for `UNION_OF` and `INTERSECTION_OF`, N may be any value ≥ 2.

## Resolution Priority

The engine resolves a persisted Set in this order:

1. if the Set has composition relations: resolve all operand Sets, then apply the composition operation;
2. if the Set has `additional_cypher`: apply it to the composed result;
3. if the Set has no composition relations: execute `source_cypher` as the primary definition.

## Relations

### Definition Relations

- `(:Set)-[:UNION_OF]->(:Set)`
- `(:Set)-[:INTERSECTION_OF]->(:Set)`
- `(:Set)-[:DIFF_OF {role: "base"|"subtracted"}]->(:Set)`
- `(:Set)-[:SYMMETRIC_DIFF_OF]->(:Set)`

### Navigation / Exposition Relations

- `EXPOSES_SET`
- `OVERLAPS_WITH`
- `NEIGHBORS_SET`
- `TRAVERSES_TO`

These do not affect resolution.

## Validation

At creation or update, the engine should be able to:

1. resolve the Set on a subgraph or the current graph;
2. produce a schema of the result;
3. verify that the schema is acceptable for the intended runtime usage.

Validation metadata should include validation status, validation date, schema version, definition hash, estimated cost, and cache policy.
