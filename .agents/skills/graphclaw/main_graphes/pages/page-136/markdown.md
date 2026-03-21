Chapitre IV. Coloriage

|  g | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  cg | 4 | 7 | 8 | 9 | 10 | 11 | 12 | 12 | 13 | 13 | 14  |

TABLE IV.1. Les première valeurs de  $c_{g}$

une surface de genre  $g$ , alors ses faces peuvent être colorées avec  $c_{g}$  couleurs où

$$
c _ {g} = \left\lfloor \frac {1}{2} (7 + \sqrt {1 + 4 8 g}) \right\rfloor .
$$

La partie délicate consiste à montré qu'il existe effectivement un graphe nécessitant ce nombre de couleurs. En particulier, pour  $g = 0,1$ , on retrouve le théorème des 4 et des 7 couleurs.

# 3. Polynôme chromatique

Jusqu'à présent, nous nous sommes intéressés à la possibilité de colorier ou non un graphe donné à l'aide de  $k$  couleurs. La question suivante serait de déterminer le nombre de facons distinctes de réaliser un tel coloriage.

Définition IV.3.1. Soit  $G = (V, E)$  un multi-graphe non orienté ayant  $n$  sommets. On dénote par  $m_{k,G}$ , le nombre de coloriages propres distincts de  $G$  utilisant exactement  $k$  couleurs et par  $z^k$ , le polynôme en  $z \in \mathbb{C}$  de degré  $k$

$$
z ^ {\underline {{k}}} = z (z - 1) \dots (z - k + 1).
$$

Avec ces notations, le polynôme chromatique de  $G$  est donné par

$$
\begin{array}{l} \pi_ {G} (z) = \sum_ {i = 1} ^ {n} \frac {m _ {i , G}}{i !} z ^ {\underline {{i}}} \\ = \frac {m _ {1 , G}}{1 !} z + \frac {m _ {2 , G}}{2 !} z (z - 1) + \frac {m _ {3 , G}}{3 !} z (z - 1) (z - 2) + \dots \\ \dots + \frac {m _ {n , G}}{n !} z (z - 1) \dots (z - n + 1). \\ \end{array}
$$

Il s'agit d'un polynôme de degré  $n$  en la variable  $z$ .

Remarque IV.3.2. La quantité

$$
\frac {m _ {k , G}}{k !}
$$

est le nombre de partitions de  $V$  en  $k$  sous-ensembles (disjoints et non vides) donnant lieu à tous les coloriages propres possibles de  $G$  utilisant exactement  $k$  couleurs. En fait, cette quantité équivaut au nombre de partitions de  $V$  en  $k$  sous-ensembles (disjoints et non vides) de sommets indépendants.

Exemple IV.3.3. Soit le graphe de la figure IV.16. Considérons les par