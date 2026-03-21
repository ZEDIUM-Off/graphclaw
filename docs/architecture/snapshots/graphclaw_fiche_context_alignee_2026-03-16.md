# Fiche contexte — GraphClaw (alignement actuel)

## Statut documentaire

Cette fiche reste utile comme document d'alignement et de cadrage.

Pour le vocabulaire stable et le premier slice playground, les sources normatives a privilegier sont maintenant :

- `graph-context-engine.md`
- `views-and-sets.md`
- `set.md`
- `view.md`
- `packability.md`
- `view-system-spec-v0.md`

Cette fiche ne doit donc pas etre lue comme la source principale pour le cycle de vie v0 ni pour le nommage final de l'artefact d'export.

## 1. Définition

GraphClaw n’est pas un simple fork d’OpenClaw ou de ZeroClaw avec une couche mémoire en plus.

GraphClaw vise à devenir un **runtime agentique piloté par un système de views composables**, où le contexte n’est plus assemblé comme une concaténation implicite de mémoire, d’historique et de texte système, mais **résolu explicitement depuis un graphe** puis **projeté** dans un prompt sous contrôle d’un moteur central.

L’idée structurante actuelle est la suivante :

> un agent ne manipule ni le graphe brut, ni des permissions, ni un prompt directement ; il navigue dans un espace de views déjà borné, et le moteur compile ensuite une projection contextuelle traçable et stable.

---

## 2. Ce que GraphClaw est

GraphClaw est un projet de **refonte architecturale progressive** d’un runtime hérité, vers un système :

- **graph-native** dans sa logique de contexte ;
- **portable** pour les agents, leurs dépendances et leurs surfaces de navigation ;
- **gouverné** par un moteur de résolution explicite ;
- **traçable** dans la construction du contexte ;
- **modulaire** dans la définition et le partage des views.

GraphClaw veut permettre :

1. de définir des agents comme **packages portables** ;
2. de définir des surfaces de visibilité comme **views composables** ;
3. de laisser les agents naviguer dans un graphe **déjà borné** ;
4. de compiler un contexte utile sous forme de **projection résolue** ;
5. d’éviter qu’une logique critique dépende d’un prompt implicite ou d’une chaîne de pensée cachée.

---

## 3. Ce que GraphClaw n’est pas

GraphClaw n’est pas :

- une simple base de données graphe branchée derrière un agent ;
- un RAG vectoriel enrichi ;
- un système où l’agent interroge librement le graphe ;
- un moteur où la sécurité vit dans le prompt ;
- un assembleur de texte système déguisé en “context engine” ;
- un produit défini par sa DB plutôt que par ses contrats de résolution.

Le produit cible n’est pas “le graphe”.

Le produit cible est :

- le **système de views** ;
- le **modèle de portabilité des agents** ;
- la **gouvernance de la navigation** ;
- la **projection du contexte** ;
- les **artefacts explicites** de résolution, mutation et traçabilité.

---

## 4. Problème à résoudre

Les systèmes agents classiques finissent par empiler :

- identité système ;
- instructions de rôle ;
- historique de conversation ;
- mémoire rappelée ;
- sorties d’outils ;
- documents ;
- règles ;
- tool schemas ;
- résumés opportunistes.

Le résultat est fragile :

- duplication de contraintes ;
- oublis d’invariants ;
- bruit contextuel ;
- difficulté d’audit ;
- forte dépendance à des heuristiques de rappel ;
- confusion entre stockage, accès, navigation et projection.

GraphClaw cherche à remplacer ce modèle par un système où le contexte est :

- **défini comme sous-ensemble de graphe** ;
- **résolu via des views** ;
- **borné par une frontière d’accès** ;
- **navigué via des tools de parcours** ;
- **projeté dans un prompt** seulement à la fin.

---

## 5. Thèse centrale

La thèse actuelle peut se formuler simplement :

> Le cœur de GraphClaw n’est pas le prompt, ni la DB, ni même le retriever. Le cœur est un système de views composables, gouverné par un engine, capable de produire une projection contextuelle stable à partir d’un graphe borné et navigable.

