# View

## Statut

Ce fichier est la source canonique de definition de `View`.

Il releve de la documentation d'architecture cible.

## Ancrages De Reference

- graph theory reference: [`../../../.agents/skills/graphclaw/main_graphes/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/markdown.md)
- directed graph, neighbors, predecessors, successors: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-6/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-6/markdown.md)
- paths and shortest paths: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md)
- connectivity and strongly connected components: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-38/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-38/markdown.md)
- `Graph of Thoughts` reference: [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)

## Definition

Une `View` est un [`Set`](set.md) runtime, transitoire, de premiere classe, utilise comme sous-graphe de travail pour l'exploration, la navigation, le filtrage et la composition temporaire a partir d'un ou plusieurs [`Set`](set.md).

## Est

Une `View` est :

- un `Set` runtime actif ;
- un sous-graphe complet avec noeuds et relations ;
- un support de navigation et de selection ;
- porteuse de provenance, de regles de construction, et d'implications de budget ;
- potentiellement lazy ou materialisee ;
- derivee, et non gouvernee comme un objet persiste.

## N'est pas

Une `View` n'est pas :

- un [`Set`](set.md) persiste ;
- un sous-graphe packable final ;
- un [`ContextFrame`](context-frame.md) ;
- un [`ContextPack`](../interfaces/context-pack-interface.md) ;
- une simple liste de fragments de prompt ;
- une identite du graphe tout entier.

## Invariants

- une `View` est definie comme sous-graphe de travail complet, pas par sa seule projetabilite NL ;
- une `View` peut contenir des noeuds et relations semantiques, runtime, ou de trace, si ces elements restent dans la portee active autorisee ;
- une `View` peut etre recomposee a chaque etape de reflexion ;
- une `View` n'est egale ni a sa projection en langage naturel, ni a un [`ContextFrame`](context-frame.md), ni au [`ContextPack`](../interfaces/context-pack-interface.md) final.

## Relations

- une `View` est le seul espace de manipulation runtime des noeuds et relations pendant le turn ;
- une `View` peut alimenter plusieurs [`ContextFrame`](context-frame.md) distincts pour une meme invocation ou pour plusieurs invocations d'un meme turn ;
- une [`SessionFrame`](session-frame.md) peut etre derivee d'une portion de la `View` lorsqu'un contexte de session doit etre expose ;
- un [`ContextFrame`](context-frame.md) reconcilie une portion projectable de la `View`, sa projection NL, et ses metadonnees de gouvernance ;
- le [`ContextPack`](../interfaces/context-pack-interface.md) compose des `ContextFrame` ordonnes, il n'est donc pas une `View` serialisee.

## Representation Actuelle

Pendant un turn, un agent peut :

- partir de son [`Set`](set.md) de bounding ;
- referencer plusieurs sous-ensembles visibles ;
- effectuer des operations temporaires de composition ;
- produire plusieurs `View` intermediaires ;
- n'en persister aucune en base.

Dans la runtime heritee, ces manipulations restent encore largement implicites dans l'orchestration, le chargement memoire, et la composition du prompt. La `View` sert ici a fixer le concept de sous-graphe de travail avant sa traduction en [`ContextFrame`](context-frame.md) puis en [`ContextPack`](../interfaces/context-pack-interface.md).

## Contenu

Selon l'etape, une `View` peut contenir ou referencer :

- des noeuds ;
- des relations ;
- de la provenance ;
- des regles de construction ;
- des metadonnees de ranking ou de filtrage ;
- des references vers des documents ou fragments attaches ;
- des notes de selection utiles pour le packing ou l'audit ;
- des implications de budget.

## `View` Et Projectabilite

Une `View` n'est pas definie par la seule projectabilite textuelle.

Le fait qu'une partie d'une `View` puisse etre rendue en langage naturel est une question distincte de son appartenance au sous-graphe de travail.

Voir [`projection-governance.md`](projection-governance.md) pour `ProjectionRegistry` et `NLProjection`, puis [`context-frame.md`](context-frame.md) pour l'artefact qui transforme une portion projectable de `View` en unite composee pour une invocation.

La distinction a conserver est :

- `View` = sous-graphe de travail complet ;
- projection NL = rendu gouverne d'une partie autorisee de ce sous-graphe ;
- [`ContextFrame`](context-frame.md) = distillation d'une portion projectable de la `View` pour un role d'invocation ;
- [`ContextPack`](../interfaces/context-pack-interface.md) = composition ordonnee de `ContextFrame` pour un appel provider.

## Vue Maximale D'Agent

Pour un agent donne, il peut exister une `View` active maximale representant l'espace runtime gouverne le plus large qu'il peut parcourir a cet instant.

Elle derive des [`Set`](set.md) de bounding de l'agent et de leur resolution.

## Views Intermediaires

Des `View` plus etroites ou composees a l'interieur de ce perimetre maximal peuvent :

- se concentrer sur un voisinage conceptuel ;
- se concentrer sur des skills rattaches a un concept ;
- se concentrer sur des documents ou notes associes a des noeuds selectionnes ;
- se concentrer sur une `View` de travail composee pendant la reflexion.

Ce sont des sous-graphes de travail derives, pas des mondes persistants distincts.

## Lazy Versus Materialisee

### `View` Lazy

Elle est definie par une graine, une regle, une expression, ou une chaine de filtres sans evaluation complete immediate.

Elle est utile pour la navigation, l'expansion differee, l'exploration gouvernee par politique, et la recomposition peu couteuse lorsque le focus change.

### `View` Materialisee

Elle est evaluee et fixee comme resultat explicite pour un moment donne ou pour un horizon d'audit.

Elle est utile pour la tracabilite, un packing repetable, l'inspection de budget, et d'eventuels mecanismes de cache backend.

GraphClaw peut avoir besoin des deux formes.
