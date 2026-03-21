69

# CHAPITRE II

# Un peu de théorie algébrique des graphes

Dans ce chapitre, nous ne faisons qu'esquisser quelques résultats mettant en évidence les liens entre la théorie des graphes et l'algèbre linéaire (par exemple, on étudie le spectre des graphes réguliers ou bipartis). On y présente de manière détaillée la théorie de Perron-Frobenius (sans démonstration du théorème principal) avec comme application directe, l'estimation du nombre de chemins de longueur $n$ que l'on peut trouver dans un graphe à composantes connexes primitives. On présente également diverses questions relatives aux sous-arbres couvrants : nombre de tels sous-arbres, recherche d'un arbre de poids minimal, algorithme de Prim et de Kruskal. En particulier, ce chapitre met en lumière quelques raisonnements de combinatoire énumérative.

## 1. Matrice d'adjacence

**Définition II.1.1.** Soit $G = (V, E)$ un multi-graphe *non orienté* dont les sommets sont ordonnés par $V = \{v_1, \ldots, v_n\}$. La matrice d'adjacence de $G$ est la matrice $A(G)$ dont l'élément $[A(G)]_{i,j}$ est égal au nombre d'arêtes $\{v_i, v_j\}$ présentes dans $E$, $1 \leq i, j \leq n$. (Pour rappel, $E$ est en général un multi-ensemble.) Il s'agit donc d'une matrice symétrique à coefficients entiers naturels. Le polynôme caractéristique de $G$, noté $\chi_G(\lambda)$, est le polynôme caractéristique de sa matrice d'adjacence $A(G)$. Par abus de langage, on parlera des valeurs propres de $G$, étant sous-entendu qu'il s'agit des valeurs propres de $A(G)$. On parlera donc aussi du spectre de $G$.

On peut remarquer que les éléments de la matrice d'adjacence d'un graphe *simple* appartiennent à $\{0,1\}$ et que la trace de cette matrice vaut 0.

**Remarque II.1.2.** En se rappelant quelques résultats du cours d'algèbre de première année, on remarque que la matrice d'adjacence d'un graphe non orienté est toujours diagonalisable par une matrice orthogonale (pour chaque valeur propre, les multiplicités algébrique et géométrique coïncident) et que ses valeurs propres sont réelles.

**Exemple II.1.3.** Considérons le graphe (simple) $G$ de la figure II.1. Avec les notations précédentes, on a aussi

$$
\chi_G(\lambda) = -\lambda^5 + 8\lambda^3 + 10\lambda^2 + \lambda - 2.
$$

**Proposition II.1.4.** Deux graphes $G_1$ et $G_2$ sont isomorphes si et seulement si ils ont, à une permutation près, la même matrice d'adjacence.