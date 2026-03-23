# Gouvernance Des Definitions Conceptuelles

## Statut

Ce document fixe la politique documentaire de GraphClaw pour les definitions conceptuelles canoniques.

Il s'applique aux concepts d'architecture sous `docs/architecture/`, a `AGENTS.md`, et a tous les `CONTEXT.md`.

## But

GraphClaw ne doit pas porter plusieurs definitions concurrentes pour un meme concept.

La regle est la suivante :

- un concept GraphClaw a une et une seule definition canonique ;
- toute autre occurrence doit referencer cette source canonique en syntaxe Markdown ;
- `AGENTS.md` et `CONTEXT.md` contextualisent la codebase et le runtime local, mais ne redefinissent pas les concepts ;
- le glossaire sert d'index de routage, pas de seconde source de verite.

## Separation Des Roles

### `docs/`

`docs/` porte la reference conceptuelle :

- definition normative ;
- invariants ;
- distinctions semantiques ;
- limites ;
- liens entre concepts ;
- relation entre architecture cible, runtime actuel, et migration.

### `AGENTS.md` et `CONTEXT.md`

`AGENTS.md` et `CONTEXT.md` portent la contextualisation :

- ou lire avant de modifier ;
- comment un concept est represente dans cette zone de code ;
- quels types, alias, modules, DTO, ou conventions s'en rapprochent aujourd'hui ;
- quelles divergences existent encore entre l'implementation actuelle et la cible conceptuelle.

Ils ne doivent pas :

- poser une nouvelle definition normative ;
- resumer un concept de maniere concurrente ;
- corriger localement une definition canonique au lieu de la mettre a jour a sa source.

## Regle De Langue

La langue canonique des definitions conceptuelles GraphClaw est le francais.

Pendant la transition, certains documents canoniques restent en anglais. Ils doivent etre traites comme des sources canoniques transitoires jusqu'a leur reecriture. Toute nouvelle definition canonique ou toute revision lourde d'une definition existante doit tendre vers une prose francaise directe, technique, et concise.

## Format Canonique Obligatoire

Chaque concept canonique doit suivre cette structure minimale :

1. `Definition`
2. `Est`
3. `N'est pas`

Peuvent s'ajouter si necessaire :

4. `Invariants`
5. `Relations`
6. `Representation actuelle`

Contraintes de redaction :

- aller au sujet directement ;
- rester explicite ;
- eviter le commentaire narratif ;
- distinguer le concept, son usage runtime, et son support backend ;
- ne pas confondre objet persiste, objet runtime, projection packable, et artefact final.

## Regle D'Ecriture Hors Source Canonique

Lorsqu'un document non canonique cite un concept :

- il doit lier la source canonique ;
- il peut donner un rappel local non normatif d'une phrase maximum si cela evite une ambiguite immediate ;
- il doit privilegier la contextualisation implementationnelle.

Forme recommandee dans un `CONTEXT.md` de code :

> Dans cette zone, `Set` est actuellement represente par `ViewTemplate` et des DTO de playground.  
> Voir la definition canonique dans [`set.md`](set.md).

## Registre Canonique Initial

| Concept | Source canonique actuelle |
| --- | --- |
| `Set` | [`set.md`](set.md) |
| `ResolvedSet` | [`resolved-set.md`](resolved-set.md) |
| `View` | [`view.md`](view.md) |
| `Graph of Thoughts` (`GoT`) | [`got.md`](got.md) |
| `Graph of Operations` (`GoO`) | [`goo.md`](goo.md) |
| `GoTState` | [`got.md#gotstate`](got.md#gotstate) |
| `SessionFrame` | [`session-frame.md`](session-frame.md) |
| `ContextPack` | [`context-pack-interface.md`](../interfaces/context-pack-interface.md) |
| `ContextFrame` | [`context-frame.md`](context-frame.md) |
| `StrategyResolution` | [`strategy-resolver-interface.md`](../interfaces/strategy-resolver-interface.md) |
| `ContextMutationProposal` | [`graph-context-engine.md#contextmutationproposal`](graph-context-engine.md#contextmutationproposal) |
| `ResolutionTrace` | [`graph-context-engine.md#resolutiontrace`](graph-context-engine.md#resolutiontrace) |
| `TaskIntent` | [`graph-context-engine.md#taskintent`](graph-context-engine.md#taskintent) |
| `AgentPackage` | [`graph-context-engine.md#agentpackage`](graph-context-engine.md#agentpackage) |

## Regle De Migration

Lorsqu'un concept passe d'un document transitoire vers un document canonique plus approprie :

1. mettre a jour ce registre ;
2. convertir les anciennes definitions en liens ;
3. mettre a jour le glossaire ;
4. mettre a jour `AGENTS.md`, `CONTEXT.md`, et les hubs de documentation si leur routage change.

## Controle Minimal

Le depot doit refuser au minimum les cas suivants :

- une definition de concept en style glossaire dans `AGENTS.md` ;
- une definition concurrente dans un `CONTEXT.md` ;
- une seconde serie de definitions completes dans `glossary.md` ;
- un concept du registre canonique sans document de destination identifiable.

Voir `scripts/ci/docs_canonical_concepts_gate.sh`.
