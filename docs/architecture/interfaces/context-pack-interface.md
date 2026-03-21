# ContextPack Interface

## Statut

Ce document definit le role cible de `ContextPack` comme interface de migration pour GraphClaw.

Il ne pretend pas que la runtime heritee expose deja un type `ContextPack` concret en code.

Il fixe la frontiere minimale necessaire pour que le contexte final visible par le modele devienne explicite, budgete, inspectable, et distinct de l'etat plus large d'exploration.

## Pourquoi Cette Interface Compte

La runtime heritee peut deja envoyer du contexte visible dans le prompt aux providers, mais ce contexte reste encore souvent assemble a partir d'un melange de sections de prompt, de chargement memoire, et de conventions runtime transitoires.

Cela laisse plusieurs questions insuffisamment precisees :

- quel contexte exact a finalement ete expose au modele ;
- comment le budget et la summarisation ont modifie le resultat ;
- quelle matiere visible est restee exploratoire sans jamais devenir visible au modele ;
- en quoi le resultat packe final differe de la [`View`](../concepts/view.md) de travail plus large ;
- quelle trace doit exister pour la decision finale de packing.

`ContextPack` est la plus petite interface qui transforme "ce que le modele recoit effectivement" en artefact gouverne plutot qu'en effet de bord d'implementation.

## Ancrages De Reference

- reference de theorie des graphes : [`../../../.agents/skills/graphclaw/main_graphes/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/markdown.md)
- chemins et plus courts chemins pour conserver une structure relationnelle minimale intelligible : [`../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-22/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-25/markdown.md)
- connectivite, coupes, articulation, et Menger pour preserver la structure critique pendant le retrecissement final du pack : [`../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-37/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-44/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-44/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-46/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-46/markdown.md), [`../../../.agents/skills/graphclaw/main_graphes/pages/page-49/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-49/markdown.md)
- intuition de ranking via PageRank : [`../../../.agents/skills/graphclaw/main_graphes/pages/page-87/markdown.md`](../../../.agents/skills/graphclaw/main_graphes/pages/page-87/markdown.md)
- reference GoT locale : [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)
- scoring et ranking GoT avant packing final : section 3.3 de [`../../../.agents/skills/graphclaw/graph-of-thought/markdown.md`](../../../.agents/skills/graphclaw/graph-of-thought/markdown.md)

## Definition

Un `ContextPack` est l'artefact final, budgete, visible par le modele, compile pour une invocation provider donnee a partir d'un ensemble plus etroit d'artefacts gouvernes, notamment des [`ContextFrame`](../concepts/context-frame.md), des candidats packables, des resumés, et des selections contraintes par politique.

## Est

Un `ContextPack` est :

- l'artefact final visible par le modele pour une invocation provider ;
- derive d'un etat gouverne plus etroit que la simple recuperation brute ;
- compose de [`ContextFrame`](../concepts/context-frame.md) ordonnes ;
- sensible au budget, a la politique, et a la phase du turn ;
- inspectable et diffable ;
- traçable dans sa construction.

## N'est pas

Un `ContextPack` n'est pas :

- la [`View`](../concepts/view.md) active complete ;
- l'historique integral de la conversation ;
- un resultat brut de retrieval ;
- un dump non borne de tout le graphe disponible ;
- une simple concatenation de texte sans gouvernance.

## Entrees Et Sorties

### Entrees Typiques

- un ou plusieurs [`ContextFrame`](../concepts/context-frame.md) derives de la [`View`](../concepts/view.md) active et d'autres etats gouvernes adjacents ;
- des candidats de sous-graphe packable ou des projections bornees equivalentes ;
- les entrees de strategie ou de politique de packing actives ;
- des resultats de resume, de compaction, ou de remplacement ;
- des contraintes de droits, de budget, et de redaction ;
- des liens de preuve ou de provenance necessaires pour garder le pack intelligible.

### Sorties Typiques

