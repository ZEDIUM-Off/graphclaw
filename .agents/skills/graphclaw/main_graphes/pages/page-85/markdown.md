II.2. Théorie de Perron-Frobenius

**Lemme II.2.17.** Soit $A \geq 0$ une matrice carrée irréductible de période $p \geq 1$. Soit $i$ un indice de $A$. Il existe $N_i \geq 0$ tel que pour tout $n \geq N_i$, $[A^{np}]_{i,i} &gt; 0$.

**Démonstration.** Supposons tout d'abord que $[A^{kp}]_{i,i} &gt; 0$ et $[A^{\ell p}]_{i,i} &gt; 0$. Dès lors (même raisonnement que dans la preuve du lemme II.2.14),

$$
[A^{(k+\ell)p}]_{i,i} \geq [A^{kp}]_{i,i} [A^{\ell p}]_{i,i} &gt; 0.
$$

Cela signifie que l'ensemble $\mathcal{S}$ des multiples $np$ de $p$ qui sont tels que $[A^{np}]_{i,i} &gt; 0$ est stable pour l'addition (et $\mathcal{S}$ contient au moins un multiple de $p$). De plus, par définition, le p.g.c.d. des éléments de $\mathcal{S}$ vaut $p$. La conclusion découle alors du lemme suivant.

**Lemme II.2.18.** Soit $X \subseteq \mathbb{N}$ un ensemble d'entiers stable pour l'addition. Alors $X$ contient tous les multiples du p.g.c.d. des éléments de $X$ à l'exception éventuellement d'un nombre fini d'entre eux.

**Démonstration.** Soit $p$ le p.g.c.d. des éléments de $X$. Quitte à diviser les éléments de $X$ par $p$, on peut supposer que $p = 1$. Dès lors, il existe un ensemble fini⁹ $\{x_1, \ldots, x_k\} \subseteq X$ tel que

$$
\text{p.g.c.d. } \{x_1, \ldots, x_k\} = 1.
$$

Par le théorème de Bezout, il existe des entiers relatifs $\lambda_1, \ldots, \lambda_k \in \mathbb{Z}$ tels que

$$
\lambda_1 x_1 + \cdots + \lambda_k x_k = 1.
$$

Si on regroupe tous les termes dont les coefficients $\lambda_i$ sont positifs (resp. négatifs), cette somme se réécrit

$$
m - n = 1
$$

avec $m, n \in X$ car $X$ est stable pour l'addition. Soit $q$ un entier tel que $q \geq n(n-1)$. Par division euclidienne,

$$
q = a n + b, \quad 0 \leq b &lt; n.
$$

De plus, $a \geq n - 1$. Puisque $m - n = 1$, il vient

$$
q = a n + b (m - n) = (a - b) n + b m
$$

⁹Nous savons que le p.g.c.d. de $X = \{x_1 &lt; x_2 &lt; \cdots\}$ vaut 1. Si on considère tout d'abord $X$ restreint à $\{x_1\}$ seul, le p.g.c.d. potentiel serait $x_1$. Puis, quand on considère $\{x_1, x_2\}$, le p.g.c.d. potentiel ne peut que diminuer (ou au mieux rester constant) puisqu'on doit considérer cette fois les facteurs premiers communs à $x_1$ et à $x_2$. A l'étape suivante, on considère $\{x_1, x_2, x_3\}$ et ainsi de suite (à chaque étape, le p.g.c.d. décroît). Nous affirmons qu'il existe $k$ tel que le p.g.c.d. de $\{x_1, \ldots, x_k\}$ soit 1 car si tel n'était pas le cas, le p.g.c.d. de $X$ serait &gt; 1, ce qui est contraire à notre supposition. Remarquons en particulier que $k$ peut être &gt; 2, ne serait-ce par exemple qu'avec l'ensemble fini $\{6, 10, 15\}$ dont le p.g.c.d. vaut 1 mais qui est tel que ses éléments pris deux à deux ne sont pas premiers entre eux.