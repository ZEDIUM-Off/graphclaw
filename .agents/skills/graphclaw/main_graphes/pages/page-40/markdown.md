Chapitre I. Premier contact avec les graphes

l'avons déjà remarqué plus haut (remarque I.4.28), le condensé de  $G$  (ayant les  $C_i$  pour sommets) ne possède pas de cycle. On en conclus $^{22}$  qu'il existe (au moins) une composante  $C_j$  telle que tout arc issu d'un sommet de  $C_j$  possède sa destination également dans  $C_j$  (dans l'exemple correspondant à la figure I.38, c'est le cas de la composante  $C_4$ ). Par hypothèse,  $G$  est s. connexe. Il existe donc une autre composante  $C_i$  et un arc joignant un sommet  $x$  de  $C_i$  à un sommet  $y$  de  $C_j$ .

Dans le graphe  $G' = (V', E')$  restreint à la composante  $C_j$ , i.e., dans le graphe où l'on considère uniquement les sommets de  $C_j$  et les arcs ayant leurs deux extrémités dans  $C_j$ , la "handshaking formula" est satisfaite, i.e.,

$$
\sum_ {v \in V ^ {\prime}} \mathrm {d} ^ {-} (v) = \sum_ {v \in V ^ {\prime}} \mathrm {d} ^ {+} (v).
$$

Il est sous-entendu que si le symbole de sommation fait référence à  $V'$ , alors on considère le graphe  $G'$  correspondant. Revenons au graphe  $G$  de départ. Dans celui-ci, tout arc ayant son origine dans  $C_j$  a aussi son extrémité dans  $C_j$ . Ainsi, considérer le graphe  $G$  dans son entièreté ou sa restriction  $G'$  à  $C_j$  ne modifie pas le demi-degré sortant des sommets de  $C_j$  et on a

$$
\sum_ {v \in C _ {j}} \mathrm {d} ^ {+} (v) = \sum_ {v \in V ^ {\prime}} \mathrm {d} ^ {+} (v).
$$

Par contre,  $y$  est un sommet de  $C_j$  et nous savons qu'au moins un arc n'ayant pas son origine dans  $C_j$  aboutit en  $y$ . Autrement dit, nous avons cette fois,

$$
\sum_ {v \in C _ {j}} \mathrm {d} ^ {-} (v) &gt; \sum_ {v \in V ^ {\prime}} \mathrm {d} ^ {-} (v).
$$

De là, on en conclus que

$$
\sum_ {v \in C _ {j}} \mathrm {d} ^ {-} (v) &gt; \sum_ {v \in C _ {j}} \mathrm {d} ^ {+} (v)
$$

ce qui contredit l'hypothèse selon laquelle  $d^{+}(v) = d^{-}(v)$  pour tout  $v \in V$  et donc que  $\sum_{v \in C_j} \mathrm{d}^{-}(v) = \sum_{v \in C_j} \mathrm{d}^{+}(v)$ .

Une preuve alternative bien plus courte mais exploitant un résultat précédent. En fait, la preuve ci-dessus reprend les mêmes raisonnements que ceux développés dans la preuve du théorème I.4.16.

Démonstration. Supposons le graphe s. connexe. Puisqu'il est connexe et que pour tout  $v \in V$ ,  $d^{+}(v) = d^{-}(v)$ , on peut appliquer le théorème I.4.21. On conclut directement que  $G$  possède un circuit eulérien. Il est donc f. connexe.