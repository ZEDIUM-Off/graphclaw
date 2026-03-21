Chapitre II. Un peu de théorie algébrique des graphes

Avec les notations du théorème de Perron, si  $A$  est une matrice primitive, alors il est possible de montré que

$$
A ^ {k} = \lambda_ {A} ^ {k} v _ {A} \widetilde {w _ {A}} + o \left(\lambda_ {A} ^ {k}\right) \tag {3}
$$

ou  $v_{A}$  et  $\widetilde{w_{A}}$  sont des vecteurs propres choisis de telle sorte que  $\widetilde{w_{A}}.v_{A} = 1$ . Remarquez que  $v_{A}\widetilde{w_{A}}$  est une matrice carrée de dimension  $n$  dont les éléments sont tous strictement positifs. Autrement dit, ce résultat stipule qu'asymptotiquement, tout élément de  $A^{k}$  est proportionnel à  $\lambda_{A}^{k}$  (la constante de proportionnalité dépendant de l'élement envisagé). Tout comme nous avons admis le théorème de Perron, nous admettrons aussi ce résultat sortant du cadre que nous nous sommes fixés dans ce cours.

**Remarque** II.2.24. Il est même possible d'obtenir des développements plus fins du terme d'erreur en l'exprimant à l'aide de la deuxième valeur propre (par module décroissant) de  $A$ . Voir par exemple [26].

**Exemple II.2.25.** Si on poursuit l'exemple II.2.8, pour tout couple  $(i,j) \in \{1,2,3\} \times \{1,2,3\}$ , il existe une constante  $d_{i,j} &gt; 0$  telle que le nombre  $c_{i,j}(n)$  de chemins de longueur  $n$  joignant  $i$  à  $j$  satisfasse

$$
\lim  _ {n \to \infty} \frac {c _ {i , j} (n)}{d _ {i , j} \lambda_ {A} ^ {n}} = 1.
$$

**Remarque** II.2.26. Dans le cas d'un graphe f. connexe qui n'est pas primitif, on dispose du théorème de Perron-Frobenius. Néanmoins, si on a plusieurs valeurs propres de module maximum, des compensations entre celles-ci peuvent se produit, et fournir une estimation des  $c_{i,j}(n)$  semblable à celle donnée ci-dessus n'est pas si simple. En effet, si on reprend une fois encore l'exemple II.2.9 et que l'on s'intéresse à la suite formée des nombres de chemins de longueur  $n$  joignant 1 à 3 pour  $n = 0,1,2,\ldots$ , on obtient

$$
0, 0, 1, 0, 0, 1, 0, 0, 1, \dots
$$

qui est clairément une suite divergente. Ainsi, la limite

$$
\lim  _ {n \to \infty} \frac {c _ {1 , 3} (n)}{\lambda_ {A} ^ {n}}
$$

n'existe pas! Ceci s'explique par le fait que des combinaisons convenables de puissances des racines de l'unité s'annulent $^{12}$ :

$$
\frac {(e ^ {2 i \pi / 3}) ^ {n} + (e ^ {4 i \pi / 3}) ^ {n} + 1}{3} = 0, \mathrm {s i} n \equiv 1, 2 \pmod {3}.
$$

$M^k)_{i,j} = \sum_{t=1}^{p} P_{i,j}^{(t)} \lambda_t^k$