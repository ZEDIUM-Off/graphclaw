IV.4. Coloriage d'arêtes et théorème de Ramsey

Démonstration. Si on considère tous les coloriages propres de $G - e$ avec exactement $k$ couleurs. Il y en a de deux types : ceux pour lesquels on assigne aux extrémités de $e$ deux couleurs distinctes (resp. une même couleur). Ceux du premier type sont en bijection avec les coloriages propres de $G$ utilisant $k$ couleurs. Pour conclure, on remarque que ceux du second type sont en bijection avec les coloriages propres de $G \cdot e$ utilisant $k$ couleurs.

Corollaire IV.3.11. Le polynôme chromatique d'un arbre $T$ à $n$ sommets vaut

$$
\pi_T(z) = z(z - 1)^{n-1}.
$$

Démonstration. On procède par récurrence sur $n$. Le cas $n = 1$ est immédiat. Supposons le résultat acquis pour $n \geq 1$ et vérifions-le pour $n + 1$. Si un arbre possède $n + 1$ sommets, il a au moins un sommet de degré 1 et soit $e$, l'arête incidente à ce sommet. Ainsi, $T - e$ possède deux composantes : un sommet isolé dont le polynôme chromatique vaut $z$ et un arbre à $n$ sommets (qui n'est autre que la contraction $T \cdot e$) de polynôme chromatique $z(z - 1)^{n-1}$ (par hypothèse de récurrence). On en conclut que

$$
\pi_{T - e}(z) = z \pi_{T \cdot e}(z).
$$

Par la proposition précédente,

$$
\pi_T(z) = \pi_{T - e}(z) - \pi_{T \cdot e}(z) = (z - 1) \pi_{T \cdot e}(z) = z(z - 1)^n.
$$

## 4. Coloriage d'arêtes et théorème de Ramsey

Bien évidemment, on peut considérer le problème de coloriage "dual" consistant à vouloir attribuer à chaque arête d'un graphe une couleur de façon telle que deux arêtes adjacentes reçoivent des couleurs distinctes. Nous préférons dans cette section introduire le célèbre théorème de Ramsey qui possède de nombreuses applications dans diverses branches des mathématiques$^{8}$; aussi bien en théorie des nombres qu'en analyse harmonique ou qu'en géométrie.

La question que l'on se pose est la suivante. Etant donné un entier $s$, existe-t-il un entier $n$ (dépendant de $s$) tel pour tout coloriage des arêtes de $K_n$ avec deux couleurs (par convention, le rouge et le bleu), $K_n$ contient un sous-graphe $K_s$ formé d'arêtes d'une même couleur ? Il n'est pas évident qu'à priori cette question possède une solution. En effet, on pourrait penser que certains coloriages permettent de mettre en défaut la propriété désirée quel que soit $n$. En fait, il n'en est rien, pour $n$ suffisamment grand, il existe toujours dans $K_n$ une copie de $K_s$ monochromatique et ce, quel que soit le coloriage envisagé.

$^{8}$V. Rosta, *Ramsey Theory Applications*, www.combinatorics.org/Surveys/ds13.pdf