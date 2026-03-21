Chapitre II. Un peu de théorie algébrique des graphes

Tester la régularité d'un graphe peut se faire grâce à l'algèbre d'adjacence du graphe. On dénote par  $J_{n}$ , ou simplement  $J$ , la matrice carrée dont tous les éléments sont égaux à 1,  $J_{n} = (1)_{1 \leq i,j \leq n}$ . Il est clair que si on dispose d'un graphe  $k$ -régulier, alors

$$
A (G). J = k J = J. A (G).
$$

**Théorème II.4.5 (Hoffman (1963))**. La matrice  $J$  appartient à l'algèbre d'adjacence  $\mathcal{A}_G$  si et seulement si  $G$  est un multi-graphe (non orienté) connexe et régulier.

**Démonstration.** Supposons tout d'abord que  $J$  appartienne à  $\mathcal{A}_G$ . Puisque  $J$  est un polynôme de  $A(G) = A$ , il est clair $^{20}$  que  $AJ = JA$ . On a

$$
(A J) _ {i, j} = \deg (v _ {i}) \quad \text {et} \quad (J A) _ {i, j} = \deg (v _ {j}).
$$

Par conséquent, tous les sommets ont même degré et le graphe est régulier. Il nous reste à vérifier que  $G$  est connexe. S'il ne l'était pas, il existerait deux sommets  $v_{i}$  et  $v_{j}$  qui ne seraient joints par aucun chemin de longueur  $\ell \geq 0$ . Autrement dit,  $[A(G)^{\ell}]_{i,j} = 0$  pour tout  $\ell \geq 0$ . Cette relation restant valable pour tout polynôme en  $A(G)$ , on en concludeait que  $J$  ne peut appartenir à  $\mathcal{A}_G$ .

La condition est suffisante. Supposons  $G$  connexe et  $k$ -régulier. Ainsi, par la proposition précédente, puisque  $G$  est  $k$ -régulier,  $k$  est une valeur propre de  $A(G)$  et le polynôme minimum de  $A(G)$  est de la forme $^{21}$ $\mathcal{M}(\lambda) = (\lambda - k) q(\lambda)$  avec  $q(k) \neq 0$ . En évaluant ce polynôme en  $A$ , on obtient

$$
A q (A) = k q (A).
$$

Autrement dit, chaque colonne de  $q(A)$  est un vecteur propre de  $A$  de valeur propre  $k$ . Puisque  $G$  est connexe,  $k$  est une valeur propre simple et on en conclus que chaque colonne de  $q(A)$  est un multiple de  $(1, \ldots, 1)$ . Puisque  $A$  est symétrique,  $q(A)$  aussi et donc toutes les colonnes de  $q(A)$  sont égales à un même multiple de  $(1, \ldots, 1)$ . Autrement dit,  $q(A)$  est un multiple de  $J$  et cette dernière matrice appartient donc à  $\mathcal{A}_G$ .

Nous terminons cette section par un corollaire de nature algébrique.

**Corollaire II.4.6.** Soit  $G$  un multi-graphe non orienté connexe et  $k$ -régulier possédant  $n$  sommets et ayant  $k = \lambda_1 &gt; \lambda_2 &gt; \dots &gt; \lambda_s$  comme valeurs propres distinctes. Si

$$
q (\lambda) = \prod_ {i = 2} ^ {s} (\lambda - \lambda_ {i}),
$$