II.3. Une application : l'algorithm de PageRank

Démonstration. Soit  $M \in \mathbb{Q}_r^r$ . En multipliant tous les éléments de  $M$  par le p.p.c.m.  $\gamma$  des dénominateurs des éléments de  $M$ , la matrice

$$
M ^ {\prime} = \gamma M
$$

obtenue est telle que la somme des éléments de chaque ligne vaut  $\gamma \in \mathbb{N}$ . Il s'agit donc de la matrice d'adjacence d'un graphe  $\gamma$ -régulier. Comme nous le verrons, il suffit d'appliquer la proposition II.4.3 pour voir que  $\gamma M$  possède  $\gamma$  comme valeur propre dominante (i.e., toute autre valeur propre  $\mu$  est telle que  $|\mu| \leq \gamma$ ). La conclusion suit aisément en divisant par  $\gamma$ .

Par construction, la matrice  $G$  est primitive puisque toutes ses entrées sont strictement positives. On peut appliquer le théorème de Perron, or par le lemme précédent, nous savons déjà que 1 est valeur propre dominante de  $G$ . Par conséquent, la valeur propre dominante 1 est simple et il existe un unique vecteur colonne  $x &gt; 0$  (resp. ligne  $y &gt; 0$ ) tel que

$$
\sum_ {i = 1} ^ {n} x _ {i} = 1 \left(\text {r e s p .} \sum_ {i = 1} ^ {n} y _ {i} = 1\right) \quad \text {e t} \quad G x = x \left(\text {r e s p .} y G = y\right).
$$

Ainsi, déterminer le vecteur  $\pi$  des "PageRanks" revient à chercher le vecteur propre  $y$  de Perron à gauche de  $G$  (ou le vecteur propre de Perron à droite de  $\widetilde{G}$ ). En appliquant le résultat asymptotique (3) énoncé à la page 84, puisque  $e = (1\cdots 1)$  est un vecteur propre à droite de  $G$  de valeur propre 1 ( $G$  est stochastique) et que  $\pi.e = 1$  (puisque les scores sont normalisés), on a que

(7)  $G^{k} = e\pi + o(1)$  autrement dit,  $\lim_{k \to \infty} G^{k} = e\pi = \begin{pmatrix} 1 \\ \vdots \\ 1 \end{pmatrix} \left( \begin{array}{ccc} \pi_{1} &amp; \dots &amp; \pi_{n} \end{array} \right)$ .

Nous pouvons à présent obtenir aisément une méthode itérative pour estimer  $\pi$ . Soit  $p^{(0)} = \left(p_1^{(0)} \cdots p_n^{(0)}\right) &gt; 0$  un vecteur tel que  $\sum_{i} p_i^{(0)} = 1$ . Pour tout  $k \geq 1$ , on pose  $p^{(k)} = p^{(0)} G^k = p^{(k-1)} G$ . Il nous reste à montré que

$$
\lim  _ {k \rightarrow \infty} p ^ {(k)} = \pi
$$

ainsi, il suffit des lors de partir d'une distribution initiale et d'appliquer  $G$  de manière itérative jusqu'à obtenir la précision voulue mesurée par  $p^{(k)} - p^{(k-1)}$  (différence en norme, bien entendu). C'est immédiat, au vu de (7),

$$
\lim  _ {k \to \infty} G ^ {k} = \left( \begin{array}{c c c c} \pi_ {1} &amp; \pi_ {2} &amp; \dots &amp; \pi_ {n} \\ \vdots &amp; \vdots &amp; &amp; \vdots \\ \pi_ {1} &amp; \pi_ {2} &amp; \dots &amp; \pi_ {n} \end{array} \right) =: P
$$

et

$$
[ p ^ {(0)} P ] _ {j} = \sum_ {i = 1} ^ {n} p _ {i} ^ {(0)} \pi_ {j} = \pi_ {j} \underbrace {\sum_ {i = 1} ^ {n} p _ {i} ^ {(0)}} _ {= 1} = \pi_ {j}.
$$