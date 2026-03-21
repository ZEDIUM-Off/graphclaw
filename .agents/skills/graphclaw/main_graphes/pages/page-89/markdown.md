II.2. Théorie de Perron-Frobenius

2.3. Cas d'un graphe ayant plusieurs composantes f. connexes. Nous pouvons obtenir des résultats plus fins que ceux obtenus ci-dessus. On considère le condensé  $\mathcal{C}$  d'un graphe  $G$  (ou graphe acyclique des composantes cf. définition I.4.27) dont les sommets sont les composantes f. connexes de  $G$ . Puisque le condensé est sans cycle, on peut ordonner ses sommets par tri topologique. Supposons disposer d'un tel ordonnancement. La matrice d'adjacence de  $\mathcal{C}$  a alors une forme triangulaire supérieure et si on ordonne les éléments de  $G$  en considérant les sommets des composantes f. connexes prises une à une en respectant le tri topologique, la matrice d'adjacence de  $G$  est alors une matrice bloc triangulaire supérieure.

Example II.2.27. Poursuivons l'exemple I.4.29. Pour celui-ci, on obtient la matrice suivante si on considère l'ordre donné à la figure I.38.

|  A(G)= | 0 1 0 | 0 0 0 | 0 0 0 | 0 0 0 | 0 0 0 | 0 0 0  |
| --- | --- | --- | --- | --- | --- | --- |
|   |  0 0 1 | 0 0 0 | 0 0 0 | 0 0 0 | 0 0 0 | 0 0 0  |
|   |  1 0 0 | 1 0 0 | 0 1 0 | 0 0 0 | 0 0 0 | 0 0 0  |
|   |  0 0 0 | 0 1 0 | 1 0 0 | 0 0 0 | 0 0 0 | 0 0 0  |
|   |  0 0 0 | 0 0 1 | 0 0 0 | 0 0 0 | 0 0 0 | 0 0 0  |
|   |  0 0 0 | 0 0 1 | 0 0 0 | 0 0 0 | 0 0 1 | 0 0 0  |
|   |  0 0 0 | 0 0 0 | 0 0 0 | 0 1 0 | 0 0 0 | 0 0 0  |
|   |  0 0 0 | 0 0 0 | 0 0 0 | 0 0 1 | 0 0 0 | 0 0 0  |
|   |  0 0 0 | 0 0 0 | 0 0 0 | 0 0 0 | 1 0 0 | 0 0 0  |
|   |  0 0 0 | 0 0 0 | 0 0 0 | 0 0 0 | 1 1 0 | 0 0 0  |

Remarque II.2.28. Au vu de l'exemple précédent, il est immédiat d'observer que le spectre d'un graphe est l'union des spectres de ses composantes connexes.

Nous nous restreignons une fois encore au cas où les composantes f. connexes sont des sous-graphes primitifs.

Soit un graphe fini possèdant deux composantes f. connexes primitives  $A$  et  $B$  et deux sommets  $a \in A$  et  $b \in B$  tels que  $a \to b$ . Les chemins joignant un sommet de  $A$  à un sommet de  $B$  sont en nombre fini et de longueur bornée. Si  $\lambda_A$  et  $\lambda_B$  sont les valeurs propres de Perron de  $A$  et de  $B$  respectivement, on en déduit que le nombre  $c_{a,b}(n)$  de chemins de longueur  $n$  joignant  $a$  à  $b$  est proportionnel $^{13}$  à

$$
\sum_ {i = 0} ^ {n} \lambda_ {A} ^ {i} \lambda_ {B} ^ {n - i} = \lambda_ {B} ^ {n} \sum_ {i = 0} ^ {n} \left(\frac {\lambda_ {A}}{\lambda_ {B}}\right) ^ {i}.
$$