II.4. Algèbre d'adjacence

linéairement indépendants et que  $\mathcal{A}_G$  contient donc au moins  $k + 1$  éléments linéairement indépendants.

Soit  $G = (V, E)$ , un multi-graphe non orienté connexe ayant  $A(G)$  comme matrice d'adjacence. Il est clair que puisque  $A(G)$  est diagonalisable, son polynôme minimum ne possède que des zéros simples et son degré est donc égal au nombre de valeurs propres distinctes de  $G$ . Remarquons aussi que si le polynôme minimum de  $A(G)$  est de degré  $k$ , il existe alors une relation linéaire liant  $A(G)^k, A(G)^{k-1}, \ldots, I$ . Par conséquent, la dimension de  $\mathcal{A}_G$  ne peut dépasser  $k$ .

Corollaire II.4.2. Soit  $G = (V, E)$ , un multi-graphe non orienté connexe ayant  $A(G)$  comme matrice d'adjacence et  $\mathcal{A}_G$  comme algèbre d'adjacence. Si  $\mathrm{diam}(G) = k$ , alors  $G$  a au moins  $k + 1$  valeurs propres distinctes.

Rappelons qu'un graphe non orienté est  $k$ -régulier si pour tout sommet  $v$ ,  $\deg(v) = k$ . Une fois encore, ces graphes possèdent des propriétés algébriques importantes.

Proposition II.4.3. Soit  $G = (V, E)$  un multi-graphe non orienté  $k$ -régulier. Alors

-  $k$  est une valeur propre de  $G$ ,
- pour toute valeur propre  $\lambda$  de  $G$ , on a  $|\lambda| \leq k$ ,
- si  $G$  est connexe,  $k$  est valeur propre simple (i.e., les multiplicités géométrique et algébrique valent 1).

Remarque II.4.4. Cette proposition peut aussi se réexprimer en termes de multi-graphes orientés  $k$ -réguliers. Il suffit de préciser au troisième point que  $G$  est fortement connexe.

Démonstration. Le premier point est immédiat. Il est clair que le vecteur  $(1, \ldots, 1)$  est un vecteur propre de  $A(G)$  de valeur propre  $k$ .

Passons au deuxième point et considérons une valeur propre  $\lambda$  de  $A(G)$  ayant  $y \neq 0$  comme vecteur propre. Soit  $y_{j}$  une composante de  $y$  de module maximum. On a

$$
| \lambda | | y _ {j} | = | [ A (G) y ] _ {j} | \leq \sum_ {i = 1} ^ {n} [ A (G) ] _ {j, i} | y _ {i} | \leq | y _ {j} | \sum_ {i = 1} ^ {n} [ A (G) ] _ {j, i} = k | y _ {j} |
$$

et donc,  $|\lambda| \leq k$ . On a utilisé le fait que les coefficients  $[A(G)]_{j,i}$  sont  $\geq 0$ , qu'au plus  $k$  d'entre eux sont non nuls et que  $\sum_{i=1}^{n} [A(G)]_{j,i} = k$ .

Pour le dernier point, puisque  $G$  est connexe,  $A(G)$  est irreductible et on peut donc utiliser le théorème de Perron-Frobenius. La matrice  $A(G)$  possède une unique valeur propre réelle dominante et au vu du point précédent, il s'agit de  $k$ .