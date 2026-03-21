# Packability

## Status

This file is the canonical definition source for `Packable Subgraph`, `Packability`, `Bounded Complement`, `Condensation`, and `Projection Into A Packable Subgraph`.

It is target-architecture documentation.

## Reference Anchors

- graph theory reference: [`../../../.agents/skills/graphclaw/main_graphes/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/markdown.md)
- paths and shortest paths: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md)
- connectivity, cuts, articulation, Menger: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-44/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-44/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-46/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-46/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-49/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-49/markdown.md)

## Packable Subgraph

### Definition

Un sous-graphe packable est une projection bornee derivee d'une [`View`](view.md), preparee pour une inclusion possible dans le `ContextPack` final.

### Est

Un sous-graphe packable est :

- derive d'une ou plusieurs [`View`](view.md) ;
- plus proche de l'artefact final visible par le modele qu'une `View` exploratoire ;
- borne par politique, budget, et intelligibilite.

### N'est pas

Un sous-graphe packable n'est pas :

- une [`View`](view.md) complete ;
- le graphe de session entier ;
- le `ContextPack` lui-meme.

## Packability

### Definition

La packabilite est la propriete conditionnelle selon laquelle un element ou un sous-graphe peut etre retenu dans une projection finale utile au modele.

### Est

La packabilite depend au minimum :

- de la `View` active ;
- du [`Set`](set.md) de bounding et des contraintes de politique ;
- du besoin du tour ;
- du budget ;
- du fait qu'une forme resumee soit ou non autorisee ;
- du fait que le contexte relationnel necessaire soit ou non preservable.

### N'est pas

La packabilite n'est pas :

- une propriete intrinseque et absolue d'un noeud ;
- un synonyme de navigabilite ;
- un synonyme de visibilite immediate.

## Bounded Complement

### Definition

Le complement borne retire un ensemble d'un univers de travail explicitement borne, comme la [`View`](view.md) courante, une fenetre de session, ou un ensemble de candidats limite par politique.

### Est

Le complement borne est :

- une operation de travail gouvernee ;
- toujours relatif a un univers explicite ;
- utile pour exclure localement sans supposer "tout le reste du graphe".

### N'est pas

Le complement borne n'est pas :

- "tout dans la base sauf ceci" ;
- une negation globale sur le graphe entier.

## Condensation

### Definition

La condensation reduit un sous-graphe large ou redondant en une structure plus petite tout en preservant les distinctions requises pour la reflexion ou la projection finale.

### Est

La condensation est :

- une reduction structurelle ;
- un outil de budget et d'intelligibilite ;
- un passage important entre exploration large et projection plus serree.

### N'est pas

La condensation n'est pas :

- un simple tri par score ;
- une suppression aveugle de noeuds.

## Projection Into A Packable Subgraph

### Definition

La projection vers un sous-graphe packable transforme un ensemble logique ou une [`View`](view.md) en un sous-graphe borne qui conserve les noeuds, relations, provenance, et liens resumes necessaires a une future inclusion dans le `ContextPack`.

### Est

Cette projection est :

- intermediaire entre exploration et artefact final ;
- gouvernee par budget, droits, et intelligibilite ;
- distincte de la simple navigation.

### N'est pas

Cette projection n'est pas :

- une simple serialisation brute de la `View` ;
- une garantie d'inclusion finale.

## Relationship To `Set`, `View`, And `ContextPack`

The stable sequence is:

1. one or more [`Set`](set.md) objects define governed graph-bounded matter;
2. a runtime [`View`](view.md) derives from one or more resolved or composed Sets;
3. Views are manipulated, composed, filtered, and refined at runtime;
4. a packable subgraph is derived from some of those Views;
5. the final `ContextPack` retains only the budgeted result of that derivation.
