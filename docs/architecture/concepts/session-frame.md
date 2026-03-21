# SessionFrame

## Statut

Ce fichier est la source canonique de definition de `SessionFrame`.

Il releve de la documentation d'architecture cible.

## Definition

Un `SessionFrame` est un [`ContextFrame`](context-frame.md) specialise, derive de la [`View`](view.md) active, qui selectionne et projette en langage naturel les noeuds et relations de session retenus pour une invocation provider donnee.

## Est

Un `SessionFrame` est :

- un `ContextFrame` ;
- borne par la [`View`](view.md) active ;
- compose pour une invocation provider et une phase de turn ;
- porteur d'une projection NL des elements de session retenus ;
- distinct de l'espace de travail mutable.

## N'est pas

Un `SessionFrame` n'est pas :

- la [`View`](view.md) elle-meme ;
- un espace de manipulation runtime ;
- un etat mutable intermediaire entre la `View` et le [`ContextPack`](../interfaces/context-pack-interface.md) ;
- la totalite de la session ;
- le [`ContextPack`](../interfaces/context-pack-interface.md) complet.

## Invariants

- l'agent manipule les noeuds et relations dans la [`View`](view.md), pas dans le `SessionFrame` ;
- le `SessionFrame` ne fait que declarer quel sous-graphe de session de la `View` est projete en contexte et comment ;
- un `SessionFrame` peut varier selon la phase du turn, la strategie active, et le budget ;
- plusieurs invocations d'un meme turn peuvent produire des `SessionFrame` differents a partir de la meme [`View`](view.md).

## Relations

- la [`View`](view.md) reste le seul espace de manipulation runtime ;
- le [`ContextFrame`](context-frame.md) fixe l'abstraction generale de frame ;
- le `SessionFrame` en est une specialisation orientee session ;
- le [`ContextPack`](../interfaces/context-pack-interface.md) compose un ou plusieurs frames, dont un `SessionFrame` si la session doit etre exposee a cette invocation.

## Representation Actuelle

La runtime heritee ne compose pas encore explicitement des `SessionFrame`.

Aujourd'hui, des fragments de session comparables existent surtout dans l'historique de messages, les resultats d'outils, et d'autres conventions implicites de prompt. Le but de `SessionFrame` est de rendre cette projection inspectable et gouvernable sans ajouter un artefact mutable supplementaire entre la [`View`](view.md) et le [`ContextPack`](../interfaces/context-pack-interface.md).