Cela implique trois idées fortes :

### 5.1 Le contexte est un résultat, pas une source
Le contexte injecté au modèle n’est pas le graphe.
Ce n’est pas non plus une mémoire brute.
C’est une **projection résolue** d’un ensemble de views matérialisées.

### 5.2 Une view est une expression d’ensemble sur graphe
Une view n’est pas d’abord une permission ou un texte à injecter.
Une view est une **définition d’ensemble**, extensionnelle ou intentionnelle, sur un sous-graphe.

### 5.3 L’agent ne gère jamais les permissions
L’agent navigue uniquement dans des views déjà autorisées.
Les tools de parcours appliquent une politique d’accès décidée par l’engine.
L’agent n’a donc jamais à raisonner explicitement en termes de droits.

---

## 6. Les quatre piliers du projet

## 6.1 Système de views
Le système de views est la brique centrale.
Il doit permettre de :

- définir des views spécialisées ;
- composer des views ;
- partager des views ;
- binder des views sur des graphes locaux ;
- distinguer clairement leurs finalités.

## 6.2 Packages d’agents portables
Un agent n’est pas juste un dossier.
C’est un package transportable contenant :

- identité ;
- policies ;
- capabilities ;
- skills ;
- views déclarées ;
- dépendances de graphe ;
- règles de packing ;
- budgets ;
- stratégie d’import ;
- contrats de mutation.

## 6.3 Context Engine
Le Context Engine n’est pas un simple retriever.
Il gouverne :

- la frontière d’accès ;
- la résolution des views ;
- la session window ;
- la projection contextuelle ;
- les mutations de contexte ;
- l’import et les bindings des agents.

## 6.4 Projection de prompt
Le prompt final n’est qu’un **rendu**.
La vraie source de vérité est une **projection canonique résolue**, ordonnée, traçable, sous budget, issue de vues résolues.

---

## 7. Modèle conceptuel de la view

## 7.1 Définition
Une view est une **expression déclarative d’un ensemble sur graphe**.
Elle peut être :

- **extensionnelle** : liste explicite de nœuds et/ou relations ;
- **intentionnelle** : règle de composition, de filtrage, d’expansion ou de projection.

## 7.2 Une view n’est pas une permission
Une view décrit une surface logique.
Le fait qu’elle soit effectivement accessible ou non dépend du moteur.

## 7.3 Une view n’est pas forcément destinée au prompt
Certaines views servent à naviguer.
D’autres servent à définir une frontière.
D’autres servent au runtime.
D’autres servent seulement à la projection contextuelle finale.

---

## 8. Typologie actuelle des views

Pour éviter que le mot “view” ne devienne un mot fourre-tout, GraphClaw doit distinguer au moins quatre catégories.

## 8.1 SemanticView
View orientée sens métier ou conceptuel.

Exemples :
- concepts HIGHFINITY ;
- logique business partagée ;
- policies de conformité ;
- fichiers de référence produit.

Elle sert à décrire un domaine sémantique du graphe.

## 8.2 BoundaryView
View servant de frontière d’accès ou d’espace navigable maximal pour un agent ou une classe d’agents.

Exemples :
- enveloppe d’un agent comptabilité business ;
- enveloppe d’un agent commercial ;
- enveloppe d’un agent conformité.

Une `AccessEnvelope` peut être comprise comme **la matérialisation résolue d’une BoundaryView**.
Ce n’est donc pas une permission brute séparée du système de views, mais une view de statut système, gouvernée par l’engine.

## 8.3 RuntimeView
View opératoire utilisée pendant l’exécution.

Exemples :
- session window ;
- working set courant ;
- nœuds épinglés ;
- contexte récent actif.

Elle est volatile, mutable, fortement liée à la session.

## 8.4 ProjectionView
View destinée au rendu contextuel dans le prompt.

Exemples :
- active policies ;
- business summary ;
- evidence pack ;
- reference excerpts.

Elle n’est pas faite pour naviguer, mais pour être projetée.

---

