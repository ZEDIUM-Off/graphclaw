Chapitre I. Premier contact avec les graphes

$\triangleright$  Le graphe  $G$  est hamiltonien si et seulement si sa fermeture  $\mathcal{F}(G)$  l'est.
$\triangleright$  Si la fermetre  $\mathcal{F}(G)$  de  $G$  est un graphe complet, alors  $G$  est hamiltonien.

Démonstration. Pour la première partie,  $G$  est un sous-graphe de  $\mathcal{F}(G)$ . Ainsi, si  $G$  est hamiltonien, alors  $\mathcal{F}(G)$  l'est aussi. Pour la reciproque, considérons une suite de graphes  $G_0 = G, \ldots, G_{k-1}, G_k = \mathcal{F}(G)$  donnant la fermetre de  $G$ . Si  $G_k$  est hamiltonien, alors  $G_{k-1}$  l'est aussi. C'est une conséquence immédiate du théorème d'Ore I.11.6. De proche en proche, on en conclus que  $G_0 = G$  est hamiltonien.

La deuxième partie est immédiate. Tout graphe complet étant hamiltonien, il suffit d'appliquer la première partie de ce théorème.

Remarque I.11.13. La condition suffisante donnée dans le théorème précédent n'est pas nécessaire. En effet, considérons un graphe constitué de  $n &gt; 4$  sommets et de  $n$  arêtes et formant un unique circuit hamiltonien. Chaque sommet étant de degré 2, le graphe est égal à sa fermetre et bien que le graphe soit hamiltonien, la fermetre n'est pas le graphe complet  $K_{n}$ .

Remarque I.11.14 (Preuve du corollaire I.11.7). Nous pouvons à présent déduire du théorème précédent la preuve du corollaire I.11.7. (Notons que le théorème I.11.12 repose entièrement sur le premier théorème d'Ore I.11.6. La seule raison pour laquelle nous avons attendu pour donner la preuve du corollaire I.11.7 (deuxième théorème d'Ore) réside dans l'utilisation de la fermetre d'un graphe permettant d'alleger les développements).

En effet, si un graphe  $G$  satisfait les conditions données dans l'énoncé du corollaire I.11.7, on en déduit immédiatement que sa fermetre est un graphe complet. (On peut joindre chaque paire de sommets non voisins.)

Voici enfin l'énoncé et la preuve du théorème de Chvátal. Notons que ce théorème est parfois attribué à Bondy-Chvátal.

Théorème I.11.15 (Chvátal). Soit  $G$  un graphe (simple et non orienté) ayant  $n \geq 3$  sommets ordonnés par degré croissant, i.e.,

$\deg (v_{1})\leq \deg (v_{2})\leq \dots \leq \deg (v_{n}).$

Si, pour tout  $k &lt; n / 2$ , le graphe satisfait

(1)  $\deg (v_k)\leq k\Rightarrow \deg (v_{n - k})\geq n - k,$

alors  $G$  possède un circuit hamiltonien.

Remarque I.11.16. La condition du théorème de Chvátal peut facilement être testée pour un graphe donné. Il suffit d'ordonner la suite des degrés et de vérifier une condition combinatoire élémentaire. Rappelons que d'un point de vue strictement logique, si pour  $k &lt; n / 2$ , on a  $\deg(v_k) &gt; k$ , alors l'implication  $\deg(v_k) \leq k \Rightarrow \deg(v_{n-k}) \geq n - k$  est toujours vraie.