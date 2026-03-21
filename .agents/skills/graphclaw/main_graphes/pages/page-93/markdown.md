II.3. Une application : l'algorithm de PageRank

|  α | nombre d'itérations  |
| --- | --- |
|  0,5 | 34  |
|  0,75 | 81  |
|  0,8 | 104  |
|  0,85 | 142  |
|  0,9 | 219  |
|  0,95 | 449  |
|  0,99 | 2292  |
|  0,999 | 23015  |

TABLE II.1. Rôle du paramètre  $\alpha$

de calcul). Autrement dit, il n'y a pas de raison evidente pour laquelle 1 est valeur propre de  $H$  et qui plus est, valeur propre simple.

L'astuce pour pouvoir appliquer la théorie de Perron-Frobenius est double. Tout d'abord, pour se débarrasser des "puits", i.e., des pages ne pointant vers aucune autre page et pour obtenir une matrice stochastique, on introduit une matrice  $S$  (" $S$ " comme "stochastique") définie par

$$
S _ {i j} = \left\{ \begin{array}{l l} A (G) _ {i j} / d ^ {+} (i) &amp; \mathrm {s i} d ^ {+} (i) &gt; 0 \\ 1 / n &amp; \mathrm {s i} d ^ {+} (i) = 0. \end{array} \right.
$$

Ensuite, pour assurer la forte connexité du graphe (i.e., le caractère irreductible), on construit une matrice  $G$  (" $G$ " comme Google) donnée par la combinaison affine (et même convexe) suivante avec un réel  $\alpha \in [0,1]$  fixé,

$$
G = \alpha S + (1 - \alpha) J / n
$$

ou  $J = (1)_{1\leq i,j\leq n}$  . L'equation initiale (6) est replacée par

$$
\pi = \pi G.
$$

(La matrice  $J / n$  est parfois appelée matrice de téléportation, cf. remarque suivante.) Le lecteur préférant manipuler des vecteurs propres à droite écrire  $\widetilde{\pi} = \widetilde{G}\widetilde{\pi}$ . Google attribue à  $\alpha$  une valeur de 0,85. Ce besoin n'est pas arbitraire. Au plus  $\alpha$  est proche de 1, au moins on approche le modele "naturel" (6) proposé initialement : on diminue le role artificiel de la matrice de téléportation. Cependant, on peut montré que ce paramètre  $\alpha$  contrôle la vitesse de convergence de la méthode de calcul développée et donc le nombre d'iterations à effectuer pour obtenir une estimation du vecteur  $\pi$  (par calcul de puissances successives de  $G$  qui contient plus de  $8.10^{9}$  entrées). Quand  $\alpha$  tend vers 1, ce nombre devient prohibitif comme le montre la table II.1 (tirée de l'ouvrage de C. Meyer et A. Langville). Ainsi,  $\alpha = 0,85$  semble un bon compromis entre le caractère artificiel introduit par la matrice de téléportation et la masse de calculs à réaliser. De plus, on peut

$G$  est un point du segment  $[G, J / n]$  de l'espace des matrices.

Monsieur Spock!