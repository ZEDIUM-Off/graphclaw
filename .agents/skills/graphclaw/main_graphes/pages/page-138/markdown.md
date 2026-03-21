Chapitre IV. Coloriage

Exemple IV.3.7. Avec le graphe de la figure IV.16, si on calcule  $\pi_G(4)$ , on trouve  $\pi_G(4) = 84$  et donc, il y est possible de colorier proprement  $G$  avec au plus 4 couleurs de 84 façon distinctes.

Démonstration. Tout d'abord, il est inutile de considérer dans l'expression de  $\pi_G(k)$ , les termes d'exposant  $&gt;k$ . En effet, si  $i &gt; k$ , alors  $z^i$  évalué en  $k$  est nul. Ainsi, on a

$$
\pi_ {G} (k) = \sum_ {i = 1} ^ {\mathbf {k}} \frac {m _ {i , G}}{i !} k ^ {\underline {{i}}}.
$$

Ensuite, pour tout  $i \leq k$ , on dispose de  $\frac{m_{i,G}}{i!}$  partitions de  $V$  en  $i$  sous-ensembles de sommets indépendants. Considérons ces partitions et autorisons-nous cette fois àCHOISIR  $i$  couleurs parmi  $k$ , pour chaque partition  $C_1 \cup \dots \cup C_i$  de  $V$ , à  $C_1$ , on peut assigner  $k$  couleurs, pour  $C_2$ , on a  $k - 1$  choix possibles, ... et pour  $C_i$ , on a  $k - i + 1$  choix possibles. Ainsi, chacune des  $\frac{m_{i,G}}{i!}$  partitions de  $V$  en  $i$  sous-ensembles donnent lieu à  $k^i$  coloriages utilisant exactement  $i$  des  $k$  couleurs disponibles. La conclusion en découle.

Example IV.3.8. En continuant l'exemple précédent, le nombre de coloriages du graphe repris à la figure IV.16 utilisant au plus 3 couleurs vaut

$$
\pi_ {G} (3) = \underbrace {m _ {1 , G}} _ {= 0} 3 + \frac {m _ {2 , G}}{2 !} 3. 2 + \frac {m _ {3 , G}}{3 !} 3. 2. 1 = 0 + 6 + 2. 6 = 1 8.
$$

Ces coloriages sont donnés par

|  v1 | v2 | v3 | v4  |
| --- | --- | --- | --- |
|  1 | 2 | 1 | 2  |
|  2 | 1 | 2 | 1  |
|  1 | 3 | 1 | 3  |
|  3 | 1 | 3 | 1  |
|  2 | 3 | 2 | 3  |
|  3 | 2 | 3 | 2  |
|  v1 | v2 | v3 | v4  |
| --- | --- | --- | --- |
|  1 | 2 | 1 | 3  |
|  1 | 3 | 1 | 2  |
|  2 | 1 | 2 | 3  |
|  2 | 3 | 2 | 1  |
|  3 | 1 | 3 | 2  |
|  3 | 2 | 3 | 1  |
|  v1 | v2 | v3 | v4  |
| --- | --- | --- | --- |
|  2 | 1 | 3 | 1  |
|  3 | 1 | 2 | 1  |
|  1 | 2 | 3 | 2  |
|  3 | 2 | 1 | 2  |
|  1 | 3 | 2 | 3  |
|  2 | 3 | 1 | 3  |

Corollaire IV.3.9. Le nombre chromatique de  $G$  est le plus petit entier  $k$  tel que  $\pi_G(k) \neq 0$ .

Ainsi, la détermination du nombre chromatique d'un graphe à  $n$  sommets peut se ramener à l'estimation des zéros d'un polynôme de degré  $n$ . Par exemple, avec le graphe repris à la figure IV.16, on a  $\pi_G(1) = 0$  et  $\pi_G(2) \neq 0$ .

Le polynôme chromatique d'un graphe peut être obtenu par une technique semblable à celle développée pour obtenir la formule de Cayley comptant le nombre de sous-arbres couvrants (proposition II.5.4).

Proposition IV.3.10. Si  $e$  est une arête de  $G$  (qui n'est pas une boucle), alors le polynôme chromatique satisfait la relation

$$
\pi_ {G} (z) = \pi_ {G - e} (z) - \pi_ {G \cdot e} (z).
$$