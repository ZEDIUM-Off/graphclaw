# References

Ce fichier regroupe les references a utiliser avec le skill `graphclaw-graph-theory-memgraph`.

Les liens externes ci-dessous ont ete verifies pendant la creation du skill.

## Docs internes GraphClaw

Lire d'abord, selon le sujet :

- `README.md`
- `CONTEXT.md`
- `docs/architecture/README.md`
- `docs/architecture/concepts/graph-context-engine.md`
- `docs/architecture/concepts/views-and-sets.md`
- `docs/architecture/concepts/context-artifacts.md`
- `docs/architecture/runtime/turn-runtime-logic.md`
- `docs/architecture/interfaces/strategy-resolver-interface.md`
- `docs/backends/memgraph.md`
- `crates/memgraph/CONTEXT.md`

## Theorie des graphes

References generales :

- [Theorie des graphes - Wikipedia FR](https://fr.wikipedia.org/wiki/Th%C3%A9orie_des_graphes)
- [Graph theory - Wikipedia EN](https://en.wikipedia.org/wiki/Graph_theory)
- [Graph (discrete mathematics) - Wikipedia EN](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics))

References de travail utiles pour les operations :

- [Graph traversal - Wikipedia EN](https://en.wikipedia.org/wiki/Graph_traversal)
- [Shortest path problem - Wikipedia EN](https://en.wikipedia.org/wiki/Shortest_path_problem)
- [Centrality - Wikipedia EN](https://en.wikipedia.org/wiki/Centrality)

## Memgraph hubs

Points d'entree fournis par l'utilisateur :

- [Memgraph documentation pages](https://github.com/memgraph/documentation/tree/main/pages)
- [Fundamentals](https://github.com/memgraph/documentation/tree/main/pages/fundamentals)
- [Data modeling](https://github.com/memgraph/documentation/tree/main/pages/data-modeling)
- [Querying](https://github.com/memgraph/documentation/tree/main/pages/querying)
- [Data streams](https://github.com/memgraph/documentation/tree/main/pages/data-streams)
- [Advanced algorithms](https://github.com/memgraph/documentation/tree/main/pages/advanced-algorithms)
- [AI ecosystem](https://github.com/memgraph/documentation/tree/main/pages/ai-ecosystem)

## Memgraph pages a privilegier

### Modelisation semantique

- [Graph data model](https://github.com/memgraph/documentation/blob/main/pages/data-modeling/graph-data-model.mdx)
- [Graph data modeling best practices](https://github.com/memgraph/documentation/blob/main/pages/data-modeling/best-practices.mdx)

Utiliser ces pages pour :

- relier sommets, aretes, poids, labels et proprietes au modele LPG ;
- choisir entre propriete et relation ;
- eviter duplication, sur-modelisation et supernodes ;
- penser le graphe en fonction des parcours et des usages futurs.

### Requetes, plans et optimisation

- [Querying best practices](https://github.com/memgraph/documentation/blob/main/pages/querying/best-practices.mdx)
- [Query plan](https://github.com/memgraph/documentation/blob/main/pages/querying/query-plan.mdx)
- [Parallel execution](https://github.com/memgraph/documentation/blob/main/pages/querying/parallel-execution.mdx)
- [Functions](https://github.com/memgraph/documentation/blob/main/pages/querying/functions.mdx)
- [Indexes](https://github.com/memgraph/documentation/blob/main/pages/fundamentals/indexes.mdx)

Utiliser ces pages pour :

- verifier qu'un parcours ou une selection sont bien bornees ;
- comprendre l'impact des operateurs du plan ;
- utiliser `PROFILE` ;
- raisonner sur `project()` et les projections de sous-graphes ;
- choisir les indexes en fonction des parcours reels.

### Flux et rapidite d'execution

- [Kafka Connect](https://github.com/memgraph/documentation/blob/main/pages/data-streams/kafka.mdx)
- [Graph stream processing with Kafka](https://github.com/memgraph/documentation/blob/main/pages/data-streams/graph-stream-processing-with-kafka.mdx)
- [Manage streams query](https://github.com/memgraph/documentation/blob/main/pages/data-streams/manage-streams-query.md)
- [Transformation modules](https://github.com/memgraph/documentation/blob/main/pages/data-streams/transformation-modules.mdx)

Utiliser ces pages pour :

- concevoir des mises a jour graphe quasi temps reel ;
- penser la fraicheur du contexte et les mutations de graphe ;
- distinguer ingestion, transformation et projection runtime.

### Parcours et algorithmes

- [Deep path traversal](https://github.com/memgraph/documentation/blob/main/pages/advanced-algorithms/deep-path-traversal.mdx)
- [Run algorithms](https://github.com/memgraph/documentation/blob/main/pages/advanced-algorithms/run-algorithms.mdx)
- [Available algorithms](https://github.com/memgraph/documentation/blob/main/pages/advanced-algorithms/available-algorithms.mdx)

Utiliser ces pages pour :

- choisir entre BFS, DFS et shortest path ;
- executer un algorithme sur un graphe projete plutot que sur tout le graphe ;
- appliquer ranking, communaute ou similarite a un sous-domaine pertinent.

### IA et contexte graphe

- [GraphRAG with Memgraph](https://github.com/memgraph/documentation/blob/main/pages/ai-ecosystem/graph-rag.mdx)
- [MCP](https://github.com/memgraph/documentation/blob/main/pages/ai-ecosystem/mcp.mdx)
- [Skills](https://github.com/memgraph/documentation/blob/main/pages/ai-ecosystem/skills.mdx)
- [Agents](https://github.com/memgraph/documentation/blob/main/pages/ai-ecosystem/agents.mdx)

Utiliser ces pages pour :

- relier le contexte graphe a des usages LLM et GraphRAG ;
- montrer que la valeur ne vient pas seulement du stockage, mais aussi des relations, parcours et sous-graphes ;
- cadrer l'usage de Memgraph comme support de contexte structure, sans confondre backend et moteur conceptuel GraphClaw.

## Regle d'usage

Quand une explication ou une conception touche fortement la theorie des graphes ou Memgraph :

1. citer au moins un document interne GraphClaw ;
2. citer au moins un lien de theorie des graphes ;
3. citer au moins un lien Memgraph precis ;
4. expliciter le mapping :
   - principe graphe
   - concept GraphClaw
   - mecanisme Memgraph
   - application concrete
