# ResolvedSet

## Statut

Ce fichier est la source canonique de definition de `ResolvedSet`.

Il releve de la documentation d'architecture cible.

## Ancrages De Reference

- graph theory reference: [`../../../.agents/skills/graphclaw/main_graphes/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/markdown.md)
- adjacency matrix and graph structure: [`../../../.agents/skills/graphclaw/main_graphes/pages/page-69/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-69/markdown.md)

## Definition

Un `ResolvedSet` est l'artefact derive produit par la resolution d'un [`Set`](set.md) sur l'etat courant du graphe.

## Est

Un `ResolvedSet` est :

- un resultat derive ;
- un support possible de cache, d'audit, de trace, ou d'export ;
- distinct dans son cycle de vie du [`Set`](set.md) qui l'a engendre.

## N'est pas

Un `ResolvedSet` n'est pas :

- la definition canonique du [`Set`](set.md) ;
- un objet de gouvernance primaire ;
- un substitut permanent au [`Set`](set.md).

## Invariants

- un `ResolvedSet` depend d'un etat de graphe et d'un instant de resolution ;
- un `ResolvedSet` peut servir de source de comparaison ou d'export ;
- un `ResolvedSet` ne doit pas remplacer la definition du `Set` qui le produit.

## Representation Actuelle

Un `ResolvedSet` devrait porter au minimum :

- la reference du [`Set`](set.md) d'origine ;
- l'horodatage de resolution ;
- un instantane de schema ;
- un cout ;
- une trace de composition.
