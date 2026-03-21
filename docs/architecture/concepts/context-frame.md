# ContextFrame

## Statut

Ce fichier est la source canonique de definition de `ContextFrame`.

Il releve de la documentation d'architecture cible.

## Definition

Un `ContextFrame` est un artefact runtime de distillation gouvernee, derive d'une portion concernee de la [`View`](view.md) active, qui reconcilie une selection graphe projectable, sa projection en langage naturel, et les metadonnees de gouvernance utiles a une invocation provider donnee.

## Est

Un `ContextFrame` est :

- une unite de contexte composee pour une invocation ;
- borne par la [`View`](view.md) active et par les politiques courantes ;
- alimente par une `NLProjection` gouvernee ;
- porteur d'un role explicite dans le `ContextPack` ;
- sensible a la phase du turn ;
- distinct de la seule selection graphe brute.

## N'est pas

Un `ContextFrame` n'est pas :

- la [`View`](view.md) elle-meme ;
- le [`ContextPack`](../interfaces/context-pack-interface.md) complet ;
- une concatenation opportuniste de texte ;
- une projection libre sans regles de gouvernance ;
- un dump brut de noeuds et relations.

## Invariants

- un `ContextFrame` ne projette pas le graphe brut ; il projette une portion gouvernee et interpretable de la [`View`](view.md) ;
- un `ContextFrame` doit garder un lien clair entre selection graphe, projection NL, et metadonnees de gouvernance ;
- un `ContextFrame` peut etre omis, inclus, ou reformule selon la phase du turn sans changer la definition de la [`View`](view.md) source ;
- plusieurs `ContextFrame` peuvent etre derives de la meme [`View`](view.md) pour des roles differents ;
- un `ContextFrame` est compose pour une invocation provider, pas pour representer toute la session ou tout l'espace d'exploration.

## Relations

- la [`View`](view.md) fournit le sous-graphe de travail complet ;
- le [`SessionFrame`](session-frame.md) specialise cette abstraction pour la projection des noeuds et relations de session ;
- le [`ContextPack`](../interfaces/context-pack-interface.md) compose plusieurs `ContextFrame` ordonnes pour une invocation provider ;
- la projection naturelle d'un `ContextFrame` depend des regles de [`projection-governance.md`](projection-governance.md) ;
- l'assemblage de `ContextFrame` doit rester coherent avec la phase du turn documentee dans [`agent-loop.md`](agent-loop.md) et [`../runtime/turn-runtime-logic.md`](../runtime/turn-runtime-logic.md).

## Familles De Frames Typiques

Les familles suivantes sont deja suffisamment stables pour cadrer la documentation :

- `SystemFrame` ;
- `PhaseFrame` ;
- `GoTFrame` ;
- `CapabilityFrame` ;
- `ViewFrame` ;
- `SessionFrame` ;
- `IdentitySetFrame` ;
- `WorkspaceSetFrame`.

Ces familles sont des roles documentaires utiles. Elles n'imposent pas encore une structure de classe ou un schema de stockage.

## Representation Actuelle

La runtime heritee ne compose pas encore explicitement des `ContextFrame`.

Aujourd'hui, des fragments equivalents existent surtout sous forme de sections de prompt, de chargement memoire, de resultats d'outils, et de conventions implicites. Le but de `ContextFrame` est de rendre cette composition inspectable et gouvernable sans presenter cette cible comme deja implementee.