- une representation finale du contexte visible par le modele ;
- une composition ordonnee de frames adaptee a une phase de turn et a une invocation provider ;
- des choix d'ordre, de regroupement, ou de resume pour la consommation provider ;
- des metadonnees de budget sur ce qui a survécu au packing ;
- des decisions de packing traçables pour `ResolutionTrace` ;
- une structure eventuellement plus facile a inspecter pour gateway ou UI.

## Responsabilites Minimales

L'interface devrait a terme pouvoir au minimum :

- representer le contexte final borne effectivement passe a l'execution modele ;
- composer des [`ContextFrame`](../concepts/context-frame.md) typés plutot que tout aplatir dans une exportation indifferenciee ;
- preserver la distinction entre etat runtime de travail et sortie finale visible par le modele ;
- refleter explicitement les choix de resume ou de degradation plutot que de les cacher dans le texte du prompt ;
- exposer assez de structure pour que les appels provider restent des consommateurs aval du pack plutot que ses proprietaires implicites ;
- rester assez stable pour etre inspecte, compare, et trace meme si les strategies de formatage evoluent plus tard.

## Invariants Minimaux

1. `ContextPack` est l'artefact final visible par le modele pour une invocation provider a l'interieur d'un turn.
2. `ContextPack` derive d'un etat runtime gouverne plus etroit que la recuperation non bornee.
3. `ContextPack` est contraint par budget et par politique.
4. `ContextPack` reste distinct de la [`View`](../concepts/view.md) active et des candidats bruts de retrieval.
5. `ContextPack` est compose d'un ou plusieurs [`ContextFrame`](../concepts/context-frame.md) ordonnes.
6. `ContextPack` peut contenir des resumes, des extraits, ou des remplacements plutot qu'un contenu en fidelite complete.
7. La creation du `ContextPack` doit rester traçable via des decisions explicites de packing ou de degradation.

## Relation A `View` Et `SessionFrame`

La relation stable doit etre lue ainsi :

1. la [`View`](../concepts/view.md) active porte l'etat de travail dans le graphe et reste le seul espace de manipulation ;
2. un ou plusieurs [`ContextFrame`](../concepts/context-frame.md) reconcilient des selections graphe gouvernees avec leurs projections NL pour l'invocation courante ;
3. un [`SessionFrame`](../concepts/session-frame.md) peut faire partie de cet ensemble lorsque de la matiere de session doit etre exposee ;
4. la logique de packing choisit quel sous-ensemble ou quelle representation devient visible par le modele ;
5. le `ContextPack` est le resultat compile de ce choix.

Cette distinction compte parce que GraphClaw veut un contexte runtime gouverne sans supposer que tout element visible doit etre packe en entier.

Pour cette lecture, voir aussi :

- [`../concepts/view.md`](../concepts/view.md) pour le sous-graphe de travail ;
- [`../concepts/context-frame.md`](../concepts/context-frame.md) pour l'unite de composition orientee invocation ;
- [`../concepts/packability.md`](../concepts/packability.md) pour les conditions de sous-graphe packable ;
- [`../concepts/projection-governance.md`](../concepts/projection-governance.md) pour la projection NL qui alimente les frames.

## Modele De Frames

La lecture documentaire stable suppose maintenant un modele par frames a l'interieur du pack.

Les familles minimales deja suffisamment justifiees sont :

- `SystemFrame` pour les instructions systeme stables et la base d'identite agent ;
- `PhaseFrame` pour les instructions specifiques a la phase, comme l'entree de turn, la progression GoT, ou la reponse finale ;
- `GoTFrame` pour les operations GoT, les reperes de branche, et la guidance de graphe de pensee dependante de la strategie ;
- `CapabilityFrame` pour les capabilities exposees sur cette invocation ;
- `ViewFrame` pour la projection de la [`View`](../concepts/view.md) active ;
- `SessionFrame` pour la projection session-oriented derivee de la [`View`](../concepts/session-frame.md) active ;
- `IdentitySetFrame` et `WorkspaceSetFrame` en option quand la strategie courante demande d'exposer ces ensembles comme matiere de travail directe.

