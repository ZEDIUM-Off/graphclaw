Chapitre II. Un peu de théorie algébrique des graphes

le nombre d'arbres ayant  $n$  sommets (quel que soit leur degré). Ainsi, cela revient à calculer la somme suivante

$$
\begin{array}{l} \sum_{\substack{d_{1},\ldots ,d_{n}\geq 1\\ d_{1} - 1 + \dots +d_{n} - 1 = n - 2}}\binom {n - 2}{d_{1} - 1,\ldots ,d_{n} - 1} = \sum_{\substack{i_{1},\ldots ,i_{n}\geq 0\\ i_{1} + \dots +i_{n} = n - 2}}\binom {n - 2}{i_{1},\ldots ,i_{n}} \\ = \underbrace {(1 + \cdots + 1) ^ {n - 2}} _ {n \times} = n ^ {n - 2}. \\ \end{array}
$$

Corollaire II.5.12 (Clarke (1958)). Le nombre d'arbres ayant  $n$  sommets de label respectif  $x_{1},\ldots ,x_{n}$  et dont les degrés sont disponibles par  $\deg (x_1) = d_1 = k$ , ...,  $\deg (x_n) = d_n$  vaut

$$
\mathrm {C} _ {n - 2} ^ {k - 1} (n - 1) ^ {n - k - 1}.
$$

Démonstration. En effet, vu le théorème II.5.8, il suffit de calculer

$$
\sum_{\substack{d_{2},\ldots ,d_{n}\geq 1\\ k - 1 + d_{2} - 1 + \dots +d_{n} - 1 = n - 2}}\binom {n - 2}{k - 1,d_{2} - 1,\ldots ,d_{n} - 1}.
$$

Cette somme se réécrit

$$
\frac{(n - 2)!}{(k - 1)!(n - k - 1)!}\sum_{\substack{d_{2},\ldots ,d_{n}\geq 1\\ d_{2} - 1 + \dots +d_{n} - 1 = n - k - 1}}\binom {n - k - 1}{d_{2} - 1,\ldots ,d_{n} - 1}.
$$

D'ou la conclusion en procédant comme dans la remarque précédente.

5.3. Nombre de sous-arbres couvrants. Nous allons tout d'abord étendre le problème posé initialement et compter les sous-arbres couvrants, pointés et orientés depuis la racine dans un multi-graphe orienté  $G = (V, E)$  où  $V = \{v_1, \ldots, v_n\}$ . Nous supposerons de plus que ce multi-graphe est sans boucle. Ceci ne constitue pas une veritable restriction par rapport au problème envisagé ici27.

Un arbre pointé est orienté depuis la racine si les arcs constituant celui-ci sont tous orientés des sommets de niveau  $i$  vers les sommets de niveau  $i + 1$  (pour  $i$  allant de 0 à la hauteur de l'arbre). Bien évidemment, le graphe non orienté sous-jacent est lui-même un arbre.

Exemple II.5.13. A la figure II.18, on a représenté un graphe orienté et un sous-arbre couvrant orienté depuis la racine 1.

Définition II.5.14. Soit  $G = (V, E)$  un multi-graphe orienté sans boucle dont les sommets sont ordonnés par  $V = \{v_1, \ldots, v_n\}$ . La matrice  $D(G)$  de demi-degré entrant est définie par

$$
[ D (G) ] _ {j, j} = d ^ {-} (v _ {j}) \quad \text {e t} \quad [ D (G) ] _ {i, j} = - (\# (\omega^ {+} (v _ {i}) \cap \omega^ {-} (v _ {j}))), \quad \text {s i} i \neq j.
$$