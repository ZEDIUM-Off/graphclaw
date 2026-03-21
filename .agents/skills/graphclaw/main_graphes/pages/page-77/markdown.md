II.2. Théorie de Perron-Frobenius

résultat vérifié pour $n &gt; 0$ et vérifions-le pour $n + 1$. On a bien sûr

$$
[ A (G) ^ {n + 1} ] _ {i, j} = \sum_ {s = 1} ^ {k} [ A (G) ^ {n} ] _ {i, s} [ A (G) ] _ {s, j}.
$$

Par hypothèse de récurrence, $[A(G)^n]_{i,s}$ compte le nombre de chemins de longueur $n$ joignant $v_i$ à $v_s$. De plus, $[A(G)]_{s,j}$ compte le nombre d'arcs/arêtes joignant $v_s$ à $v_j$. Par conséquent, $[A(G)^n]_{i,s}[A(G)]_{s,j}$ compte le nombre de chemins de longueur $n + 1$ joignant $v_i$ à $v_j$ en passant par $v_s$, d'où la conclusion.

## 2. Théorie de Perron-Frobenius

La connexité d'un graphe se traduit par une propriété immédiate de sa matrice d'adjacence. On peut même, dans certains cas, obtenir des renseignements plus fins sur la longueur des chemins joignant deux sommets quelconques d'une composante connexe. Donnons tout d'abord deux définitions concernant les matrices à coefficients positifs ou nuls (faites attention dans les deux énoncés à l'ordre des quantificateurs). Nous verrons ensuite le rapport entre ces matrices et les graphes.

**Définition II.2.1.** Une matrice carrée $A = (a_{ij})_{1\leq i,j\leq n}$ à coefficients (réels) positifs ou nuls est *irréductible* si pour tous $i,j \in \{1,\ldots,n\}$, il existe⁴ un entier $N(i,j) \geq 0$ tel que

$$
[ A ^ {N (i, j)} ] _ {i, j} &gt; 0.
$$

**Définition II.2.2.** Une matrice carrée $A = (a_{ij})_{1\leq i,j\leq n}$ à coefficients (réels) positifs ou nuls est *primitive* s'il existe un entier $N &gt; 0$ tel que pour tous $i,j \in \{1,\ldots,n\}$

$$
[ A ^ {N} ] _ {i, j} &gt; 0
$$

ce que l'on s'autorise à noter $A^N &gt; 0$ étant sous-entendu que les inégalités sont interprétées composante à composante. On remarque aussi que toute matrice primitive est irréductible.

Dans cette section, pour deux matrices réelles $A$ et $B$ de même dimension, il sera commode d'écrire $A &lt; B$ (resp. $\leq, \geq, &gt;$ ) si l'inégalité a lieu composante à composante. Cela n'entraîne pas d'ambiguité particulière.

**Proposition II.2.3.** Un multi-graphe orienté (resp. non orienté) est fortement connexe (resp. connexe) si et seulement si sa matrice d'adjacence est irréductible.

**Démonstration.** C'est une conséquence directe du théorème II.1.12.

⁴ Avec un peu d'expérience, le lecteur pourra se convaincre que, dans cette définition, on aurait aussi pu imposer de manière équivalente $N(i,j) &gt; 0$.