## 9. Cycle de vie d’une view

Pour stabiliser le système, une view ne doit pas être pensée comme un objet unique, mais comme un cycle.

## 9.1 ViewTemplate
Définition réutilisable.
Elle déclare :

- son identifiant ;
- son type ;
- sa finalité ;
- ses paramètres ;
- ses opérations autorisées ;
- ses règles de compatibilité ;
- sa politique de coût par défaut.

## 9.2 BoundView
Instantiation locale d’un template avec :

- un point d’ancrage ;
- des paramètres ;
- un agent ;
- une instance ;
- une enveloppe de résolution.

## 9.3 ResolvedView
Vue effectivement matérialisée :

- nœuds ;
- relations ;
- complétude ;
- dégradation ;
- coût estimé ;
- provenance ;
- trace de résolution.

## 9.4 ViewExport
Forme retenue pour l'export externe d'une `ResolvedView` dans le slice playground :

- inline ;
- résumé ;
- teaser ;
- stub ;
- ou trace seulement.

Dans les documents plus recents, cet artefact doit etre lu comme un export de playground ou d'explication externe, et non comme le futur `ContextPack` complet du Context Engine.

---

## 10. Algèbre de composition des views

La puissance du système vient de la composition, mais cette composition doit rester gouvernable.

GraphClaw doit supporter une algèbre limitée, explicite et traçable.

Opérations minimales visées :

- `union(a, b)`
- `intersection(a, b)`
- `difference(a, b)`
- `expand(view, relation, depth)`
- `filter(view, predicate)`
- `project(view, semantic_class)`
- `slice(view, order, limit)`
- `compact(view, strategy)`
- `add_anchors(view, refs)`

Le but n’est pas de créer un langage arbitraire.
Le but est de créer une **grammaire composable mais bornée**.

---

## 11. BoundaryView et AccessEnvelope

L’idée actuelle importante est la suivante :

> une enveloppe d’accès peut être modélisée comme une view composée, résolue par l’engine à partir d’autres views et de règles de compatibilité.

Exemple d’usage :

- union de la view HIGHFINITY et de la view MC STUDIO ;
- projection sur les nœuds relevant de la logique business pure ;
- exclusion des concepts spécifiques à chaque activité ;
- résultat : surface partagée “business logic” utilisable comme frontière d’un agent comptable business.

Cela ouvre deux possibilités puissantes :

1. créer des agents spécialisés via des frontières de views ;
2. partager des views génériques entre utilisateurs, puis les binder localement.

Mais cette puissance impose une contrainte :

- une `BoundaryView` n’est pas une view libre comme les autres ;
- elle doit rester **système-owned**, **auditée**, **stable**, et **résolue par l’engine**.

---

## 12. SessionWindow

La `SessionWindow` est une notion clé.

Elle doit être comprise comme :

> une `RuntimeView` spécialisée, mutable, persistante au niveau runtime, opérant uniquement sur les nœuds de session et servant de surface de travail active pour le tour courant.

Elle n’est ni :

- toute la conversation brute ;
- toute la mémoire ;
- toute la frontière accessible ;
- tout le graphe disponible.

Elle est :

- un working set ;
- borné par budget ;
- mutable par propositions ;
- gouverné par l’engine ;
- résumable, pinnable, compactable.

L’agent ne modifie jamais directement son prompt.
Il produit des **intentions structurées** portant sur sa SessionWindow.

---

## 13. Navigation : ce que voit réellement l’agent

L’agent ne doit jamais voir :

- RBAC ;
- ACL ;
- namespaces autorisés ;
- types interdits ;
- détails de sécurité internes.

L’agent voit seulement :

- les views navigables disponibles ;
- les opérations de parcours autorisées depuis ces views ;
- des coûts estimés ;
- des statuts de complétude ou de dégradation ;
- les possibilités de pin, expand, collapse, summarize.

Autrement dit :

> l’agent n’interagit pas avec la sécurité ; il interagit avec un univers déjà filtré.

---

## 14. NavigationTools

