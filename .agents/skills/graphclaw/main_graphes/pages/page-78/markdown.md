Chapitre II. Un peu de théorie algébrique des graphes

Remarque II.2.4. Si la matrice d'adjacence d'un graphe (orienté ou non) est primitive, cela entraîne que le graphe est non seulement connexe mais qu'il existe  $N$  tel que, quelle que soit la paire de sommets considérée, il existe un chemin de longueur  $N$  les joignant. Par abus de langage, on s'autorisera alors à parler de graphe primitif.

Lorsqu'une matrice est irréductible (ou, en particulier primitive), on dispose du puissant théorème de Perron-Frobenius.

Théorème II.2.5 (Perron-Frobenius). Soit  $A \geq 0$  une matrice carrée irréductible de dimension  $n$ .

$\triangleright$  La matrice  $A$  possède un vecteur propre  $v_{A} \in \mathbb{R}^{n}$  (resp.  $w_{A} \in \mathbb{R}^{n}$ ) dont les composantes sont toutes strictement positives et correspondant à une valeur propre  $\lambda_{A} &gt; 0$ ,

$$
A v _ {A} = \lambda_ {A} v _ {A} (r e s p. \widetilde {w _ {A}} A = \lambda_ {A} \widetilde {w _ {A}}).
$$

$\triangleright$  Cette valeur propre  $\lambda_{A}$  possède une multiplicité algébrique (et géométrique) simple.
$\triangleright$  Tout vecteur propre de  $A$  dont les composantes sont strictement positives est un multiple de  $v_{A}$ .
$\triangleright$  Toute autre valeur propre  $\mu \in \mathbb{C}$  de  $A$  est telle que  $|\mu |\leq \lambda_A$
Si  $\mu$  est une valeur propre de  $A$  telle que  $|\mu | = \lambda_{A}$  , alors

$$
\mu = \lambda_ {A} e ^ {2 i k \pi / d}
$$

pour un certain  $d \geq 1$  et  $k \in \{0, \dots, d - 1\}$ . De plus, pour tout  $k \in \{0, \dots, d - 1\}$ ,  $\lambda_A e^{2ik\pi / d}$  est une valeur propre de  $A$ .

$\triangleright$  Soit  $B$  une matrice réelle à coefficients positifs ou nuls de même dimension que  $A$ . Si  $B \leq A$ , alors pour toute valeur propre  $\mu$  de  $B$ , on a  $|\mu| \leq \lambda_A$  et l'égalité a lieu si et seulement si  $A = B$ .

La valeur propre  $\lambda_{A}$  est appelée la valeur propre de Perron de  $A$ . Autrement dit, ce théorème stipule qu'une matrice irréductible possède toujours une valeur propre réelle dominante  $\lambda_{A}$ . Cependant, on peut avoir d'autres valeurs propres de module égal à  $\lambda_{A}$  mais dans ce cas, celles-ci sont exactement obtenues par multiplication de  $\lambda_{A}$  par les racines  $d$ -ièmes de l'unité $^{5}$ .

Démonstration. Puisqu'il s'agit d'un cours de théorie des graphes et pas d'un cours d'algebre, nous ne donnons pas ici la preuve de ce résultat. Cette preuve n'est pas difficile mais assez longue (elle fait d'ailleurs intervenir des raisonnements de continuité et de passage à la limite). Le lecteur interressé trouvera une preuve classique dans [26] ou [9]. Une illustration assez géométrique de la preuve dans le cas de la dimension 2 est donnée dans [14]. On trouve aussi une preuve reposant sur la notion élégante de fonctions sous-harmoniques (sorte de généralisation de la notion de vecteur propre)