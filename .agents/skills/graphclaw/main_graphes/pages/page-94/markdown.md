Chapitre II. Un peu de théorie algébrique des graphes

montrer par une discussion plus fine sur les valeurs propres des matrices envisagées qu'au plus  $\alpha$  est proche de 1, au plus le vecteur  $\pi$  est sensible aux petites perturbations de la matrice  $H$  (ce qui s'avère être génant vu la grande volatilité du web et de sa structure, de nombreux liens apparaisent et disparaisent chaque jour). Idéalement, on désirerait obtenir une mesure peu sensible à de telles perturbations.

Remarque II.3.1. On peut donner une interprétation "probabiliste" de la matrice  $G$ . Considérons un surfeur qui, se trouvant sur une page quelconque, a deux choix possibles. Soit, avec une probabilité  $\alpha$ , il clique au hasard et de manière uniforme sur l'un des liens de la page pour changer de page. Soit, avec une probabilité  $1 - \alpha$ , il désisit au hasard et de manière uniforme l'une des  $n$  pages de l'Internet tout entier. Ainsi,  $G_{ij}$  représente la probabilité de transition lorsque le surfeur se trouve sur la page  $i$  de passer à la page  $j$ .

Ainsi, partant d'une distribution initiale de probabilités, par exemple  $\pi^{(0)} = (\frac{1}{n},\dots ,\frac{1}{n})$ , l'application de  $G^k$  permet d'estimer la probabilité de notre surfeur de se trouver sur l'une des pages  $1,\ldots ,n$  après  $k$  clics,

$$
\pi^ {(k)} = \pi^ {(0)} G ^ {k}.
$$

Nous allons à présent montrer que l'utilisation de cette matrice  $G$  (à la place de  $H$ ) assure l'existence et l'unicité d'une distribution limite  $\pi$ .

Remarque II.3.2. Par rapport à l'équation initiale (5), l'emploi de la matrice  $G$  donne la formule suivante pour la détermination des "nouveaux"  $\pi_j$  qui seront effectivement calculés. On utilise la définition de  $S_{ij}$  pour obtenir

$$
\begin{array}{l} \pi_ {j} = \sum_ {i = 1} ^ {n} \pi_ {i} \left(\alpha S _ {i j} + (1 - \alpha) \frac {1}{n}\right) \\ = \alpha \sum_ {i \in \operatorname {p r e d} (j)} \frac {\pi_ {i}}{d ^ {+} (i)} + \frac {1}{n} \left(1 - \alpha + \alpha \sum_ {i: d ^ {+} (i) = 0} \pi_ {i}\right). \\ \end{array}
$$

Nous nous sommes donc éloignés qu'une peu du modele initialement proposé mais nous allons voir que ces modifications sont permettre un calcul efficace (en assurant de plus l'existence et l'unicité d'une solution!). On observe que le premier terme, à une constante multiplicative pres, est exactement (5). Alors que le second terme est une constante $^{18}$  &lt; 1 multipliee par  $1/n$  (avec  $n$  grand).

Proposition II.3.3. Les matrices  $S$ ,  $J / n$  et  $G$  sont stochastiques, autrement dit, la somme des éléments d'une ligne quelconque vaut 1.

Démonstration. Il s'agit d'une simple vérification.

Lemma II.3.4. Si  $M$  est une matrice stochastique, alors 1 est valeur propre dominante de  $M$ .