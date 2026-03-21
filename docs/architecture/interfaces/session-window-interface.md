# SessionWindow Interface

## Statut

`SessionWindow` est un terme de transition retire.

Il ne constitue plus un concept actif de l'architecture GraphClaw.

## Remplacement

La lecture actuelle est :

- la [`View`](../concepts/view.md) reste le seul espace de manipulation runtime ;
- les [`ContextFrame`](../concepts/context-frame.md) derivent des sous-graphes projectables de cette `View` ;
- le [`SessionFrame`](../concepts/session-frame.md) est le frame specialise pour la projection de matiere de session ;
- le [`ContextPack`](context-pack-interface.md) compose les frames utiles a une invocation provider donnee.

## Pourquoi Le Terme Est Retire

`SessionWindow` servait a nommer un etat intermediaire entre la `View` et le payload provider.

Cette distinction a ete retiree parce qu'elle dupliquait des responsabilites maintenant portees plus clairement par :

- la [`View`](../concepts/view.md) pour le travail et la mutabilite runtime ;
- le [`SessionFrame`](../concepts/session-frame.md) pour la projection session-oriented ;
- le [`ContextPack`](context-pack-interface.md) pour le payload final phase-aware.

## Regle De Migration

Quand un ancien document ou une ancienne discussion mentionne `SessionWindow`, il faut desormais lire :

- soit [`View`](../concepts/view.md) si le sujet est la manipulation runtime ;
- soit [`SessionFrame`](../concepts/session-frame.md) si le sujet est la projection de session vers le provider ;
- soit [`ContextPack`](context-pack-interface.md) si le sujet est le payload final envoye.
