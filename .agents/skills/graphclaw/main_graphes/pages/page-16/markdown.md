Chapitre I. Premier contact avec les graphes

$\varnothing \in \mathcal{I}$  (de manière équivalente,  $\mathcal{I}$  est non vide),
si  $X\in \mathcal{I}$  et  $Y\subseteq X$  , alors  $Y\in \mathcal{I}$
$\triangleright$  si  $X,Y\in \mathcal{I}$  et  $\# X &gt; \# Y$  , alors il existe  $x\in X\setminus Y$  tel que  $Y\cup \{x\} \in \mathcal{I}$

Les ensembles de  $\mathcal{I}$  sont dits indépendants. Enfin, un matroïde est qualifié de normal si tous les singletons sont indépendants.

Exemple I.2.11. Soient  $E$  un espace vectoriel,  $M$  une partie finie de  $E$  et  $\mathcal{I}$  la collection des parties libres de  $M$ . Le couple  $(M, \mathcal{I})$  est un matroïde.

Exemple I.2.12. Soit  $G = (V, E)$  un graphe non orienté simple. On considère le matroïde  $(E, \mathcal{I})$  où une partie  $X$  de  $E$  appartient à  $\mathcal{I}$  si et seulement si  $X$  ne contient aucun cycle (i.e.,  $X$  est une forêt).

Remarque I.2.13. Tout matroïde  $(M, \mathcal{I})$  est bien évidemment un hypergraphe. Les sommets de ce dernier sont les éléments de  $M$  et les hyper-arêtes sont les éléments de  $\mathcal{I}$ .

# 3. Quelques exemples

Nous allonsprésent dans cette courte section quelques exemples de graphes permettant la modélisation de divers problèmes. Puisqu'il ne s'agit que d'exemples, certaines définitions sont volontairement omises. Elles seront précisées en temps utiles. Nous espérons que la variété des exemplesprésentés servira de motivation profonde à l'étude théorique réalisée dans les sections et chapitres suivants. Pour des raisons évidentes de presentation, nous ne donnons que des exemples de "petite taille" qui peuvent souvent être résolus sans véritable méthode. Dans des problèmes réels, il faut imaginer des graphes pouvant avoir plus de  $10^{6}$  sommets. Dans ce cas, la solution parait nettement moins évidente!

Exemple I.3.1 (Circuit eulérien). Les ouvrages de théorie des graphes reprennent toujours le célibre exemple des ponts de Königsberg. Nous ne dérogerons pas à cette règle. Au XVII-ème siècle, les habitants de Königsberg (actuel Kaliningrad, ville de Russie proche de la Lituanie et de la Pologne où coule la rivière Pregel) désiraient se promener le dimanche en passant une et une seule fois par chacun des sept ponts de la ville. Les ponts étaient disposés comme à la figure I.12. Une modélisation du problème revient à considérer un graphe ayant comme sommets, les deux rives et les deux iles et comme arêtes, les sept ponts. Puisqu'il n'y a aucune contrainte sur le sens de parcours des ponts, nous avons choisi un multi-graphe non orienté.

La question générale sous-jacente est donc de déterminer pour un multi-graphe donné (éventuellement orienté) s'il existe un circuit, i.e., un chemin fermé, passant une et une seule fois par chaque arête.

Exemple I.3.2 (Circuit hamiltonien — TSP). Le problème du voyageur de commerce (Travel Salesman Problem) est en quelque sorte un “problème