Ces noms de frame sont des roles d'architecture, pas des noms de sections de wire format geles.

## Sensibilite A La Phase

`ContextPack` ne doit pas etre lu comme un pack monolithique unique pour tout le turn.

La lecture stable est :

- un meme turn peut impliquer plusieurs invocations provider ;
- des invocations differentes peuvent demander des compositions de frames differentes ;
- des invocations GoT peuvent donc recomposer `ContextPack` a plusieurs reprises dans un meme turn lorsque la branche active, l'operation GoT, ou la strategie changent ;
- l'entree de turn, la progression GoT, et la reponse finale peuvent donc consommer des packs differents tout en restant dans le meme turn logique.

Cette lecture garde explicite la variation de payload selon la phase sans transformer chaque operation interne en phase de plus haut niveau.

## Relation Aux Providers

Les providers doivent consommer un `ContextPack` ou une representation derivee de celui-ci.

Ils ne doivent pas definir :

- ce qui compte comme packable ;
- comment les resumes sont choisis ;
- quelle matiere visible est exclue ;
- comment budget et politique gouvernent le resultat final.

Les contraintes de formatage ou de capacite propres a un provider peuvent affecter la representation finale, mais le sens conceptuel de `ContextPack` reste au-dessus des adapters provider.

## Erreurs, Refus, Et Degradation

L'interface doit laisser une place a des issues telles que :

- exclure une matiere trop lourde parce que le budget final est trop petit ;
- remplacer une forme complete par un resume ou un extrait ;
- redacter certaines portions parce que les droits ou la politique interdisent l'exposition complete ;
- preserver la provenance tout en degradant le niveau de detail ;
- refuser une expansion demandee si elle viole la `View` active ou la politique courante.

Ces issues sont des decisions de packing, pas de simples effets de bord de formatage de prompt.

C'est ici que les intuitions de theorie des graphes comme coupes, articulation, et preservation d'une structure de chemin suffisante comptent le plus : le pack final doit pouvoir retrecir agressivement sans rendre le contexte survivant inintelligible.

## Compatibilite Avec La Runtime Heritee

Le premier pas de migration doit preserver le chemin provider-facing actuel.

La regle de compatibilite est :

> l'assemblage herite du prompt peut continuer a faconner les payloads provider au debut, mais il doit progressivement consommer une frontiere `ContextPack` plutot que rester l'unique endroit ou le contexte final existe.

Cela permet une premiere implementation conservative :

- commencer avec une petite structure ou un wrapper autour du contexte final deja assemble ;
- garder le formatage actuel du prompt tout en nommant et en tracant explicitement le resultat packe ;
- laisser les adapters provider inchanges sauf pour consommer une representation derivee du pack ;
- enrichir plus tard les metadonnees d'ordre, de provenance, et de resume.

## Consommateurs Probables Dans Le Code

### `src/agent/`

Role probable :

- consommateur principal du contexte final visible dans le prompt ou pour l'appel provider.

Fichiers de seam probables :

- `src/agent/prompt.rs`
- `src/agent/loop_.rs`
- `src/agent/dispatcher.rs`

### `src/providers/`

Role probable :

- consommer le resultat packe a travers une frontiere d'invocation provider.

Precaution documentaire :

- les adapters provider doivent consommer le pack, pas en definir la semantique.

### `src/gateway/` et `web/`

Role probable :

- exposer plus tard des sorties packees, des resumes, ou des surfaces d'inspection reliees a la trace.

## Fichiers Initiaux A Traiter Comme Seams

- `src/agent/prompt.rs`
- `src/agent/dispatcher.rs`
- `src/agent/loop_.rs`
- `src/providers/traits.rs`
- `src/providers/router.rs`
- `src/gateway/api.rs`
- `src/gateway/sse.rs`

## Ordre Minimal D'Introduction Recommande

