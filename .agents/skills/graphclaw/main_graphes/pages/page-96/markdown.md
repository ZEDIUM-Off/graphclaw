Chapitre II. Un peu de théorie algébrique des graphes

Remarque II.3.5. En pratique, une centaine d'iterations suffisent pour obtenir une approximation utilisable et ce calcul peut être réalisé hors ligne, par exemple, une fois par mois, pourmettre à jour le vecteur des scores.

Remarque II.3.6. En pratique, on se ramène à la matrice creuse  $H$  (possédant de nombreux zéros) :

$$
\begin{array}{l} p ^ {(k)} = p ^ {(k - 1)} G \\ = p ^ {(k - 1)} \left(\alpha S + (1 - \alpha) \frac {J}{n}\right) \\ = \alpha p ^ {(k - 1)} S + (1 - \alpha) \frac {\widetilde {e}}{n} \\ = \alpha p ^ {(k - 1)} \left(H + a \frac {\widetilde {e}}{n}\right) + (1 - \alpha) \frac {\widetilde {e}}{n} \\ = \alpha p ^ {(k - 1)} H + (\alpha p ^ {(k - 1)} a + (1 - \alpha)) \frac {\widetilde {e}}{n} \\ \end{array}
$$

ou

$$
a = \left( \begin{array}{c} a _ {1} \\ \vdots \\ a _ {n} \end{array} \right)
$$

est tel que  $a_{i} = 1$  si  $d^{+}(i) = 0$  et  $a_{i} = 0$  si  $d^{+}(i) &gt; 0$ .

# 4. Algèbre d'adjacence

Le but principal de cette section est de fournir quelques résultats élémentaires à propos des graphes réguliers.

Soit  $G = (V, E)$ , un multi-graphe non orienté. On dénote par  $\mathcal{A}_G$  l'algebre $^{19}$  (sur  $\mathbb{C}$ ) des polynômes en la matrice d'adjacence de  $G$ . On la dénomme l'algebre d'adjacence de  $G$ .

Proposition II.4.1. Soit  $G = (V, E)$ , un multi-graphe non orienté connexe ayant  $A(G)$  comme matrice d'adjacence et  $\mathcal{A}_G$  comme algèbre d'adjacence. Si  $\mathrm{diam}(G) = k$ , alors la dimension de  $\mathcal{A}_G$  est supérieure ou égale à  $k + 1$ .

Démonstration. Soient  $a$  et  $b$  deux sommets réalisant le diamètre de  $G$ , i.e., tels que  $\mathrm{d}(a, b) = k$ . Soit

$$
\{a = v _ {0}, v _ {1} \}, \{v _ {1}, v _ {2} \}, \dots , \{v _ {k - 1}, v _ {k} = b \}
$$

un chemin de longueur  $k$  réalisant cette distance (cela implique en particulier que les  $v_{i}$  sont tous distincts). Par définition même du diamètre, pour tout  $i \in \{1, \ldots, k\}$ , il existe un chemin de longueur  $i$  joignant  $v_{0}$  à  $v_{i}$  mais aucun chemin de longueur inférieure. Par conséquent, pour tout  $i \in \{1, \ldots, k\}$ ,  $A^{i}$  (resp.  $A^{j}$ ,  $0 \leq j &lt; i$ ) contient une entrée strictement positive (resp. nulle) pour l'entrée correspondant à  $v_{0}, v_{i}$ . On en conclus que  $I, A, \ldots, A^{i}$  sont