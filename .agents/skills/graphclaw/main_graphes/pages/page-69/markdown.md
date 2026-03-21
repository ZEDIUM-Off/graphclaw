I.11. Graphes hamiltoniens

A ce jour, on ne connait pas de condition suffisante plus générale pour qu'un graphe soit hamiltonien. Dans certains cas, on peut obtenir des conditions plus fortes mais en se restreignant à des classes particulières de graphes.

Démonstration. Nous pouvons tout d'abord supposer que $G$ est égal à sa fermeture, $G = \mathcal{F}(G)$. En effet, au vu du théorème I.11.12, un graphe est hamiltonien si et seulement si sa fermeture l'est. De plus, si un graphe $G$ satisfait la condition (1), alors le graphe $G + e$ la satisfait encore (c'est même encore plus "facile" puisque$^{37}$ moins de sommets vont vérifier $\deg(v_k) \leq k$). Nous allons montré qu'avec cette supposition, $G = K_n$.

Procédons par l'absurde et supposons $G \neq K_n$. Soient $u, v$ deux sommets de $G$ tels que $\deg(u) \leq \deg(v)$, $\{u, v\} \notin E$ et tels que $\deg(u) + \deg(v)$ soit maximal (nécessairement $&lt; n$, car sinon l'arête $\{u, v\}$ appartiendrait à $E$). On peut d'abord remarquer que

$$
\deg(u) = i &lt; n/2
$$

(car sinon, $\deg(u) + \deg(v) \geq n$). Soit

$$
A = \{w \in V \mid \{w, v\} \notin E \text{ et } w \neq v\}.
$$

Par notre choix de $u$, $\deg(w) \leq i$ pour tout $w \in A$ (en effet, on a choisi $\deg(u) + \deg(v)$ maximal). De plus, dans $A$, se trouvent tous les sommets distincts de $v$ et non voisins de $v$, ainsi,

$$
\#A = (n - 1) - \deg(v) \geq \deg(u) = i.
$$

On en conclut que dans $G$, il y a au moins $i$ sommets de degré au plus $i$. Si comme dans l'énoncé, on a ordonné les sommets de $G$ par degré croissant, on en conclut que $\deg(v_i) \leq \deg(u) = i$. Par hypothèse, cela entraîne que

$$
\deg(v_{n-i}) \geq n - i. \tag{2}
$$

On effectue à présent le même raisonnement avec

$$
B = \{w \in V \mid \{u, w\} \notin E \text{ et } w \neq u\}.
$$

Dès lors, pour tout $w \in B$,

$$
\deg(w) \leq \deg(v) &lt; n - \deg(u) = n - i
$$

et de plus,

$$
\#B = (n - 1) - \deg(u) = n - 1 - i.
$$

On a donc $n - 1 - i$ sommets de degré $&lt; n - i$. En outre, le sommet $u$ est tel que $\deg(u) &lt; n - i$. En effet, $i + \deg(v) = \deg(u) + \deg(v) &lt; n$ et

$^{37}$Soit $k$ entiers ordonnés par ordre croissant $x_1 \leq x_2 \leq \cdots \leq x_k$. Soit $i \in \{1, \ldots, k\}$. Si on remplace $x_i$ par $x_i + 1$ et que l'on note les éléments réordonnés $x_1' \leq \cdots \leq x_k'$, alors il existe $j \geq i$ tel que $x_i + 1 = x_j'$. Pour tout $t \in \{1, \ldots, k\}$, montrons que $x_t \leq x_t'$ (cela montre donc que si (1) est satisfait, l'ajout d'une arête ne va pas modifier cette propriété). Tout d'abord, si $i = j$, il n'y a rien à démontrer (l'ordre des éléments n'a pas changé). On peut donc supposer $j &gt; i$. Si $t &lt; i$ ou $t &gt; j$, alors $x_t = x_t'$. Si $i \leq t &lt; j$, alors $x_t' = x_{t+1} \geq x_t$. Si $t = j$, alors $x_j' \geq x_{t-1}' = x_t = x_j$.