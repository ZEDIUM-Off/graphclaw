Chapitre II. Un peu de théorie algébrique des graphes

**Exemple II.2.13.** Dans l'exemple II.2.8, on a $[A(G)^n]_{3,3} &gt; 0$ pour tout $n \geq 3$ et le p.g.c.d. des éléments de l'ensemble $X = \{3,4,5,\ldots\}$ est 1. Ainsi, le sommet 3 est de période 1.

Par contre, dans l'exemple II.2.9, $[A(G)^n]_{i,i} &gt; 0$ si et seulement si $n$ est un multiple de 3. Ainsi, les périodes des sommets de ce graphe sont toutes égales à 3.

**Lemme II.2.14.** Soient $i, j$ deux indices de $A \geq 0$. S'il existe $m, n$ tels que $[A^m]_{i,j} &gt; 0$ et $[A^n]_{j,i} &gt; 0$, alors $p(i) = p(j)$.

**Remarque II.2.15.** En utilisant la remarque II.2.12, le lemme se réexprime comme suit. S'il existe dans $G_A$ un chemin de longueur $m$ joignant $i$ et $j$ et un chemin de longueur $n$ joignant $j$ et $i$, autrement dit si $i \leftrightarrow j$, alors les deux sommets ont même période. (Ou encore, tous les sommets d'une composante f. connexe ont même période.)

**Démonstration.** Pour tout $s$ tel que $[A^s]_{j,j} &gt; 0$, on a

$$
\begin{aligned}
[ A^{m + s + n} ]_{i, i} &amp;= \sum_{k = 1}^{d} [ A^{m + s} ]_{i, k} [ A^{n} ]_{k, i} \\
&amp;\geq [ A^{m + s} ]_{i, j} [ A^{n} ]_{j, i} \\
&amp;= \sum_{k = 1}^{d} [ A^{m} ]_{i, k} [ A^{s} ]_{k, j} [ A^{n} ]_{j, i} \\
&amp;\geq [ A^{m} ]_{i, j} [ A^{s} ]_{j, j} [ A^{n} ]_{j, i} &gt; 0
\end{aligned}
$$

Les deux inégalités proviennent du fait que les éléments de $A$ sont positifs ou nuls. Pour un tel $s$, on a aussi $[A^{2s}]_{j,j} &gt; 0$ (en effet, si on effectue le produit matriciel $[A^s.A^s]_{j,j}$, on retrouve le terme $[A^s]_{j,j}.[A^s]_{j,j}$ et les autres termes de la somme sont positifs ou nuls). Dès lors, on a aussi

$$
[ A^{m + 2s + n} ]_{i, i} &gt; 0.
$$

Par conséquent, $p(i)$ divise $m + 2s + n$ et $m + s + n$ et aussi leur différence, $s$. En conclusion, pour tout $s$ tel que $[A^s]_{j,j} &gt; 0$, $p(i)$ divise $s$ et donc $p(i) \leq p(j)$. Par symétrie, on a aussi que $p(j) \leq p(i)$ et donc $p(i) = p(j)$.

Grâce à ce lemme, nous pouvons introduire la définition suivante. En effet, pour une matrice irréductible, toutes les périodes sont nécessairement identiques. En particulier, tous les sommets d'une composante f. connexe de $G_A$ ont même période.

**Définition II.2.16.** Une matrice irréductible $A \in \mathbb{R}_d^d$ est cyclique de période $p$ si tous les indices de $A$ sont de période $p &gt; 1$. Sinon, tous les indices sont de période $p = 1$ et $A$ est dite acyclique.

$^8$ $p(i)$ est un diviseur commun des éléments de l'ensemble $\{s &gt; 0 \mid [A^s]_{j,j} &gt; 0\}$ et d'autre part, $p(j)$ le p.g.c.d. de cet ensemble.