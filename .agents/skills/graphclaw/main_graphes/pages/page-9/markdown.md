CHAPITRE I

Premier contact avec les graphes

Un petit dessin vaut mieux
qu'un grand discours,
Napoléon.

Introduire une nouvelle matière n'est pas toujours chose plaisante car il s'agit souvent d'une accumulation de définitions ! Et c'est hélas la situation rencontrée ici. Nous allons donc agrémenter cette présentation, autant que faire se peut, d'exemples mettant en lumière l'intérêt pratique de la théorie des graphes. Comme l'indique le titre de ce chapitre, nous nous intéressons aux graphes et comme le lecteur va très vite s'en apercevoir, il en existe de plusieurs types: simple, multi-graphe, digraphe, hyper-graphe,...

1. Graphes orientés

**Définition I.1.1.** Soient $V$ un ensemble (fini ou infini) et $E$ une partie de $V \times V$ (i.e., une relation sur $V$). Le graphe $G = (V, E)$ est la donnée du couple $(V, E)$. Les éléments de $V$ sont appelés les sommets¹ ou noeuds de $G$. Les éléments de $E$ sont appelés les arcs² ou arêtes de $G$. Si $V$ est fini, on parlera de graphe fini (en particulier, $E$ est alors fini et contient au plus $(\#V)^2$ arcs).

**Remarque I.1.2.** Observons que l'ordre au sein des couples appartenant à $E$ est intrinsèquement présent³. On parlera donc parfois de graphe orienté ou de graphe dirigé. Soit $I$, un ensemble d'indices. Si $V = \{v_i \mid i \in I\}$ et si $a = (v_i, v_j)$, $i, j \in I$, on pourra alors parler de l'origine $v_i$ et de la destination $v_j$ de l'arc $a$. On dit que $v_i$ et $v_j$ sont les extrémités de l'arc $a$ et que $a$ relie $v_i$ à $v_j$. Si $b = (v_i, v_j)$, on parle généralement de la boucle $b$. Il est souvent commode de donner une représentation sagittale d'un graphe. Les sommets

Cette distinction va
devenir rapidement
indispensable, lorsqu'on
introduira les graphes non
orientés.

¹En anglais, cela se dit "vertex" (au pluriel, "vertices"). D'où l'initiale $V$ pour désigner l'ensemble des sommets. Dans ces notes, nous ne dérogerons pas à la coutume anglo-saxonne de noter un graphe $G = (V, E)$.

²En anglais, cela se dit "edge". D'où l'initiale usuelle $E$.

³On parle de couple et non de paire. Un couple est une paire ordonnée. On distingue d'ailleurs les notations $(x, y)$ et $\{x, y\}$.