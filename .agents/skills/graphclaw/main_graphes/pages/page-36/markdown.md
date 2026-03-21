Chapitre I. Premier contact avec les graphes

Le cas des multi-graphes orientés se traite de manière analogue. Les preuves sont laissées à titre d'exercices.

Théorème I.4.21. Un multi-graphe fini orienté simplement connexe  $G = (V, E)$  possède un circuit eulérien si et seulement si le demi-degré entrant de chaque sommet est égal à son demi-degré sortant.

Démonstration. Laissée à titre d'exercices. On adaptera facilement le raisonnement de la preuve du théorème I.4.16.

Corollaire I.4.22. Un multi-graphe fini orienté connexe  $G = (V, E)$  possède un chemin eulérien si et seulement si il existe deux sommets  $v_0$  et  $v_1$  tel que

$\triangleright$  pour tout  $v \in V \setminus \{v_0, v_1\}$ ,  $d^{-}(v) = d^{+}(v)$ ,
$d^{+}(v_{0}) = d^{-}(v_{0}) + 1,$
$d^{-}(v_{1}) = d^{+}(v_{1}) + 1.$

Démonstration. Laisser à titre d'exercices.

4.3. Connexité des graphes non orientés. Nous venons de voir qu'une condition nécessaire pour qu'un graphe soit eulérien est qu'il soit connexe. Nous allons donc fournir un algorithme naïf permettant de decide si un graphe non orienté est connexe. Il est clair que lorsqu'on s'intéresse à cette question, il suffit de considérer des graphes simples. (En effet, l'existence de boucles ou d'arêtes multiples n'a pas d'incidence sur la connexité du graphe.)

Algorithme I.4.23. La donnée fournie à cet algorithme est un graphe non orienté  $G = (V, E)$ , l'ensemble des sorties possibles est {oui, non}.

```txt
Choisir au hasard un sommet  $v_{0} \in V$
Composante:  $= \{v_{0}\}$ , New:  $= \{v_{0}\}$
Tant que New≠∅
Voisins:  $= \emptyset$
pour tout sommet  $v$  appartenant à New
Voisins:  $= \text{Voisins} \cup \nu(v)$
New:  $= \text{Voisins} \backslash \text{Composante}$
Composante:  $= \text{Composante} \cup \text{New}$
Si Composante  $= V$
alors sortie : "oui, G connexe"
sinon sortie : "non, G non connexe"
```

Il s'agit d'un algorithme "tache d'huile" ou "glouton" (on détermine de proche en proche, les sommets accessibles depuis le sommet  $v_{0}$ ). La variable Composante est destinée à contenir la composante connexe du sommet  $v_{0}$ . La variable New contient les sommets qui, à l'étape en cours d'exécution, sont