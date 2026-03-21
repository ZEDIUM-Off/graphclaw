Chapitre II. Un peu de théorie algébrique des graphes

avec $a - b \geq 0$. On en conclut que $q$ appartient à $X$ (car $m, n \in X$). Nous avons donc montré que tout entier $q \geq n(n - 1)$ appartient à $X$. Cela termine la preuve.

Le résultat suivant montre que l'exemple II.2.9 est l'archétype même de la situation rencontrée dans le cas d'un graphe irréductible.

**Théorème** II.2.19. Soit $A \geq 0$ une matrice carrée irréductible de période $p \geq 1$. Pour toute paire $i, j$ d'indices de $A$, il existe un unique entier $r_{i,j} \in \{0, \ldots, p - 1\}$ tel que

- $[A^n]_{i,j} &gt; 0$ entraîne $n \equiv r_{i,j} \pmod{p}$ et
- il existe $N_{i,j}$ tel que $[A^{np + r_{i,j}}]_{i,j} &gt; 0$ pour tout $n \geq N_{i,j}$.

**Exemple** II.2.20. On peut reprendre l'exemple II.2.9. Dans cet exemple, $p = 3$ et si on fixe le sommet $i = 2$, on a $r_{2,1} = 2$, $r_{2,2} = 0$ et $r_{2,3} = 1$. En effet, $[A(G)^{3n + 2}]_{2,1} = 1$, $[A(G)^{3n + 0}]_{2,2} = 1$ et $[A(G)^{3n + 1}]_{2,3} = 1$.

**Démonstration.** Supposons que $[A^m]_{i,j} &gt; 0$ et $[A^n]_{i,j} &gt; 0$. Nous allons montrer que $m \equiv n \pmod{p}$. Puisque $A$ est irréductible, il existe $\ell$ tel que $[A^\ell]_{j,i} &gt; 0$. Dès lors,

$$
[ A ^ {m + \ell} ] _ {i, i} \geq [ A ^ {m} ] _ {i, j} [ A ^ {\ell} ] _ {j, i} &gt; 0 \quad \text{et} \quad [ A ^ {n + \ell} ] _ {i, i} &gt; 0.
$$

La période $p$ divise donc $m + \ell$ et $n + \ell$ donc leur différence. Autrement dit, $m - n \equiv 0 \pmod{p}$.

Passons à la deuxième partie. Puisque $A$ est irréductible, il existe $\ell$ tel que $[A^\ell]_{i,j} &gt; 0$ et au vu de la première partie,

$$
\ell = m p + r _ {i, j}.
$$

Posons

$$
N _ {i, j} = N _ {i} + m
$$

(avec $N_{i}$ donné dans le lemme II.2.17). Par définition de $N_{i}$, on a

$$
\forall n \geq N _ {i}, [ A ^ {n p} ] _ {i, i} &gt; 0.
$$

De là, si $k \geq N_{i,j}$, alors

$$
k p + r _ {i, j} = (n + m) p + r _ {i, j} \quad \text{avec} \quad n \geq N _ {i}.
$$

et

$$
[ A ^ {k p + r _ {i, j}} ] _ {i, j} \geq [ A ^ {n p} ] _ {i, i} [ A ^ {m p + r _ {i, j}} ] _ {i, j} &gt; 0.
$$

**Proposition** II.2.21. Une matrice irréductible est acyclique si et seulement si elle est primitive.

**Démonstration.** Si la matrice est acyclique (i.e., de période $p = 1$), alors avec les notations du théorème II.2.19, $r_{i,j} = 0$ quels que soient les indices $i$ et $j$. On en conclut que

$$
[ A ^ {n} ] _ {i, j} &gt; 0 \quad \text{si} \quad n \geq N _ {i, j}.
$$