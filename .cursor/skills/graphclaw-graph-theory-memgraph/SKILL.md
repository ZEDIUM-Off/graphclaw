---
name: graphclaw-graph-theory-memgraph
description: Relie les concepts de theorie des graphes aux concepts d'architecture GraphClaw et aux mecanismes Memgraph. Use when writing or reviewing GraphClaw architecture docs, designing semantic graph models in Memgraph, composing View/GraphSet based context flows, or optimizing Cypher traversals, projections, and graph algorithms.
---

# GraphClaw, theorie des graphes et Memgraph

## But

Ce skill sert a relier trois couches sans les confondre :

1. theorie des graphes ;
2. concepts GraphClaw ;
3. mecanismes concrets Memgraph.

Il doit aider a concevoir, expliquer ou revoir des operations graphe en gardant une trace claire entre le concept, la semantique d'orchestration de contexte, et la mise en oeuvre pratique.

## Quand l'utiliser

Utiliser ce skill quand la demande porte sur :

- la documentation d'architecture GraphClaw ;
- la modelisation semantique d'un graphe dans Memgraph ;
- des `View`, `GraphSet`, `SessionWindow`, `ThinkingContext` ou `ContextPack` ;
- des parcours, projections, classements, chemins, sous-graphes ou algorithmes ;
- des requetes Cypher optimisees pour des operations graphe ;
- la facon d'ancrer GraphClaw dans une lecture plus explicite de la theorie des graphes.

## Regles de lecture

Avant de produire une reponse substantielle ou une modification importante :

1. Lire les docs internes GraphClaw les plus proches du sujet.
2. Consulter au moins une reference de theorie des graphes et au moins une reference Memgraph pertinentes depuis `references.md`.
3. Ne pas ecrire comme si l'architecture cible GraphClaw etait deja entierement implementee.

## Ordre de raisonnement obligatoire

Toujours raisonner dans cet ordre :

1. concept GraphClaw ;
2. besoin de capacite graphe ;
3. principe de theorie des graphes ;
4. mecanisme Memgraph ;
5. impact runtime, budget, trace ou mutation.

Ne jamais partir d'un catalogue Memgraph pour retro-definir le modele GraphClaw.

## Noyau conceptuel GraphClaw

Quand le sujet touche l'orchestration de contexte, garder ces distinctions nettes :

- un `View` est une projection gouvernee sur le graphe ; ce n'est pas une simple requete ni un simple sous-graphe ;
- un `GraphSet` est un ensemble logique de travail dans un `View` ;
- un sous-graphe packable est derive d'un ou plusieurs `GraphSet` ;
- un `ContextPack` est l'artefact final budgete visible par le modele ;
- un `ResolutionTrace` enregistre les choix, degradations, refus et selections.

Ne pas ecrire `View = subgraph`. Au mieux :

- le `View` borne l'espace gouverne ;
- le `GraphSet` manipule des ensembles dans cet espace ;
- un sous-graphe packable peut etre derive ensuite ;
- le `ContextPack` n'en garde qu'une forme finale budgetee.

## Table de correspondance utile

Traiter la table ci-dessous comme une approximation guidee, pas comme une identite stricte.

| Theorie des graphes | GraphClaw | Memgraph |
| --- | --- | --- |
| graphe | espace relationnel gouverne ou backend graphe | graphe LPG stocke |
| sommet / vertex | noeud de contexte ou entite | node |
| arete / edge | relation utile a la navigation ou a la selection | relationship |
| parcours | exploration dans un `View` via `GraphSet` | `MATCH`, expansions, path search |
| sous-graphe | projection travaillee ou packable | projection de graphe, sous-ensemble matche |
| chemin | chaine de dependance, de provenance ou de contexte | path pattern, shortest path, deep traversal |
| poids / cout | budget, score, priorite, degradation | proprietes, score de requete, ranking |
| centralite / importance | signal de priorisation | PageRank et autres procedures |
| composante / communaute | regroupement ou frontiere structurelle | algorithmes MAGE ou logique applicative |

## Workflow de reponse

Quand la demande est de nature architecture, doc, design, requete ou revue, structurer l'analyse avec ce canevas :

### 1. Principe

- quel principe de theorie des graphes est mobilise ;
- pourquoi il est pertinent ici.

### 2. Traduction GraphClaw

- quel concept GraphClaw porte ce principe ;
- quelle limite de gouvernance, de budget ou de trace s'applique.

### 3. Traduction Memgraph

- quelle structure LPG, requete Cypher, projection ou procedure Memgraph supporte ce besoin ;
- quelles contraintes de perf ou de modelisation apparaissent.

### 4. Application concrete

- ce qu'il faut modeliser ;
- ce qu'il faut requeter ;
- ce qu'il faut garder lazy, materialise, packable ou trace.

### 5. Limites

- ce qui reste conceptuel dans GraphClaw ;
- ce qui est implementation actuelle ;
- ce qui depend d'un choix backend ou edition Memgraph.

## Heuristiques de modelisation

Pour la modelisation semantique dans Memgraph :

- partir des entites et des relations utiles aux parcours, pas des tables heritees ;
- mettre une propriete sur un noeud ou une relation seulement si elle decrit cet element lui-meme ;
- transformer en relation ce qui doit etre partage, parcours, ou filtre frequemment ;
- eviter les supernodes et la duplication de donnees ;
- penser en termes de requetes futures, de projection de sous-graphe et de budget de contexte.

Pour GraphClaw :

- documenter les seeds, filtres, relations autorisees et couts attendus d'un `GraphSet` ;
- distinguer `GraphSet` lazy et materialise ;
- expliciter quand un sous-graphe devient packable ;
- conserver la difference entre exploration de `ThinkingContext` et resultat final `ContextPack`.

## Heuristiques de requetes et parcours

Pour les requetes Memgraph :

- borner les patterns par labels, types de relations et proprietes ;
- verifier le plan de requete avant de conclure qu'une strategie est bonne ;
- utiliser `PROFILE` pour observer les operateurs et les goulets d'etranglement ;
- utiliser une projection de graphe quand un algorithme doit s'appliquer a un sous-domaine plutot qu'au graphe entier ;
- choisir BFS, DFS, shortest path, ranking ou communaute selon le besoin de contexte, pas par effet de mode.

## Regles de formulation

Quand tu relies theorie des graphes et GraphClaw :

- parler de GraphClaw comme d'un projet ancre dans un modele graphe et une orchestration de contexte gouvernee ;
- distinguer clairement architecture cible, runtime actuel et support backend ;
- citer les docs internes pertinentes ;
- citer des liens externes reels depuis `references.md` pour toute affirmation importante sur la theorie des graphes ou Memgraph.

## References

Lire `references.md` pour les liens externes et la liste des pages Memgraph a privilegier.
