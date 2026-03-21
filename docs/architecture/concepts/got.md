# Graph Of Thoughts

## Statut

Ce document cadre l'usage mono-agent de `Graph of Thoughts` dans GraphClaw.

Il releve de la documentation d'architecture cible.

Il ne pretend pas que la runtime heritee execute deja cette boucle.

## Ancrage De Reference

- reference GoT locale : [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)

Les sections les plus utiles sont :

- section 1 pour la structure de graphe arbitraire et les transformations supportees par graphe ;
- section 3.1 pour le modele dirige du raisonnement ;
- section 3.2 pour `Aggregate`, `Refine`, et `Generate` ;
- section 3.3 pour le scoring et le ranking ;
- section 4.5 pour la distinction entre `Graph of Operations` et `Graph Reasoning State`.

Les pages de theorie des graphes les plus utiles ici sont :

- graphe comme relation dirigee sur un ensemble de sommets : [`../../../.agents/skills/graphclaw/main_graphes/pages/page-5/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-5/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-6/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-6/markdown.md) ;
- ordre topologique et lecture acyclique des dependances : [`../../../.agents/skills/graphclaw/main_graphes/pages/page-46/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-46/markdown.md) ;
- intuition de ranking : [`../../../.agents/skills/graphclaw/main_graphes/pages/page-87/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-87/markdown.md).

## Definition

Dans GraphClaw, GoT designe le graphe runtime des pensees, branches, refinements, aggregations, et choix intermediaires qui guide la recomposition de la [`View`](view.md) active et la recompilation du [`ContextPack`](../interfaces/context-pack-interface.md) au fil d'un turn.

## Est

GoT est :

- un mode d'orchestration de la reflexion ;
- un graphe de pensees runtime ;
- un support de branchement, de refinement, d'agregation, et de ranking ;
- un guide pour recomposer la [`View`](view.md) et les [`ContextFrame`](context-frame.md) utiles ;
- un mecanisme qui peut entrainer plusieurs recompositions du [`ContextPack`](../interfaces/context-pack-interface.md) dans un meme turn.

## N'est pas

GoT n'est pas :

- le graphe semantique persiste ;
- la [`View`](view.md) elle-meme ;
- le [`ContextPack`](../interfaces/context-pack-interface.md) ;
- un simple chain-of-thought lineaire ;
- une seconde zone de travail distincte de la [`View`](view.md).

## Invariants

- la [`View`](view.md) reste l'espace de travail runtime ;
- GoT n'introduit pas un second graphe de travail a cote de la `View` ;
- les variations de contexte GoT doivent passer par la recomposition des [`ContextFrame`](context-frame.md), pas par une redefinition ad hoc du payload ;
- un step GoT peut produire un nouveau [`ContextPack`](../interfaces/context-pack-interface.md) si la strategie, la branche, ou l'operation utile change ;
- le graphe des pensees reste distinct du graphe semantique persiste.

## Diagramme De Position

```mermaid
flowchart LR
    V[View active]
    G[Etat GoT courant]
    F[Ensemble de ContextFrame]
    P[ContextPack]
    I[Invocation provider]

    V --> F
    G --> F
    F --> P --> I
```

Ce diagramme est conceptuel uniquement.

Il montre que :

- la `View` fournit la matiere de travail ;
- l'etat GoT courant contraint quels frames sont utiles pour l'invocation courante ;
- le `ContextPack` est la composition ordonnee de ces frames pour ce step.

## Operations Utiles

Les operations GoT les plus utiles a conserver comme base documentaire sont :

- `Generate` : ouvrir une ou plusieurs branches candidates depuis un etat courant ;
- `Refine` : retravailler une branche existante ;
- `Aggregate` : fusionner plusieurs branches ou plusieurs pensees ;
- `Score` : evaluer une branche ou une pensee ;
- `KeepBest` : conserver les branches priorisees ;
- `Repeat` : explorer des variantes.

Ces operations se rattachent surtout :

- aux transformations graphe de la section 3.2 de [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md) ;
- au scoring et au ranking de la section 3.3 de cette meme reference.

## `GoTFrame`

Un `GoTFrame` est un [`ContextFrame`](context-frame.md) specialise qui expose, pour une invocation GoT donnee :

- les operations GoT pertinentes ;
- les contraintes de branche ou de strategie utiles ;
- les reperes necessaires pour comprendre quel type de transformation est attendu au step courant.

## Diagramme D'Invocation GoT

```mermaid
flowchart TD
    S[Strategie GoT]
    O[Operation GoT courante]
    V[View active]
    GF[GoTFrame]
    VF[ViewFrame]
    CP[ContextPack]

    S --> GF
    O --> GF
    V --> GF
    V --> VF
    GF --> CP
    VF --> CP
```

Ce diagramme est conceptuel uniquement.

Il montre que :

- `GoTFrame` ne remplace pas `ViewFrame` ;
- `GoTFrame` ajoute au [`ContextPack`](../interfaces/context-pack-interface.md) les instructions et reperes GoT utiles pour le step courant ;
- si la strategie ou l'operation change, le `GoTFrame` change aussi, et le `ContextPack` est recompose.

## Relations

- la [`View`](view.md) fournit le sous-graphe de travail ;
- GoT orchestre la generation, la critique, l'agregation, et le ranking sur cette `View` ;
- les sorties de GoT aident a recomposer la prochaine [`View`](view.md) ;
- le moteur peut recompiler un nouveau [`ContextPack`](../interfaces/context-pack-interface.md) a chaque step GoT utile en changeant l'ensemble actif de [`ContextFrame`](context-frame.md), y compris `GoTFrame` si necessaire.
