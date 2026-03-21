# GraphClaw Architecture Diagrams

Ce document regroupe des diagrammes GraphClaw separes des schemas herites de ZeroClaw.

Il ne decrit que des points conceptuellement assez matures pour etre representes sans laisser croire qu'ils sont deja implementes dans la runtime heritee.

## 1. Chaine D'Artefacts De Contexte

```mermaid
flowchart LR
    TI[TaskIntent]
    SR[StrategyResolution]
    V[View]
    PSG[Sous-graphe packable]
    CF[ContextFrame]
    SF[SessionFrame]
    CP[ContextPack]
    RT[ResolutionTrace]

    TI --> SR
    SR --> V
    V --> PSG
    V --> CF
    V --> SF
    PSG --> CF
    CF --> CP
    SF --> CP
    CP --> RT
```

Lecture :
- la [`View`](../architecture/concepts/view.md) reste le sous-graphe de travail ;
- la [`SessionFrame`](../architecture/concepts/session-frame.md) est derivee de cette `View` quand un contexte de session doit etre expose ;
- le [`ContextPack`](../architecture/interfaces/context-pack-interface.md) est compose de [`ContextFrame`](../architecture/concepts/context-frame.md) ordonnes.

## 2. Distinction View / Session / Payload

```mermaid
flowchart TD
    subgraph Travail[Travail runtime]
        V[View]
    end

    subgraph Invocation[Invocation provider]
        VF[ViewFrame]
        SF[SessionFrame]
        PF[PhaseFrame]
        CPF[CapabilityFrame]
        CP[ContextPack]
    end

    V --> VF
    V --> SF
    PF --> CP
    CPF --> CP
    VF --> CP
    SF --> CP
```

Lecture :
- une `View` n'est pas un payload ;
- une `SessionFrame` n'est pas un espace de manipulation runtime ;
- les frames sont les unites de distillation qui entrent dans le `ContextPack`.

## 3. Composition D'Un ContextPack Par Invocation

```mermaid
flowchart LR
    SYS[SystemFrame]
    PH[PhaseFrame]
    CAP[CapabilityFrame]
    VF[ViewFrame]
    SF[SessionFrame]
    ID[IdentitySetFrame]
    WS[WorkspaceSetFrame]
    CP[ContextPack]
    PI[ProviderInvocation]

    SYS --> CP
    PH --> CP
    CAP --> CP
    VF --> CP
    SF --> CP
    ID --> CP
    WS --> CP
    CP --> PI
```

Lecture :
- tous les frames ne sont pas obligatoires a chaque invocation ;
- `IdentitySetFrame` et `WorkspaceSetFrame` sont conditionnels ;
- le `ContextPack` est specifique a une invocation provider, pas a toute la session.

## 4. Lecture De Boucle Mono-Agent

```mermaid
flowchart TD
    A[TaskIntent]
    B[Activer les Set de depart]
    C[Composer la View initiale]
    D[Operations GoT sur la View]
    E[Recomposer la View suivante]
    F[Choisir la phase d'invocation]
    G[Deriver les ContextFrame]
    H[Composer ContextPack]

    A --> B --> C --> D --> E
    E --> D
    E --> F --> G --> H
```

Lecture :
- la boucle GraphClaw recompose la `View`, elle n'empile pas seulement du texte ;
- la selection de phase d'invocation intervient avant la composition finale du pack ;
- GoT reste distinct du graphe semantique persiste et du `ContextPack`, mais opere sur la `View`.
