Chapitre II. Un peu de théorie algébrique des graphes

est tel que  $p(S) \leq p(T)$  (car  $p(e) \leq p(e')$ ). Or  $T$  est de poids minimal. Donc  $p(S) = p(T)$  et on en conclus que  $e$  appartient bien à un arbre couvrant  $G$  et de poids minimal.

Remarque II.6.3. L'algorithmme s'adapte aisément pour rechercher un sous-arbre couvrant de poids maximal. Il est facile de se convaincre que l'algorithmme de Prim a une complexité temporelle en  $\mathcal{O}((\# V)^2)$ .

L'algorithmme de Prim n'est pas le seul répondant à cette question. On rencontres souvent l'algorithmme de Kruskal brievement décrit ci-après.

Algorithm II.6.4 (Kruskal). La donnée de l'algorithmme est un graphe connexe non orienté  $G = (V, E)$ .

```txt
Enumérer les arêtes  $e_1, e_2, \ldots$  de  $G$  de telle sorte que si  $p(e_i) &gt; p(e_j)$ , alors  $i &gt; j$  A:=∅
Pour i allant de 1 à #E
Si  $\mathsf{A} \cup \{e_i\}$  est acyclique alors A:=A∪{ei}.