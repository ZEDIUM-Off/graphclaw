I.8. Graphes orientés sans circuit et tri topologique

Proposition I.8.2. Soit  $G = (V, E)$  un graphe simple orienté. Ce graphe est sans cycle si et seulement si il existe un sommet  $v$  tel que  $d^{-}(v) = 0$  et pour tout sommet  $v$  tel que  $d^{-}(v) = 0$ , le graphe  $G - v$  est sans cycle.

Démonstration. La condition est nécessaire. Si  $G$  est sans cycle, on peut appliquer le lemme précédent. De plus, tout sous-graphe  $G - v$  d'un graphe sans cycle  $G$  est bien sur sans cycle.

La condition est suffisante. Soit  $v$  un sommet tel que  $d^{-}(v) = 0$ . Par hypothèse,  $G - v$  est sans cycle. Par conséquent, si le graphe  $G$  possède un cycle, ce dernier doit nécessairement passer par  $v$ . Si un cycle passé par  $v$ , on en conclus que  $d^{-}(v) \geq 1$ . C'est impossible, donc  $G$  est dépourvu de cycle.

Ce résultat nous fournit un premier algorithme permettant de decide si un graphe est sans cycle.

Algorithm 1.8.3. La donnée fournie à cet algorithme est un graphe simple orienté  $G = (V, E)$ .

```txt
Tant qu'il existe  $v \in V$  tel que  $d^{-}(v) = 0$ ,  $G := G - v$
```

```txt
Si  $G = \emptyset$  alors sortie : "oui, G sans cycle" sinon sortie : "non, G possède un cycle"
```

Remarque I.8.4. Si on implémente cet algorithme à l'aide de listes d'adjacence (cf. chapitre V), la détction d'un sommet  $v$  pour lequel  $d^{-}(v) = 0$  nécessite de parcourir l'ensemble du graphe. Un tel parcours est effectué à chaque étape de la boucle. Cela a pour conséquence de fournir un algorithme dont la complexité est quadratique en fonction de  $\# E + \# V$  et celle-ci peut être améliorée.

Théorème I.8.5. Soit  $G = (V, E)$  un graphe simple orienté. Le graphe  $G$  est sans cycle si et seulement si il est possible d'énumérer26 les sommets de  $V = \{v_1, \ldots, v_n\}$  de manière telle que, pour tout  $i = 1, \ldots, n$ , le demi-degré entrant de  $v_i$  restreint au graphe  $G_i = G - v_1 - \dots - v_{i-1}$  soit nul, ce que l'on note  $d_{G_i}^- (v_i) = 0$ .

Démonstration. La condition est nécessaire. Supposons  $G$  sans cycle. Ainsi, par le lemme I.8.1, il existe un sommet  $v_{1}$  de  $G = G_{1}$  tel que  $d^{-}(v_{1}) = d_{G_{1}}^{-}(v_{1}) = 0$ . D'après la proposition précédente,  $G_{1} - v_{1} = G_{2}$  est sans cycle. Ainsi, il existe un sommet  $v_{2}$  tel que  $d_{G_{2}}^{-}(v_{2}) = 0$ . On continue de la sorte de proche en proche et on obtient l'énumération proposée.

La condition est suffisante. Supposons disposer d'une énumération des sommets ayant les propriétés indiquées. Procedons par récurrence. Le