Les tools de parcours ne donnent jamais accès au graphe brut.
Ils appliquent une politique d’accès déjà décidée par l’engine.

Leur rôle est de :

- faire passer d’une view à une autre ;
- produire des BoundViews ou des ResolvedViews ;
- permettre des expansions contrôlées ;
- respecter profondeur, cardinalité, budget, compatibilités.

Ils ne sont pas la source de vérité de la sécurité.
Ils sont le **point d’application opérationnel** d’une politique portée par l’engine.

---

## 15. View Registry modulaire

Le `ViewRegistry` doit devenir une brique centrale et modulaire.

## 15.1 Core registry
Views fondamentales du système.

Exemples :
- union ;
- intersection ;
- session_window ;
- active_policies ;
- agent_tools ;
- entity_concepts.

## 15.2 Package registry
Views livrées avec un agent ou un module.

## 15.3 Workspace registry
Views partagées à l’échelle d’une organisation.

## 15.4 User / Instance registry
Bindings locaux, overrides, spécialisations.

Cette structure est importante, car GraphClaw vise non seulement des agents portables, mais aussi des **views portables et partageables**.

---

## 16. Portabilité et partage des views

Une view réutilisable ne doit pas dépendre d’identifiants locaux codés en dur comme vérité centrale.

Elle doit dépendre de :

- paramètres ;
- anchors ;
- taxonomies ;
- règles de composition ;
- contrats de binding.

Ainsi, une `business_logic_view` peut être :

- conçue une fois ;
- partagée ;
- puis bindée sur l’ancrage d’une autre entreprise.

La portabilité des views devient alors une extension naturelle de la portabilité des agents.

---

## 17. Ontologie commune et classes sémantiques

La composition robuste des views suppose un socle sémantique minimal commun.

Exemples de classes utiles :

- `business_core`
- `finance`
- `commercial`
- `legal`
- `operations_specific`
- `brand_specific`
- `compliance`

Sans ce socle, des views comme “business logic only” deviennent fragiles et dépendent d’heuristiques implicites.

---

## 18. Modèle d’agent

GraphClaw distingue clairement :

## 18.1 AgentPackage
Artefact portable, versionné, déclaratif.

Il contient :
- identité ;
- policies ;
- capabilities ;
- skills ;
- views déclarées ;
- budgets ;
- règles de packing ;
- dépendances ;
- stratégies d’import ;
- tests comportementaux ;
- mappings attendus.

## 18.2 AgentInstance
Instanciation locale dans une instance GraphClaw.

Elle contient :
- bindings locaux ;
- namespace ;
- surcharges ;
- session state ;
- historique des packs ;
- apprentissage local éventuel.

## 18.3 GraphDependencies
Dépendances de concepts, taxonomies, policies, templates, connecteurs, ontologies, skills.

Elles doivent être **déclarées explicitement**.

---

## 19. Rôle exact du Context Engine

Le Context Engine est l’orchestrateur central.

Il n’est pas défini par la DB, mais par ses responsabilités.

Il doit au minimum :

1. charger et valider les packages d’agents ;
2. résoudre leurs dépendances ;
3. choisir et appliquer une stratégie d’import ;
4. calculer la `BoundaryView` effective de l’agent ;
5. exposer les views navigables ;
6. gouverner la SessionWindow ;
7. résoudre les views en `ResolvedView` ;
8. compiler une projection contextuelle ;
9. arbitrer les mutations proposées ;
10. conserver les traces de résolution, de dégradation, de rebinding et de refus.

---

## 20. PromptProjection

L’une des clarifications majeures du projet est la suivante :

> il ne faut pas définir directement le prompt comme artefact primaire.

Il faut définir d’abord une **projection résolue et canonique**, puis seulement son rendu textuel.

Le prompt final doit donc être vu comme :

- un rendu provider-specific ;
- d’une projection stable ;
- composée de sections typées ;
- ordonnées ;
- traçables ;
- sous budget.

Une projection de prompt pourrait typiquement agréger :

- agent frame ;
- active policies ;
- eligible capabilities ;
- selected projection views ;
- session window excerpt ;
- evidences / references utiles.

