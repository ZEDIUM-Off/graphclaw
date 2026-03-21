Chapitre II. Un peu de théorie algébrique des graphes

$u \in V_1$  et  $v \in V_2$ . Si on ordonne les sommets de  $V$  de manière à considérer tout d'abord les sommets de  $V_1$ , alors  $A(G)$  a la forme

$$
\left( \begin{array}{c c} 0 &amp; B \\ \widetilde {B} &amp; 0 \end{array} \right)
$$

ou  $B$  est une matrice de dimension  $\# V_{1} \times \# V_{2}$ . Soit  $x$  un vecteur propre non nul de  $A(G)$  de valeur propre  $\lambda$ . Appelons  $x_{1}$  (resp.  $x_{2}$ ) le vecteur obtenu en considérant les  $\# V_{1}$  premières (resp. les  $\# V_{2}$  dernières) composantes de  $x$ . Ainsi,

$$
\left( \begin{array}{c c} 0 &amp; B \\ \widetilde {B} &amp; 0 \end{array} \right) \left( \begin{array}{c} x _ {1} \\ x _ {2} \end{array} \right) = \left( \begin{array}{c} B x _ {2} \\ \widetilde {B} x _ {1} \end{array} \right) = \lambda \left( \begin{array}{c} x _ {1} \\ x _ {2} \end{array} \right).
$$

Nous pouvons à présent facilement exuber un vecteur propre non nul de valeur propre  $-\lambda$

$$
\left( \begin{array}{c c} 0 &amp; B \\ \widetilde {B} &amp; 0 \end{array} \right) \left( \begin{array}{c} x _ {1} \\ - x _ {2} \end{array} \right) = \left( \begin{array}{c} - B x _ {2} \\ \widetilde {B} x _ {1} \end{array} \right) = \lambda \left( \begin{array}{c} - x _ {1} \\ x _ {2} \end{array} \right) = - \lambda \left( \begin{array}{c} x _ {1} \\ - x _ {2} \end{array} \right).
$$

Nous montrérons un peu plus loin qu'on dispose également de la réciropque de ce résultat (cf. corollaire II.2.6).

On peut aussi définir la matrice d'adjacence d'un multi-graphe orienté. La définition est analogue à celle donnée précédemment sauf qu'on tient ici compte de l'orientation. Il en résultat que la matrice obtenue n'est en général pas symétrique.

Definition II.1.10. Soit  $G = (V, E)$  un multi-graphe orienté dont les sommets sont ordonnés par  $V = \{v_1, \ldots, v_n\}$ . La matrice d'adjacence de  $G$  est la matrice  $A(G)$  dont l'élement  $[A(G)]_{i,j}$  est égal au nombre d'arcs  $(v_i, v_j)$  présents dans  $E$ ,  $1 \leq i, j \leq n$ .

Example II.1.11. Si on considère le graphe de la figure I.2, on obtient la matrice d'adjacence suivante.

$$
\left( \begin{array}{c c c c c} 0 &amp; 1 &amp; 0 &amp; 0 &amp; 1 \\ 0 &amp; 1 &amp; 1 &amp; 0 &amp; 0 \\ 0 &amp; 0 &amp; 1 &amp; 1 &amp; 1 \\ 1 &amp; 0 &amp; 0 &amp; 0 &amp; 0 \\ 1 &amp; 0 &amp; 0 &amp; 1 &amp; 0 \end{array} \right)
$$

Théorème II.1.12. Soit  $G = (V, E)$  un multi-graphe (orienté ou non) tel que  $V = \{v_1, \ldots, v_k\}$ . Pour tous  $i, j \in \{1, \ldots, k\}$  et pour tout  $n &gt; 0$ ,

$$
[ A (G) ^ {n} ] _ {i, j}
$$

est exactement le nombre de chemins de longueur n joignant  $v_{i}$  à  $v_{j}$ .

Démonstration. On procède par récurrence sur  $n$ . Le cas de base  $n = 1$  découle de la définition même de la matrice d'adjacence. Supposons le