1. encapsuler l'assemblage actuel du contexte final dans une frontiere de pack explicite ;
2. garder le comportement de formatage du prompt stable tout en rendant l'artefact packe inspectable ;
3. emettre des informations de trace grossieres au niveau du pack ;
4. introduire plus tard des metadonnees plus riches comme resumes, exclusions, et provenance ;
5. exposer les sorties packees en lecture seule dans le transport ou l'UI seulement apres stabilisation de la frontiere.

## Slice JSON

Ce slice est un artefact d'orientation, pas une affirmation d'implementation.

```json
{
  "nodes": [
    {
      "id": "n0",
      "position": { "x": -650, "y": -50 },
      "caption": "SessionFrame",
      "labels": ["SessionFrame"],
      "properties": {
        "file_origin": "future session-oriented frame",
        "role": "Session-specific projection composed before final packing"
      },
      "style": {}
    },
    {
      "id": "n1",
      "position": { "x": -300, "y": -180 },
      "caption": "Packing Strategy",
      "labels": ["PackingStrategyDefinition"],
      "properties": {
        "file_origin": "future strategy seam",
        "role": "Shapes what representation survives into the final pack"
      },
      "style": {}
    },
    {
      "id": "n2",
      "position": { "x": -300, "y": 80 },
      "caption": "Budget And Policy Constraints",
      "labels": ["PolicyDefinition"],
      "properties": {
        "file_origin": "src/runtime/ + src/security/",
        "role": "Constrain the final model-visible result"
      },
      "style": {}
    },
    {
      "id": "n3",
      "position": { "x": 70, "y": -50 },
      "caption": "ContextPack",
      "labels": ["ContextPack"],
      "properties": {
        "file_origin": "future pack compiler seam",
        "role": "Final packed context artifact for provider consumption"
      },
      "style": {}
    },
    {
      "id": "n4",
      "position": { "x": 430, "y": -50 },
      "caption": "Provider Invocation",
      "labels": ["Resource"],
      "properties": {
        "file_origin": "src/providers/",
        "role": "Consumes the packed context under a uniform provider contract"
      },
      "style": {}
    },
    {
      "id": "n5",
      "position": { "x": 790, "y": -50 },
      "caption": "ResolutionTrace",
      "labels": ["ResolutionTrace"],
      "properties": {
        "file_origin": "future trace layer",
        "role": "Records packing, degradation, and exclusion decisions"
      },
      "style": {}
    }
  ],
  "relationships": [
    {
      "id": "r0",
      "type": "CONTEXT_PACK_DERIVED_FROM",
      "style": {},
      "properties": {},
      "fromId": "n3",
      "toId": "n0"
    },
    {
      "id": "r1",
      "type": "PLAN_USES_STRATEGY",
      "style": {},
      "properties": {},
      "fromId": "n3",
      "toId": "n1"
    },
    {
      "id": "r2",
      "type": "RELATED_TO",
      "style": {},
      "properties": {},
      "fromId": "n3",
      "toId": "n2"
    },
    {
      "id": "r3",
      "type": "HAS_TOPIC",
      "style": {},
      "properties": {},
      "fromId": "n3",
      "toId": "n4"
    },
    {
      "id": "r4",
      "type": "TRACE_IMPLEMENTED_STRATEGY",
      "style": {},
      "properties": {},
      "fromId": "n5",
      "toId": "n3"
    }
  ]
}
```

## References Liees

- [`../concepts/context-artifacts.md`](../concepts/context-artifacts.md) pour les frontieres d'artefacts
- [`../concepts/view.md`](../concepts/view.md) pour le sous-graphe de travail runtime
- [`../concepts/packability.md`](../concepts/packability.md) pour les conditions de sous-graphe packable
- [`../concepts/projection-governance.md`](../concepts/projection-governance.md) pour la projection vers une forme visible par le modele
- [`../runtime/turn-runtime-logic.md`](../runtime/turn-runtime-logic.md) pour la sequence plus large du turn
- [`../migration/future-integration-seams.md`](../migration/future-integration-seams.md) pour le placement des seams
- [`../concepts/session-frame.md`](../concepts/session-frame.md) pour le concept de frame oriente session
