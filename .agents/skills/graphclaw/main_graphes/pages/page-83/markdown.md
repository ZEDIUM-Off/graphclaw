II.2. Théorie de Perron-Frobenius

Les valeurs propres sont ici les racines cubiques de l'unité

$$
1, e ^ {2 i \pi / 3}, e ^ {4 i \pi / 3}
$$

et donc, on a ici, par opposition avec l'exemple précédent, plusieurs valeurs propres de module maximum  $(= 1)$ .

Il est aussi intéressant de noter que pour joindre deux sommets fixés, uniquement certaines longueurs de chemin peuvent être considérées. Par exemple, pour joindre les sommets 2 et 3, uniquement des chemins de longueur congrue à 1 modulo 3 peuvent être envisagés. Ce phénomène est en fait tout à fait général et sera explicité à la section suivante.

**Corollaire** II.2.10. Soit $A \geq 0$ une matrice carrée. Les assertions suivantes sont équivalentes.

i) $A$ est primitive,
ii) il existe $N \geq 1$ tel que $A^N &gt; 0$,
iii) il existe $N \geq 1$ tel que $A^n &gt; 0$ pour tout $n \geq N$.

**Démonstration.** Par définition, i) $\Rightarrow$ ii) et ii) $\Rightarrow$ i). Montrons que ii) $\Rightarrow$ iii). Puisque $A^N &gt; 0$, on en déduit que toute colonne de $A$ contient au moins un élément strictement positif. Par conséquent, si $A^k &gt; 0$, alors $A^k \cdot A &gt; 0$ et de proche en proche, $A^{k + i} &gt; 0$ pour tout $i \geq 0$. Enfin, il est immédiat que iii) $\Rightarrow$ ii).

---

**2.1. Période d'une matrice irréductible.** Soit $(a_{ij})_{1\leq i,j\leq d} = A \geq 0$ une matrice carrée de dimension $d$ à coefficients positifs ou nuls. Par indice de $A$, on entend un élément de $\{1,\ldots ,d\}$.

**Définition** II.2.11. Soit $i$ un indice. S'il existe $N &gt; 0$ tel que $[A^N]_{i,i} &gt; 0$, alors la période de l'indice $i$ est le p.g.c.d. de l'ensemble des entiers $n &gt; 0$ pour lesquels

$$
[ A ^ {n} ] _ {i, i} &gt; 0.
$$

On la note $p(i)$. Bien évidemment, le p.g.c.d. d'un ensemble infini d'entiers $X = \{x_{1} &lt; x_{2} &lt; \dots \} \subseteq \mathbb{N}$ est le plus grand entier $p$ appartenant à l'ensemble fini $\{1,2,\ldots ,x_1\}$ tel que pour tout $k \geq 1$, $p$ divise $x_{k}$.

**Remarque** II.2.12. Cette définition est donnée en termes de matrices, mais elle possède un analogue immédiat en termes de graphes. En effet, à la matrice $A = (a_{ij})_{1\leq i,j\leq d}$, on fait correspondre un graphe $G_{A} = (V_{A},E_{A})$ ayant pour ensemble de sommets $V_{A} = \{1,\ldots ,d\}$ et il existe un arc joignant $i$ à $j$ si et seulement si $a_{i,j} &gt; 0$. Ainsi, pour définir la période d'un sommet $i\in V_A$ appartenant à une composante f. connexe du graphe $G_{A}$, on recherche le p.g.c.d. de l'ensemble des entiers $k$ pour lesquels il existe au moins un circuit de longueur $k$ passant par $i$.