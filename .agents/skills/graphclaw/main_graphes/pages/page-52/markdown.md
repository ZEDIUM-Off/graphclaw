Chapitre I. Premier contact avec les graphes

graphe  $G_{n}$  est restreint à l'une somme  $v_{n}$  et est donc sans cycle. Le graphe  $G_{n-1}$  contient les sommets  $v_{n}$  et  $v_{n-1}$ . De plus,  $d_{G_{n-1}}^{-}(v_{n-1}) = 0$ . En d'autres termes,  $G_{n-1}$  possède au moins un arc de  $v_{n-1}$  à  $v_{n}$ . Il est donc sans cycle. Appliquons ce raisonnement pour une étape  $i$  quelconque. Si le graphe  $G_{i+1}$  est sans cycle, alors le graphe  $G_{i}$  se compose du graphe  $G_{i+1}$  auquel on ajoute  $v_{i}$  et évientuèlement des arcs de  $v_{i}$  vers les sommets de  $G_{i+1}$ . On en conclus que  $G_{i}$  est sans cycle.

On déduit de ce résultat un algorithme efficace.

Algorithm 1.8.6. La donnée fournie à cet algorithme est un graphe simple orienté  $G = (V, E)$ .

```txt
Pour tout  $v\in V$  ,initialiser  $\mathsf{d}(v) = 0$
Pour tout  $v\in V$  Pour tout  $w\in \mathrm{succ}(v)$ $\mathsf{d}(w) = \mathsf{d}(w) + 1$
aTraiter  $\coloneqq$  0
nbSommet  $\coloneqq 0$
Pour tout  $v\in V$  Si  $\mathsf{d}(v) = 0$  ,alors aTraiter=aTraiterU{v} nbSommet:=nbSommet+1
Tant que aTraiter  $\neq \emptyset$  ,faire Soit  $v$  ,le premier element de aTraiter aTraiter  $\coloneqq$  aTraiter  $\backslash \{v\}$  Pour tout  $w\in \mathrm{Succ}(v)$  ,faire  $\mathsf{d}(w) = \mathsf{d}(w) - 1$  si  $\mathsf{d}(w) = 0$  ,alors aTraiter=aTraiterU{w} nbSommet  $\coloneqq$  nbSommet+1
Si nbSommet  $\equiv$  #V alors sortie : "oui,  $G$  sans cycle" sinon sortie : "non,  $G$  possède un cycle"
```

La variable d associée à chaque sommet du graphe permet de stocker le demi-degré entrant du sommet (par rapport au graphe envisagé au moment de la construction). A chaque fois qu'un élément  $v$  est enlevé de la liste aTraiter, celui-ci est énuméré. Ainsi, on énumère d'abord les sommets de demi-degré entrant nul. Ensuite, lorsqu'un sommet est traité, on le supprime du graphe et on modifie en conséquence les demi-degrés entrants. Si tous les sommets ont été traités, cela correspond à dire que tous les sommets ont été énumérés. Au vu du théorème précédent, on en conclus que le graphe est sans cycle.

Définition I.8.7. Soit  $G = (V, E)$  un graphe simple orienté. Un tri topologique de  $G$  est une énumération  $v_1, \ldots, v_n$  des sommets de  $G$  de manière telle que si  $(v_i, v_j)$  est un arc de  $G$ , alors  $i &lt; j$ .