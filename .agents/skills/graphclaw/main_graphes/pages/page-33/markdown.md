I.4. Chemins et circuits

des données d’un point à un autre. Dans un internet, il est effectué en propageant de petites portions de données, les paquets, le long de points interconnectés, les passerelles. Lorsque chaque paquet passe par une passerelle, un routeur étudie où le paquet doit se rendre et décide alors de la passerelle suivante où l’envoyer. Le but de chaque routeur est de propager un paquet vers un point de plus en plus près de sa destination finale.

Afin de rapprocher un paquet de sa destination, chaque routeur gère des informations sur la structure, ou topologie, de l’internet. Ces informations sont stockées dans une table de routage, contenant une entrée pour chaque passerelle que le routeur sait atteindre. Chaque entrée indique la passerelle suivante à laquelle envoyer les paquets destinés à une autre passerelle que le routeur lui-même.

Pour que les paquets soient continuellement envoyés par la meilleure route possible, les routeurs mettent régulièrement à jour leurs tables de routage afin de tenir compte des changements dans l’internet. Dans l’un des types de routage, appelé routage SPF (Shortest Path First), chaque routeur gère sa propre carte de l’internet afin de mettre à jour sa table de routage en calculant les plus courts chemins entre lui et les autres destinations. Sa carte est un graphe orienté et pondéré, dont les sommets sont les passerelles et les arcs les connexions entre celles-ci. Chaque arc est pondéré par les dernières performances constatées sur la connexion.

##### 4.2. Graphes et chemins eulériens

La définition suivante est valable aussi bien dans le cas de graphes orientés que non orientés. Nous supposerons à chaque fois qu’il sera question de problèmes eulériens être en présence de graphes connexes.

###### Définition

###### 1.4.14.

Un chemin (resp. un circuit) d’un multi-graphe $G$ est eulérien s’il passe une et une seule fois par chaque arête/arc de $G$. (Un tel chemin (resp. circuit) peut bien évidemment passer plus d’une fois par un même sommet.) Autrement dit, un chemin (resp. un circuit) eulérien est une piste (resp. une piste fermée) passant par chaque arête/arc de $G$. Un multi-graphe eulérien est un graphe qui possède un circuit eulérien.

###### Remarque

###### 1.4.15.

Dans le problème consistant à déterminer si un graphe $G$ possède ou non un chemin eulérien, l’existence de boucles au sein de $G$ n’a aucune importance. En effet, soient $G$ un graphe et $G^{\prime}$ le graphe obtenu en supprimant les boucles de $G$. Il est évident que $G$ possède un chemin ou un circuit eulérien si et seulement si $G^{\prime}$ en possède un.

Considérons le cas des multi-graphes finis non orientés. Il est évident que pour que $G$ possède un circuit eulérien, i.e., pour que $G$ soit eulérien, il est nécessaire que $G$ soit connexe et que le degré de chaque sommet soit pair. Comme le montre le résultat suivant, ces conditions sont également suffisantes. On peut donc constater que le fait d’être eulérien est une propriété “locale” (elle ne fait intervenir que le degré de chaque sommet pris “isolément”).

18On évite comme cela les pathologies de graphes possédant des points isolés.