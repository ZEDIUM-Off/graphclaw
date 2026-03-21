# Sets, Views, And Packability

## Statut

Ce fichier est un hub de routage pour la famille `Set` / `View`.

Il ne porte plus la definition canonique de chaque concept de cette famille. Les definitions canoniques vivent maintenant dans des fichiers dedies afin que le depot garde une seule source stable par concept.

## Lire Ici D'Abord

- `Set`: [`set.md`](set.md)
- `ResolvedSet`: [`resolved-set.md`](resolved-set.md)
- `View`: [`view.md`](view.md)
- sous-graphe packable, packabilite, complement borne, condensation, et projection vers un sous-graphe packable : [`packability.md`](packability.md)

## Pourquoi Ce Hub Existe

`Set`, `ResolvedSet`, `View`, et la packabilite restent etroitement lies.

Ce hub garde cette famille lisible sans obliger a commencer par une page unique trop large. Il sert a router rapidement, pas a redefinir les concepts lies.

## Ordre De Lecture

1. Lire [`set.md`](set.md) pour l'abstraction de base du `Set` gouverne et sa representation persistante.
2. Lire [`resolved-set.md`](resolved-set.md) pour l'artefact derive produit par la resolution.
3. Lire [`view.md`](view.md) pour le concept de sous-graphe de travail runtime.
4. Lire [`packability.md`](packability.md) pour les sous-graphes packables, la packabilite, et leur relation avec [`ContextFrame`](context-frame.md) et [`ContextPack`](../interfaces/context-pack-interface.md).

## Docs Liees

- frontiere du moteur et routage conceptuel : [`graph-context-engine.md`](graph-context-engine.md)
- artefacts de contexte et couches de budget : [`context-artifacts.md`](context-artifacts.md)
- theorie des graphes, gouvernance de projection, GoT, et boucle agent : [`graph-governed-agentics.md`](graph-governed-agentics.md), [`projection-governance.md`](projection-governance.md), [`got.md`](got.md), [`agent-loop.md`](agent-loop.md)
- slice playground : [`../playground/set-system-spec-v0.md`](../playground/set-system-spec-v0.md)
