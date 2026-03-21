# Set

## Statut

Ce fichier est la source canonique de definition de `Set`.

Il releve de la documentation d'architecture cible.

Il ne pretend pas que la runtime heritee expose deja toutes les formes de `Set` decrites ici.

## Ancrages De Reference

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

## Representation Actuelle

Dans la representation graphe cible, un `Set` persiste peut etre represente par un noeud `(:Set)` avec une definition source ou composee.

Cette representation persiste est importante, mais elle ne doit pas etre prise pour l'unique lecture du concept.

## Formes De Definition Persistante

### Set Source

Un `Set` persiste sans relation de composition est defini par une expression Cypher source portee comme propriete `source_cypher` sur le noeud `(:Set)` lui-meme.

Exemples :

- `MATCH (n:Agent) RETURN n`
- `MATCH (n:Concept {type: "Finance"})<-[r]-(m) RETURN n, r, m`

Ces expressions doivent rester en lecture seule, bornees, documentees, et validees.

### Set Compose

Un `Set` persiste defini par composition porte une ou plusieurs relations de composition sortantes, toutes du meme type, vers des `Set` operandes.

| Relation type | Set-algebraic operation | Arity | Associative | Commutative | Idempotent |
| --- | --- | --- | --- | --- | --- |
| `UNION_OF` | A ∪ B ∪ C ∪ ... | n-ary (N ≥ 2) | yes | yes | yes |
| `INTERSECTION_OF` | A ∩ B ∩ C ∩ ... | n-ary (N ≥ 2) | yes | yes | yes |
| `DIFF_OF` | A \ B | binary (N = 2, ordered) | no | no | no |
| `SYMMETRIC_DIFF_OF` | A △ B | binary (N = 2) | yes | yes | no |

Comme `UNION_OF` et `INTERSECTION_OF` sont associatives et commutatives, un meme noeud `Set` peut referencer directement N ≥ 2 operandes.

Comme `DIFF_OF` n'est ni associative ni commutative, cette relation reste strictement binaire et ordonnee.

Comme la difference symetrique n-aire a une semantique moins lisible, `SYMMETRIC_DIFF_OF` reste ici limitee a une forme binaire.

Un `Set` compose peut aussi porter une propriete `additional_cypher` qui applique des ajustements supplementaires au resultat compose.

## Invariants De Composition

Un `Set` persiste doit avoir soit 0, soit N relations de composition sortantes avec N ≥ 2 :

- 0 relation : le `Set` est un `Set` source defini par `source_cypher` ;
- N ≥ 2 relations du meme type : le `Set` est un `Set` compose.

Regles minimales de validite structurelle :

- un `Set` avec exactement 1 relation de composition est invalide ;
- toutes les relations de composition d'un meme `Set` doivent etre du meme type ;
- pour `DIFF_OF` et `SYMMETRIC_DIFF_OF`, N doit etre exactement egal a 2 ;
- pour `UNION_OF` et `INTERSECTION_OF`, N peut prendre toute valeur ≥ 2.

## Priorite De Resolution

Le moteur resout un `Set` persiste dans cet ordre :

1. si le `Set` porte des relations de composition : resoudre tous les `Set` operandes, puis appliquer l'operation de composition ;
2. si le `Set` porte `additional_cypher` : l'appliquer au resultat compose ;
3. si le `Set` ne porte aucune relation de composition : executer `source_cypher` comme definition principale.

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

A la creation ou a la mise a jour, le moteur devrait pouvoir :

1. resoudre le `Set` sur un sous-graphe ou sur le graphe courant ;
2. produire un schema du resultat ;
3. verifier que ce schema est acceptable pour l'usage runtime vise.

Les metadonnees de validation devraient inclure au minimum le statut de validation, la date de validation, la version de schema, le hash de definition, le cout estime, et la politique de cache.