Le rendu texte n’est pas la source de vérité.

---

## 21. Mutations de contexte

Le système ne doit pas dépendre d’une chaîne de pensée cachée pour ajuster le contexte.

Le modèle peut produire des artefacts explicites du type :

- `ContextEditPlan`
- `ContextMutationProposal`
- `SelectionRationale`
- `PackingDecision`
- `SummaryReplacementPlan`

Ces artefacts doivent être :

- structurés ;
- bornés ;
- auditables ;
- validables ;
- éventuellement persistables.

L’engine décide ensuite quoi appliquer, différer, refuser ou dégrader.

---

## 22. Sécurité et gouvernance

Les principes actuels sont :

- whitelists de views et d’opérations ;
- budgets de tokens et de cardinalité ;
- profondeur bornée ;
- distinction stricte entre navigation, résolution et projection ;
- mutations en mode proposition, pas en écriture libre ;
- traçabilité de la provenance et des décisions ;
- dégradation contrôlée plutôt qu’échec brutal quand c’est possible.

La sécurité ne doit pas être un effet de bord du prompt.
Elle doit être une propriété du système de résolution.

---

## 23. Réalité de migration

GraphClaw n’est pas encore l’architecture cible complète.

Le projet doit être lu comme une **migration progressive** depuis un runtime hérité vers un runtime à frontières explicites.

Les grandes étapes visées sont :

1. créer une frontière de contexte explicite dans le runtime ;
2. formaliser `SessionWindow`, `ContextPack`, traces et autres artefacts ;
3. introduire une frontière d’adaptation vers le graphe ;
4. définir la couche package / bindings / import ;
5. ne faire la migration de naming qu’une fois l’architecture réellement stabilisée.

---

## 24. Position actuelle la plus juste

La meilleure formulation actuelle du projet est la suivante :

> GraphClaw est un runtime agentique en migration vers un moteur de résolution contextuelle piloté par un système de views composables. Les agents y sont des packages portables, les frontières d’accès y sont modélisées comme des BoundaryViews résolues, la SessionWindow y est une RuntimeView spécialisée, et le contexte injecté au modèle n’est qu’une projection finale issue de vues résolues gouvernées par un engine central.

---

## 25. Décisions structurantes déjà acquises

À ce stade, les décisions conceptuelles les plus solides sont :

1. le prompt n’est pas l’artefact primaire ;
2. une view est une expression d’ensemble sur graphe ;
3. les views doivent être typées par finalité ;
4. l’agent n’interagit jamais avec la sécurité brute ;
5. l’AccessEnvelope peut être comprise comme une BoundaryView résolue ;
6. la SessionWindow est une RuntimeView spécialisée ;
7. le Context Engine gouverne, les tools appliquent ;
8. les agents et les views doivent être portables ;
9. les mutations doivent passer par des artefacts explicites ;
10. la migration doit être progressive et non cosmétique.

---

## 26. Questions encore ouvertes

Plusieurs points restent à définir précisément :

- schéma minimal exact du `ViewTemplate` ;
- schéma minimal exact du `ResolvedView` ;
- compatibilités de composition entre types de views ;
- politique de coût et de matérialisation ;
- format canonique de `PromptProjection` ;
- ontologie minimale commune pour les classes sémantiques ;
- modèle de partage/versioning des views ;
- machine d’état d’import, de binding et de dégradation.

---

## 27. Prochaine étape logique

La suite cohérente n’est pas d’implémenter tout le moteur d’un bloc.

Le dépôt dispose maintenant d’un document de cadrage dédié :

- `View System Spec v0.1`

La prochaine étape logique est donc de :

1. réaligner l’implémentation effective sur cette spec ;
2. vérifier que la résolution de view respecte des invariants de sous-graphe cohérents ;
3. garder la distinction entre export de view, working set, et futur `ContextPack` ;
4. clarifier ensuite seulement les cas plus lourds comme `AccessEnvelope`, `SessionWindow` et les artefacts du Context Engine.
