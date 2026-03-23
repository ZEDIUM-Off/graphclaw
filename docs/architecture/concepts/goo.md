# Graph Of Operations

## Statut

Ce fichier est la source canonique de definition de `Graph of Operations` (`GoO`).

Il releve de la documentation d'architecture cible.

Il ne pretend pas que la runtime heritee execute deja un `GoO` explicite en code.

## Ancrage De Reference

- reference GoT locale : [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)

Les sections les plus utiles sont :

- section 3.2 pour les transformations de base ;
- section 3.3 pour scoring, ranking, et selection ;
- section 4.5 pour la distinction entre `Graph of Operations` et `Graph Reasoning State`.

## Definition

Dans GraphClaw, un `GoO` est le graphe d'operations typees qui determine le flux de reflexion d'un turn.

Il decrit quelles operations doivent etre executees, dans quel ordre logique, avec quelles dependances, et avec quels points de branchement, d'agregation, de repetition, ou de selection.

## Est

Un `GoO` est :

- un graphe d'operations runtime ;
- une structure executable apres validation, resolution, expansion, et compilation ;
- un plan de reflexion gouvernable et tracable ;
- compose d'operations connues du runtime ;
- selectionnable, composable, et versionnable ;
- reutilisable comme graphe principal de turn ou comme sous-graphe dans un `GoO` plus large.

## N'est pas

Un `GoO` n'est pas :

- la [`View`](view.md) active ;
- le graphe semantique persiste ;
- le graphe runtime des pensees documente dans [`got.md`](got.md) ;
- le [`ContextPack`](../interfaces/context-pack-interface.md) ;
- une structure libre purement textuelle executee telle quelle ;
- une machine d'execution separee du runtime principal.

## Invariants

- le modele peut proposer un `GoO`, mais uniquement dans une forme structuree, gouvernable, et tracable ;
- GraphClaw doit valider, resoudre, expandre, puis compiler cette proposition avant execution ;
- l'alphabet minimal d'operations de base est : `Generate`, `Refine`, `Aggregate`, `Score`, `KeepBest`, `Repeat` ;
- des operations agentiques supplementaires peuvent etre documentees plus tard, sans etre presentees comme deja implementees ;
- un `GoO` persiste n'est pas une classe runtime differente : c'est un `GoO` versionne et reutilisable ;
- un `GoO` persiste peut etre appele comme graphe principal du turn ou comme sous-graphe reference dans un autre `GoO` ;
- les references a des `GoO` persistants doivent etre integrees dans un seul `GoO` final compile pour le turn ;
- la provenance des sous-graphes persistants integres doit rester presente dans les metadonnees du `GoO` final ;
- le moteur suit un seul `GoO` compile pendant la reflexion du turn.

## Operations De Base

Les operations de base actuellement retenues comme noyau documentaire sont :

- `Generate` : produire de nouvelles branches candidates ou hypotheses ;
- `Refine` : retravailler une branche existante ;
- `Aggregate` : combiner plusieurs branches, pensees, ou resultats intermediaires ;
- `Score` : evaluer une branche ou un resultat selon les criteres courants ;
- `KeepBest` : retenir les branches ou resultats priorises ;
- `Repeat` : reiterer une portion du graphe avec variation ou contrainte differente.

Ces operations ne sont pas de simples labels.

La lecture stable est :

- chaque operation a une semantique d'entree ;
- chaque operation a une semantique de sortie ;
- chaque operation occupe une place explicite dans le graphe de dependances du `GoO`.

## `GoO` Persiste Et Reutilisation

Un "workflow" reutilisable doit etre lu comme un `GoO` persiste.

Le terme "workflow" peut etre utile localement pour parler d'un usage, mais il ne designe pas une nature runtime differente.

La lecture cible est :

- certains `GoO` sont proposes ou composes specifiquement pour un turn ;
- certains `GoO` sont persistes, versionnes, et reutilisables ;
- un `GoO` persiste peut etre selectionne comme point de depart principal ;
- un `GoO` persiste peut aussi etre reference comme sous-graphe dans un autre `GoO` ;
- apres resolution et expansion, ces references convergent vers un seul `GoO` final compile.

## Promotion Selective

Tous les runs de reflexion ne doivent pas devenir des actifs persistants.

La persistance vise une promotion selective de motifs reutilisables.

La lecture stable est :

- un run concret peut reveler un motif d'operations utile ;
- ce motif doit etre abstrait du cas particulier avant persistance ;
- le resultat persiste doit rester un `GoO` reutilisable plutot qu'une capture brute d'un run unique ;
- la promotion est selective, gouvernee, et versionnee.

## Relations

- [`graph-context-engine.md`](graph-context-engine.md) situe `GoO` dans la chaine cible du turn ;
- [`got.md`](got.md) definit le graphe runtime des pensees et le `GoTState` produits pendant l'execution d'un `GoO` ;
- [`view.md`](view.md) reste l'espace de travail runtime sur lequel les operations raisonnent ;
- [`context-frame.md`](context-frame.md) et [`../interfaces/context-pack-interface.md`](../interfaces/context-pack-interface.md) restent les artefacts de projection et de packing derives du travail sur la `View` ;
- [`../interfaces/strategy-resolver-interface.md`](../interfaces/strategy-resolver-interface.md) cadre la selection du regime de strategie qui gouverne la selection, la proposition, ou la composition du `GoO`.

## Representation Actuelle

Dans la runtime heritee, cette frontiere n'est pas encore exposee comme objet executable explicite.

Le document fixe donc une cible documentaire :

- le modele peut proposer une structure `GoO` ;
- GraphClaw en garde la validation et la compilation ;
- le runtime n'introduit pas une seconde boucle concurrente ;
- le turn suit finalement un seul graphe d'operations compile.
