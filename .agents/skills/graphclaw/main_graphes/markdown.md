Université de Liège

Faculté des sciences
Département de mathématiques

# Théorie des graphes

![img-0.jpeg](img-0.jpeg)

Deuxièmes bacheliers en sciences mathématiques
Année académique 2009–2010
Michel Rigo



“哈，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，

# Table des matières

Introduction 1

## Chapitre I. Premier contact avec les graphes 5

1. Graphes orientés 5
2. Graphes non orientés 8
3. Quelques exemples 12
4. Chemins et circuits 22
4.1. Recherche du plus court chemin 25
4.2. Graphes et chemins eulériens 29
4.3. Connexité des graphes non orientés 32
4.4. Décomposition en composantes fortement connexes 33
5. Sous-graphes 37
6. Coupes, points d'articulation, $k$-connexité 38
7. Théorème(s) de Menger 44
8. Graphes orientés sans circuit et tri topologique 46
9. Arbres 49
9.1. Parcours d'arbres 51
10. Isomorphismes de graphes 53
11. Graphes hamiltoniens 56
11.1. Fermeture d'un graphe et théorème de Chvátal 62

## Chapitre II. Un peu de théorie algébrique des graphes 69

1. Matrice d'adjacence 69
2. Théorie de Perron-Frobenius 73
2.1. Période d'une matrice irréductible 79
2.2. Estimation du nombre de chemins de longueur $n$ 83
2.3. Cas d'un graphe ayant plusieurs composantes f. connexes 85
3. Une application : l'algorithme de PageRank 87
4. Algèbre d'adjacence 92
5. Arbres couvrants 95
5.1. Une formule de Cayley 96
5.2. Une preuve bijective 98
5.3. Nombre de sous-arbres couvrants 102
6. Arbres couvrants de poids minimal 108

## Chapitre III. Graphes planaires 111

1. Introduction 111

Chapitre. Table des matières

2. Formule d'Euler 113
3. Graphes homéomorphes 115
4. Théorème de Kuratowski 116

Chapitre IV. Coloriage 123
1. Nombre chromatique 123
2. Le théorème des cinq couleurs 124
3. Polynôme chromatique 132
4. Coloriage d'arêtes et théorème de Ramsey 135

Chapitre V. Annexe : implémentation en C 141
1. Pointeurs, allocation mémoire et listes chaînées 141
1.1. Adresses 141
1.2. Allocation dynamique 144
1.3. Structures 145
1.4. Listes chaînées 146
2. Liste d'adjacence 148
2.1. Une manipulation simplifiée 150
2.2. Utilisation des fonctions 152
2.3. Détail de l'implémentation 153

Bibliographie 161

Liste des figures 163

Index 169

# Introduction

La théorie des graphes est, avec la combinatoire, une des pierres angulaires de ce qu’il est commun de désigner par *mathématiques discrètes*. Cependant, elle n’a reçu qu’assez tardivement une attention soutenue de la part de la communauté mathématique. En effet, bien que les graphes eulériens soient l’émanation du célèbre problème des sept ponts de Königsberg étudié par Euler en 1736, on peut dire que les premiers développements majeurs de la théorie des graphes datent du milieu du vingtième siècle (N. Biggs, C. Berge, W.T. Tutte, …). Ainsi, un des premiers ouvrages, si pas le premier, traitant de théorie des graphes “*Theorie der endlichen und unendlichen Graphen*” écrit par König remonte à 1936. Depuis cette époque, la théorie des graphes s’est largement développée et fait à présent partie du cursus standard en mathématiques de bon nombre d’universités. Ces notes de cours constituent le support écrit du cours dispensé aux *deuxièmes bacheliers en sciences mathématiques* de l’Université de Liège.

Un graphe $G=(V,E)$ est essentiellement défini par une relation binaire $E\subseteq V\times V$ sur un ensemble $V$ le plus souvent fini (nous ne ferons que de brèves incursions dans le monde des graphes infinis, ce qui sera d’ailleurs l’occasion de donner un avant-goût de la théorie des langages formels). Les graphes sont utilisés pour modéliser de nombreuses situations et leurs applications sont par conséquent aussi nombreuses que variées : dans d’autres branches des mathématiques (algèbre, combinatoire, …), en informatique, en recherche opérationnelle (tournées de distribution, ordonnancement de tâches, construction de circuits imprimés, …), en cartographie (coloriage de cartes), en chimie, etc …

Selon nous, il est impensable de vouloir traiter *efficacement* un problème réel (i.e., de taille non négligeable) sans posséder de solides bases. Ainsi, nos développements seront principalement théoriques. Néanmoins, le caractère appliqué de la théorie sera mis en exergue par de nombreux exemples (en particulier, au premier chapitre, nous détaillons une série de problèmes issus du “monde réel”.)

Par essence, en mathématiques discrètes et *a fortiori* en théorie des graphes, de nombreux raisonnements ont une composante combinatoire importante. Ainsi, des preuves pouvant être jugées “difficiles” consistent souvent en une succession, parfois longue, de raisonnements simples (mais pas toujours aisés à saisir). Dès lors, certaines parties paraîtront peut-être complexes sans pour autant être compliquées ! Nous espérons que des exemples

Chapitre . Introduction

nchoisis pourront alors aider le lecteur dans sa compréhension. Bien évidemment, les méthodes mises ici en évidence sont souvent tout autre que celles des “mathématiques du continu” comme l’analyse. Nous espérons que cette présentation permettra ainsi à l’étudiant d’enrichir sa palette de techniques et de raisonnements mathématiques. (Les mathématiques ne sont bien sûr pas cloisonnées et les interactions entre les diverses branches sont nombreuses et souvent fort riches: “It is unquestionable that interplay between ideas from different sources, and elaborate techniques successfully applied, are among the features that make much of mathematics fascinating. Moreover, mathematics does often display a tendency to unify itself and to build up a body of technique. Therefore one may well guess that graph theory, as it matures, will continue to develop its own characteristic techniques and that many of its results will become increasingly unified, both among themselves and with the rest of mathematics.”.)

Dans le premier chapitre, nous présentons la notion de graphe et ses variantes (graphe non orienté, arbre, multi-graphe, hypergraphe). Les principaux résultats du chapitre concernent les graphes eulériens et les graphes hamiltoniens (théorème de Dirac, d’Ore et de Chvátal).

Le deuxième chapitre donne quelques résultats de la théorie algébrique des graphes. En effet, à un graphe $G$, on associe de manière classique une matrice, appelée matrice d’adjacence. Dès lors, l’application de résultats standards d’algèbre linéaire permet de déduire des renseignements non triviaux sur $G$. Par exemple, on dénombre le nombre d’arbres couvrants d’un graphe donné par un simple calcul de mineur. Nous présentons également quelques éléments concernant la théorie de Perron-Frobenius qui permet une estimation assez fine du nombre de chemins de longueur $n$ dans un graphe.

Les trois chapitres suivants sont assez classiques. Les thèmes évoqués sont présentés dans bon nombre d’ouvrages d’introduction à la théorie des graphes. On y étudie les graphes planaires, i.e., les graphes pouvant être représentés dans le plan euclidien par une figure dont les arcs ne se coupent pas. Ensuite, on s’intéresse à quelques problèmes de coloriage (théorème des quatre/cinq couleurs) et en particulier, à l’important théorème de Ramsey qui permet d’introduire sommairement la théorie extrémale des graphes. Enfin, suivant le temps disponible, quelques heures du cours seront consacrées à présenter l’un ou l’autre problèmes “d’actualité” de recherche en mathématiques discrètes.

Vu le caractère appliqué indubitable du sujet, le dernier chapitre est une annexe présentant une implémentation en langage C des graphes finis (orientés) par liste d’adjacence. Cette structure de données est particulièrement bien adaptée aux graphes peu denses. En effet, tout au long du présent texte, divers algorithmes sont présentés sous forme de pseudo-code. Il est donc indispensable de disposer d’une interface raisonnable pour implémenter

^{1}
C. ST. J. A. Nash-Williams, dans la préface de [3].

et tester ces algorithmes sur des jeux de données conséquents. Signalons que de nombreux problèmes rencontrés en théorie des graphes sont, du point de vue de la complexité, difficiles. Cependant, nous avons volontairement omis les notions de complexité algorithmique et de calculabilité : problèmes NP, NP-difficiles ou NP-complets. (Cette problématique est présentée en troisième année de bacheliers en sciences mathématiques.)

Une fois n'est pas coutume, je voudrais aussi remercier (et les futurs étudiants ayant ces notes de cours entre les mains s'associeront certainement à moi) l'ensemble des étudiants de deuxième bachelier de l'année académique 2005-2006 et en particulier : Cédric H., Damien G., Damien K., Damien L., Kastriot, Marie, Mehdi, Mélissa, Naïm, Nicolas, Rukiye, ... Leur intérêt marqué, leurs questions pertinentes, leurs interrogations m'ont permis d'améliorer (et parfois de corriger) la présentation de ce cours. Je n'oublierai pas non plus les retours et les suggestions de Laurent Waxweiler.

.

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

Chapitre I. Premier contact avec les graphes

arcs adjacents

sont représentés par des points et si  $(v_{i}, v_{j})$  est un arc, alors on trace une flèche de  $v_{i}$  vers  $v_{j}$  (cf. figure I.1). Deux arcs sont adjacents s'ils ont au moins une extrémité en commun.

![img-1.jpeg](img-1.jpeg)
FIGURE I.1. Un arc reliant deux sommets, une boucle.

arc incident à un sommet

DDefinition I.1.3. Soit  $a = (v_i, v_j) \in E$ . On dit que  $a$  est un arc sortant de  $v_i$  ou encore que  $a$  est un arc incident à  $v_i$  vers l'extérieur (resp. un arc entrant dans  $v_j$  ou encore que  $a$  est un arc incident à  $v_j$  vers l'intérieur). L'ensemble des arcs sortant de  $v_i$  est noté  $\omega^{+}(v_i)$  et l'ensemble des arcs entrant dans  $v_j$  est noté  $\omega^{-}(v_j)$ . L'ensemble des arcs incidents à un sommet  $v$  est  $\omega(v) := \omega^{+}(v) \cup \omega^{-}(v)$ . On définit le demi-degré sortant (resp. demi-degré entrant) d'un sommet  $v$  par

$$
d ^ {+} (v) = \# (\omega^ {+} (v)) \quad (\text {r e s p .} d ^ {-} (v) = \# (\omega^ {-} (v))).
$$

Handshaking formula.

Si  $G = (V,E)$  est un graphe fini, il est clair que

$$
\sum_ {v \in V} d ^ {+} (v) = \sum_ {v \in V} d ^ {-} (v).
$$

Enfin, le degré de  $v$  est  $\deg(v) = d^{+}(v) + d^{-}(v)$ . L'ensemble des successeurs d'un sommet  $v$  est l'ensemble  $\operatorname{succ}(v) = \{s_1, \ldots, s_k\}$  des sommets  $s_i$  tels que  $(v, s_i) \in \omega^{+}(v)$ , i.e.,  $(v, s_i) \in E$ . De manière analogue, l'ensemble des prédécesseurs d'un sommet  $v$  est l'ensemble  $\operatorname{pred}(v) = \{s_1, \ldots, s_k\}$  des sommets  $s_i$  tels que  $(s_i, v) \in \omega^{-}(v)$ , i.e.,  $(s_i, v) \in E$ . Enfin, l'ensemble des voisins de  $v$  est simplement

$$
\nu (v) = \operatorname {p r e d} (v) \cup \operatorname {s u c c} (v).
$$

sommets adjacents

Si  $u$  appartiennent à  $\nu (v)$  , on dit que  $u$  et  $v$  sont des sommets voisins ou adjacents.

Example I.1.4. Soit le graphe  $G = (V, E)$  où  $V = \{a, b, c, d, e\}$  et

$$
E = \{(a, b), (a, e), (b, b), (b, c), (c, c), (c, d), (c, e), (d, a), (e, a), (e, d) \}.
$$

Celui-ci est représenté à la figure I.2. Par exemple,  $\omega^{+}(a) = \{(a,b),(a,e)\}$

![img-2.jpeg](img-2.jpeg)
FIGURE I.2. Un exemple de graphe.

I.1. Graphes orientés

et  $\omega^{-}(d) = \{(c,d),(e,d)\}$ . On a aussi  $\operatorname{succ}(a) = \{b,e\}$ ,  $\operatorname{succ}(b) = \{b,c\}$ ,  $\operatorname{pred}(d) = \{c,e\}$  et  $\nu(a) = \{b,d,e\}$ . On voit aussi que les arcs  $(e,a)$  et  $(d,a)$  sont adjacents. Enfin, le demi-degré sortant de  $c$  est  $d^{+}(c) = 3$ .

Définition I.1.5. Un multi-ensemble $^4$  est un ensemble au sein duquel un même élément peut être répété plus d'une fois. Ainsi, on s'intéresse non seulement à savoir si un élément appartient ou non à un multi-ensemble donné, mais également à sa multiplicité. Par exemple,  $\{1,1,2,3\}$ ,  $\{1,2,3\}$  et  $\{1,2,2,3\}$  sont des multi-ensembles distincts. Pour désigner les copies d'un même élément  $x$ , il est commode de les indicier. Par exemple, on considère le multi-ensemble  $\{1_1,1_2,1_3,2_1,2_2,3\}$ . Cette manière de procéder nous permettra de définir facilement des fonctions définies sur un multi-ensemble.

Un multi-graphe  $G = (V, E)$  est un graphe pour lequel l'ensemble  $E$  des arcs est un multi-ensemble. Autrement dit, il peut exister plus d'un arc reliant deux sommets donnés. Un exemple de représentation d'un multi-graphe est donné à la figure I.3. Un multi-graphe  $G = (V, E)$  est fini si  $V$

![img-3.jpeg](img-3.jpeg)
FIGURE I.3. Un exemple de multi-graphe.

et  $E$  sont finis. (En effet, dans le cas des multi-graphes, supposer  $V$  fini n'implique pas que  $E$  soit fini.)

Soit  $p \geq 1$ . Un  $p$ -graphe est un multi-graphe  $G = (V, E)$  pour lequel tout arc de  $E$  est repété au plus  $p$  fois. En particulier, un 1-graphe est un graphe.

Remarque I.1.6. On peut observer que la remarque I.1.2, la définition I.1.3 et la "handshaking formula" s'appliquent également au cas des multi-graphes. Il est laissé au lecteur le soin d'adapter les définitions de  $\omega^{+}(v)$ ,  $d^{+}(v)$ ,  $\operatorname{succ}(v)$  et  $\omega^{-}(v)$ ,  $d^{-}(v)$ ,  $\operatorname{pred}(v)$ . En particulier,  $\omega^{+}(v)$  et  $\omega^{-}(v)$  sont en général des multi-ensembles.

Définition I.1.7. Un graphe  $G = (V, E)$  est dit simple (ou strict) s'il ne s'agit pas d'un multi-graphe et si  $E$  est irréflexif, c'est-à-dire que quel que soit  $v \in V$ ,  $(v, v)$  n'appartient pas à  $E$  (i.e.,  $G$  ne contient pas de boucle). Un exemple de graphe simple est donné à la figure I.4.

Chapitre I. Premier contact avec les graphes

![img-4.jpeg](img-4.jpeg)
FIGURE I.4. Un exemple de graphe simple.

# 2. Graphes non orientés

Les graphes non orientés sont en fait un cas particulier de graphes (orientés).

Definition I.2.1. Soit  $G = (V, E)$  un graphe (resp. un multi-graphe). Si  $E$  est une relation symétrique sur  $V$ , on dira que  $G$  est un graphe (resp. un multi-graphe) non dirigé ou non orienté. Autrement dit,  $G$  est non dirigé si

$$
\forall v _ {1}, v _ {2} \in V: (v _ {1}, v _ {2}) \in E \Rightarrow (v _ {2}, v _ {1}) \in E.
$$

Dans ce cas, on simplifie la représentation sagittale de  $G$  en traçant simplement un segment entre  $v_{1}$  et  $v_{2}$ . Pour alléger l'écriture, on identifiera les arcs  $(v_{i}, v_{j})$  et  $(v_{i}, v_{j})$  avec une unique "arête non orientée" donnée par la paire  $\{v_{i}, v_{j}\}$ . Dans le cas dirigé (resp. non dirigé), nous nous efforcerons de parler d'arcs (resp. d'arêtes).

Si par contre, on désire insister sur le caractère non symétrique de  $E$ , on parlera de graphe dirigé ou, par abus de langage, digraphé.

Les définitions rencontres précédemment s'adaptent aisément au cas non orienté.

Definition I.2.2. Soient  $G = (V, E)$ , un multi-graphe non orienté et  $a = \{v_i, v_j\}$  une de ses arêtes. On dit que  $a$  est incident aux sommets  $v_i$  et  $v_j$ . Le nombre d'arêtes incidentes à  $v_i$  est le degré de  $v_i$ , noté  $\deg(v_i)$ . On suppose en outre que les boucles apportent une double contribution au degré d'un sommet. L'ensemble des arêtes incidentes à  $v_i$  se note  $\omega(v_i)$ . Il est clair que, dans un graphe simple,  $\deg(v_i) = \#(\omega(v_i))$ . Ces notations sont bien évidemment compatibles avec celles données dans le cas orienté. Deux arêtes sont adjacentes si elles ont au moins une extrémité en commun.

Deux sommets  $v_{i}, v_{j} \in V$  sont adjacents si l'arête  $\{v_{i}, v_{j}\}$  appartient à  $E$ . On dit aussi qu'ils sont voisins. L'ensemble des voisins de  $v$  se note  $\nu(v)$ . Enfin, la définition d'un  $p$ -graphe est analogue à cette donnée dans le cas orienté.

Remarque I.2.3 (Handshaking lemma). Si  $G = (V, E)$  est un multi-graphe non orienté, alors

$$
\sum_ {v \in V} \deg (v) = 2 \# E.
$$

I.2. Graphes non orientés

C'est immédiat. (Et on comprend mistrés la double contribution des boucles pour le degré d'un sommet...)

L'exemple suivant illustré les différentes classes de graphes rencontres jusqu'à présent. Bien sur, tout graphe simple est un graphe et tout graphe est un multi-graphe.

Exemple I.2.4. A la figure I.5, on a représenté, dans le cas dirigé, un graphe simple, un graphe et enfin, un multi-graphe. La figure I.6 reprend

![img-5.jpeg](img-5.jpeg)
FIGURE I.5. Un graphe (dirigé) simple, un graphe et un multi-graphe.

les mêmes éléments dans le cas non orienté.

![img-6.jpeg](img-6.jpeg)
FIGURE I.6. Un graphe (non dirigé) simple, un graphe et un multi-graphe.

Definition I.2.5. Soit  $k \geq 1$ . Un multi-graphe orienté (resp. non orienté)  $G = (V, E)$  est  $k$ -régulier si pour tout  $v \in V$ ,  $d^{+}(v) = k$  (resp.  $\deg(v) = k$ ). Le graphe de gauche (resp. de droite) de la figure I.7 est 3-régulier (resp. 4-régulier). Le graphe de droite de la figure I.7 est en particulier simple et

![img-7.jpeg](img-7.jpeg)
FIGURE I.7. Des graphes non orientés 3 et 4-réguliers.

complet. Un graphe  $G = (V, E)$  est complet si  $E = V \times V$ , plus exactement, on suppose souvent que

$$
E = V \times V \setminus \{(v, v) \mid v \in V \}
$$

(autrement dit, on ne tient pas compte des boucles). En particulier, un graphe complet est symétrique. On note  $K_{n}$  le graphe simple non orienté

Chapitre I. Premier contact avec les graphes

![img-8.jpeg](img-8.jpeg)
FIGURE I.8. Un multi-graphe orienté 2-régulier.

complet à  $n$  sommets. Ainsi, la figure I.7 représenté le graphe  $K_{5}$ . Dans ce cours, lorsqu'on parlera de graphes complets, il sera sous-entendu qu'il s'agit de graphes simples et non orientés.

Definition I.2.6. Un graphe  $G = (V, E)$  est dit biparti si  $V$  peut être partitionné en deux ensembles  $V_{1}$  et  $V_{2}$  de manière telle que  $E \subseteq V_{1} \times V_{2}$ . Si  $\# V_{1} = m$ ,  $\# V_{2} = n$  et  $E = V_{1} \times V_{2}$ , alors on parle du graphe biparti

![img-9.jpeg](img-9.jpeg)
FIGURE I.9. Un graphe biparti (non complet).

complet et il est noté  $K_{m,n}$ . On peut généraliser cette notion et définir des graphes  $n$ -partis, pour  $n \geq 2$ . Pour ce faire,  $V$  doit être partitionné en  $n$  sous-ensembles  $V_{1}, \ldots, V_{n}$  de manière telle que

$$
E \subseteq \bigcup_ {i \neq j} V _ {i} \times V _ {j}.
$$

Definition I.2.7. Un multi-graphe  $G = (V, E)$  (orienté ou non) est étiqueté (par  $f$ ) s'il existe une fonction

$$
f: E \to \Sigma
$$

où  $\Sigma$  est un ensemble quelconque. Si  $\Sigma \subseteq \mathbb{R}^{+} = [0, +\infty[$ , on parle souvent de multi-graphe pondéré et on dit que  $f$  est une fonction de poids. Un étiquetage peut par exemple servir à préciser des coûts (coût de transport, des distances, des couleurs, etc...). Si  $a$  est un arc,  $f(a)$  est l'étiquette, le label ou encore le poids de  $a$ . On peut de la même manière définir un étiquetage des sommets au moyen d'une fonction  $g: V \to \Sigma$ .

Example I.2.8. Le graphe de la figure I.10 représenté quelques villes belges connectées par un réseau autoroutier. L'étiquette de chaque arête représenté la distance, par autoroute, entre les deux extrémités de celle-ci. Nous avons choisi un graphe non orienté car les autoroutes belges sont toujours dans les deux sens.

I.2. Graphes non orientés

![img-10.jpeg](img-10.jpeg)
FIGURE I.10. Graphé étiqueté par les distances entre villes (itinétaire "express" calculé par www.mappy.fr).

Pour terminer cette section, nous généralisons le concept de graphe en autorisant non plus des relations binaires entre sommets (autrement dit, des arcs) mais des relations d'arité quelconque (des hyper-arêtes). Ensuite, nous donnons une brève introduction aux matroides.

Définition I.2.9. Un hyper-graphe  $H = (V, E)$  est la donnée de deux ensembles  $V$  et  $E$ . L'ensemble  $V$ , comme dans le cas d'un graphe, est l'ensemble des sommets de  $H$ . Par contre,  $E$  est une partie de  $\mathcal{P}(V)$  (l'ensemble des parties de  $V$ ). Un élément de  $E$  est appelé hyper-arête. Soient  $V = \{a, b, c, d, e\}$  et

$$
E = \{\{a, b \}, \{a, c, d \}, \{b, d, e \} \}.
$$

On dispose de l'hyper-graphe  $H = (V, E)$  représenté à la figure I.11. Un

![img-11.jpeg](img-11.jpeg)
FIGURE I.11. Un exemple d'hyper-graphe.

hyper-graphe  $H = (V,E)$  est fini si  $V$  est fini.

La notion de matroïde est due à H. Whitney (1935) et a été développée par W. Tutte. Elle permet notamment l'étude axiomatique des propriétés de l'indépendance linéaire ou aussi l'étude des cycles et des arbres.

Définition I.2.10. Un matroïde  $(M, \mathcal{I})$  est la donnée d'un ensemble fini  $M$  et d'une partie  $\mathcal{I}$  de  $\mathcal{P}(M)$ , i.e., d'une collection de sous-ensembles de  $M$ , vérifiant les trois propriétés suivantes

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

I.3. Quelques exemples

![img-12.jpeg](img-12.jpeg)
FIGURE I.12. Les sept points de Königsberg.

![img-13.jpeg](img-13.jpeg)

dual" de l'exemple précédent (on s'intéresse ici à trouver un circuit passant une et une seule fois par chaque sommet et non par chaque arête). Oncherche un circuit permettant non seulement de relier  $n$  villes en passant une et une seule fois par chacune d'entre elles, mais de plus, ce circuit doit minimiser la distance totale parcourue. On pourrait par exemple considérer les villes belges et les données de la figure I.10 et en déterminer un circuit optimal. Dans certains cas, on a recours à un graphe orienté plutôt qu'à un graphe non orienté comme à la figure I.10. Cela a pour avantage de permettre la modélisation de coûts différents pour aller d'un sommet  $A$  à un sommet  $B$  plutôt que de  $B$  à  $A$  (ceci permet, par exemple, de prendre en compte des sens uniques, des payages, des temps de transports différents suivant la direction choisisie, etc...).

Il s'agit donc de problèmes typiques rencontres dans la détermination de tournées, de circuits de distribution, etc...

Exemple I.3.3 (Coloriage). Considerons un cube. Si chaque sommet (resp. chaque arête) d'un graphe représentée un sommet (resp. une arête) du cube, on obtient le graphe de gauche de la figure I.13, on parle du squelette du cube. Par contre, si on représentée les faces du cube par les sommets d'un graphe et si les sommets du graphe correspondant à des faces du cube ayant une arête commune sont adjacents, on obtient celui de droite. On peut alors

![img-14.jpeg](img-14.jpeg)
FIGURE I.13. Deux représentations d'un cube.

![img-15.jpeg](img-15.jpeg)

poser la question générale de déterminer le nombre de couleurs nécessaire et suffisant pour colorier les sommets d'un multi-graphe donné, de manière telle que deux sommets adjacents ne recoivent pas la même couleur. Si on

Chapitre I. Premier contact avec les graphes

répond à cette question dans le cas de notre exemple initial, on s'aperçoit que pour colorier les sommets (resp. les faces d'un cube), deux (resp. trois) couleurs sont suffisantes.

Example I.3.4 (Cartographie). Quel est le nombre minimum de couleurs nécessaires pour colorier les pays d'une carte $^6$  géographique de manière telle que des couleurs distinctes soient attribuées à des pays voisins? Ce problème

![img-16.jpeg](img-16.jpeg)
FIGURE I.14. La carte des USA.

se ramène au précédent. On considère un graphe ayant pour sommet les différents pays de la carte. Deux sommets sont adjacents si et seulement si ils partagent une frontière.

Example I.3.5 (Graphe d'incompatibilité). Pour le transport de produits chimiques par rail, certains de ces produits ne peuvent être transportés dans un même wagon (des produits co-combustibles sont placés dans des wagons distincts). La figure I.15 représenté le graphe d'incompatibilité de transport (deux sommets adjacents ne peuvent être placés dans le même wagon). On

![img-17.jpeg](img-17.jpeg)
FIGURE I.15. Graphe d'incompatibilité.

Par rapport à l'exemple précédent, le graphe n'est pas nécessairement planaire.

demande de minimiser le nombre de wagons nécessaires au transport. Ce problème se ramène donc à un problème de coloriage. Deux sommets adjacents doivent avoir des couleurs distinctes, une couleur correspondant à un wagon.

I.3. Quelques exemples

Exemple I.3.6 (Coloriage d'arêtes). Au lieu de pouvoir colorier les sommets d'un multi-graphe, on peut aussi s'intéresser au problème suivant. Déterminer le nombre de couleurs nécessaires et suffisantes pour colorier les arêtes d'un multi-graphe donné, de manière telle que deux arêtes adjacentes ne recoivent pas la même couleur.

Exemple I.3.7 (Graphe de Cayley). Soit  $G$  un groupe et  $S$  un ensemble de générateurs de  $G$  (tout élément de  $G$  s'obtient comme produit d'un nombre fini d'éléments de  $S$  ou d'inverses d'éléments de  $S$ ). Le graphe de Cayley du groupe  $G$  (par rapport à  $S$ ) est un graphe orienté  $\mathcal{C}_S(G)$  ayant pour sommets les éléments de  $G$ . Ses arcs sont définis comme suit. Pour tous  $g \in G$ ,  $s \in S$ , l'arc  $(g, gs)$  est un arc du graphe ayant  $s$  pour label. Soit le groupe symétrique  $S_3$  des permutations à 3 éléments ayant pour ensemble générateur

$$
S = \{x = (1 2), y = (1 3), z = (2 3) \}.
$$

Il est clair que

$$
x x = y y = z z = i d, \quad x y = (1 2) (1 3) = (1 3 2),
$$

$$
x z = (1 \quad 2) (2 \quad 3) = (1 \quad 2 \quad 3), \quad y z = (1 \quad 3) (2 \quad 3) = (1 \quad 3 \quad 2)
$$

et

$$
y x = (1 \quad 2 \quad 3), z x = (1 \quad 3 \quad 2), z y = (1 \quad 2 \quad 3).
$$

Le graphe de Cayley correspondant est représenté à la figure I.16.

![img-18.jpeg](img-18.jpeg)
FIGURE I.16. Graphe de Cayley de  $S_{3}$

On peut obtenir un autre graphe de Cayley de  $S_{3}$  en considérant comme ensemble de générateurs  $\{a = (1\quad 2\quad 3), b = (1\quad 2)\}$ . On obtient alors le graphe de la figure I.17 (les détails de calcul sont laissés au lecteur). Bien évidemment, le graphe de Cayley d'un groupe ne donne aucun renseignement qui ne saurait être obtenu sous une forme "algébrique" classique. Néanmoins, un tel diagramme a l'avantage de fournir presque immédiatement certaines informations qui seraient plus pénibles à obtenir par d'autres moyens (imaginez un groupe défini par certaines relations, par exemple,  $cd = dc$ ,  $c^2 dc = d$ , etc...). Ainsi, il est ici immédiat de vérifier sur la figure I.17 que les éléments

Chapitre I. Premier contact avec les graphes

![img-19.jpeg](img-19.jpeg)
FIGURE I.17. Un autre graphe de Cayley de  $S_3$ .

baaba et  $a^2$  sont identiques (il suffit de suivre les labels de chemins depuis id). L'utilisation des graphes de Cayley se révèle être un outil souvent bien utile en théorie des représentations ou dans la résolution de problèmes précis à l'aide de l'ordinateur.

Example I.3.8 (Arbre couvrant). Une société de téléphonie souhaite cabler entièrement, au moyen de nouvelles fibres optiques, une ville en minimisant le nombre de connexions à réaliser. Le nouveau cablage s'appuie sur le réseau électrique déjà existant et bien évidemment, tous les points de la ville doivent être déservis. La figure I.20 représenté le réseau actuel de la ville et ses connexions. A droite, se trouve les sélections envisagées. La question générale

![img-20.jpeg](img-20.jpeg)
FIGURE I.18. Sous-graphe couvrant.

![img-21.jpeg](img-21.jpeg)

qui est posée est de rechercher un sous-graphe (ou un sous-arbre) couvrant dans un graphe donné. On peut aussi envisager une version pondérée dans laquelle chaque arc aurait un coût et on rechercherait un sous-graphe (ou un sous-arbre) couvrant de poids minimal.

Example I.3.9 (Forte connexité). Suite à divers problèmes de circulation, des responsables communaux désirant placer les rues d'un quartier à sens unique. Si un graphe modélise les rues et leurs croisements, la question qui se pose est donc d'orienter les arcs d'un graphe non orienté de manière telle

I.3. Quelques exemples

qu'il existe un chemin orienté entre toute paire de sommets. Un tel exemple est donné à la figure I.19.

![img-22.jpeg](img-22.jpeg)
FIGURE I.19. Orientation d'un graphe.

![img-23.jpeg](img-23.jpeg)

Exemple I.3.10 (Distance). Considérons le problème de routage de paquets de données devant transiter sur un réseau (du type internet). Si un utilisateur désire acceder au contenu d'une page web présente sur un serveur, une connexion entre ce serveur et la machine doit être établie. Cette connexion n'est en général pas directe mais doit passer par une série de machines relais. Pour préserver la bande passante, pour accélérer le transfert ou encore pour minimiser les coûts, on essaie de minimiser le nombre de "hops" (i.e., le nombre de machines relais utilisées). Si chaque ordinateur, routeur, passerelle ou serveurprésents sur l'internet représentée un sommet d'un graphe dont les arcs représentent les connexions entre ceux-ci, il s'agit d'un problème délicat! Voici un exemple de "route" suivie par un paquet de données :

```txt
&gt; traceroute www.google.be
traceroute to www.google.be (66.249.85.99), 30 hops max, 40 byte packets
1 mont3-0014.gw.ulg.ac.be (139.165.159.1) 0.177 ms 0.177 ms 0.163 ms
2 segi3-0813-mont3.gw.ulg.ac.be (193.190.228.125) 0.221 ms 0.179 ms 0.184 ms
3 inet3-3031.gw.ulg.ac.be (139.165.192.49) 0.734 ms 0.612 ms 0.597 ms
4 fe.m20.access.liege.belnet.net (193.191.10.17) 0.791 ms 0.707 ms 0.713 ms
5 oc48.m160.core.science.belnet.net (193.191.1.185) 2.266 ms 2.239 ms 16.679 ms
6 oc192.m160.ext.science.belnet.net (193.191.1.2) 2.252 ms 2.260 ms 2.235 ms
7 216.239.43.88 7.954 ms 7.922 ms 7.731 ms
8 64.233.175.249 14.932 ms 14.906 ms 14.911 ms
9 216.239.46.49 14.628 ms 14.431 ms 14.407 ms
10 66.249.85.99 14.409 ms 14.722 ms 14.624 ms
```

Le problème général sous-jacent est donc de déterminer, dans un graphe donné, le chemin le plus court permettant de relier deux sommets quelconques d'un graphe. On pourra envisager des variantes pondérées ou orientées. Dans la version pondérée, on recherchera le chemin de poids minimal.

Example I.3.11 (Planarité). Lors de l'élaboration de circuits électriques, les connexions reliant les différents composants ne peuvent se toucher sous peine de court-circuit. La question de théorie des graphes qui est posée est donc de déterminer si, dans le plan, il existe une configuration géométrique des sommets et des arcs du graphe assurant qu'aucune paire d'arcs ne se

Chapitre I. Premier contact avec les graphes

![img-24.jpeg](img-24.jpeg)
FIGURE I.20. Un graphe planaire.

![img-25.jpeg](img-25.jpeg)

croise. Si tel est le cas, on parle de graphe planaire. Plutôt que de représentier un graphe dans un plan, on pourrait aussi imaginer des représentations sur une sphère ou un tore et se poser la même question. Ainsi, le graphe  $K_{3,3}$  de la figure I.21 n'est pas planaire (cf. Lemme III.4.2), mais si on le représentée sur un tore, on peut obtenir une représentation dont les arcs ne se coupent pas (cf. figure I.22). Notons enfin que, dans l'élaboration des circuits

![img-26.jpeg](img-26.jpeg)
FIGURE I.21. Un graphe non planaire.

![img-27.jpeg](img-27.jpeg)
FIGURE I.22.  $K_{3,3}$  sur un tore.

imprimés (VLSI $^8$  par exemple), il est possible de réaliser de tels circuits sur plusieurs couches. Ainsi, un autre problème serait de déterminer le nombre minimum de couches à envisager pour la conception du circuit imprimé.

Remarque I.3.12. Le graphe de la figure I.21 est classiquement illustré par le problème des trois villas et des trois usines. Imaginez que trois villas doivent être reliées au gaz, à l'eau et à l'électricité. Est-il possible de construire des canalisations alimentant chaque villa depuis chaque usine de manière telle que ces canalisations ne se chevauchent pas?

Example I.3.13 (Graphe d'intervalles). Considérons les intervalles ouverts  $]0,2[, ]1,4[, ]2,5[, ]3,4[, ]3,8[$  et  $]6,8[$ . A chaque intervalle correspond un sommet d'un graphe non orienté. Si  $]a,b[\cap ]c,d[\neq \emptyset$ , alors une arête relié dans le graphe les sommets correspondant à ces deux intervalles. Le graphe

I.3. Quelques exemples

d'intervalles correspondant aux intervalles donnés ci-dessus est représenté à la figure I.23. Ces graphes sont parfois utilisés en archéologie ou encore en

![img-28.jpeg](img-28.jpeg)
FIGURE I.23. Graphe d'intervalles.

génétique pour exprimer ou mettre en évidence des éléments communs se produit à travers le temps (par exemple, des caractéristiques communes d'une période de l'histoire ou des mutations génétiques au sein du génome).

Exemple I.3.14 (Problème d'affection). Soient  $O_1, \ldots, O_k$  des ouvriers et  $T_1, \ldots, T_t$  des postes de travail. Chaque ouvrier  $O_i$  possède certaines qualifications lui permettant de travailler sur certains postes  $T_{i,1}, \ldots, T_{i,d_i}$ . Comment répartir les ouvriers pour que chaque poste de travail soit occupé par au moins un ouvrier? Pour modéliser ce problème, on utilise un graphe biparti dont les sommets représentent les ouvriers et les postes. On trace un arc entre  $O_i$  et  $T_j$  si  $O_i$  possède la qualification pour travailler au poste  $T_j$ .

![img-29.jpeg](img-29.jpeg)
FIGURE I.24. Problème d'affection.

Example I.3.15 (Tri topologique). Dans la majorité des exemples décrits précédemment, nous avons rencontres des graphes non orientés. Un exemple de graphe orienté est le suivant. On rencontres parfois des problèmes pour lesquels on recherche un ordre acceptable dans l'ordonnancement de tâches dépendant les une des autres. On peut par exemple penser à la fabrication d'une voiture sur une chaîne de montage : on peut monter de façon indépendante le moteur et la carrosserie, avant de les assembler. Un autre exemple provient des choix de cours réalisés par des étudiants. En effet, certains prérequis sont parfois nécessaires. On considérera un graphe dont les sommets sont les tâches (resp. les cours) à réaliser (resp. à suivre) et si une tâche (resp. un cours) doit être réalisée (resp. suivi) avant une autre (resp. un autre), on traçera un arc orienté entre les deux sommets

Chapitre I. Premier contact avec les graphes

correspondant. La question sous-jacente est de déterminer une indexation des sommets d'un graphe orienté sans cycle de manière telle que s'il existe un arc de  $v_{i}$  à  $v_{j}$ , alors  $i &lt; j$ .

![img-30.jpeg](img-30.jpeg)
FIGURE I.25. Une application du tri topologique.

Example I.3.16 (Tournoi). On imagine un ensemble d'équipes ou de joueurs et une compétition où chaque joueur affronte tout autre joueur exactement une fois. Le seul résultat possible est la victoire ou la défait. On peut alors considérer un graphe dont les sommets sont les joueurs et un arc relié le joueur  $i$  au joueur  $j$  si  $i$  a battu  $j$  lors de leur confrontation directe. La question naturelle qui se pose est alors d'essayer de déterminer un vainqueur pour la compétition.

Example I.3.17 (Graphe de De Bruijn). En combinatoire des mots, on étudie entre autres, les mots infinis (i.e., les suites infinies  $w: \mathbb{N} \to \Sigma$  dont les éléments sont à valeurs dans un ensemble fini  $\Sigma$ ). Le graphe de De Bruijn d'ordre  $k$  du mot  $w$  a pour sommets les facteurs de longueur  $k$  de  $w$  (i.e., les mots finis de la forme  $w_i \cdots w_{i+k-1}$ ) et on dispose d'un arc de label  $\sigma \in \Sigma$  entre les sommets  $\tau x$  et  $x\sigma$  si et seulement si  $\tau x\sigma$  est un facteur de  $w$  de longueur  $k+1$ . La figure I.26 représenté le graphe de De Bruijn d'ordre 3 du mot périodique abababab... Ces graphes sont en relation directe avec

![img-31.jpeg](img-31.jpeg)
FIGURE I.26. Le graphe de De Bruijn d'ordre 3 de ababab...

certaines propriétés combinatoires des mots correspondants. Par exemple, un mot infini est ultimement périodique (i.e., il existe des mots finis  $u$  et  $v$  tels que  $w = uvv \cdots$ ) si et seulement si, pour tout  $k$  suffisamment grand, son graphe de De Bruijn d'ordre  $k$  contient un unique cycle dont tous les sommets ont un demi-degré sortant égal à 1.

Example I.3.18 (Flot). Une société hydro-électrique dispose d'un ensemble de canalisations interconnectées de divers diamètres et désiré acheminer

I.3. Quelques exemples

une quantité d'eau maximale d'un point  $A$  à un point  $B$ . Quelle quantité d'eau peut effectivement être acheminée et comment l'eau est-elle distribuée le long du réseau de canalisations? On peut imaginer que des contraintes techniques imposent en outre que chaque canalisation ait un débit maximum et minimum. Un tel exemple est représenté à la figure I.27 (les bornes de chaque canalisation sont représentées par un intervalle, une solution optimale est représentée par un nombre associé à chaque arc). On peut également

![img-32.jpeg](img-32.jpeg)
FIGURE I.27. Un problème de flot maximum.

prendre en compte une version pondérée où les coûts d'entretien des canalisations pourraient varier. On voudrait alors disposer d'un flot maximum pour un coût minimum.

Exemple I.3.19 (Chemin critique). Un entrepreneur charge de construire une maison doit planifier l'accomplissement de plusieurs tâches :

A: Creusage des fondations
B: Construction du gros-oeuvre
C: Installation électrique
D: Installation du chauffage central
E: Réalisation des peintures extérieures
F: Réalisation des peintures interieures

Ces diverses opérations sont soumises à des contraintes temporelles. Certaines tâches ne peuvent débuter avant que d'autres ne soient achevées; par contre, certaines peuvent aussi être réalisées en parallèle. Par exemple, les fondations doivent être creusées avant de débuter le gros oeuvre. De même, l'installation électrique doit être achevée avant de débuter les peintures interieures. Par contre, on peut effectuer simultanément les peintures extérieures et l'installation du chauffage. On considère un graphe dont les sommets correspondent à des instants marquant la fin de certaines étapes et le début d'autres. Les arcs sont pondérés par le temps nécessaire à la réalisation d'une tâche. On obtient pour notre exemple, le graphe représenté à la figure I.28 où 0 représentée le début des travaux, 1 le début du gros-oeuvre, 2 le début de l'installation électrique, 3 celui du chauffage, 4 celui de la peinture extérieure, 5 celui de la peinture interieure et enfin, 6 représentée la fin des travaux. Sur notre exemple, on voit des pondérations différentes au sortir du sommet 1. Cela peut par exemple signifier que la tâche 2 peut débuter sans que le gros-oeuvre ne soit totalement achevé (par contre, pour

Chapitre I. Premier contact avec les graphes

![img-33.jpeg](img-33.jpeg)
FIGURE I.28. Chemin critique dans la planification de tâches.

débuter 4, le gros oeuvre doit être entièrement terminé, ce qui explique une pondération plus importante). Se dégage alors la notion de chemin critique, chemin de poids maximal entre le début et la fin des travaux. En effet, pour achever la maison, toutes les tâches auront été entièrement exécutées et par conséquent, il faudra également emprunter le chemin correspondant à la réalisation des tâches les plus lentes. Ce chemin critique permet donc de déterminer le temps minimum nécessaire à l'achèvement des travaux.

# 4. Chemins et circuits

Les définitions suivantes sont somme toute assez naturelles et intuitives, mais il faut bien les préciser au moins une fois de manière rigoureuse pour savoir de quoi on parle exactement.

Definition I.4.1. Soit  $G = (V, E)$  un multi-graphe non orienté. Un chemin de longueur  $k \geq 1$  est une suite ordonnée  $(e_1, \ldots, e_k)$  de  $k$  arêtes adjacentes  $e_i = \{e_{i,1}, e_{i,2}\}$ , i.e., pour tous  $i \in \{1, \ldots, k-1\}$ ,  $e_{i,2} = e_{i+1,1}$ . Ce chemin de longueur  $k$  joint les sommets  $e_{1,1}$  et  $e_{k,2}$ . On dit que le chemin  $(e_1, \ldots, e_k)$  passes par les arêtes  $e_1, \ldots, e_k$  (resp. par les sommets  $e_{1,1}, e_{2,1}, \ldots, e_{k,1}$  et  $e_{k,2}$ ). On supposera qu'un chemin de longueur 0 (correspondant à la suite vide) joint toujours un sommet à lui-même.

Si les extrémités du chemin sont égales, i.e., si  $e_{1,1} = e_{k,2}$ , on parle只不过 de cycle, de circuit ou encore de chemin fermé. Si on désire préciser que le chemin considéré n'est pas un cycle, on parlera de chemin ouvert.

Il se peut que les arêtes d'un chemin soient toutes distinctes (cela n'implique pas que les sommets du chemin soient tous distincts). On parle alors de piste ou de chemin élémentaire (voir par exemple, la figure I.29).

Si les arêtes d'un chemin sont toutes distinctes et si de plus, les sommets sont tous distincts $^{10}$ , on parle alors de chemin simple.

Bien sur, les circuits étant des chemins particuliers, on parle aussi de circuit élémentaire ou simple (voir par exemple, la figure I.30). Evidemment, dans la définition d'un circuit simple, on admet que les sommets  $e_{1,1}$  de départ et  $e_{k,2}$  d'arrivée puissant être égaux, mais seulement eux. Il est clair

I.4. Chemins et circuits

![img-34.jpeg](img-34.jpeg)
FIGURE I.29. Une piste (ou chemin élémentaire) et un chemin simple

![img-35.jpeg](img-35.jpeg)
FIGURE I.30. Un circuit, un circuit élémentaire (ou piste fermée) et un circuit simple.

que tout circuit peut se décomposer en circuits simples.

Remarque I.4.2. Suivant les auteurs, la terminologie peut changer enormément. Ainsi, il n'est pas rare (par exemple, pour C. Berge), d'inverser les définitions des adjectifs simple et élémentaire. Pour d'autres, la notion de cycle contient de plus le caractère élémentaire, voire simple.

Remarque I.4.3. Dans le cas d'un graphe (qui n'est pas un multi-graphe), c'est-à-dire pour un 1-graphe, un chemin est aussi univoquement déterminé par une suite de sommets  $(v_{1},\ldots ,v_{k})$  de manière telle que  $\{v_{i},v_{i + 1}\}$  est une arête du graphe.

Définition I.4.4. Deux sommets  $a$  et  $b$  sont connectés s'il existe un chemin les joignant, ce que l'on notera  $a \sim b$ . La relation  $\sim$  "être connecté" est une relation d'équivalence sur  $V$ . Une classe d'équivalence pour  $\sim$  est une composante connexe de  $G$ . Un multi-graphe non orienté est connexe si  $V / \sim$  contient une seule classe d'équivalence, i.e.,  $G$  possède une seule composante connexe. Autrement dit, un multi-graphe non orienté est connexe si, pour toute paire de sommets, il existe un chemin les joignant. On supposera de plus que  $G = (\{v\}, \emptyset)$  est connexe (ce qui revient à supposer qu'un chemin de longueur 0 joint toujours un sommet à lui-même).

Chapitre I. Premier contact avec les graphes

**Définition I.4.5.** Soit $G = (V, E)$ un multi-graphe non orienté connexe$^{11}$. La distance$^{12}$ entre deux sommets $a$ et $b$ est la longueur du plus court chemin joignant $a$ et $b$. On la note $\mathrm{d}(a, b)$. Le diamètre de $G$ est défini par

$$
\operatorname{diam}(G) = \max_{a, b \in V} \mathrm{d}(a, b).
$$

Si $G$ est en outre pondéré par la fonction de poids $f: E \to \mathbb{R}^+$, la distance entre les sommets $a$ et $b$ est égale au poids minimal des chemins joignant $a$ et $b$, i.e.,

$$
\mathrm{d}(a, b) = \min_{\substack{\text{chemin} (e_1, \ldots, e_t) \\ \text{joignant } a \text{ et } b}} \sum_{i=1}^{t} f(e_i).
$$

Les définitions données précédemment s'adaptent aisément au cas d'un multi-graphe orienté. Il suffit de respecter en plus le sens de parcours imposé par les arcs. Donnons quelques précisions.

**Définition I.4.6.** Soit $G = (V, E)$ un multi-graphe orienté$^{13}$. Un chemin de longueur $k \geq 1$ est une suite ordonnée $(v_1, \ldots, v_k)$ de $k$ arcs $v_i = (v_{i,1}, v_{i,2})$ tels que pour tous $i \in \{1, \ldots, k-1\}$, $v_{i,2} = v_{i+1,1}$. Ce chemin de longueur $k$ joint les sommets $v_{1,1}$ et $v_{k,2}$.

S'il existe un chemin joignant deux sommets $a$ et $b$, on notera $a \to b$. Si $a \to b$ et $b \to a$, on dira que $a$ et $b$ sont fortement connectés et on notera $a \leftrightarrow b$. Si on impose à $\leftrightarrow$ d'être réflexive (i.e., on suppose que $a \leftrightarrow a$), on vérifie aisément que la relation $\leftrightarrow$ "être fortement connecté" est une relation d'équivalence sur $V$. Une classe d'équivalence pour $\leftrightarrow$ est une composante fortement connexe (ou, plus court, $f.$ connexe) de $G$. Si $V / \leftrightarrow$ contient une seule classe, on dira que $G$ est fortement connexe (ou $f.$ connexe).

Les sommets appartenant à un cycle maximal, i.e., un cycle auquel on ne peut adjoindre de nouveaux sommets, constituent une composante $f.$ connexe. Autrement dit, un multi-graphe orienté $G$ est $f.$ connexe si et seulement si il existe un cycle passant par chaque sommet de celui-ci.

Si on supprime l'orientation des arcs de $G$ et si le multi-graphe non orienté obtenu de cette manière est connexe, alors on dira que $G$ est simplement connexe (ou $s.$ connexe). On pourra bien entendu définir, de manière évidente, les composantes simplement connexes (ou $s.$ connexe de $G$).

**Remarque I.4.7.** Les notions de distance et de diamètre données dans le cas non orienté s'adaptent facilement au cas d'un multi-graphe orienté fortement connexe. On remarquera cependant qu'ici, la fonction $\mathrm{d}(\cdot, \cdot)$ n'est en général pas symétrique$^{14}$.

$^{11}$ Si $G$ n'était pas connexe, la fonction distance ne serait pas une fonction totale définie sur $V \times V$ tout entier.

$^{12}$ Vérifier qu'il s'agit effectivement d'une distance.

$^{13}$ Ce graphe peut avoir ou non des boucles, peu importe.

$^{14}$ Il ne peut donc pas s'agir à proprement parler d'une distance.

I.4. Chemins et circuits

Exemple I.4.8. Considerons le multi-graphe orienté de la figure I.31 dont l'ensemble des sommets est  $\{a, \ldots, e\}$  et l'ensemble des arcs est  $\{1, \ldots, 7\}$ . Ce graphe est simplement connexe mais il n'est pas fortement connexe. En

![img-36.jpeg](img-36.jpeg)
FIGURE I.31. Un graphe orienté.

effet, il n'existe pas de chemin joignant les sommets  $d$  à  $a$ . L'ensemble  $\{b, c, d\}$  est une composante fortement connexe du graphe (les deux autres composantes sont  $\{a\}$  et  $\{e\}$ ). Un chemin est par exemple donné par  $(1, 3, 7)$ , un cycle par  $(3, 4, 5)$  et une piste par  $(1, 3, 4, 5, 6)$ . La distance entre  $d$  et  $c$  vaut 1. Par contre, la distance entre  $c$  et  $d$  vaut 2.

4.1. Recherche du plus court chemin. Soit  $G = (V, E)$  un digraphe pondéré par la fonction  $p: E \to \mathbb{R}^+$ . Nous allonsprésenter l'algorithmme de Dijkstra de recherche d'un plus court chemin (i.e., un chemin de poids minimal) d'un sommet  $u$  fixé à un sommet quelconque de  $G$ . Il est clair que l'on peut restreindre ce problème au cas d'un digraphe simple $^{15}$ . Pour rappel, si  $u$  et  $v$  sont deux sommets tels que  $u \to v$ , le poids d'un chemin  $(e_1, \ldots, e_k)$  les joignant est  $\sum_{i} p(e_i)$ . La distance de  $u$  à  $v$  est alors le poids minimal de tels chemins. Si  $u \nrightarrow v$ , on n'a pas à proprement parler de distance $^{16}$  et par convention, on posera que la "distance" de  $u$  à  $v$  est alors  $+\infty$ .

L'algorithm de Dijkstra s'applique également à un graphe non orienté. Il suffit de replacer l'arête  $\{u, v\}$  de poids  $\alpha$  par deux arcs  $(u, v)$  et  $(v, u)$  ayant chacun un poids  $\alpha$  et ainsi obtenir un digraphe.

Remarque I.4.9. Quitte à supposer  $p$  à valeurs dans  $\mathbb{R}^+ \cup \{+\infty\}$ , on étend  $p$  de  $E$  à  $V \times V$  tout entier en posant  $p(x, x) = 0$ , pour tout  $x \in V$  et  $p(x, y) = +\infty$ , si  $(x, y) \notin E$ . C'est ce que nous supposerons par la suite.

Intuitivement, l'algorithmie fonctionne de la manière suivante. A chaque sommet  $v$  de  $G$ , on associe une valeur  $\mathbb{T}(v)$ , initialisée à  $p(u,v)$  et une liste de sommets  $\mathbb{C}(v)$  censée correspondre à un chemin de  $u$  à  $v$ . Lorsque

Chapitre I. Premier contact avec les graphes

l'algorithmme s'achève,  $\mathsf{T}(v)$  contient le poids minimal des chemins joignant  $u$  à  $v$  et  $\mathsf{C}(v)$  réalise un tel chemin (ou alors,  $\mathsf{T}(v) = +\infty$  si  $u \nrightarrow v$ ). L'idée est de construire de proche en proche un ensemble  $X \subseteq V$  de manière telle qu'un chemin de poids minimal de  $u$  à  $v \in X$  passes uniquement par des sommets de  $X$ . L'ensemble  $X$  est initialisé à  $\{u\}$  et à chaque étape, on ajoute un sommet à l'ensemble.

Algorithm I.4.10 (Algorithmé de Dijkstra). Les données sont un digraphé simple  $G = (V, E)$  pondéré par une fonction  $p: V \times V \to \mathbb{R}^+ \cup \{+\infty\}$  (cf. remarque I.4.9) et un sommet  $u$ .

```latex
Pour tout sommet  $v\in V$  ，T(v):=p(u,v)，C(v):=(u,v)
$\mathbf{X}:=\{u\}$
Tant que  $\mathbf{X}\neq V$  ，répéter
Choisir  $v\in V\backslash \mathbf{X}$  tel que, pour tout  $y\in V\backslash \mathbf{X}$  ，T(v)≤T(y)17
$\mathbf{X}:=\mathbf{X}\cup\{v\}$
Pour tout  $y\in V\backslash \mathbf{X}$
Si  $\mathrm{T}(y) &gt; \mathrm{T}(v) + p(v,y)$  ，alors  $\mathrm{T}(y):=\mathrm{T}(v)+p(v,y)$  et  $\mathrm{C}(y):=[\mathrm{C}(v),y]$
```

Dans cet algorithme, la notation  $[\mathbb{C}(v),y]$  représentée la liste  $\mathbb{C}(v)$  à laquelle on ajoute un élément  $y$ . Intuitivement, lorsqu'on ajoute un sommet  $\mathbf{v}$  à  $\mathbf{X}$ , on regarde s'il est avantageux pour les sommets  $\mathbf{y}$  ne se trouvant pas dans  $\mathbf{X}$  de passer par ce sommet  $\mathbf{v}$  nouvellement ajouté à  $\mathbf{X}$ . Si tel est le cas, on met à jour les informations concernant  $\mathbf{y}$ .

Avant de démontré l'exactitude de cet algorithme, donnons un exemple d'application de ce dernier.

Example I.4.11. Voici une application de l'algorithm de Dijkstra au graphe représenté à la figure I.32. Pour l'initialisation, prenons  $\mathbf{X} = \{a\}$

![img-37.jpeg](img-37.jpeg)
FIGURE I.32. Un digraphesimplepondéré.

et on a

|  v | a | b | c | d | e | f  |
| --- | --- | --- | --- | --- | --- | --- |
|  T(v) | 0 | 1 | 1 | 4 | +∞ | +∞  |
|  C(v) | (a,a) | (a,b) | (a,c) | (a,d) | (a,e) | (a,f)  |

I.4. Chemins et circuits

Si le sommet  $b$  est choisi (on a leCHOIX entre  $b$  et  $c$ ), on a  $\mathbf{X} = \{a,b\}$  et le tableau est mis à jour:

|  v | a | b | c | d | e | f  |
| --- | --- | --- | --- | --- | --- | --- |
|  T(v) | 0 | 1 | 1 | 3 | +∞ | +∞  |
|  C(v) | (a,a) | (a,b) | (a,c) | (a,b,d) | (a,e) | (a,f)  |

A l'étape suivante, on est force deCHOISIR  $c$  . Ainsi,  $\mathbf{X} = \{a,b,c\}$  et le tableau devient :

|  v | a | b | c | d | e | f  |
| --- | --- | --- | --- | --- | --- | --- |
|  T(v) | 0 | 1 | 1 | 3 | 9 | +∞  |
|  C(v) | (a,a) | (a,b) | (a,c) | (a,b,d) | (a,c,e) | (a,f)  |

A present,  $d$  est selectionné,  $\mathbf{X} = \{a,b,c,d\}$  et la valeur de  $\mathbb{C}(e)$  peut être améliorée,

|  v | a | b | c | d | e | f  |
| --- | --- | --- | --- | --- | --- | --- |
|  T(v) | 0 | 1 | 1 | 3 | 8 | +∞  |
|  C(v) | (a,a) | (a,b) | (a,c) | (a,b,d) | (a,b,d,e) | (a,f)  |

Ensuite,  $e$  et  $f$  sont choisis successivement mais le tableau ne change plus. Par exemple, on en conclus que le plus court chemin joignant  $a$  à  $e$  passes par  $b$  et  $d$  et son poids vaut 8.

Démonstration. Il est clair que l'algorithmme s'achève toujours puisqu'à chaque itération de la boucle, un nouvel état est ajouté à  $X$ . Il nous suffit donc de vérifier qu'il s'achève avec le résultat attendu. Pour ce faire, on procède par récurrence sur  $\# X$  et on vérifie les deux assertions suivantes pour toutes les valeurs de  $\# X$ , d'ou le résultat annoncé quand  $X = V$ .

i) Pour tout  $v \in X$ ,  $\mathbb{T}(v)$  est le poids minimal de tous les chemins joignant  $u$  à  $v$ .
ii) Pour tout  $v \notin X$ ,  $\mathbb{T}(v)$  est le poids minimal des chemins joignant  $u$  à  $v$  qui, à l'exception de  $v$ , passent uniquement par des sommets de  $X$ .

Pour  $\# X = 1$ , c'est immédiat car  $X = \{u\}$  et l'initialisation effectuee dans l'algorithmme correspond aux assertions. Supposons le résultat vérifie pour  $\# X = n$  et vérifiions-le pour  $\# X = n + 1$ ,  $1 \leq n &lt; \# V$ .

Lorsque  $X$  contient  $n$  sommets, l'algorithmme stipule de lui ajouter un sommet  $v$  ayant une valeur  $\mathbb{T}(v)$  minimale parmi les sommets n'appartenant pas à  $X$ . Par l'hypothèse de récurrence ii), puisqu'à ce stade  $\# X = n$  et  $v \notin X$ ,  $\mathbb{T}(v)$  est le poids minimal des chemins joignant  $u$  à  $v$  qui, à l'exception de  $v$ , passent uniquement par des sommets de  $X$ . Nous ajoutons à présent  $v$  à  $X$  pour obtenir un ensemble de taille  $n + 1$ . Procédons par l'absurde et supposons que  $\mathbb{T}(v)$  n'est pas le poids minimal des chemins joignant  $u$  à  $v$  (c'est-à-dire que l'assertion i) n'est pas vérifiée). Par conséquent, il existe un sommet  $y \notin X$  et un chemin de  $u$  à  $v$  passant par  $y$  dont le poids est strictement inférieur à  $\mathbb{T}(v)$ . De là, on en conclus que  $\mathbb{T}(y) &lt; \mathbb{T}(v)$ , ce qui est impossible au vu du choix du sommet  $v$  au sein de l'algorithmme.

Chapitre I. Premier contact avec les graphes

![img-38.jpeg](img-38.jpeg)
FIGURE I.33. Illustration de l'algorithm de Dijkstra.

Pour la seconde assertion, on procède de manière analogue en utilisant cette fois la dernière ligne de l'algorithmie pour obtenir une contradiction. Plus précisé, passons d'un ensemble  $X$  à  $n$  sommets à un ensemble  $X'$  à  $n + 1$  sommets en lui ajoutant un sommet  $v$ . Supposons à présent que ii) n'est plus satisfait et qu'il existe donc un sommet  $y \notin X'$  tel que  $\mathbb{T}(y)$  est strictement supérieur au poids minimal des chemins joignant  $u$  à  $y$  qui, à l'exception de  $y$ , passent uniquement par des sommets de  $X'$ . Or, par hypothèse de récurrence, nous savons que l'assertion ii) était satisfaite pour  $\# X = n$ . Autrement dit, avant d'ajouter le sommet  $v$ ,  $\mathbb{T}(y)$  était minimal

![img-39.jpeg](img-39.jpeg)
FIGURE I.34. Illustration de l'algorithm de Dijkstra.

pour les chemins joignant  $u$  à  $y$  qui, à l'exception de  $y$ , passent uniquement par des sommets de  $X$ . Ainsi, en ajoutant le sommet  $v$  à  $X$ , on aurait replacé  $\mathbb{T}(y)$  par une valeur supérieure, ce qui est en contradiction avec les prescriptions de l'algorithmie (à la dernière ligne, on spécifique de replacer  $\mathbb{T}(y)$  uniquement lorsqu'il est préféable de passer par  $v$ ).

Remarque I.4.12. On peut facilement voir que l'algorithmie de Dijkstra a une complexité temporelle en  $\mathcal{O}((\# V)^2)$ . Avec une implémentation minutieuse, utilisant les listes d'adjacence et les files de priorité, on obtient même une complexité en  $\mathcal{O}((\# E + \# V) \log \# V)$ .

Remarque I.4.13. [16] Le routage des données entre les réseaux d'un internet est l'une des applications où les plus courts chemins jouent un role important. Le routage est le processus consistant à prendre des décisions sur la façon de déplacer

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

Chapitre I. Premier contact avec les graphes

Théorème I.4.16. Un multi-graphe fini non orienté connexe  $G = (V, E)$  possède un circuit eulérien si et seulement si le degré de chaque sommet est pair.

Démonstration. Supposons donc que chaque sommet de  $G$  est de degré pair. Débutons la construction d'une piste avec un sommet  $a_1$  de  $G$ . A chaque étape  $i \geq 1$  de cette construction, on désit un sommet  $a_{i+1}$  de manière telle qu'une arête  $\{a_i, a_{i+1}\} \in E$  est sélectionnée parmi les  $\# E - i + 1$  arêtes non déjà sélectionnées. Puisque chaque sommet est de degré pair, cette sélection est toujours possible ("lorsqu'on aboutit dans un sommet, on peut toujours en repartir"). Puisque le graphe est fini, cette procédure s'achève toujours.

On dispose alors d'une piste  $P$  joignant  $a_1$  à un certain sommet  $a_\ell$ . En fait, on peut supposer que cette piste est fermée, i.e.,  $a_\ell = a_1$ . En effet, si  $a_\ell$  diffère de  $a_1$ , puisque le degré de chaque sommet est pair, on peut étendre la piste en ajoutant une arête  $\{a_\ell, a_{\ell+1}\}$ . En continuant de la sorte $^{19}$ , on épuise les sommets jusqu'à revenir en  $a_1$ .

Si la piste fermée  $P$  est un circuit eulérien, le théorème est démontré. Sinon, il existe un sommet  $b$  de  $P$  qui est l'extrémité d'un nombre pair d'arêtes n'apparaissant pas dans  $P$ . (Une illustration est donnée à la figure I.35. Depuis  $b$ , il est possible de construire une piste fermée  $Q$  formée

![img-40.jpeg](img-40.jpeg)
FIGURE I.35. Construction d'un circuit eulérien.

uniquement d'arêtes n'apparaissant pas dans  $P$ . (On utilise la même procédure que précédemment; il est clair que le degré de chaque sommet est encore pair.) De cette façon, nous avons étendu la piste  $P$  en une piste plus longue  $P \cup Q$  (couvrant un plus grand nombre d'arêtes). On obtient alors un circuit eulérien en répétant cette étape un nombre fini de fois.

Corollaire I.4.17. Le problème des sept points de Königsberg donné dans l'exemple I.3.1 n'admet pas de solution.

I.4. Chemins et circuits

Démonstration. C'est immédiat, le graphe de la figure I.12 possède un sommet de degré 5 et 3 sommets de degré 3.

Corollaire I.4.18. Un multi-graphe non orienté connexe possède un chemin eulérien joignant deux sommets  $a$  et  $b$  si et seulement si  $a$  et  $b$  sont les deux seuls sommets de degré impair.

Démonstration. Pour se ramener au théorème précédent, il suffit de considérer le graphe  $G$  auquel on ajoute une arête supplémentaire joignant les sommets  $a$  et  $b$ .

Exemple I.4.19. Fort des résultats precedents, on peut par exemple répondre à la question: "le dessin suivant peut-il ou non être trace d'un seul trait, sans lever le crayon et sans repasser deux fois par un trait déjà trace ?". C'est une simple application du corollaire précédent $^{20}$ .

![img-41.jpeg](img-41.jpeg)
FIGURE I.36. Une maison à tracer d'un seul trait.

Nous présentons succinctement l'algorithmme de Fleury permettant de construire un circuit eulérien (à supposer qu'un tel circuit existe). Cet algorithme repose sur la notion d'arête de coupure représentée à la section 6. Nous conseillons donc au lecteur de passer cet algorithme en première lecture.

Algorithme I.4.20 (Fleury). La donnée fournie à cet algorithme est un graphe simple eulérien.

```txt
Choisir un sommet  $v_{0}\in V$ $i\coloneqq 1$
Répéter tant que possible
Choisir une arête  $e_i = \{v_{i - 1},v_i\} \in V$  telle que  $\triangleright e_i$  diffère des arêtes déjà choisis  $e_1,\ldots ,e_{i - 1}$  autant que possible,  $e_i$  ne doit pas être une arête de coupure de  $G_{i} = G - \{e_{1},\dots ,e_{i - 1}\}$ $i\coloneqq i + 1$
```

Cet algorithme fournit une suite d'arêtes  $e_1, e_2, \ldots$  qui constituent un circuit eulérien.

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

I.4. Chemins et circuits

nouvellement déterminés comme appartenant à la composante connexe. Il s'agit donc de deux variables de type ensembliste. Lorsqu'il n'y a plus de nouveaux sommets à ajouter à la liste des sommets appartenant à la composante connexe, l'algorithmme s'achève. Il reste alors à déterminer si cette composante connexe contient ou non tous les sommets de  $G$ . En particulier, à la fin de cet algorithme, la variable Composante est égale à la composante connexe de  $v_{0}$ .

L'emploi de cet algorithme est facilité par la construction préalable d'un dictionnaire des voisins. Il s'agit d'une liste associant à chaque sommet  $v$ , l'ensemble  $\nu(v)$  de ses voisins.

Example I.4.24. Considérons le graphe simple représenté à la figure I.37 et appliquons-lui l'algorithmme I.4.23. Les résultats de l'algorithmme se trouvent dans le tableau I.2. Le tableau I.1 donne le dictionnaire des voisins

![img-42.jpeg](img-42.jpeg)
FIGURE I.37. Un graphe simple (connexe).

construit une fois pour toutes. Le tableau I.2 montre l'évolution des vari-

|  v | ν(v)  |
| --- | --- |
|  a | {b, c, f}  |
|  b | {a, c}  |
|  c | {a, b, d}  |
|  d | {c, e}  |
|  e | {d, f, g}  |
|  f | {a, e, g}  |
|  g | {e, f}  |

TABLE I.1. Dictionnaire des voisins.

ables apparaissant dans l'algorithmme avec comme choix initial,  $v_{0} = c$ .

4.4. Décomposition en composantes fortement connexes. La détermination de la composante fortement connexe contenant un sommet donné  $v_{0}$  s'inspire de l'algorithmme I.4.23. Il est clair que, pour ce problème, il suffit de considérer le cas de graphes orientés simples. Déterminer si un graphe est connexe, autrement dit s'il possède une seule composante connexe, est l'une des deux étapes permettant de decide si un graphe donné possède

Chapitre I. Premier contact avec les graphes

|  Composante | New | Voisins  |
| --- | --- | --- |
|  {c} | {c} | ∅ ν(c) = {a,b,d}  |
|  {c} ∪ {a,b,d} = {a,b,c,d} | {a,b,d} \ {c} = {a,b,d} | ∅ ν(a) ∪ ν(b) ∪ ν(d) = {a,b,c,e,f}  |
|  {a,b,c,d} ∪ {e,f} = {a,b,c,d,e,f} | {a,b,c,e,f} \ {a,b,c,d} = {e,f} | ∅ ν(e) ∪ ν(f) = {a,d,e,f,g}  |
|  {a,b,c,d,e,f} ∪ {g} = {a,b,c,d,e,f,g} | {a,d,e,f,g} \ {a,b,c,d,e,f} = {g} | ∅ ν(g) = {e,f}  |
|  {a,b,c,d,e,f,g} ∪ {e,f} = {a,b,c,d,e,f,g} | {e,f} \ {a,b,c,d,e,f,g} = ∅ |   |

TABLE I.2. Application de l'algorithm de test de connexité.

ou non une structure d'arbre ( comme nous le verrons, les arbres sont particulièrement importants à bien des égards).

Definition I.4.25. La clôture réflexive et transitive de l'application succ est définie comme suit,

$$
\operatorname {s u c c} ^ {*} (v) := \bigcup_ {j = 0} ^ {\infty} \operatorname {s u c c} ^ {j} (v) \quad \text {o u} \left\{ \begin{array}{l} \operatorname {s u c c} ^ {0} (v) = v \\ \operatorname {s u c c} ^ {j + 1} (v) = \operatorname {s u c c} (\operatorname {s u c c} ^ {j} (v)) \end{array} \right..
$$

Si le graphe  $G$  est fini, il est clair que

$$
\operatorname {s u c c} ^ {*} (v) = \bigcup_ {j = 0} ^ {\# E} \operatorname {s u c c} ^ {j} (v).
$$

De même, s'il existe  $k &lt; \# E$  tel que  $\operatorname{succ}^k(v) = \operatorname{succ}^{k+1}(v)$ , alors

$$
\operatorname {s u c c} ^ {*} (v) = \bigcup_ {j = 0} ^ {k} \operatorname {s u c c} ^ {j} (v).
$$

Avec les notations précédentes, pour tous  $a,b\in V$

$$
a \to b \Leftrightarrow b \in \operatorname {s u c c} ^ {*} (a).
$$

Remarque I.4.26. On définit de manière semblable la clôture réflexive et transitive de l'application pred. On a bien évidemment, pour tous sommets  $a$  et  $b$ ,

$$
b \to a \Leftrightarrow b \in \operatorname {p r e d} ^ {*} (a).
$$

I.4. Chemins et circuits

Ainsi,

$$
a \leftrightarrow b \Leftrightarrow b \in \operatorname {s u c c} ^ {*} (a) \cap \operatorname {p r e d} ^ {*} (a).
$$

L'algorithmme I.4.23 peut facilement être adapté pour calculer  $\mathrm{succ}^* (a)$  (resp.  $\mathrm{pred}^* (a)$ ). Il suffit principalement d'initialiser les variables Composante et New à  $\{a\}$  et de replacer  $\nu (v)$  par  $\mathrm{succ}(v)$  (resp.  $\mathrm{pred}(v)$ ). En recherchant ainsi l'intersection des deux ensembles, on détermine la composante f. connexe du sommet  $a$ . Si cette composante est égale à l'ensemble des sommets, alors le graphe est f. connexe.

Definition I.4.27. Soit  $G = (V, E)$  a graphe orienté. On construit un nouveau graphe orienté, appelé graphe acyclique des composantes $^{21}$  ou encore condensé de  $G$ , dont les sommets sont les composantes f. connexes de  $G$ . Un arc joint deux composantes f. connexes  $A$  et  $B$ , s'il existe  $a \in A$  et  $b \in B$  tels que  $a \to b$ .

Remarque I.4.28. Bien évidemment, s'il existe  $a \in A$  et  $b \in B$  tels que  $a \to b$ , alors il n'existe aucun  $a' \in A$  ni aucun  $b' \in B$  tels que  $b' \to a'$ . En effet, si tel était le cas,  $A \cup B$  serait alors une composante f. connexe de  $G$ . Ceci est impossible vu la maximalité des composantes connexes. D'une manière générale, le graphe des composantes ne contient pas de cycle.

Example I.4.29. La figure I.38 représenté un digraphé et ses composantes f. connexes ainsi que le graphe des composantes correspondant.

![img-43.jpeg](img-43.jpeg)
FIGURE I.38. digraphé et graphe des composantes.

Le résultat suivant donne un lien entre la simple connexité et la forte connexité.

Lemma I.4.30. Soit  $G$  un digraph (simple) tel que pour tout  $v \in V$ ,  $d^{+}(v) = d^{-}(v)$ . Alors,  $G$  est f. connexe si et seulement si il est s. connexe.

Démonstration. Il est clair que la f. connexité implique la s. connexité. Supposons que  $G$  est s. connexe, que pour tout  $v$ ,  $d^{+}(v) = d^{-}(v)$ , mais que  $G$  n'est pas f. connexe. Notre but est d'obtenir une contradiction. Soient  $C_1, \ldots, C_r$  les composantes f. connexes de  $G$  (ou  $r \geq 2$ ). Comme nous

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

I.5. Sous-graphes

# 5. Sous-graphes

Dans la preuve du lemme I.4.30, nous avons considéré la restriction d'un graphe à une de ses composantes. Dans cette section, nous formalisons ce type de construction. Grosso modo, un sous-graphe d'un graphe donné est un graphe obtenu en supprimant certains sommets et/ou arêtes.

Definition I.5.1. Soient  $G = (V, E)$  et  $G' = (V', E')$  deux graphes (orientés ou non). Le graphe  $G'$  est un sous-graphe de  $G$  si

$V^{\prime}\subseteq V$
$E^{\prime}\subseteq E\cap (V^{\prime}\times V^{\prime}).$

Ainsi,  $G'$  est un sous-graphe de  $G$  s'il est obtenu en enlevant à  $G$  certains sommets et/ou certains arcs ou arêtes. En particulier, si on enlève un sommet  $v$  de  $G$ , il faut nécessairement enlever tous les arcs (ou arêtes) incidents à  $v$ . Par contre, pour construire  $G'$ , on peut très bien enlever un arc (ou une arête) de  $G$  sans pour autant enlever le moindre sommet de  $G$ . Le graphe  $G'$  est un sous-graphe propre de  $G$  si  $E' \subsetneq E$  ou  $V' \subsetneq V$ . Dans le premier

![img-44.jpeg](img-44.jpeg)
FIGURE I.39. Un graphe et deux sous-graphes.

![img-45.jpeg](img-45.jpeg)

![img-46.jpeg](img-46.jpeg)

sous-graphe de la figure I.39, on a enlevé uniquement certaines arêtes. Dans le second, on a enlevé un sommet et les arêtes adjacentes.

Soient  $v \in V$  et  $e \in E$ . On note  $G - e$  (resp.  $G - v$ ) le sous-graphe  $G'$  de  $G$  obtenu en supprimant l'arc (ou l'arête)  $e$  (resp. le sous-graphe  $G'$  obtenu en supprimant le sommet  $v$  et les arcs (ou les arêtes) adjacents). Par analogie, on notera  $G = G' + e$  (resp.  $G = G' + v$ ), le graphe obtenu par adjonction à  $G'$  d'une arête ou d'un sommet.

On peut bien évidemment étendre ces notations à un ensemble fini de sommets. Ainsi, si  $W = \{v_{1},\ldots ,v_{k}\} \subseteq V$ , alors  $G - W$  est le sous-graphe

$$
\left(\dots \left(\left(G - v _ {1}\right) - v _ {2}\right) \dots - v _ {k - 1}\right) - v _ {k} := G - v _ {1} - \dots - v _ {k}.
$$

On procède de même pour un ensemble fini d'arcs (ou d'arêtes) et on introduit une notation  $G - F$  pour un sous-ensemble  $F$  de  $E$ .

Soit  $W \subseteq V$ . Le sous-graphe de  $G$  induit par  $W$  est le sous-graphe  $G' = (W, E')$  avec  $E' = E \cap (W \times W)$ .

Si  $W \subseteq V$  est tel que le sous-graphe induit par  $W$  ne contient aucune arête, alors les sommets de  $W$  sont dits indépendants. Le nombre maximal de sommets indépendants de  $G$  est noté  $\alpha(G)$ .

Chapitre I. Premier contact avec les graphes

![img-47.jpeg](img-47.jpeg)
FIGURE I.40. Des sommets indépendants.

Definition I.5.2. Soient  $G = (V, E)$  un graphe et  $G' = (V', E')$  un de ses sous-graphes. On dit que  $G'$  est un sous-graphe couvrant  $G$ , si  $V' = V$  et si

$$
\forall v \in V, \exists z \in V: \{z, v \} \in E ^ {\prime}.
$$

On dira que  $E'$  est une couverture (des sommets) de  $G$ . Autrement dit, tout sommet de  $G$  est une extrémité d'une arête de  $E'$ .

Comme nous le verrons bientôt, on s'intéressera en particulier aux sous-graphes couvrants qui sont des arbres. Dans ce cas, on parlera naturellement de sous-arbre couvrant.

Exemple I.5.3. Le premier sous-graphe de la figure I.39 est couvrant. Un exemple de sous-arbre couvrant a été donné à l'exemple I.3.8.

# 6. Coupes, points d'articulation,  $k$ -connexité

Certains sommets ou arêtes d'un graphe (ou d'une composante) connexe jouent un role particulier : les enlever rendrait le graphe non connexe. D'un point de vue pratique (par exemple, pour un réseau électrique ou de distribution d'eau, ou encore pour l'Internet), il s'agit de composants cruciaux du réseau. En effet, une panne localisée en un tel endroit priverait par exemple toute une région de ressources peut-être vitales.

Définition I.6.1. Soit  $H = (V, E)$  un multi-graphe $^{23}$  non orienté connexe (ou une composante connexe d'un multi-graphe non orienté). Le sommet  $v$  est un point d'articulation, si  $H - v$  n'est plus connexe, ou réduit à un point. D'une manière générale, on dit aussi que  $v$  est un point d'articulation d'un multi-graphe  $H$  si  $H - v$  contient plus de composantes connexes que  $H$  (ou est réduit à un point).

Si  $H$  est connexe et ne contient aucun point d'articulation, alors on dira que  $H$  est au moins 2-connexe.

Définition I.6.2. On peut étendre la notion de point d'articulation de la manière suivante. Un ensemble d'articulation est un ensemble  $W$  de sommets du multi-graphe connexe  $H = (V, E)$  qui est tel que  $H - W$  n'est plus

I.6. Coupes, points d'articulation,
k
-connexité

![img-48.jpeg](img-48.jpeg)
FIGURE I.41. Un graphe et ses points d'articulation.

![img-49.jpeg](img-49.jpeg)
FIGURE I.42. Un graphe au moins 2-connexe.

connexe (ou réduit à un sommet). D'une manière générale, pour un multi-graphe  $H$  quelconque, on dit aussi que  $W$  est un ensemble d'articulation si  $H - W$  contient plus de composantes connexes que  $H$ .

Definition I.6.3. Pour un graphe connexe  $H$ , on note  $\kappa(H)$  la taille minimale d'un ensemble d'articulation de  $H$ ,

$\kappa (H) = \min \{\# W\mid W\subseteq V:H - W$  disconnecté ou réduit à un sommet}.

En particulier, on a que  $\kappa(K_n) = n - 1$ . Pour un graphe  $G$  non connexe, on pose  $\kappa(G) = 0$ . Si  $\kappa(H) = k \geq 1$ , alors on dit que  $H$  est  $k$ -connexe. Ainsi, dans un graphe connexe  $G$ ,  $\kappa(G) = k$  signifie que quels que soient les  $k - 1$  sommets supprimés,  $G$  reste connexe mais il est possible de supprimer  $k$  sommets bien choisis pour disconnecter  $G$  (ou le rendre trivial). Pour un graphe  $G$  restreint à une unique arête, on a  $\kappa(G) = 1$  et pour le graphe vide, on pose  $\kappa(\emptyset) = 0$ .

Dans la définition I.6.1, lorsque nous avons parlé de graphe "au moins 2-connexe", cela signifie simplement que  $\kappa(G) \geq 2$ . Par exemple, le graphe de la figure I.42 qualifié d'au moins 2-connexe est en fait 2-connexe, comme on peut l'observer à la figure I.43; il suffit d'enlever deux sommets bien choisis pour le disconnector. D'une manière générale, on dira qu'un graphe

![img-50.jpeg](img-50.jpeg)
FIGURE I.43. Un graphe 2-connexe.

![img-51.jpeg](img-51.jpeg)

Chapitre I. Premier contact avec les graphes

$G$  est au moins  $k$ -connexe si  $\kappa(G) \geq k$ . En particulier, il est trivial $^{24}$  que si  $G$  est au moins  $(k + 1)$ -connexe, alors il est au moins  $k$ -connexe (i.e.,  $\kappa(G) \geq k + 1 \Rightarrow \kappa(G) \geq k$ ).

Remarque I.6.4. Si un graphe  $G = (V, E)$  contient au moins un point d'articulation, il est souvent commode de séparer  $G$  en ce qu'il est coutume d'appeler ses composantes 2-connexes. Il s'agit des composantes connexes obtenues après suppression des points d'articulation. Cette terminologie

![img-52.jpeg](img-52.jpeg)
FIGURE I.44. Un graphe et ses composantes 2-connexes.

consacrée n'est pas nécessairement la meilleure car, comme on peut le noter sur la figure I.44, une composante 2-connexe n'est pas nécessairement un graphe 2-connexe! En fait, les composantes 2-connexes permettent simplement demettre en évidence les zones du graphe robustes vis-à-vis de la connexité et de la suppression évientuelle d'un sommet.

Après s'être interressé aux sommets qui, lorsqu'on les supprime, disconnectent un graphe, nous considérons à présent les arêtes ayant une propriété analogue.

Definition I.6.5. Soit  $H = (V, E)$  un multi-graphe non orienté connexe (ou une composante connexe d'un multi-graphe non orienté). L'arête  $e$  est une arête de coupure si  $H - e$  n'est plus connexe. Au moins une extrémité d'une arête de coupure est bien évidemment un point d'articulation de  $H$ . Comme le montre le graphe de droite à la figure I.45, il y a des situations où seule une extrémité de l'arête de coupure est un point d'articulation.

![img-53.jpeg](img-53.jpeg)
FIGURE I.45. Un graphe et ses arêtes de coupure.

I.6. Coupes, points d'articulation,
k
-connexité

Le résultat suivant illustré le concept d'arête de coupure et permet aussi de répondre à la question de la mise à sens unique de routes évoqué dans l'exemple I.3.9.

Proposition I.6.6. Une arête  $e$  est une arête de coupure du graphe  $H = (V, E)$  si et seulement si  $e$  n'appartient à aucune piste fermée de  $H$ .

Démonstration. Si  $e$  est une arête de coupure, il existe des sommets  $u$  et  $v$  qui sont connectés dans  $H$  mais qui ne sont plus connectés dans  $H - e$ . Il existe donc un chemin joignant  $u$  et  $v$  qui passse par  $e$ . Dans  $H - e$ , une partie de ce chemin joint  $u$  à une extrémité de  $e$ , appelons-la  $e_1$  et l'autre partie du chemin joint  $v$  à l'autre extrémité de  $e$ , appelons-la  $e_2$ . Si  $e$  appartient à une piste fermée, il existe un chemin joignant  $e_1$  à  $e_2$  et ne passant pas par  $e$ . On peut donc en conclude que  $u$  et  $v$  sont encore connectés dans  $H - e$ , ce qui est impossible.

![img-54.jpeg](img-54.jpeg)
FIGURE I.46. Illustration de la proposition I.6.6

Passons à la réciproque et supposons que  $e = \{e_1, e_2\}$  n'est pas une arête de coupure. Si  $H$  est connexe,  $H - e$  l'est encore. Ainsi, il existe dans  $H - e$  une piste joignant  $e_1$  et  $e_2$ . De là, on conclut que dans  $H$ ,  $e$  appartient à une piste fermée.

Remarque I.6.7. Nous pouvons donner une solution au problème posé dans l'exemple I.3.9. En effet, si on dispose d'un graphe non orienté connexe et que l'on désire orienter ses arêtes de manière telle que le graphe résultat soit f. connexe, alors les arêtes de coupure doivent nécessairement être remplacées par deux arcs (pas de sens unique). Par contre, les autres arêtes appartiennent toutes à une piste fermée qu'il est aisé d'orienter (création de sens uniques).

Définition I.6.8. Soit  $H = (V, E)$  un multi-graphe non orienté connexe (ou une composante connexe d'un multi-graphe non orienté). L'ensemble  $F \subset E$  est un ensemble de coupure ou plus simplement une coupe ou coupure si  $F$  est un ensemble minimal (pour l'inclusion) tel que  $H - F$  n'est pas connexe. A la figure I.48, on a représenté en traits discontinus une des coupures du graphe.

coupe, coupure

Chapitre I. Premier contact avec les graphes

![img-55.jpeg](img-55.jpeg)
FIGURE I.47. Mise à sens unique

![img-56.jpeg](img-56.jpeg)
FIGURE I.48. Un graphe et une coupure.

Definition I.6.9. La taille minimale d'une coupe de  $H$  se note  $\lambda(H)$ ,

$\lambda (H) = \min \{\# F\mid F\subseteq E:H - F$  disconnecté}.

Si  $H$  n'est pas connexe, on pose  $\lambda(H) = 0$ . Notons encore que dans la litterature, on rencontres souvent la notation  $\kappa'(H)$  plutôt que  $\lambda(H)$ . Si  $H$  est connexe et si  $\lambda(H) = k$ , on dit que  $H$  est  $k$ -connexe (pour les arêtes). Autrement dit, si on enlève  $k-1$  arêtes à un graphe  $k$ -connexe (pour les arêtes), il reste connexe; par contre, il est possible d'enlever  $k$  arêtes pour le disconnector. On veillera à ne pas confondre les notions de  $k$ -connexité et de  $k$ -connexité pour les arêtes.

Enfin, on dira qu'un graphe  $G$  est au moins  $k$ -connexe (pour les arêtes), si  $\lambda(G) \geq k$ .

Au vu de la définition ci-dessus, de la proposition I.6.6 et de la remarque I.6.7, on a immédiatement le résultat suivant.

Corollaire I.6.10 (Théorème de H.Robbins (1939)). On peut orienter un graphe connexe pour le rendre  $f$ . connexe si et seulement si ce graphe est au moins 2-connexe pour les arêtes.

Remarque I.6.11. Sur la figure I.49, le multi-graphe est 2-connexe pour les arêtes et pourtant, il suffit d'enlever un seul sommet pour le disconnector. Avec nos notations,  $\kappa(G) = 1$  et  $\lambda(G) = 2$ .

![img-57.jpeg](img-57.jpeg)
FIGURE I.49. Un multi-graphe tel que  $\lambda (G) = 2$  et  $\kappa (G) = 1$

De plus, les nombres  $\lambda(G)$  et  $\kappa(G)$  peuvent être très différents. En effet, pour le graphe de la figure I.50, on a  $\kappa(G) = 1$  et  $\lambda(G) = k$ .

I.6. Coupes, points d'articulation,
k
-connexité

![img-58.jpeg](img-58.jpeg)
FIGURE I.50. Un multi-graphe tel que  $\lambda (G) = k$  et  $\kappa (G) = 1$

Mème dans le cas d'un graphe simple, il n'existe pas de lien direct entre  $\lambda(G)$  et  $\kappa(G)$ . On s'en convainc en considérant deux copies de  $K_{n}$  comme illustré à la figure I.51.

![img-59.jpeg](img-59.jpeg)
FIGURE I.51. Un graphe simple tel que  $\lambda(G) = 4$  et  $\kappa(G) = 1$ .

Enfin, il est clair que nous avons toujours

$$
\lambda (G) \leq \min  _ {v \in V} \deg (v).
$$

En effet, si  $v$  est un sommet de degré  $k$ , il suffit de supprimer les  $k$  arêtes incidentes à  $v$  pour isoler  $v$  des autres sommets du graphe. Signalons aussi un théorème de Whitney (1932),

$$
\kappa (G) \leq \lambda (G).
$$

Il semble évident que只不过 que de supprimer une arête, il suffirait d'en supprimer au plus une extrémité.

Remarque I.6.12. Si  $F$  est une coupure d'un graphe  $G$  connexe, alors, de par la minimalité de  $F$ ,  $G - F$  possède exactement deux composantes connexes et l'ensemble des sommets de  $G$  est donc partitionné en deux sous-ensembles correspondant à ces deux composantes.

Terminons cette section par une dernière définition.

Définition I.6.13. Une clique d'un graphe non orienté et simple  $G = (V, E)$  est un sous-graphe complet de  $G$ . La taille d'une clique est le nombre de sommets qui la composent. La taille maximale d'une clique de  $G$  est notée  $\omega(G)$ . Comme nous le verrons plus loin, les nombres  $\alpha(G)$  définis en page 37 et  $\omega(G)$  sont deux paramètres importants d'un graphe.

Chapitre I. Premier contact avec les graphes

![img-60.jpeg](img-60.jpeg)
FIGURE I.52. Une clique de taille 4,  $\omega (G) = 4$

# 7. Théorème(s) de Menger

Nous nous plaçons ici dans le cas de graphes simples non orientés. Comme le lecteur pourra s'en convaincre, il n'y a aucune différence à considérer le cas de multi-graphes. La notion définie ci-dessous est proche de la notion d'ensemble d'articulation.

Definition I.7.1. Soient un graphe  $G = (V, E)$  et  $u, v$  deux sommets distincts de  $G$ . Un sous-ensemble  $S \subseteq V \setminus \{u, v\}$  sépare  $u$  et  $v$  s'il n'existe aucun chemin joignant  $u$  et  $v$  dans le sous-graphe de  $G$  induit par  $V \setminus S$ .

Definition I.7.2. Deux chemins joignant  $u$  et  $v$  sont indépendants si les seuls sommets qu'ils ont en commun sont  $u$  et  $v$ .

Théorème I.7.3 (Menger (1927)). Soient  $u, v$  deux sommets non adjacents d'un graphe connexe  $G = (V, E)$ . La taille minimum d'un sous-ensemble de sommets séparant  $u$  et  $v$  est égale au nombre maximum de chemins deux à deux indépendants joignant  $u$  et  $v$ .

Démonstration. La preuve de ce résultat est assez longue. Dans ce cours introductif, nous ne faisons que l'esquisser sommairement. Si un sous-ensemble  $S \subset V$  sépare  $u$  et  $v$ , alors tout chemin joignant  $u$  et  $v$  passée nécessairement par un sommet de  $S$ . On en conclus que  $\# S$  majoré le nombre de chemins indépendants joignant  $u$  et  $v$ .

La seconde partie de la preuve consiste à montré par récurrence sur  $\# E + \# V$  que si un ensemble  $S \subset V$  de taille minimum sépare  $u$  et  $v$ , alors le nombre de chemins indépendants joignant  $u$  et  $v$  vaut au moins  $\# S$ .

Corollaire I.7.4 (Menger (1927)). Soit  $k \geq 2$ . Un graphe  $G = (V, E)$  est au moins  $k$ -connexe (pour les sommets) si et seulement si toute paire de sommets distincts de  $G$  est connectée par au moins  $k$  chemins indépendants.

Démonstration. Il est clair $^{25}$  que si toute paire de sommets est connectée par au moins  $k$  chemins indépendants, alors  $\kappa(G) \geq k$ .

La condition est nécessaire. Procedons par l'absurde. Supposons que  $\kappa(G) \geq k$  mais qu'il existe deux sommets  $u$  et  $v$  joints par au plus  $k - 1$

I.7. Théorème(s) de Menger

![img-61.jpeg](img-61.jpeg)

![img-62.jpeg](img-62.jpeg)
FIGURE I.53. Une illustration du théorème de Menger: 3 chemins indépendants joignent  $u$  et  $v$ , 3 sommets séparant  $u$  et  $v$ .

![img-63.jpeg](img-63.jpeg)

chemins indépendants. Au vu du théorème précédent, on en conclus que  $u$  et  $v$  sont adjacents, i.e., il existe une arête  $e = \{u,v\} \in E$ . Dans le graphe  $G - e$ ,  $u$  et  $v$  sont joints par au plus  $k - 2$  chemins indépendants. Puisque  $u$  et  $v$  ne sont pas adjacents dans  $G - e$ , on tire du théorème précédent qu'ils peuvent être séparés, dans  $G - e$ , par un ensemble  $S$  de taille minimale ne dépassant pas  $k - 2$  sommets, i.e.,  $\# S \leq k - 2$ .

Puisque  $\kappa(G) \geq k$ , cela implique en particulier que  $\# V &gt; k$ . Il existe donc un sommet  $w$  n'appartenant pas à  $S \cup \{u, v\}$ . Dans  $(G - e) - S$ , il ne peut y avoir simultanément deux chemins joignant  $w$  respectivement à  $u$  et à  $v$  car sinon on disposerait dans  $(G - e) - S$  de chemins joignant  $u$  à  $w$  et  $w$  à  $v$  et on pourrait en conclude que  $u$  et  $v$  ne sont pas séparés par  $S$ . Supposons dès lors qu'aucun chemin ne joint  $w$  et  $u$  dans  $(G - e) - S$ . L'ensemble  $S \cup \{v\}$  possède (au plus)  $k - 1$  éléments et sépare  $w$  et  $u$  dans  $G$ . Ceci contredit le fait que  $\kappa(G) \geq k$ .

Remarque I.7.5. Les résultats énoncés dans cette section concernent des propriétés de connexité relatives aux sommets d'un graphe. Il existe l'analogue de ces résultats en termes d'arêtes: Un graphe est au moins  $k$ -connexe

Chapitre I. Premier contact avec les graphes

![img-64.jpeg](img-64.jpeg)
FIGURE I.54. Une illustration de la preuve corollaire de Menger.

pour les arêtes si et seulement si toute paire de sommets est connectée par au moins  $k$  chemins ne partageant aucune arête.

# 8. Graphes orientés sans circuit et tri topologique

Dans cette courte section, on considère un graphe simple orienté et on désire tester algorithmiquement si celui-ci possède ou non un cycle. Comme nous le verrons bientôt, pouvoir répondre à cette question est une des étapes permettant de decide si un graphe donné possède une structure d'arbre. Lorsqu'on prend en compte cette question, il est inutile de considérer un graphe possédant des arcs multiples (cela ne change rien à l'existence d'un cycle) ou des boucles aux sommets. C'est pour cette raison que nous nous restreignons au cas de graphes simples. Comme application, nous allons, en fin de section, reconsiderer le problème général du tri topologique sommairementprésenté à l'exemple I.3.15.

Voici tout d'abord une condition nécessaire pour qu'un graphe soit sans circuit.

Lemma I.8.1. Si un graphe simple orienté  $G = (V, E)$  est sans cycle, alors il existe un sommet  $v$  tel que  $d^{-}(v) = 0$  (resp.  $d^{+}(v) = 0$ ).

Démonstration. Considérons un chemin simple  $(x_{1},\ldots ,x_{k})$  de  $G$  de longueur maximale déterminé par des sommets de  $G$  (autrement dit, ce chemin passé par des sommets distincts et il n'est pas possible d'avoir un chemin passant par plus de sommets). Si  $d^{-}(x_1) &gt; 0$ , alors il existe  $y\in \operatorname {pred}(x_1)$ . Si  $y$  était égal à un des  $x_{j}$ , on aurait alors un cycle  $(y,x_{1},\dots ,x_{j})$ , ce qui est impossible par hypothèse. Or par maximalité du chemin  $(x_{1},\ldots ,x_{k})$ , il n'est pas possible d'avoir un sommet distinct des  $x_{j}$  et tel que  $(y,x_{1})\in E$ .

On peut en déduire une condition nécessaire et suffisante pour qu'un graphe soit sans circuit.

I.8. Graphes orientés sans circuit et tri topologique

Proposition I.8.2. Soit  $G = (V, E)$  un graphe simple orienté. Ce graphe est sans cycle si et seulement si il existe un sommet  $v$  tel que  $d^{-}(v) = 0$  et pour tout sommet  $v$  tel que  $d^{-}(v) = 0$ , le graphe  $G - v$  est sans cycle.

Démonstration. La condition est nécessaire. Si  $G$  est sans cycle, on peut appliquer le lemme précédent. De plus, tout sous-graphe  $G - v$  d'un graphe sans cycle  $G$  est bien sur sans cycle.

La condition est suffisante. Soit  $v$  un sommet tel que  $d^{-}(v) = 0$ . Par hypothèse,  $G - v$  est sans cycle. Par conséquent, si le graphe  $G$  possède un cycle, ce dernier doit nécessairement passer par  $v$ . Si un cycle passé par  $v$ , on en conclus que  $d^{-}(v) \geq 1$ . C'est impossible, donc  $G$  est dépourvu de cycle.

Ce résultat nous fournit un premier algorithme permettant de decide si un graphe est sans cycle.

Algorithm 1.8.3. La donnée fournie à cet algorithme est un graphe simple orienté  $G = (V, E)$ .

```txt
Tant qu'il existe  $v \in V$  tel que  $d^{-}(v) = 0$ ,  $G := G - v$
```

```txt
Si  $G = \emptyset$  alors sortie : "oui, G sans cycle" sinon sortie : "non, G possède un cycle"
```

Remarque I.8.4. Si on implémente cet algorithme à l'aide de listes d'adjacence (cf. chapitre V), la détction d'un sommet  $v$  pour lequel  $d^{-}(v) = 0$  nécessite de parcourir l'ensemble du graphe. Un tel parcours est effectué à chaque étape de la boucle. Cela a pour conséquence de fournir un algorithme dont la complexité est quadratique en fonction de  $\# E + \# V$  et celle-ci peut être améliorée.

Théorème I.8.5. Soit  $G = (V, E)$  un graphe simple orienté. Le graphe  $G$  est sans cycle si et seulement si il est possible d'énumérer26 les sommets de  $V = \{v_1, \ldots, v_n\}$  de manière telle que, pour tout  $i = 1, \ldots, n$ , le demi-degré entrant de  $v_i$  restreint au graphe  $G_i = G - v_1 - \dots - v_{i-1}$  soit nul, ce que l'on note  $d_{G_i}^- (v_i) = 0$ .

Démonstration. La condition est nécessaire. Supposons  $G$  sans cycle. Ainsi, par le lemme I.8.1, il existe un sommet  $v_{1}$  de  $G = G_{1}$  tel que  $d^{-}(v_{1}) = d_{G_{1}}^{-}(v_{1}) = 0$ . D'après la proposition précédente,  $G_{1} - v_{1} = G_{2}$  est sans cycle. Ainsi, il existe un sommet  $v_{2}$  tel que  $d_{G_{2}}^{-}(v_{2}) = 0$ . On continue de la sorte de proche en proche et on obtient l'énumération proposée.

La condition est suffisante. Supposons disposer d'une énumération des sommets ayant les propriétés indiquées. Procedons par récurrence. Le

Chapitre I. Premier contact avec les graphes

graphe  $G_{n}$  est restreint à l'une somme  $v_{n}$  et est donc sans cycle. Le graphe  $G_{n-1}$  contient les sommets  $v_{n}$  et  $v_{n-1}$ . De plus,  $d_{G_{n-1}}^{-}(v_{n-1}) = 0$ . En d'autres termes,  $G_{n-1}$  possède au moins un arc de  $v_{n-1}$  à  $v_{n}$ . Il est donc sans cycle. Appliquons ce raisonnement pour une étape  $i$  quelconque. Si le graphe  $G_{i+1}$  est sans cycle, alors le graphe  $G_{i}$  se compose du graphe  $G_{i+1}$  auquel on ajoute  $v_{i}$  et évientuèlement des arcs de  $v_{i}$  vers les sommets de  $G_{i+1}$ . On en conclus que  $G_{i}$  est sans cycle.

On déduit de ce résultat un algorithme efficace.

Algorithm 1.8.6. La donnée fournie à cet algorithme est un graphe simple orienté  $G = (V, E)$ .

```txt
Pour tout  $v\in V$  ,initialiser  $\mathsf{d}(v) = 0$
Pour tout  $v\in V$  Pour tout  $w\in \mathrm{succ}(v)$ $\mathsf{d}(w) = \mathsf{d}(w) + 1$
aTraiter  $\coloneqq$  0
nbSommet  $\coloneqq 0$
Pour tout  $v\in V$  Si  $\mathsf{d}(v) = 0$  ,alors aTraiter=aTraiterU{v} nbSommet:=nbSommet+1
Tant que aTraiter  $\neq \emptyset$  ,faire Soit  $v$  ,le premier element de aTraiter aTraiter  $\coloneqq$  aTraiter  $\backslash \{v\}$  Pour tout  $w\in \mathrm{Succ}(v)$  ,faire  $\mathsf{d}(w) = \mathsf{d}(w) - 1$  si  $\mathsf{d}(w) = 0$  ,alors aTraiter=aTraiterU{w} nbSommet  $\coloneqq$  nbSommet+1
Si nbSommet  $\equiv$  #V alors sortie : "oui,  $G$  sans cycle" sinon sortie : "non,  $G$  possède un cycle"
```

La variable d associée à chaque sommet du graphe permet de stocker le demi-degré entrant du sommet (par rapport au graphe envisagé au moment de la construction). A chaque fois qu'un élément  $v$  est enlevé de la liste aTraiter, celui-ci est énuméré. Ainsi, on énumère d'abord les sommets de demi-degré entrant nul. Ensuite, lorsqu'un sommet est traité, on le supprime du graphe et on modifie en conséquence les demi-degrés entrants. Si tous les sommets ont été traités, cela correspond à dire que tous les sommets ont été énumérés. Au vu du théorème précédent, on en conclus que le graphe est sans cycle.

Définition I.8.7. Soit  $G = (V, E)$  un graphe simple orienté. Un tri topologique de  $G$  est une énumération  $v_1, \ldots, v_n$  des sommets de  $G$  de manière telle que si  $(v_i, v_j)$  est un arc de  $G$ , alors  $i &lt; j$ .

I.9. Arbres

**Théorème I.8.8.** Un graphe simple orienté admet un tri topologique si et seulement si il est sans cycle.

**Démonstration.** Il est clair que si un graphe possède un cycle, alors quelle que soit l’énumération de ses sommets, il ne peut s’agir d’un tri topologique.

Si un graphe est sans cycle, alors une énumération de ses sommets donnant lieu à un tri topologique est immédiatement donnée par le théorème I.8.5.

---

**Remarque I.8.9.** Il n’y a pas qu’un seul tri topologique pour un graphe donné $G = (V, E)$. En effet, si on dénote par

$$
S(G) = \{v \in V \mid d^{-}(v) = 0\}
$$

l’ensemble des sources de $G$, alors l’ensemble des tris topologiques de $G$ est donné par la formule récursive suivante

$$
\Pi(G) = \bigcup_{v \in S(G)} \{v.\sigma \mid \sigma \in \Pi(G - v)\}
$$

où $\sigma$ est un tri topologique de $G - v$ et où $v.\sigma$ désigne l’énumération des sommets de $G$ en débutant par $v$ puis en suivant l’énumération prescrite par $\sigma$.

---

## 9. Arbres

Les graphes connexes acycliques jouent un rôle prépondérant dans diverses applications (cf. l’exemple I.3.8 ou encore de nombreux exemples tirés de l’informatique : arbres binaires de recherche, bases de données, etc...).

**Définition I.9.1.** Un graphe simple non orienté $A = (V, E)$ est un arbre s’il est connexe et sans cycle (sous-entendu, un “véritable” cycle : une piste fermée, pas un cycle “artificiel” comme un trivial ($\{a, b\}, \{b, a\}$) qui est un cycle de longueur 2; le fait d’imposer l’absence d’une piste fermée évite de telles arêtes doublées et on pourrait de manière équivalente imposer l’absence de circuit simple). Une forêt est un graphe simple non orienté dont chaque composante connexe est un arbre. Un arbre $A = (V, E)$ est qualifié de $n$-aire si pour tout sommet $v \in V$, $\deg(v) \leq n$.

---

**Proposition I.9.2.** Soit $G = (V, E)$ un arbre ayant $n$ sommets.

- Toute paire de sommets distincts de $G$ est connectée par exactement un chemin simple.
- Soit $e \in (V \times V) \setminus E$ qui n’est pas une boucle. Le graphe $G + e$ contient un cycle (i.e., une piste fermée), c’est-à-dire, $G + e$ n’est plus un arbre.
- Le graphe $G$ a exactement $n - 1$ arêtes.

**Démonstration.** C’est immédiat.

Chapitre I. Premier contact avec les graphes

![img-65.jpeg](img-65.jpeg)
FIGURE I.55. Un arbre.

On pourra en particulier observer que tout arbre est 1-connexe, la réciropque étant bien évidemment fausse.

Proposition I.9.3. Un graphe  $G = (V, E)$  simple connexe est un arbre si et seulement si chacune de ses arêtes est une arête de coupure.

Démonstration. Soient  $G$  un arbre et  $e$  une de ses arêtes. Puisque  $G$  est sans cycle, il ne possède aucune piste fermée. En appliquant la proposition I.6.6, on en déduit que  $e$  est une arête de coupure. Inversement, si  $G$  est un graphe connexe possédant une piste fermée alors, par la proposition I.6.6, les arêtes de cette piste ne peuvent être des arêtes de coupure.

Corollaire I.9.4. Tout graphe connexe possède un sous-arbre couvrant.

Démonstration. Soient  $G = (V, E)$  un graphe connexe et  $C = (V, E')$  un sous-graphe couvrant connexe minimal (i.e., on ne peut pas replacer  $E'$  par un sous-ensemble strict tout en conservant la connexité de  $C$ ). Vu la minimalité de  $C$ , chacune de ses arêtes est une arête de coupure de  $C$ . Par la proposition précédente, on en conclus que  $C$  est un arbre.

Corollaire I.9.5. Si  $G = (V, E)$  est un graphe (simple non orienté) connexe, alors  $\# E \geq \# V - 1$ .

Démonstration. Par le corollaire précédent,  $G$  possède un sous-arbre couvrant  $C = (V, E')$ . De là, il vient

$$
\# E \geq \# E ^ {\prime} = \# V - 1
$$

ou pour la dernière égalité, on a utilisé la proposition I.9.2.

Definition I.9.6. Un arbre  $A = (V, E)$  dans lequel on a privilégie un sommet  $v_0$  est appelé arbre pointé $^{27}$ . On le notera  $(A, v_0)$ . Le sommet  $v_0$  est parfois appelé la racine de l'arbre.

I.9. Arbres

Pour un arbre pointé  $(A, v_0)$ , les sommets de  $A$  peuvent être ordonnés suivant leur distance à  $v_0$ . Si  $v$  est un sommet tel que  $d(v_0, v) = i$ , on dira que  $v$  est un sommet de niveau  $i$ . Un arbre pointé a été représenté à la figure I.56.

![img-66.jpeg](img-66.jpeg)
FIGURE I.56. Un arbre pointé.

Si  $v$  est un sommet de niveau  $i$  et si tous ses voisins sont de niveau  $i - 1$ , on dit alors que  $v$  est une feuille de l'arbre. A la figure I.56, les feuilles ont été marquées d'un cercle. La hauteur d'un arbre est le niveau maximal de ses feuilles. L'arbre de la figure I.56 est un arbre de hauteur 4.

Remarque I.9.7. Pointer un arbre définit naturellement une orientation des arêtes de l'arbre. En effet, on peut orienter les arcs de façon à ce qu'ils joignent les sommets de niveau  $i$  aux sommets de niveau  $i + 1$ . Dans ce cadre, on parle souvent des fils (resp. du père) d'un noeud  $v$  pour désigner ses successeurs (resp. son unique prédécesseur). Les descendants (resp. ancêtres) de  $v$  désignent les éléments de  $\mathrm{succ}^* (v)$  (resp.  $\mathrm{pred}^* (v)$ ).

Definition I.9.8. Un arbre pointé est  $k$ -aire si tout sommet a au plus  $k$  fils. Si  $k = 2$ , on parle naturellement d'arbre binaire. Un arbre  $k$ -aire de hauteur  $n$  possède au plus

$$
1 + k + \dots + k ^ {n} = \frac {k ^ {n} - 1}{k - 1}
$$

sommets. S'il en possède exactement ce nombre, on parle d'arbre  $k$ -aire complet.

9.1. Parcours d'arbres. Un parcours d'un arbre est une façon d'en ordonner les noeuds. On supposera implicitement que les fils d'un noeud  $v_{i}$  sont ordonnés  $v_{i,1},\ldots ,v_{i,k_i}$  et que cet ordre est connu et fixé une fois pour toutes.

On désigne trois types de parcours en profondeur: les parcours préfixe, infixe et suffixe. Soit  $(A,v_0)$  un arbre pointé. Pour le parcours préfixe, on parcourt d'abord la racine puis on parcourt, de manière récursive et dans l'ordre, les sous-arbres pointés de racine respective  $v_{0,1},\ldots ,v_{0,k_0}$ . Pour le parcours suffixe, on parcourt d'abord, de manière récursive et dans l'ordre,

Chapitre I. Premier contact avec les graphes

les sous-arbres pointés de racine  $v_{0,1}, \ldots, v_{0,k_0}$ , puis la racine  $v_0$ . Pour le parcours infixe, nous supposerons disposer d'un arbre binaire. (On peut donc parler du sous-arbre de gauche et du sous-arbre de droite.) On parcourt d'abord, de manière récursive, le sous-arbre de gauche, puis la racine, et enfin le sous-arbre de droite $^{28}$ .

Exemple I.9.9. Considerons l'arbre binaire pointé donné à la figure I.57. Les fils d'un noeud sont ordonnés par l'ordre alphabétique de leur label. Si

![img-67.jpeg](img-67.jpeg)
FIGURE I.57. Un arbre à parcourir.

on parcourt cet arbre de manière préfixielle, on obtient la suite :

$a,b,d,g,h,l,c,e,i,f,j,k.$

Pour un parcours suffixe, on a

$g,l,h,d,b,i,e,j,k,f,c,a.$

Enfin, pour un parcours infixe, on obtient

$g,d,l,h,b,a,i,e,c,j,f,k.$

On rencontres parfois des parcours en largeur. Dans ce cas, on parcourt les noeuds de l'arbre pointé par niveau croissant. Sur l'exemple précédent, on a simplement l'ordre  $a, b, c, \ldots, k, l$ .

Remarque I.9.10. On peut également définir un parcours en profondeur d'un graphe quelconque. Bien qu'il ne s'agisse pas d'un problème concernant spécifique les arbres, nous pensons qu'il s'agit du moment opportun pour le définir. Soit  $G = (V,E)$  un graphe (orienté ou non) que l'on peut supposer simple et connexe. Un parcours en profondeur de  $G$  est défini récursivement comme suit. Sélectionner un sommet  $v_{0}$ . A l'étape  $k \geq 1$ , on désit un voisin de  $v_{k-1}$  qui n'a pas encore été sélectionné. Si un tel voisin n'existe pas, oncherche dans l'ordre, un voisin non sélectionné de  $v_{k-2}, \ldots, v_{0}$ .

I.10. Isomorphismes de graphes

![img-68.jpeg](img-68.jpeg)
FIGURE I.58. Exemple de parcours en profondeur d'un graphe.

# 10. Isomorphismes de graphes

Definition I.10.1. Soient  $G_{i} = (V_{i},E_{i})$ ,  $i = 1,2$ , deux digraphes (resp. deux graphes non orientés). Une application  $f:V_{1}\to V_{2}$  est un homomorphism si

$$
(x, y) \in E _ {1} \Rightarrow (f (x), f (y)) \in E _ {2}
$$

$$
(\operatorname {r e s p .} \{x, y \} \in E _ {1} \Rightarrow \{f (x), f (y) \} \in E _ {2}).
$$

On parlera alors d'homomorphisme de  $G_{1}$  dans  $G_{2}$ . Il est clair que la composée d'homomorphismes est encore un homomorphisme.

Example I.10.2. Avec les graphes  $G$  et  $H$  de la figure I.59, on voit facilement qu'on a un homomorphisme de  $G$  dans  $H$  mais pas de  $H$  dans  $G$ . A la figure I.60, on donne un autre exemple d'homomorphisme entre deux

![img-69.jpeg](img-69.jpeg)
FIGURE I.59. Homomorphisme de  $G$  dans  $H$ .

graphes  $G$  et  $H$ . Cela montre que  $f: V_1 \to V_2$  n'est pas nécessairement injectif. Les homomorphismes de graphes réinterviendront dans les questions de coloriage.

![img-70.jpeg](img-70.jpeg)
FIGURE I.60. Homomorphisme de  $G$  dans  $H$ .

Chapitre I. Premier contact avec les graphes

Definition I.10.3. Deux digraphes (resp. deux graphes non orientés)  $G_{i} = (V_{i},E_{i})$ ,  $i = 1,2$ , sont isomorphes s'il existe une bijection  $f:V_{1}\to V_{2}$  qui est telle que

$$
(x, y) \in E _ {1} \Leftrightarrow (f (x), f (y)) \in E _ {2}
$$

(resp. telle que  $\{x,y\} \in E_1\Leftrightarrow \{f(x),f(y)\} \in E_2$ ). Cette définition s'adapte au cas de multi-graphes orientés. Deux multi-graphes  $G_{i} = (V_{i},E_{i})$ ,  $i = 1,2$ , sont isomorphes s'il existe une bijection  $f:V_{1}\to V_{2}$  telle que  $(x,y)$  est un arc de multiplicité  $k$  de  $G_{1}$  si et seulement si  $(f(x),f(y))$  est un arc de multiplicité  $k$  de  $G_{2}$ . Bien évidemment, une telle application  $f$  est qualifiée d'isomorphisme de graphes. Bien sur, si  $f$  est un isomorphisme, il en va de même pour  $f^{-1}$ .

Example I.10.4. Voici deux graphes isomorphes représentés à la figure I.61. On a un isomorphisme donné par

$$
\varphi : a \mapsto 4, b \mapsto 5, c \mapsto 6, d \mapsto 1, e \mapsto 2, f \mapsto 1.
$$

![img-71.jpeg](img-71.jpeg)
FIGURE I.61. Deux graphes isomorphes.

![img-72.jpeg](img-72.jpeg)

Definition I.10.5. Soit  $G = (V, E)$  un graphe (orienté ou non). Un automorphisme de  $G$  est un isomorphisme de  $G$  dans  $G$ . L'ensemble des automorphismes de  $G$  muni de la loi de composition d'applications forme un groupe, noté  $Aut(G)$ . Il s'agit bien évidemment d'un sous-groupe du groupe symétrique  $S_n$  des permutations de  $n = \# V$  éléments. Un graphe pour lequel  $Aut(G)$  est réduit à l'identité  $id_V$  est qualifié d'asymétrique.

Example I.10.6. Par exemple,  $Aut(K_n) = S_n$ .

La proposition suivante est immédiate et s'adapte aux autres cas de graphes.

Proposition I.10.7. Soient  $G$  un graphe (simple non orienté) et  $\varphi$  un automorphisme de  $G$ . Pour tous sommets  $u, v$ , on a

$\triangleright \deg (u) = \deg (\varphi (u))$
$\triangleright \mathrm{d}(u,v) = \mathrm{d}(\varphi (u),\varphi (v))$

I.10. Isomorphisms de graphes

Remarque I.10.8. Si deux graphes  $G_{i} = (V_{i},E_{i})$ ,  $i = 1,2$  ont leurs sommets pondérés par  $p_i:V_i\to \Sigma$ , la définition d'un isomorphisme  $f:V_1\to V_2$  doit naturellement s'étendre en respectant de plus la propriété

$$
p _ {1} (v) = p _ {2} (f (v)), \forall v \in V _ {1}.
$$

Voici un exemple qui concerne des arbres infinis et qui met en lumière la notion d'isomorphisme dans le cas pondéré. Pour simplifier, supposons disposer de deux symboles (ou lettres) notés  $a$  et  $b$ . Avec ces lettres, on peut écrire des mots comme:  $aa$ ,  $bba$ ,  $b$  ou encore  $abbbaabaa$  (i.e., des suites finies de symboles). On construit un arbre binaire infini (i.e., on dispose d'une infinité de noeuds, chaque noeud ayant exactement deux fils) dont les noeuds sont en bijection avec les mots que l'on peut écrire avec les lettres  $a$  et  $b$ . Si un noeud est en bijection avec le mot  $m$ , son fils de gauche (resp. de droite) est en bijection avec  $ma$  (resp.  $mb$ ). La racine de l'arbre correspond au mot n'ayant aucune lettre, le mot vide  $\varepsilon$ . Nous dirons qu'un tel arbre est un arbre lexicographique. Ainsi, cet arbre possède exactement  $2^i$  noeuds de niveau  $i$  et ceux-ci correspondent de gauche à droite aux mots de longueur  $i: \frac{a \cdots aa}{i \times} \cdot a \cdots ab, \ldots, b \cdots ba, \frac{b \cdots bb}{i \times}$ . Une illustration est donnée à la figure I.62.

![img-73.jpeg](img-73.jpeg)
FIGURE I.62. Un arbre lexicographique infini.

Nous n'avons pas encore considéré de pondération des noeuds. Vu la bijection entre noeuds et mots, nous ne les distinguerons plus dans ce qui suit. Si on considère un ensemble  $L$  de mots écrit sur  $\{a,b\}$ , alors on définit la fonction  $p_L$  qui à un mot  $m$  associe 1 (resp. 0) si  $m \in L$  (resp.  $m \notin L$ ). Autrement dit, la pondération est simplement un codage définissant le dictionnaire des mots de  $L$ . Si on représentée par un rond noir les mots appartenant à  $L$  (i.e., tel que  $p_L(m) = 1$ ), on a, pour le langage formé des mots commençant par un nombre arbitraire de  $a$  (éventuellement aucun) et suivi par un nombre arbitraire de  $b$  (éventuellement aucun), l'arbre pondéré  $A_L$  repris à la figure I.63.

Chapitre I. Premier contact avec les graphes

![img-74.jpeg](img-74.jpeg)
FIGURE I.63. Un arbre infini pondéré représentant le dictionnaire de  $A_L$ .

Pour tout mot  $m$ , on peut définir l'arbre  $A_{m}$  comme le sous-arbre obtenu à partir de  $A_{L}$  en considérant comme nouvelle racine le noeud  $m$  et en ne conservant dans  $A_{m}$  que les descendants de  $m$  ( $m$  inclus). On peut se convaincre que l'arbre  $A_{L}$  ne possède, à isomorphisme près, que 3 sous-arbres non isomorphes (par exemple,  $A_{L}$  lui-même,  $A_{b}$  et  $A_{ba}$ ). Un arbre  $A_{L}$  ayant une telle propriété (nombre fini de sous-arbres non isomorphes $^{29}$ ) est qualifié de régulier.

Par exemple, tout ensemble fini de mots donne lieu à un arbre régulier et l'ensemble  $M$  des mots de la forme

$$
a ^ {i} b ^ {i} := \underbrace {a \cdots a} _ {i \times} \underbrace {b \cdots b} _ {i \times}, \quad \forall i \in \mathbb {N},
$$

donne quant à lui un arbre  $A_{M}$  non régulier (cf. figure I.64). En effet, les

![img-75.jpeg](img-75.jpeg)
FIGURE I.64. Un arbre infini pondéré représentant le dictionnaire de  $A_{M}$ .

arbres  $A_{\varepsilon}, A_{a}, A_{aa}, A_{aaa}, \ldots$  sont tous distincts (on remarque que, dans  $A_{a^i}$ , le premier niveau sur lequel on rencontres un noeud noirci est le  $i$ -ème).

# 11. Graphes hamiltoniens

Nous avons vu dans la section 4.2 comment déterminer si un graphe  $G$  possédait un chemin eulérien passant une et une seule fois par chaque arête/arc de  $G$ . On peut aussi s'intéresser à la question posée initialement

I.11. Graphes hamiltoniens

par Sir W. R. Hamilton qui concerne cette fois l'existence d'un circuit passant une et une seule fois par chaque sommet $^{30}$  de  $G$ . Il est frappant de constater qu'il n'existe, à ce jour, pas de méthode efficace $^{31}$  permettant de répondre à cette question. Un chemin (resp. circuit) passant une et une seule fois par chaque sommet de  $G$  sera qualifié d'hamiltonien. Un graphe hamiltonien est un graphe possédant un circuit hamiltonien $^{32}$ .

Exemple I.11.1. On considère un dodécaèdre régulier (polyèdre régulier possédant 12 faces pentagonales et 20 sommets). Le graphe associé, ou squelette, (on procède comme à l'exemple I.3.3) est représenté à la figure I.65. La question originale posée par Hamilton était de déterminer un circuit passant une et une seule fois par chaque arête de ce graphe.

![img-76.jpeg](img-76.jpeg)
FIGURE I.65. Squelette d'un dodécaèdre et circuit hamiltonien.

![img-77.jpeg](img-77.jpeg)

Exemple I.11.2. Sur un échiquier, le cavalier se déplace de deux cases dans une direction (horizontale ou verticale) puis d'une case dans une direction orthogonale. Est-il possible qu'un cavalier passé par toutes les cases de l'échiquier sans passer deux fois par la même case et revenir à son point de départ. Il s'agit encore de la recherche d'un circuit hamiltonien. On peut considérer un graphe dont les sommets représentent les cases de l'échiquier et une arête joint deux sommets si et seulement si un cavalier peut passer de l'une des cases correspondantes à la suivante en un mouvement. (Ainsi,

Chapitre I. Premier contact avec les graphes

le degré de chaque sommet varie entre 2 et 8). On peut répondre affirmativement à cette question comme le prouve la figure I.66.

![img-78.jpeg](img-78.jpeg)
FIGURE I.66. Déplacement d'un cavalier.

![img-79.jpeg](img-79.jpeg)

Voici une première condition nécessaire pour qu'un graphe soit hamiltonien.

Proposition I.11.3. Si  $G = (V, E)$  est un graphe (simple et non orienté) hamiltonien, alors pour tout ensemble non vide  $S \subseteq V$ , le nombre de composantes connexes de  $G - S$  est inférieur ou égal à #S.

Démonstration. Soit  $S$  un sous-ensemble non vide de  $V$  et  $u$  un sommet de  $S$ . Considérons un circuit hamiltonien  $C$  passant par  $u$ . On peut voir  $C$  comme une suite ordonnée de sommets  $(u, w_1, \ldots, w_{n-1}, u)$ . Soient  $G_1, \ldots, G_k$ , les  $k$  composantes connexes de  $G - S$ . Si  $k = 1$ , le résultat est trivial. Supposons donc  $k &gt; 1$ . Pour  $i = 1, \ldots, k$ , désignons par  $u_i$  le dernier sommet de  $G_i$  dans le circuit  $C$  et  $v_i$ , le sommet suivant  $u_i$  dans ce même circuit. Les sommets  $v_1, \ldots, v_k$  appartiennent à  $S$ . En effet, on a un cycle passant par tous les sommets du graphe et on sait que la suppression des sommets de  $S$  disconnect le graphe en  $k$  composantes. Par définition de

![img-80.jpeg](img-80.jpeg)
FIGURE I.67. Une illustration de la preuve de la proposition I.11.3.

I.11. Graphes hamiltoniens

ces composantes et des sommets  $u_{i}$  et  $v_{i}$ , on en conclus $^{33}$  que  $v_{1}, \ldots, v_{k}$  sont des sommets distincts de  $S$  et donc  $\# S \geq k$ .

Exemple I.11.4. Ce premier résultat peut être utilisé pour vérifier que certains graphes ne sont pas hamiltoniens. Par exemple, le graphe de la figure I.68 ne vérifie pas la condition de la proposition précédente pour l'ensemble  $S$  constitué des deux sommets encerclés. En effet, en supprimant ces deux sommets, on obtient trois composantes connexes.

![img-81.jpeg](img-81.jpeg)
FIGURE I.68. Un graphe non hamiltonien.

Par contre, cette condition n'est pas suffisante. En effet, le graphe de Petersen (figure I.69) la vérifie et pourtant, ce dernier n'est pas hamiltonien (les vérifications sont laissées au lecteur). Le graphe de Petersen est souvent utilisé comme contre-exemple classique en théorie des graphes pourmettre en défaut certaines propriétés non généralisables.

![img-82.jpeg](img-82.jpeg)
FIGURE I.69. Le graphe de Petersen.

On dispose de plusieurs conditions suffisantes (theorèmes de Dirac, de Chvátal, ou encore de Chvátal-Erdős) pour assurer qu'un graphe est hamiltonien. On supposera toujours le graphe simple et non orienté. La première de ces conditions est une condition locale, elle exprime que si chaque sommet a suffisamment de voisins, alors le graphe est hamiltonien. Pour rappel, la

Chapitre I. Premier contact avec les graphes

partie entière par excès (resp. par défaut) d'un réel  $x$  est  $\lceil x \rceil = \inf \{y \in \mathbb{Z} \mid y \geq x\}$  (resp.  $\lfloor x \rfloor = \sup \{y \in \mathbb{Z} \mid y \leq x\}$ ).

Théorème I.11.5 (Dirac). Tout graphe  $G$  (simple et non orienté) ayant  $n \geq 3$  sommets et tel que le degré de chaque sommet est au moins égal à  $n/2$ , possède un circuit hamiltonien.

Démonstration. Notons  $\delta = \inf_{v\in V}\deg (v)$ . Puisque  $\delta \geq \lceil n / 2\rceil$ , on en conclus que  $G$  est connexe. En effet, si ce n'était pas le cas,  $G$  possèderait au moins deux composantes connexes distinctes dont une contient  $\leq \lfloor n / 2\rfloor$  sommets $^{34}$ . Or si  $\delta \geq \lceil n / 2\rceil$ , alors un des sommets de cette composante doit être voisin d'un sommet d'une autre composante, ce qui n'est pas possible (vu la maximalité des composantes connexes).

Soit  $C$  un chemin de longueur maximale passant par des sommets tous distincts (i.e., un chemin simple). Puisque  $G$  est simple,  $C$  est entièrement défini par une suite de sommets  $x_0, \ldots, x_k$ . Par maximalité, tous les voisins de  $x_0$  et de  $x_k$  appartiennent à  $C$  (en effet, sinon, on pourrait étendre  $C$  en un chemin plus long). Par conséquent, au moins  $\lceil n/2 \rceil$  sommets parmi  $x_0, \ldots, x_{k-1}$  sont voisins de  $x_k$  et au moins  $\lceil n/2 \rceil$  sommets parmi  $x_1, \ldots, x_k$  sont voisins de  $x_0$ . De plus,  $k+1 \leq n$  (ou encore  $k &lt; n$ ). Par conséquent $^{35}$ , il existe un indice  $i &lt; k$  tel que

$$
\{x _ {0}, x _ {i + 1} \} \in E \quad \text {e t} \quad \{x _ {i}, x _ {k} \} \in E.
$$

Ainsi,

![img-83.jpeg](img-83.jpeg)

$x_0,x_1,\ldots ,x_i,x_k,x_{k - 1},\ldots ,x_{i + 2},x_{i + 1},x_0$

est un circuit passant une seule fois par chacun des sommets  $x_0, \ldots, x_k$ . Ce circuit est hamiltonien. En effet, supposons qu'il existe un sommet  $y$  n'appartenant pas à ce circuit. Puisque  $G$  est connexe, il existe  $j \in \{0, \ldots, k\}$  tel que  $\{x_j, y\} \in E$ . De là, on peut construire un chemin passant par  $k + 2$  sommets distincts. C'est impossible vu le choix de  $C$  (de longueur maximale).

I.11. Graphes hamiltoniens

![img-84.jpeg](img-84.jpeg)

![img-85.jpeg](img-85.jpeg)

Le théorème suivant, premier théorème d'Ore, sera utilisé pour prouver le théorème de Chvátal (condition suffisante à ce jour la plus aboutie pour assurer qu'un graphe soit hamiltonien).

En particulier, l'idée d'adjoindre des arêtes à un graphe va nous fournir, à la section suivante, la notion importante de fermeture d'un graphe. On peut d'ores et déjà remarquer que l'adjonction d'une arête  $\{x,y\}$  est réalisée quand  $\deg (x) + \deg (y)\geq \# V$

Enfin, nous attendrons la formulation du corollaire I.11.7, le deuxième théorème d'Ore, pour obtenir une condition suffisante assurant le caractère hamiltonien plus générale que celle donnée par le théorème de Dirac.

Théorème I.11.6 ("premier" Théorème d'Ore). Soient  $G = (V, E)$  un graphe (simple et non orienté) ayant  $n \geq 3$  sommets et  $x$  et  $y$  deux sommets tels que  $\deg(x) + \deg(y) \geq n$ . Le graphe  $G$  est hamiltonien si et seulement si le graphe  $G + \{x, y\}$  l'est.

Démonstration. Si l'arête  $e = \{x, y\}$  appartient à  $E$ , il n'y a rien à démontré. Nous allons donc supposer que cette arête n'appartient pas à  $G$ . De plus, la condition est triviallement nécessaire.

Supposons donc que le graphe  $G + \{x, y\}$  possède un circuit hamiltonien  $C$  passant par l'arête  $e = \{x, y\}$ . En effet, si le circuit hamiltonien ne passse pas par  $e$ , il n'y a rien non plus à démontré. Ainsi, le circuit  $C$  peut s'écrire comme une suite ordonnée de sommets (tous distincts exceptées les deux copies de  $x$ )

$$
C = (v _ {1} = x, v _ {2}, \dots , v _ {n} = y, x).
$$

Il existe  $i$  tel que  $1 &lt; i &lt; n$ ,  $\{x, v_i\} \in E$  et  $\{v_{i-1}, y\} \in E$ : c'est une conséquence de l'hypothèse  $\deg(x) + \deg(y) \geq n$  (et c'est le même genre d'argument $^{36}$  que dans le théorème de Dirac). On en déduit (cf. figure I.70) que

$$
\left(v _ {i}, v _ {i + 1}, \dots , y, v _ {i - 1}, v _ {i - 2}, \dots , x, v _ {i}\right)
$$

est un circuit hamiltonien dans  $G$  (puisqu'il ne fait plus intervenir l'arête  $e$ ).

On utilise ce résultat pour démontré le théorème I.11.12.

Chapitre I. Premier contact avec les graphes

![img-86.jpeg](img-86.jpeg)
FIGURE I.70. Théorème d'Ore, circuit dans  $G$ .

Nous verrons rapidement que ce théorème permet d'obtenir facilement le résultat suivant (nous pourrions d'ailleurs en donner une preuve directement mais pour éviter les redondances, nous la postposons).

Corollaire I.11.7 ("deuxieme" Théorème d'Ore). Soit  $G = (V, E)$  un graphe (simple et non orienté) ayant  $n \geq 3$  sommets. Si pour tout couple de sommets non adjacents  $(x, y)$ , on a  $\deg(x) + \deg(y) \geq n$ , alors  $G$  est hamiltonien. En particulier, si  $\min_{v \in V} \deg(v) \geq n/2$ , alors  $G$  est hamiltonien.

Remarque I.11.8. On peut observer que le théorème de Dirac est un corollaire immédiat du deuxième théorème d'Ore. En effet, si pour chaque sommet  $\deg(v) \geq n/2$ , alors pour toute paire de sommets  $x$  et  $y$ , on a triviallement  $\deg(x) + \deg(y) \geq n$ .

Remarque I.11.9. Il est à noter que chronologiquement, c'est d'abord le théorème de Dirac qui fut obtenu (1952), puis celui d'Ore (1960) et enfin celui de Chvátal (1971).

11.1. Fermeture d'un graphe et théorème de Chvátal. Introduisons tout d'abord la fermetre d'un graphe simple et non orienté. Soit  $G_0 = (V_0, E_0)$  un graphe simple et non orienté. On définit une suite finie

$$
G _ {0}, G _ {1}, \dots , G _ {i} = (V _ {i}, E _ {i}), \dots , G _ {k}
$$

de graphes (simples) de la manière suivante. Pour tout  $i$ , on ajoute à  $G_{i}$  une arête comme suit:

$$
G _ {i + 1} = G _ {i} + \{u, v \}
$$

ou  $u$  et  $v$  sont des sommets de  $G_{i}$  qui sont tels que  $\{u,v\} \notin E_i$  et

$$
\deg_ {G _ {i}} (u) + \deg_ {G _ {i}} (v) \geq \# V
$$

ou  $\deg_{G_i}$  désigne bien sur le degré d'un sommet dans le graphe  $G_i$ . La procédure s'arrête lorsqu'il n'y a plus moyen d'ajouter de nouvelles arêtes à  $G_k$ . Ainsi, pour tous sommets  $u, v$ , soit  $\{u, v\}$  appartient à  $E_k$ , soit  $\deg_{G_k}(u) + \deg_{G_k}(v) &lt; \# V$ . Le graphe obtenu à la dernière étape s'appelle la fermetre de  $G_0$ . Nous allons tout d'abord montré que quels que soient les choix d'arêtes réalisés dans les étapes intermédiaires, on aboutit toujours au même graphe (ainsi la définition est bien licite) noté  $\mathcal{F}(G_0)$ .

Exemple I.11.10. A la figure I.71, on a considéré deux graphes et leur fermetre. A chaque fois, on a noté le degré des sommets considérés pour l'ajout d'une nouvelle arête. On peut noter que pour le graphe du haut,

I.11. Graphes hamiltoniens

la fermetre donne le graphe complet  $K_{6}$ , alors que dans le second cas, la couverture n'est pas le graphe complet  $K_{5}$ .

![img-87.jpeg](img-87.jpeg)
FIGURE I.71. Construction de la fermetre d'un graphe.

Lemma I.11.11. Pour tout graphe ayant au moins trois sommets, la fermetre d'un graphe est unique.

Démonstration. Supposons que  $G$  possède  $n \geq 3$  sommets et qu'il est possible de construire une fermetre de  $G$  en adjoignant de nouvelles arêtes de deux manières distinctes. Ainsi, supposons que

$$
H = G + \left\{e _ {1}, \dots , e _ {r} \right\} \quad \text {e t} \quad H ^ {\prime} = G + \left\{f _ {1}, \dots , f _ {s} \right\}
$$

sont deux fermetres de  $G$ . Nous allons montré que  $H = H'$ . On note  $H_{i} = G + \{e_{1},\ldots ,e_{i}\}$  et  $H_{i}^{\prime} = G + \{f_{1},\ldots ,f_{i}\}$ . On a  $G = H_0 = H_0'$ . Si  $H\neq H'$ , il y a au moins une arête de l'un qui n'est pas dans l'autre. Supposons que  $e_k = \{u,v\}$  est la première arête de  $H$  qui n'appartient pas à  $H^{\prime}$ . (Ainsi,  $e_1,\dots ,e_{k - 1}$  sont des arêtes de  $H^{\prime}$  et  $e_k$  diffère de tous les  $f_{i}$ .) Puisque  $e_k$  est l'arête ajoutée à  $H_{k - 1}$  pour construire  $H_{k}$ , on a que

$$
\deg_ {H _ {k - 1}} (u) + \deg_ {H _ {k - 1}} (v) \geq n.
$$

Il est clair que  $H_{k - 1}$  est un sous-graphe de  $H^{\prime}$ . De là, on en conclus que

$$
\deg_ {H ^ {\prime}} (u) + \deg_ {H ^ {\prime}} (v) \geq n
$$

et donc, l'arête  $e_k$  devra aussi être ajoutée à  $H'$ , une contradiction. Par conséquent,  $H$  est un sous-graphe de  $H'$  et par symétrie, on en tire que  $H = H'$ .

Le théorème suivant est en fait une simple conséquence du (premier) théorème d'Ore.

Théorème I.11.12. Soit  $G$  un graphe (simple et non orienté) ayant au moins trois sommets.

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

I.11. Graphes hamiltoniens

A ce jour, on ne connait pas de condition suffisante plus générale pour qu'un graphe soit hamiltonien. Dans certains cas, on peut obtenir des conditions plus fortes mais en se restreignant à des classes particulières de graphes.

Démonstration. Nous pouvons tout d'abord supposer que $G$ est égal à sa fermeture, $G = \mathcal{F}(G)$. En effet, au vu du théorème I.11.12, un graphe est hamiltonien si et seulement si sa fermeture l'est. De plus, si un graphe $G$ satisfait la condition (1), alors le graphe $G + e$ la satisfait encore (c'est même encore plus "facile" puisque$^{37}$ moins de sommets vont vérifier $\deg(v_k) \leq k$). Nous allons montré qu'avec cette supposition, $G = K_n$.

Procédons par l'absurde et supposons $G \neq K_n$. Soient $u, v$ deux sommets de $G$ tels que $\deg(u) \leq \deg(v)$, $\{u, v\} \notin E$ et tels que $\deg(u) + \deg(v)$ soit maximal (nécessairement $&lt; n$, car sinon l'arête $\{u, v\}$ appartiendrait à $E$). On peut d'abord remarquer que

$$
\deg(u) = i &lt; n/2
$$

(car sinon, $\deg(u) + \deg(v) \geq n$). Soit

$$
A = \{w \in V \mid \{w, v\} \notin E \text{ et } w \neq v\}.
$$

Par notre choix de $u$, $\deg(w) \leq i$ pour tout $w \in A$ (en effet, on a choisi $\deg(u) + \deg(v)$ maximal). De plus, dans $A$, se trouvent tous les sommets distincts de $v$ et non voisins de $v$, ainsi,

$$
\#A = (n - 1) - \deg(v) \geq \deg(u) = i.
$$

On en conclut que dans $G$, il y a au moins $i$ sommets de degré au plus $i$. Si comme dans l'énoncé, on a ordonné les sommets de $G$ par degré croissant, on en conclut que $\deg(v_i) \leq \deg(u) = i$. Par hypothèse, cela entraîne que

$$
\deg(v_{n-i}) \geq n - i. \tag{2}
$$

On effectue à présent le même raisonnement avec

$$
B = \{w \in V \mid \{u, w\} \notin E \text{ et } w \neq u\}.
$$

Dès lors, pour tout $w \in B$,

$$
\deg(w) \leq \deg(v) &lt; n - \deg(u) = n - i
$$

et de plus,

$$
\#B = (n - 1) - \deg(u) = n - 1 - i.
$$

On a donc $n - 1 - i$ sommets de degré $&lt; n - i$. En outre, le sommet $u$ est tel que $\deg(u) &lt; n - i$. En effet, $i + \deg(v) = \deg(u) + \deg(v) &lt; n$ et

$^{37}$Soit $k$ entiers ordonnés par ordre croissant $x_1 \leq x_2 \leq \cdots \leq x_k$. Soit $i \in \{1, \ldots, k\}$. Si on remplace $x_i$ par $x_i + 1$ et que l'on note les éléments réordonnés $x_1' \leq \cdots \leq x_k'$, alors il existe $j \geq i$ tel que $x_i + 1 = x_j'$. Pour tout $t \in \{1, \ldots, k\}$, montrons que $x_t \leq x_t'$ (cela montre donc que si (1) est satisfait, l'ajout d'une arête ne va pas modifier cette propriété). Tout d'abord, si $i = j$, il n'y a rien à démontrer (l'ordre des éléments n'a pas changé). On peut donc supposer $j &gt; i$. Si $t &lt; i$ ou $t &gt; j$, alors $x_t = x_t'$. Si $i \leq t &lt; j$, alors $x_t' = x_{t+1} \geq x_t$. Si $t = j$, alors $x_j' \geq x_{t-1}' = x_t = x_j$.

Chapitre I. Premier contact avec les graphes

$\deg (u)\leq \deg (v)$  et on en tire que  $\deg (u) &lt;   n - i$  . En conclusion, on a donc  $n - i$  sommets de degre  $&lt;  n - i$  et  $\deg (v_{n - i}) &lt;   n - i$  ce qui contredit (2).

Enfin, citons sans démonstration un dernier résultat concernant les graphes hamiltoniens.

Théorème I.11.17 (Chvátal-Erdős). Soient  $G$  un graphe simple et non orienté ayant au moins trois sommets,  $\alpha(G)$  le nombre maximal de sommets indépendants et  $\kappa(G)$  la taille minimale d'un ensemble d'articulation. Si  $\kappa(G) \geq \alpha(G)$ , alors  $G$  est hamiltonien.

Démonstration. Voir par exemple, R. Diestel, page 277.

Terminons cette section par le résultat suivant représentant un certain intérêt pour le problème du voyageur de commerce. Supposons disposer de  $n$  villes et de connections entre toutes ces villes. On supposera que tous les coûts de connexion d'une ville à une autre sont identiques (ainsi, toute permutation des  $n$  villes fournit une solution optimale). On est donc en présence du graphe complet  $K_{n}$ . Le voyageur de commerce désire passer une et une seule fois par chacune des villes et revenir à son point de départ (le siège de sa compagnie par exemple). Il est clair qu'une permutation quelconque des  $n - 1$  sommets du graphe (siège de la compagnie exclu) répond à la question. Cependant, on peut imaginer qu'il ne désire pas repasser par une route déjà empruntée précédemment. De cette manière, il pourra joindre l'utile à l'agréable en visitant la région de diverses manières. Ainsi, on désire non seulement construire un circuit hamiltonien mais de plus, on voudrait trouver un nombre maximum de tels circuits n'ayant aucune arête commune.

![img-88.jpeg](img-88.jpeg)
FIGURE I.72. Deux circuits hamiltoniens disjoints de  $K_{5}$ .

![img-89.jpeg](img-89.jpeg)
FIGURE I.73. Deux autres circuits hamiltoniens disjoints de  $K_{5}$ .

I.11. Graphes hamiltoniens

![img-90.jpeg](img-90.jpeg)
FIGURE I.74. Un tentative de partition de  $K_{7}$ .

![img-91.jpeg](img-91.jpeg)

![img-92.jpeg](img-92.jpeg)

Proposition I.11.18. Pour  $n \geq 3$ ,  $K_{n}$  peut être partitionné en circuits hamiltoniens disjoints si et seulement si  $n$  est impair. En particulier, le nombre de circuits formant une partition de  $K_{n}$  vaut  $(n - 1)/2$ .

Démonstration. Nous savons que  $K_{n}$  est  $(n - 1)$ -régulier (i.e., le degré de chaque sommet vaut  $n - 1$ ). De plus, un circuit hamiltonien est 2-régulier. Par conséquent, il est nécessaire que  $n - 1$  soit pair pour pouvoir décomposer  $K_{n}$  en cycles hamiltoniens disjoints. (Sinon, certaines arêtes, une par sommet, ne feraient partie d'aucun cycle).

Supposons à présent  $n$  impair. Il est évident que  $K_{n}$  peut être décomposé en au plus  $(n - 1) / 2$  circuits hamiltoniens disjoints. En effet, on sélectionne d'abord un premier cycle hamiltonien formé de  $n$  arêtes (on a donc sélectionné deux arêtes incidentes à chaque sommet). Pour le second cycle de  $K_{n}$ , puisque celui-ci doit être disjoint du précédent, le choix se restreint (ce可以选择 correspond encore une fois à sélectionner deux arêtes incidentes à chaque sommet parmi les arêtes non déjà sélectionnées). Puisque chaque cycle可以选择 sélectionner deux arêtes par sommet et que  $K_{n}$  est  $(n - 1)$ -régulier, on ne peut sélectionner qu'au maximum  $(n - 1) / 2$  circuits hamiltoniens disjoints. Notons que la construction d'exactement  $(n - 1) / 2$  circuits disjoints peut a priori être réalisée de plusieurs façon ( comme le montrent les figures I.72 et I.73).

A ce stade, le lecteur ne doit pas être convaincu (en effet, rien n'assure a priori que la procédure proposée fournit le résultatannoncé; le lecteur pourrait objecter qu'à une étape intermédiaire, il ne soit plus possible de continuer et il aurait tout à fait raison!). Comme le montre la figure I.74, un choix arbitraire de deux arêtes à chaque étape peut mener à l'impossibilité d'obtenir des cycles (la figure de droite montre deux cycles non connexes, l'un de longueur 4 et l'autre de longueur 3).

L'obtention d'exactement  $(n - 1) / 2$  circuits hamiltoniens est assurée par les deuxlemmes suivants. (Nous nous basons sur les constructions de Bollobás en donnant en plus des arguments géométriques.)

Lemma I.11.19. Si  $n$  est pair,  $K_{n}$  peut être partitionné en  $n/2$  chemins hamiltoniens disjoints.

Chapitre I. Premier contact avec les graphes

![img-93.jpeg](img-93.jpeg)
FIGURE I.75. Chemins hamiltoniens disjoints de  $K_{6}$  et  $K_{8}$ .

Pour prouver ce résultat, on peut considérer des arguments géométriques. On peut identifier les sommets de  $K_{n}$  avec les sommets d'un polygone régulier à  $n$  cots possédant une symétrie orthogonale par rapport à une horizontale (autrement dit, possédant deux cots horizontaux). On obtient un chemin hamiltonien  $\mathcal{C}$  en reliant les sommets se trouvant sur une même horizontale et en reliant le sommet gauche du niveau horizontal  $i$  avec le sommet de droite de niveau  $i + 1$ . On remarque $^{38}$  que toutes ces arêtes "obliques" ont une même pente  $\pi / n$  (c'est-à-dire mesure de l'angle qu'elles forment avec une horizontale). Nous affirmons qu'en effectuant une rotation de la figure  $\mathcal{C}$  de  $2k\pi / n$ ,  $k = 1, \ldots, n/2 - 1$ , on obtient  $n/2$  chemins hamiltoniens distincts. Pour vérifier qu'ils sont tous distincts, il suffit de raisonner sur la pente respective des arêtes les constituant. Une illustration est fournie à la figure I.75. En effet, pour  $\mathcal{C}$ , les arêtes "obliques" ont pour pente  $\pi / n$ , puis en effectuant les rotations on obtient les pentes distinctes  $3\pi / n$ ,  $5\pi / n$ , ...,  $(1 - 1/n)\pi$ .

Lemma I.11.20. Soit  $n \geq 3$  impair. Le graphe  $K_{n}$  peut être partitionné en  $(n - 1) / 2$  circuits hamiltoniens disjoints si et seulement si  $K_{n - 1}$  peut être partitionné en  $(n - 1) / 2$  chemins hamiltoniens disjoints.

Le résultat est presque immédiat. Si  $K_{n}$  peut être partitionné et qu'on lui supprime un sommet, on passes à  $K_{n-1}$  et chaque circuit hamiltonien  $C$  donne naissance à un chemin hamiltonien (les extrémités du chemin étant les sommets voisins dans  $C$  du sommet supprimé). Réciproquement, si  $K_{n-1}$  est partitionné, l'adjonction d'un sommet permet de passer à  $K_{n}$  en "ferment" chaque chemin hamiltonien pour obtenir des circuits disjoints.

69

# CHAPITRE II

# Un peu de théorie algébrique des graphes

Dans ce chapitre, nous ne faisons qu'esquisser quelques résultats mettant en évidence les liens entre la théorie des graphes et l'algèbre linéaire (par exemple, on étudie le spectre des graphes réguliers ou bipartis). On y présente de manière détaillée la théorie de Perron-Frobenius (sans démonstration du théorème principal) avec comme application directe, l'estimation du nombre de chemins de longueur $n$ que l'on peut trouver dans un graphe à composantes connexes primitives. On présente également diverses questions relatives aux sous-arbres couvrants : nombre de tels sous-arbres, recherche d'un arbre de poids minimal, algorithme de Prim et de Kruskal. En particulier, ce chapitre met en lumière quelques raisonnements de combinatoire énumérative.

## 1. Matrice d'adjacence

**Définition II.1.1.** Soit $G = (V, E)$ un multi-graphe *non orienté* dont les sommets sont ordonnés par $V = \{v_1, \ldots, v_n\}$. La matrice d'adjacence de $G$ est la matrice $A(G)$ dont l'élément $[A(G)]_{i,j}$ est égal au nombre d'arêtes $\{v_i, v_j\}$ présentes dans $E$, $1 \leq i, j \leq n$. (Pour rappel, $E$ est en général un multi-ensemble.) Il s'agit donc d'une matrice symétrique à coefficients entiers naturels. Le polynôme caractéristique de $G$, noté $\chi_G(\lambda)$, est le polynôme caractéristique de sa matrice d'adjacence $A(G)$. Par abus de langage, on parlera des valeurs propres de $G$, étant sous-entendu qu'il s'agit des valeurs propres de $A(G)$. On parlera donc aussi du spectre de $G$.

On peut remarquer que les éléments de la matrice d'adjacence d'un graphe *simple* appartiennent à $\{0,1\}$ et que la trace de cette matrice vaut 0.

**Remarque II.1.2.** En se rappelant quelques résultats du cours d'algèbre de première année, on remarque que la matrice d'adjacence d'un graphe non orienté est toujours diagonalisable par une matrice orthogonale (pour chaque valeur propre, les multiplicités algébrique et géométrique coïncident) et que ses valeurs propres sont réelles.

**Exemple II.1.3.** Considérons le graphe (simple) $G$ de la figure II.1. Avec les notations précédentes, on a aussi

$$
\chi_G(\lambda) = -\lambda^5 + 8\lambda^3 + 10\lambda^2 + \lambda - 2.
$$

**Proposition II.1.4.** Deux graphes $G_1$ et $G_2$ sont isomorphes si et seulement si ils ont, à une permutation près, la même matrice d'adjacence.

Chapitre II. Un peu de théorie algébrique des graphes

![img-94.jpeg](img-94.jpeg)
FIGURE II.1. Un graphe  $G$  et sa matrice d'adjacence.

$$
A (G) = \left( \begin{array}{c c c c c} 0 &amp; 1 &amp; 1 &amp; 1 &amp; 0 \\ 1 &amp; 0 &amp; 1 &amp; 1 &amp; 1 \\ 1 &amp; 1 &amp; 0 &amp; 1 &amp; 1 \\ 1 &amp; 1 &amp; 1 &amp; 0 &amp; 0 \\ 0 &amp; 1 &amp; 1 &amp; 0 &amp; 0 \end{array} \right)
$$

det  $A(G_{1}) = \operatorname *{det}A(G_{2})$

Autrement dit, il existe une matrice de permutation  $P$  telle que

$$
A (G _ {1}) = P ^ {- 1} A (G _ {2}) P.
$$

Démonstration. C'est immédiat et ce résultat est même transposable au cas de graphes orientés.

Definition II.1.5. On peut aussi définir la matrice d'incidence "sommets/arêtes". Si  $V = \{v_{1},\ldots ,v_{n}\}$  et  $E = \{e_1,\dots ,e_m\}$ , il s'agit d'une matrice  $B$  de dimension  $n\times m$  telle que  $B_{i,j} = 1$  si et seulement si  $e_j$  est incident à  $v_{i}$ . En poursuivant l'exemple précédent, cette matrice vaut ici

|  1 | 0 | 0 | 1 | 0 | 1 | 0 | 0  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  1 | 1 | 0 | 0 | 1 | 0 | 1 | 0  |
|  0 | 1 | 1 | 0 | 0 | 1 | 0 | 1  |
|  0 | 0 | 1 | 1 | 1 | 0 | 0 | 0  |
|  0 | 0 | 0 | 0 | 0 | 0 | 1 | 1  |

Definition II.1.6. Dans un graphe simple, on appelle triangle, tout triplet d'arêtes distinctes deux à deux de la forme  $\{a,b\}$ ,  $\{b,c\}$ ,  $\{c,a\}$  (i.e., tout circuit de longueur trois formé d'arêtes distinctes).

Proposition II.1.7. Si le polynôme caractéristique de  $G = (V, E)$  est de la forme

$$
\chi_ {G} (\lambda) = (- \lambda) ^ {n} + c _ {1} (- \lambda) ^ {n - 1} + c _ {2} (- \lambda) ^ {n - 2} + \dots + c _ {n},
$$

alors certains coefficients de  $\chi_G$  sont en relation directe avec  $G$  :

-  $c_1$  est le nombre de boucles de  $G$ , en particulier, si  $G$  est simple,  $c_1 = 0$ .
Si  $G$  est simple, alors  $-c_{2}$  est le nombre d'arêtes de  $G$ .
Si  $G$  est simple, alors  $c_{3}$  est le double du nombre de triangles de  $G$

Démonstration. Le premier point est immédiat. Le coefficient  $c_{1}$  est la somme des éléments diagonaux de  $A_{G}$ . Si  $G$  est simple, les sous-matrices

II.1. Matrice d'adjacence

diagonales $^2$  de  $A_G$  de dimension 2 sont de la forme

$$
\left( \begin{array}{c c} 0 &amp; 1 \\ 1 &amp; 0 \end{array} \right) \quad \text {o u} \quad \left( \begin{array}{c c} 0 &amp; 0 \\ 0 &amp; 0 \end{array} \right).
$$

Le coefficient  $c_{2}$  étant la somme des déterminants de ces sous-matrices ceuxci valant respectivement -1 et 0, il est clair que  $c_{2} = -\# E$ . Pour le dernier point, on raisonne de la même façon. Les sous-matrices diagonales non nulles de  $A_{G}$  de dimension 3 sont d'une des formes suivantes (à une permutation des lignes et des colonnes près, ce qui ne change pas la valeur du déterminant)

$$
\left( \begin{array}{c c c} 0 &amp; 1 &amp; 0 \\ 1 &amp; 0 &amp; 0 \\ 0 &amp; 0 &amp; 0 \end{array} \right), \quad \left( \begin{array}{c c c} 0 &amp; 1 &amp; 1 \\ 1 &amp; 0 &amp; 0 \\ 1 &amp; 0 &amp; 0 \end{array} \right) \quad \text {o u} \quad \left( \begin{array}{c c c} 0 &amp; 1 &amp; 1 \\ 1 &amp; 0 &amp; 1 \\ 1 &amp; 1 &amp; 0 \end{array} \right).
$$

Les deux premières ont un déterminant nul et la troisième a un déterminant égal à 2. Le coefficient  $c_{3}$  étant la somme des déterminants de ces sous-matrices et la dernière matrice correspondant à la présence d'un triangle dans  $G$ , on obtient le résultatannoncé.

Remarque II.1.8. On voit donc que le polynôme caractéristique de  $A(G)$  fournit des renseignements sur le graphe  $G$ . Cependant, deux graphes non isomorphes peuvent avoir le même polynôme caractéristique $^3$ . On parle alors de graphes cospectraux. Par exemple, les deux graphes de la figure II.2 ont le même spectre. En effet, ils ont tous les deux comme polynôme caractéristique,

$$
- 1 + 4 \lambda + 7 \lambda^ {2} - 4 \lambda^ {3} - 7 \lambda^ {4} + \lambda^ {6}.
$$

![img-95.jpeg](img-95.jpeg)
FIGURE II.2. Deux graphes cospectraux.

![img-96.jpeg](img-96.jpeg)

Proposition II.1.9. Soit  $G = (V, E)$  un graphe biparti. Si  $\lambda$  est valeur propre de  $G$ , alors  $-\lambda$  l'est aussi. Autrement dit, le spectre d'un graphe biparti est symétrique par rapport à 0.

Démonstration. Par hypothèse,  $V$  se partitionne en deux sous-ensembles  $V_{1}$  et  $V_{2}$  de manière telle que toute arête de  $G$  est de la forme  $\{u,v\}$  avec

Chapitre II. Un peu de théorie algébrique des graphes

$u \in V_1$  et  $v \in V_2$ . Si on ordonne les sommets de  $V$  de manière à considérer tout d'abord les sommets de  $V_1$ , alors  $A(G)$  a la forme

$$
\left( \begin{array}{c c} 0 &amp; B \\ \widetilde {B} &amp; 0 \end{array} \right)
$$

ou  $B$  est une matrice de dimension  $\# V_{1} \times \# V_{2}$ . Soit  $x$  un vecteur propre non nul de  $A(G)$  de valeur propre  $\lambda$ . Appelons  $x_{1}$  (resp.  $x_{2}$ ) le vecteur obtenu en considérant les  $\# V_{1}$  premières (resp. les  $\# V_{2}$  dernières) composantes de  $x$ . Ainsi,

$$
\left( \begin{array}{c c} 0 &amp; B \\ \widetilde {B} &amp; 0 \end{array} \right) \left( \begin{array}{c} x _ {1} \\ x _ {2} \end{array} \right) = \left( \begin{array}{c} B x _ {2} \\ \widetilde {B} x _ {1} \end{array} \right) = \lambda \left( \begin{array}{c} x _ {1} \\ x _ {2} \end{array} \right).
$$

Nous pouvons à présent facilement exuber un vecteur propre non nul de valeur propre  $-\lambda$

$$
\left( \begin{array}{c c} 0 &amp; B \\ \widetilde {B} &amp; 0 \end{array} \right) \left( \begin{array}{c} x _ {1} \\ - x _ {2} \end{array} \right) = \left( \begin{array}{c} - B x _ {2} \\ \widetilde {B} x _ {1} \end{array} \right) = \lambda \left( \begin{array}{c} - x _ {1} \\ x _ {2} \end{array} \right) = - \lambda \left( \begin{array}{c} x _ {1} \\ - x _ {2} \end{array} \right).
$$

Nous montrérons un peu plus loin qu'on dispose également de la réciropque de ce résultat (cf. corollaire II.2.6).

On peut aussi définir la matrice d'adjacence d'un multi-graphe orienté. La définition est analogue à celle donnée précédemment sauf qu'on tient ici compte de l'orientation. Il en résultat que la matrice obtenue n'est en général pas symétrique.

Definition II.1.10. Soit  $G = (V, E)$  un multi-graphe orienté dont les sommets sont ordonnés par  $V = \{v_1, \ldots, v_n\}$ . La matrice d'adjacence de  $G$  est la matrice  $A(G)$  dont l'élement  $[A(G)]_{i,j}$  est égal au nombre d'arcs  $(v_i, v_j)$  présents dans  $E$ ,  $1 \leq i, j \leq n$ .

Example II.1.11. Si on considère le graphe de la figure I.2, on obtient la matrice d'adjacence suivante.

$$
\left( \begin{array}{c c c c c} 0 &amp; 1 &amp; 0 &amp; 0 &amp; 1 \\ 0 &amp; 1 &amp; 1 &amp; 0 &amp; 0 \\ 0 &amp; 0 &amp; 1 &amp; 1 &amp; 1 \\ 1 &amp; 0 &amp; 0 &amp; 0 &amp; 0 \\ 1 &amp; 0 &amp; 0 &amp; 1 &amp; 0 \end{array} \right)
$$

Théorème II.1.12. Soit  $G = (V, E)$  un multi-graphe (orienté ou non) tel que  $V = \{v_1, \ldots, v_k\}$ . Pour tous  $i, j \in \{1, \ldots, k\}$  et pour tout  $n &gt; 0$ ,

$$
[ A (G) ^ {n} ] _ {i, j}
$$

est exactement le nombre de chemins de longueur n joignant  $v_{i}$  à  $v_{j}$ .

Démonstration. On procède par récurrence sur  $n$ . Le cas de base  $n = 1$  découle de la définition même de la matrice d'adjacence. Supposons le

II.2. Théorie de Perron-Frobenius

résultat vérifié pour $n &gt; 0$ et vérifions-le pour $n + 1$. On a bien sûr

$$
[ A (G) ^ {n + 1} ] _ {i, j} = \sum_ {s = 1} ^ {k} [ A (G) ^ {n} ] _ {i, s} [ A (G) ] _ {s, j}.
$$

Par hypothèse de récurrence, $[A(G)^n]_{i,s}$ compte le nombre de chemins de longueur $n$ joignant $v_i$ à $v_s$. De plus, $[A(G)]_{s,j}$ compte le nombre d'arcs/arêtes joignant $v_s$ à $v_j$. Par conséquent, $[A(G)^n]_{i,s}[A(G)]_{s,j}$ compte le nombre de chemins de longueur $n + 1$ joignant $v_i$ à $v_j$ en passant par $v_s$, d'où la conclusion.

## 2. Théorie de Perron-Frobenius

La connexité d'un graphe se traduit par une propriété immédiate de sa matrice d'adjacence. On peut même, dans certains cas, obtenir des renseignements plus fins sur la longueur des chemins joignant deux sommets quelconques d'une composante connexe. Donnons tout d'abord deux définitions concernant les matrices à coefficients positifs ou nuls (faites attention dans les deux énoncés à l'ordre des quantificateurs). Nous verrons ensuite le rapport entre ces matrices et les graphes.

**Définition II.2.1.** Une matrice carrée $A = (a_{ij})_{1\leq i,j\leq n}$ à coefficients (réels) positifs ou nuls est *irréductible* si pour tous $i,j \in \{1,\ldots,n\}$, il existe⁴ un entier $N(i,j) \geq 0$ tel que

$$
[ A ^ {N (i, j)} ] _ {i, j} &gt; 0.
$$

**Définition II.2.2.** Une matrice carrée $A = (a_{ij})_{1\leq i,j\leq n}$ à coefficients (réels) positifs ou nuls est *primitive* s'il existe un entier $N &gt; 0$ tel que pour tous $i,j \in \{1,\ldots,n\}$

$$
[ A ^ {N} ] _ {i, j} &gt; 0
$$

ce que l'on s'autorise à noter $A^N &gt; 0$ étant sous-entendu que les inégalités sont interprétées composante à composante. On remarque aussi que toute matrice primitive est irréductible.

Dans cette section, pour deux matrices réelles $A$ et $B$ de même dimension, il sera commode d'écrire $A &lt; B$ (resp. $\leq, \geq, &gt;$ ) si l'inégalité a lieu composante à composante. Cela n'entraîne pas d'ambiguité particulière.

**Proposition II.2.3.** Un multi-graphe orienté (resp. non orienté) est fortement connexe (resp. connexe) si et seulement si sa matrice d'adjacence est irréductible.

**Démonstration.** C'est une conséquence directe du théorème II.1.12.

⁴ Avec un peu d'expérience, le lecteur pourra se convaincre que, dans cette définition, on aurait aussi pu imposer de manière équivalente $N(i,j) &gt; 0$.

Chapitre II. Un peu de théorie algébrique des graphes

Remarque II.2.4. Si la matrice d'adjacence d'un graphe (orienté ou non) est primitive, cela entraîne que le graphe est non seulement connexe mais qu'il existe  $N$  tel que, quelle que soit la paire de sommets considérée, il existe un chemin de longueur  $N$  les joignant. Par abus de langage, on s'autorisera alors à parler de graphe primitif.

Lorsqu'une matrice est irréductible (ou, en particulier primitive), on dispose du puissant théorème de Perron-Frobenius.

Théorème II.2.5 (Perron-Frobenius). Soit  $A \geq 0$  une matrice carrée irréductible de dimension  $n$ .

$\triangleright$  La matrice  $A$  possède un vecteur propre  $v_{A} \in \mathbb{R}^{n}$  (resp.  $w_{A} \in \mathbb{R}^{n}$ ) dont les composantes sont toutes strictement positives et correspondant à une valeur propre  $\lambda_{A} &gt; 0$ ,

$$
A v _ {A} = \lambda_ {A} v _ {A} (r e s p. \widetilde {w _ {A}} A = \lambda_ {A} \widetilde {w _ {A}}).
$$

$\triangleright$  Cette valeur propre  $\lambda_{A}$  possède une multiplicité algébrique (et géométrique) simple.
$\triangleright$  Tout vecteur propre de  $A$  dont les composantes sont strictement positives est un multiple de  $v_{A}$ .
$\triangleright$  Toute autre valeur propre  $\mu \in \mathbb{C}$  de  $A$  est telle que  $|\mu |\leq \lambda_A$
Si  $\mu$  est une valeur propre de  $A$  telle que  $|\mu | = \lambda_{A}$  , alors

$$
\mu = \lambda_ {A} e ^ {2 i k \pi / d}
$$

pour un certain  $d \geq 1$  et  $k \in \{0, \dots, d - 1\}$ . De plus, pour tout  $k \in \{0, \dots, d - 1\}$ ,  $\lambda_A e^{2ik\pi / d}$  est une valeur propre de  $A$ .

$\triangleright$  Soit  $B$  une matrice réelle à coefficients positifs ou nuls de même dimension que  $A$ . Si  $B \leq A$ , alors pour toute valeur propre  $\mu$  de  $B$ , on a  $|\mu| \leq \lambda_A$  et l'égalité a lieu si et seulement si  $A = B$ .

La valeur propre  $\lambda_{A}$  est appelée la valeur propre de Perron de  $A$ . Autrement dit, ce théorème stipule qu'une matrice irréductible possède toujours une valeur propre réelle dominante  $\lambda_{A}$ . Cependant, on peut avoir d'autres valeurs propres de module égal à  $\lambda_{A}$  mais dans ce cas, celles-ci sont exactement obtenues par multiplication de  $\lambda_{A}$  par les racines  $d$ -ièmes de l'unité $^{5}$ .

Démonstration. Puisqu'il s'agit d'un cours de théorie des graphes et pas d'un cours d'algebre, nous ne donnons pas ici la preuve de ce résultat. Cette preuve n'est pas difficile mais assez longue (elle fait d'ailleurs intervenir des raisonnements de continuité et de passage à la limite). Le lecteur interressé trouvera une preuve classique dans [26] ou [9]. Une illustration assez géométrique de la preuve dans le cas de la dimension 2 est donnée dans [14]. On trouve aussi une preuve reposant sur la notion élégante de fonctions sous-harmoniques (sorte de généralisation de la notion de vecteur propre)

II.2. Théorie de Perron-Frobenius

![img-97.jpeg](img-97.jpeg)
FIGURE II.3. Disposition des valeurs propres d'une matrice irréductible.

Dans [11]. Enfin, une preuve détaillée et très bien structurée se trouve dans [1] où la théorie de Perron-Frobenius est utilisée dans le cadre des suites automatiques (fréquence d' apparition d'un symbole dans une suite, etc...).

Nous pouvons à présent prouver la reciproque de la proposition II.1.9.

Corollaire II.2.6. Si  $G = (V, E)$  est un graphe (non orienté simple) connexe dont le spectre est symétrique par rapport à 0, alors  $G$  est biparti.

Démonstration. Soient  $\lambda_G$  la valeur propre de Perron de  $G$  et  $x \neq 0$  un vecteur propre associé. Par hypothèse  $-\lambda_G$  est aussi une valeur propre de  $G$  et considérons  $y \neq 0$ , l'un de ses vecteurs propres. Bien évidemment,  $x$  et  $y$  sont linéairement indépendants. Si  $A$  est la matrice d'adjacence de  $G$ ,  $A^2$  est la matrice d'adjacence du multi-graphe  $G' = (V, E')$  où une arête  $\{a, b\}$  appartient à  $E'$  si et seulement si il existe  $c \in V$  tel que  $\{a, c\}$  et  $\{b, c\}$  appartiennent à  $E$ . On pourrait appeler  $G'$ , le graphe des chemins de longueur 2 de  $G$ .

![img-98.jpeg](img-98.jpeg)
FIGURE II.4. Une illustration des graphes  $G$  et  $G'$ .

Il est clair que  $\lambda_G^2$  est la valeur propre dominante de  $A^2$  et que  $x$  et  $y$  en sont des vecteurs propres. Par conséquent, la multiplicité de  $\lambda_G^2$  est au moins 2 et on en déduit que  $A^2$  ne peut être irreductible (i.e.,  $G'$  n'est pas

Chapitre II. Un peu de théorie algébrique des graphes

connexe). Nous allons montré que  $G$  est bipartite. En effet,  $G'$  contient au moins deux composantes connexes. En fait, il en contient exactement deux. Comme nous allons l'expliquer, cela découle du fait que  $G$  est connexe.

Soit  $u$  un sommet quelconque fixé. On définit  $V_{1}$  (resp.  $V_{2}$ ) comme l'ensemble des sommets joints à  $u$  par un chemin de longueur impaire (resp. paire) dans  $G$ . Puisque  $G$  est connexe,  $V_{1} \cup V_{2} = V$  (à ce stade, rien ne garantit qu'il s'agisse d'une partition, on pourrait imaginer qu'un sommet  $v$  soit joint à  $u$  par deux chemins, l'un de longueur paire, l'autre de longueur impaire). Tous les sommets de  $V_{2}$  joints à  $u$  par un chemin de longueur paire dans  $G$  sont connectés à  $u$  dans  $G'$ , cela découle de la définition même de  $G'$ . La restriction de  $G'$  aux sommets de  $V_{2}$  est donc connexe. Tous les sommets de  $V_{1}$  joints à  $u$  par un chemin de longueur impaire dans  $G$ , sont connectés entre eux dans  $G'$ . En effet, considérons  $v, w \in V_{1}$ . Dans  $G$ , il existe un chemin de longueur impaire entre  $v_{1}$  et  $u$  et aussi un entre  $u$  et  $v_{2}$ . Il existe donc un chemin (passant par  $u$ ) de longueur paire dans  $G$  entre  $v$  et  $w$ . Autrement dit, la restriction de  $G'$  à  $V_{1}$  est connexe. De plus,  $V_{1} \cap V_{2} = \emptyset$ , car sinon  $G'$  serait connexe. En raisonnant sur la parité des longueurs de chemins de  $G$ , on a donc bien exactement deux composantes connexes  $V_{1}$  et  $V_{2}$  dans  $G'$ .

Puisque dans  $G'$ , il n'y a aucune arête entre un sommet de  $V_{1}$  et un sommet de  $V_{2}$ , on en conclus que dans  $G$ , il n'y a aucun chemin de longueur paire entre deux tels sommets (en effet, un chemin de longueur  $2\ell$  dans  $G$  se décompose en  $\ell$  chemins de longueur 2, c'est-à-dire en un chemin de longueur  $\ell$  dans  $G'$ , ce dernier restant dans une unique composante connexe). Autrement dit, un chemin joignant dans  $G$  un sommet de  $V_{1}$  et un sommet de  $V_{2}$  est nécessairement de longueur impaire.

Supposons à présent que, dans  $G$ , un chemin de longueur impaire  $2\ell + 1$  joigne deux sommets  $u_{1}$  et  $u_{2\ell + 2}$  de  $V_{1}$ . Par symétrie, le raisonnement qui suit s'applique aussi à deux sommets de  $V_{2}$ . Ce chemin, déterminé par la suite de sommets  $(u_{1}, u_{2}, \ldots, u_{2\ell + 1}, u_{2\ell + 2})$ , se décompose en  $\ell$  chemins  $(u_{2i - 1}, u_{2i}, u_{2i + 1})$  de longueur 2,  $i = 1, \ldots, \ell$ , plus une arête  $\{u_{2\ell}, u_{2\ell + 1}\}$ . Dans  $G'$ , les  $\ell$  chemins de longueur 2 correspondent à des arêtes de la composante connexe  $V_{1}$ . Donc  $u_{1}, u_{3}, \ldots, u_{2\ell + 1}$  appartiennent à  $V_{1}$ . On en déduit que dans  $G$ , il existe alors une arête entre deux sommets  $a = u_{2\ell + 1}$  et  $b = u_{2\ell + 2}$  de  $V_{1}$ . Mais  $G$  étant connexe, il existe aussi une arête joignant un sommet  $c$  de  $V_{1}$  et un sommet  $d$  de  $V_{2}$ . De là, on en conclus qu'il existe un chemin de longueur paire joignant  $a$  à  $d$  (en effet,  $b$  et  $c$  se trouvent dans la même composante connexe de  $G'$ , ils sont donc séparés par un chemin de longueur paire dans  $G$  et il suffit d'ajouter les deux arêtes  $\{a, b\}$  et  $\{c, d\}$ ). Ceci est en contradiction avec la première partie de notre raisonnement.

II.2. Théorie de Perron-Frobenius

En conclusion, dans  $G$ , les sommets de  $V_{1}$  (resp. de  $V_{2}$ ) sont joints entre eux par des chemins de longueur paire exclusivement. En particulier, il n'y a aucune arête entre deux sommets de  $V_{1}$  (resp.  $V_{2}$ ). De plus, les sommets de  $V_{1}$  sont joints aux sommets de  $V_{2}$  par des chemins de longueur impaire exclusivement. Ceci montre que  $G$  est biparti.

Dans le cas d'une matrice primitive, les assertions du théorème de Perron-Frobenius sont encore renforcées (les trois premières étant inchangées).

Théorème II.2.7 (Perron). Soit  $A \geq 0$  une matrice carrée primitive de dimension  $n$ .

$\triangleright$  La matrice  $A$  possède un vecteur propre  $v_{A} \in \mathbb{R}^{n}$  (resp.  $w_{A} \in \mathbb{R}^{n}$ ) dont les composantes sont toutes strictement positives et correspondant à une valeur propre  $\lambda_{A} &gt; 0$ ,

$A v_{A} = \lambda_{A} v_{A}$  (resp.  $\widetilde{w_{A}} A = \lambda_{A} \widetilde{w_{A}}$ ).

$\triangleright$  Cette valeur propre  $\lambda_{A}$  possède une multiplicité algébrique (et géométrique) simple.
$\triangleright$  Tout vecteur propre de  $A$  dont les composantes sont strictement positives est un multiple de  $v_{A}$ .
Toute autre valeur propre  $\mu \in \mathbb{C}$  de  $A$  est telle que  $|\mu| &lt; \lambda_A$ .
$\triangleright$  Soit  $B$  une matrice réelle à coefficients positifs ou nuls de même dimension que  $A$ . Si  $B \leq A$ , alors pour toute valeur propre  $\mu$  de  $B$ , on a  $|\mu| \leq \lambda_A$  et l'égalité a lieu si et seulement si  $A = B$ .

Ainsi, la valeur propre de Perron  $\lambda_{A}$  est l'unique valeur propre dominante. Toute autre valeur propre de  $A$  a un module strictement inférieur à  $\lambda_{A}$ . On a donc un résultat plus fort que dans le cas irréductible (c'est assez naturel puisque les hypothèses sont plus fortes).

![img-99.jpeg](img-99.jpeg)
FIGURE II.5. Disposition des valeurs propres d'une matrice primitive.

Traitons à présent deux exemples. Nous y avons choisi des graphes orientés. Des constatations analogues peuvent naturellement être obtenues dans le cas non orienté.

Chapitre II. Un peu de théorie algébrique des graphes

![img-100.jpeg](img-100.jpeg)
FIGURE II.6. Un graphe avec une matrice d'adjacency primitive.

$A(G) = \left( \begin{array}{lll}1 &amp; 1 &amp; 0\\ 1 &amp; 0 &amp; 1\\ 1 &amp; 0 &amp; 0 \end{array} \right)$

Example II.2.8 (Cas primitif). Considérons le graphe de la figure II.6 et la matrice d'adjacence correspondante. La matrice  $A(G)$  est primitive. En effet, on a

$A(G)^{2} = \left( \begin{array}{lll}2 &amp; 1 &amp; 1\\ 2 &amp; 1 &amp; 0\\ 1 &amp; 1 &amp; 0 \end{array} \right)$  et  $A(G)^{3} = \left( \begin{array}{lll}4 &amp; 2 &amp; 1\\ 3 &amp; 2 &amp; 1\\ 2 &amp; 1 &amp; 1 \end{array} \right) &gt; 0.$

Il existe donc un chemin de longueur 3 joignant toute paire de sommets. Par exemple, on a les chemins suivants :

$1\to 1\to 1\to 1$  ，  $1\to 1\to 1\to 2$  ，  $1\to 1\to 2\to 3$
$2\to 1\to 1\to 1$  ，  $2\to 1\to 1\to 2$  ，  $2\to 1\to 2\to 3$
$3\to 1\to 1\to 1$  ，  $3\to 1\to 1\to 2$  ，  $3\to 1\to 2\to 3$

Des valeurs approchées de valeurs propres de  $A(G)$  sont

$\lambda_{A}\simeq 1.83929$  ，  $\lambda_{2}\simeq -0.41964 + 0.60629i$  et  $\lambda_3\simeq -0.41964 - 0.60629i$  et  $|\lambda_2| = |\lambda_3| &lt;   \lambda_A$

Example II.2.9 (Cas irreductible). Considérons à présent le graphe de la figure II.7 et la matrice d'adjacence correspondante. Il est clair que le

![img-101.jpeg](img-101.jpeg)
FIGURE II.7. Un graphe avec une matrice d'adjacence irreductible.

$A(G) = \left( \begin{array}{lll}0 &amp; 1 &amp; 0\\ 0 &amp; 0 &amp; 1\\ 1 &amp; 0 &amp; 0 \end{array} \right)$

graphe est f. connexe et donc, la matrice  $A(G)$  est au moins irréductible. Cependant, un chemin de longueur  $k$  joint les sommets 1 et 2 si et seulement si un chemin de longueur  $k + 1$  joint les sommets 1 et 3. Il n'existe donc pas d'entier  $n$  pour lequel tout sommet peut être joint à tout autre sommet par un chemin de longueur  $n$ . La matrice  $A(G)$  n'est donc pas primitive. On pourrait aussi s'en convaincre en montrant que, pour tout  $n$ ,

$A(G)^{3n} = \left( \begin{array}{lll}1 &amp; 0 &amp; 0\\ 0 &amp; 1 &amp; 0\\ 0 &amp; 0 &amp; 1 \end{array} \right), A(G)^{3n + 1} = \left( \begin{array}{lll}0 &amp; 1 &amp; 0\\ 0 &amp; 0 &amp; 1\\ 1 &amp; 0 &amp; 0 \end{array} \right), A(G)^{3n + 2} = \left( \begin{array}{lll}0 &amp; 0 &amp; 1\\ 1 &amp; 0 &amp; 0\\ 0 &amp; 1 &amp; 0 \end{array} \right).$

II.2. Théorie de Perron-Frobenius

Les valeurs propres sont ici les racines cubiques de l'unité

$$
1, e ^ {2 i \pi / 3}, e ^ {4 i \pi / 3}
$$

et donc, on a ici, par opposition avec l'exemple précédent, plusieurs valeurs propres de module maximum  $(= 1)$ .

Il est aussi intéressant de noter que pour joindre deux sommets fixés, uniquement certaines longueurs de chemin peuvent être considérées. Par exemple, pour joindre les sommets 2 et 3, uniquement des chemins de longueur congrue à 1 modulo 3 peuvent être envisagés. Ce phénomène est en fait tout à fait général et sera explicité à la section suivante.

**Corollaire** II.2.10. Soit $A \geq 0$ une matrice carrée. Les assertions suivantes sont équivalentes.

i) $A$ est primitive,
ii) il existe $N \geq 1$ tel que $A^N &gt; 0$,
iii) il existe $N \geq 1$ tel que $A^n &gt; 0$ pour tout $n \geq N$.

**Démonstration.** Par définition, i) $\Rightarrow$ ii) et ii) $\Rightarrow$ i). Montrons que ii) $\Rightarrow$ iii). Puisque $A^N &gt; 0$, on en déduit que toute colonne de $A$ contient au moins un élément strictement positif. Par conséquent, si $A^k &gt; 0$, alors $A^k \cdot A &gt; 0$ et de proche en proche, $A^{k + i} &gt; 0$ pour tout $i \geq 0$. Enfin, il est immédiat que iii) $\Rightarrow$ ii).

---

**2.1. Période d'une matrice irréductible.** Soit $(a_{ij})_{1\leq i,j\leq d} = A \geq 0$ une matrice carrée de dimension $d$ à coefficients positifs ou nuls. Par indice de $A$, on entend un élément de $\{1,\ldots ,d\}$.

**Définition** II.2.11. Soit $i$ un indice. S'il existe $N &gt; 0$ tel que $[A^N]_{i,i} &gt; 0$, alors la période de l'indice $i$ est le p.g.c.d. de l'ensemble des entiers $n &gt; 0$ pour lesquels

$$
[ A ^ {n} ] _ {i, i} &gt; 0.
$$

On la note $p(i)$. Bien évidemment, le p.g.c.d. d'un ensemble infini d'entiers $X = \{x_{1} &lt; x_{2} &lt; \dots \} \subseteq \mathbb{N}$ est le plus grand entier $p$ appartenant à l'ensemble fini $\{1,2,\ldots ,x_1\}$ tel que pour tout $k \geq 1$, $p$ divise $x_{k}$.

**Remarque** II.2.12. Cette définition est donnée en termes de matrices, mais elle possède un analogue immédiat en termes de graphes. En effet, à la matrice $A = (a_{ij})_{1\leq i,j\leq d}$, on fait correspondre un graphe $G_{A} = (V_{A},E_{A})$ ayant pour ensemble de sommets $V_{A} = \{1,\ldots ,d\}$ et il existe un arc joignant $i$ à $j$ si et seulement si $a_{i,j} &gt; 0$. Ainsi, pour définir la période d'un sommet $i\in V_A$ appartenant à une composante f. connexe du graphe $G_{A}$, on recherche le p.g.c.d. de l'ensemble des entiers $k$ pour lesquels il existe au moins un circuit de longueur $k$ passant par $i$.

Chapitre II. Un peu de théorie algébrique des graphes

**Exemple II.2.13.** Dans l'exemple II.2.8, on a $[A(G)^n]_{3,3} &gt; 0$ pour tout $n \geq 3$ et le p.g.c.d. des éléments de l'ensemble $X = \{3,4,5,\ldots\}$ est 1. Ainsi, le sommet 3 est de période 1.

Par contre, dans l'exemple II.2.9, $[A(G)^n]_{i,i} &gt; 0$ si et seulement si $n$ est un multiple de 3. Ainsi, les périodes des sommets de ce graphe sont toutes égales à 3.

**Lemme II.2.14.** Soient $i, j$ deux indices de $A \geq 0$. S'il existe $m, n$ tels que $[A^m]_{i,j} &gt; 0$ et $[A^n]_{j,i} &gt; 0$, alors $p(i) = p(j)$.

**Remarque II.2.15.** En utilisant la remarque II.2.12, le lemme se réexprime comme suit. S'il existe dans $G_A$ un chemin de longueur $m$ joignant $i$ et $j$ et un chemin de longueur $n$ joignant $j$ et $i$, autrement dit si $i \leftrightarrow j$, alors les deux sommets ont même période. (Ou encore, tous les sommets d'une composante f. connexe ont même période.)

**Démonstration.** Pour tout $s$ tel que $[A^s]_{j,j} &gt; 0$, on a

$$
\begin{aligned}
[ A^{m + s + n} ]_{i, i} &amp;= \sum_{k = 1}^{d} [ A^{m + s} ]_{i, k} [ A^{n} ]_{k, i} \\
&amp;\geq [ A^{m + s} ]_{i, j} [ A^{n} ]_{j, i} \\
&amp;= \sum_{k = 1}^{d} [ A^{m} ]_{i, k} [ A^{s} ]_{k, j} [ A^{n} ]_{j, i} \\
&amp;\geq [ A^{m} ]_{i, j} [ A^{s} ]_{j, j} [ A^{n} ]_{j, i} &gt; 0
\end{aligned}
$$

Les deux inégalités proviennent du fait que les éléments de $A$ sont positifs ou nuls. Pour un tel $s$, on a aussi $[A^{2s}]_{j,j} &gt; 0$ (en effet, si on effectue le produit matriciel $[A^s.A^s]_{j,j}$, on retrouve le terme $[A^s]_{j,j}.[A^s]_{j,j}$ et les autres termes de la somme sont positifs ou nuls). Dès lors, on a aussi

$$
[ A^{m + 2s + n} ]_{i, i} &gt; 0.
$$

Par conséquent, $p(i)$ divise $m + 2s + n$ et $m + s + n$ et aussi leur différence, $s$. En conclusion, pour tout $s$ tel que $[A^s]_{j,j} &gt; 0$, $p(i)$ divise $s$ et donc $p(i) \leq p(j)$. Par symétrie, on a aussi que $p(j) \leq p(i)$ et donc $p(i) = p(j)$.

Grâce à ce lemme, nous pouvons introduire la définition suivante. En effet, pour une matrice irréductible, toutes les périodes sont nécessairement identiques. En particulier, tous les sommets d'une composante f. connexe de $G_A$ ont même période.

**Définition II.2.16.** Une matrice irréductible $A \in \mathbb{R}_d^d$ est cyclique de période $p$ si tous les indices de $A$ sont de période $p &gt; 1$. Sinon, tous les indices sont de période $p = 1$ et $A$ est dite acyclique.

$^8$ $p(i)$ est un diviseur commun des éléments de l'ensemble $\{s &gt; 0 \mid [A^s]_{j,j} &gt; 0\}$ et d'autre part, $p(j)$ le p.g.c.d. de cet ensemble.

II.2. Théorie de Perron-Frobenius

**Lemme II.2.17.** Soit $A \geq 0$ une matrice carrée irréductible de période $p \geq 1$. Soit $i$ un indice de $A$. Il existe $N_i \geq 0$ tel que pour tout $n \geq N_i$, $[A^{np}]_{i,i} &gt; 0$.

**Démonstration.** Supposons tout d'abord que $[A^{kp}]_{i,i} &gt; 0$ et $[A^{\ell p}]_{i,i} &gt; 0$. Dès lors (même raisonnement que dans la preuve du lemme II.2.14),

$$
[A^{(k+\ell)p}]_{i,i} \geq [A^{kp}]_{i,i} [A^{\ell p}]_{i,i} &gt; 0.
$$

Cela signifie que l'ensemble $\mathcal{S}$ des multiples $np$ de $p$ qui sont tels que $[A^{np}]_{i,i} &gt; 0$ est stable pour l'addition (et $\mathcal{S}$ contient au moins un multiple de $p$). De plus, par définition, le p.g.c.d. des éléments de $\mathcal{S}$ vaut $p$. La conclusion découle alors du lemme suivant.

**Lemme II.2.18.** Soit $X \subseteq \mathbb{N}$ un ensemble d'entiers stable pour l'addition. Alors $X$ contient tous les multiples du p.g.c.d. des éléments de $X$ à l'exception éventuellement d'un nombre fini d'entre eux.

**Démonstration.** Soit $p$ le p.g.c.d. des éléments de $X$. Quitte à diviser les éléments de $X$ par $p$, on peut supposer que $p = 1$. Dès lors, il existe un ensemble fini⁹ $\{x_1, \ldots, x_k\} \subseteq X$ tel que

$$
\text{p.g.c.d. } \{x_1, \ldots, x_k\} = 1.
$$

Par le théorème de Bezout, il existe des entiers relatifs $\lambda_1, \ldots, \lambda_k \in \mathbb{Z}$ tels que

$$
\lambda_1 x_1 + \cdots + \lambda_k x_k = 1.
$$

Si on regroupe tous les termes dont les coefficients $\lambda_i$ sont positifs (resp. négatifs), cette somme se réécrit

$$
m - n = 1
$$

avec $m, n \in X$ car $X$ est stable pour l'addition. Soit $q$ un entier tel que $q \geq n(n-1)$. Par division euclidienne,

$$
q = a n + b, \quad 0 \leq b &lt; n.
$$

De plus, $a \geq n - 1$. Puisque $m - n = 1$, il vient

$$
q = a n + b (m - n) = (a - b) n + b m
$$

⁹Nous savons que le p.g.c.d. de $X = \{x_1 &lt; x_2 &lt; \cdots\}$ vaut 1. Si on considère tout d'abord $X$ restreint à $\{x_1\}$ seul, le p.g.c.d. potentiel serait $x_1$. Puis, quand on considère $\{x_1, x_2\}$, le p.g.c.d. potentiel ne peut que diminuer (ou au mieux rester constant) puisqu'on doit considérer cette fois les facteurs premiers communs à $x_1$ et à $x_2$. A l'étape suivante, on considère $\{x_1, x_2, x_3\}$ et ainsi de suite (à chaque étape, le p.g.c.d. décroît). Nous affirmons qu'il existe $k$ tel que le p.g.c.d. de $\{x_1, \ldots, x_k\}$ soit 1 car si tel n'était pas le cas, le p.g.c.d. de $X$ serait &gt; 1, ce qui est contraire à notre supposition. Remarquons en particulier que $k$ peut être &gt; 2, ne serait-ce par exemple qu'avec l'ensemble fini $\{6, 10, 15\}$ dont le p.g.c.d. vaut 1 mais qui est tel que ses éléments pris deux à deux ne sont pas premiers entre eux.

Chapitre II. Un peu de théorie algébrique des graphes

avec $a - b \geq 0$. On en conclut que $q$ appartient à $X$ (car $m, n \in X$). Nous avons donc montré que tout entier $q \geq n(n - 1)$ appartient à $X$. Cela termine la preuve.

Le résultat suivant montre que l'exemple II.2.9 est l'archétype même de la situation rencontrée dans le cas d'un graphe irréductible.

**Théorème** II.2.19. Soit $A \geq 0$ une matrice carrée irréductible de période $p \geq 1$. Pour toute paire $i, j$ d'indices de $A$, il existe un unique entier $r_{i,j} \in \{0, \ldots, p - 1\}$ tel que

- $[A^n]_{i,j} &gt; 0$ entraîne $n \equiv r_{i,j} \pmod{p}$ et
- il existe $N_{i,j}$ tel que $[A^{np + r_{i,j}}]_{i,j} &gt; 0$ pour tout $n \geq N_{i,j}$.

**Exemple** II.2.20. On peut reprendre l'exemple II.2.9. Dans cet exemple, $p = 3$ et si on fixe le sommet $i = 2$, on a $r_{2,1} = 2$, $r_{2,2} = 0$ et $r_{2,3} = 1$. En effet, $[A(G)^{3n + 2}]_{2,1} = 1$, $[A(G)^{3n + 0}]_{2,2} = 1$ et $[A(G)^{3n + 1}]_{2,3} = 1$.

**Démonstration.** Supposons que $[A^m]_{i,j} &gt; 0$ et $[A^n]_{i,j} &gt; 0$. Nous allons montrer que $m \equiv n \pmod{p}$. Puisque $A$ est irréductible, il existe $\ell$ tel que $[A^\ell]_{j,i} &gt; 0$. Dès lors,

$$
[ A ^ {m + \ell} ] _ {i, i} \geq [ A ^ {m} ] _ {i, j} [ A ^ {\ell} ] _ {j, i} &gt; 0 \quad \text{et} \quad [ A ^ {n + \ell} ] _ {i, i} &gt; 0.
$$

La période $p$ divise donc $m + \ell$ et $n + \ell$ donc leur différence. Autrement dit, $m - n \equiv 0 \pmod{p}$.

Passons à la deuxième partie. Puisque $A$ est irréductible, il existe $\ell$ tel que $[A^\ell]_{i,j} &gt; 0$ et au vu de la première partie,

$$
\ell = m p + r _ {i, j}.
$$

Posons

$$
N _ {i, j} = N _ {i} + m
$$

(avec $N_{i}$ donné dans le lemme II.2.17). Par définition de $N_{i}$, on a

$$
\forall n \geq N _ {i}, [ A ^ {n p} ] _ {i, i} &gt; 0.
$$

De là, si $k \geq N_{i,j}$, alors

$$
k p + r _ {i, j} = (n + m) p + r _ {i, j} \quad \text{avec} \quad n \geq N _ {i}.
$$

et

$$
[ A ^ {k p + r _ {i, j}} ] _ {i, j} \geq [ A ^ {n p} ] _ {i, i} [ A ^ {m p + r _ {i, j}} ] _ {i, j} &gt; 0.
$$

**Proposition** II.2.21. Une matrice irréductible est acyclique si et seulement si elle est primitive.

**Démonstration.** Si la matrice est acyclique (i.e., de période $p = 1$), alors avec les notations du théorème II.2.19, $r_{i,j} = 0$ quels que soient les indices $i$ et $j$. On en conclut que

$$
[ A ^ {n} ] _ {i, j} &gt; 0 \quad \text{si} \quad n \geq N _ {i, j}.
$$

II.2. Théorie de Perron-Frobenius

Ainsi, si on pose  $\mathcal{N} = \sup_{i,j} N_{i,j}$ , alors  $A^{\mathcal{N}} &gt; 0$  et  $A$  d'être primitive.

Réciproquement si  $A$  est primitive,  $A$  est nécessairement irréductible et pour  $k$  suffisamment grand et pour tout indice  $i$  de  $A$ ,  $[A^k]_{i,i} &gt; 0$  et  $[A^{k+1}]_{i,i} &gt; 0$ . Le p.g.c.d. de  $k$  et de  $k+1$  étant 1, la conclusion en découle.

Exemple II.2.22. Terminons par une application des résultats precedents et considérons les graphes représentés à la figure II.8. Le sommet  $A$  appar-

![img-102.jpeg](img-102.jpeg)
FIGURE II.8. Trois graphes f. connexes.

tient visiblement à un cycle de longueur 1 (une boucle). Par conséquent, sa période vaut 1 et le graphe est primitif au vu de la proposition précédente. Le sommet  $B$  appartient quant à lui à un cycle de longueur 2 et à un cycle de longueur 3. Ainsi, le p.g.c.d. des longueurs des cycles passant par  $B$  vaut 1 et le graphe est encore une fois primitif. En particulier, cela signifie que pour tout  $n$  suffisamment grand, il existe un chemin de longueur  $n$  entre tout couple de sommets. Enfin, tout cycle contenant  $C$  est de longueur  $4p + 2q$ ,  $p, q \geq 0$ . On en conclus que la période des différents sommets de ce dernier graphe vaut 2. Pour terminer cet exemple, notons encore que les chemins joignant  $D$  à  $E$  sont de longueur 1, 5, 7, 9, ... ainsi on dispose d'un chemin entre  $D$  et  $E$  si et seulement si sa longueur est de la forme  $1 + 2n$  avec  $n \geq 2$  ce qui illustré parfaitement la seconde partie du théorème II.2.19.

Signalons sans démonstration un dernier résultat, sorte de réciproque au théorème de Perron-Frobenius.

Proposition II.2.23. Si  $A \geq 0$  est une matrice irréductible possédant une valeur propre dominante  $\lambda$  (i.e., pour toute valeur propre  $\mu \neq \lambda$  de  $A$ ,  $|\mu| &lt; \lambda$ ), alors  $A$  est primitive.

2.2. Estimation du nombre de chemins de longueur  $n$ . Le théorème de Perron permet de donner le comportement asymptotique du nombre de chemins de longueur  $n$  joignant deux sommets quelconques d'un graphe dont la matrice d'adjacence est primitive.

Chapitre II. Un peu de théorie algébrique des graphes

Avec les notations du théorème de Perron, si  $A$  est une matrice primitive, alors il est possible de montré que

$$
A ^ {k} = \lambda_ {A} ^ {k} v _ {A} \widetilde {w _ {A}} + o \left(\lambda_ {A} ^ {k}\right) \tag {3}
$$

ou  $v_{A}$  et  $\widetilde{w_{A}}$  sont des vecteurs propres choisis de telle sorte que  $\widetilde{w_{A}}.v_{A} = 1$ . Remarquez que  $v_{A}\widetilde{w_{A}}$  est une matrice carrée de dimension  $n$  dont les éléments sont tous strictement positifs. Autrement dit, ce résultat stipule qu'asymptotiquement, tout élément de  $A^{k}$  est proportionnel à  $\lambda_{A}^{k}$  (la constante de proportionnalité dépendant de l'élement envisagé). Tout comme nous avons admis le théorème de Perron, nous admettrons aussi ce résultat sortant du cadre que nous nous sommes fixés dans ce cours.

**Remarque** II.2.24. Il est même possible d'obtenir des développements plus fins du terme d'erreur en l'exprimant à l'aide de la deuxième valeur propre (par module décroissant) de  $A$ . Voir par exemple [26].

**Exemple II.2.25.** Si on poursuit l'exemple II.2.8, pour tout couple  $(i,j) \in \{1,2,3\} \times \{1,2,3\}$ , il existe une constante  $d_{i,j} &gt; 0$  telle que le nombre  $c_{i,j}(n)$  de chemins de longueur  $n$  joignant  $i$  à  $j$  satisfasse

$$
\lim  _ {n \to \infty} \frac {c _ {i , j} (n)}{d _ {i , j} \lambda_ {A} ^ {n}} = 1.
$$

**Remarque** II.2.26. Dans le cas d'un graphe f. connexe qui n'est pas primitif, on dispose du théorème de Perron-Frobenius. Néanmoins, si on a plusieurs valeurs propres de module maximum, des compensations entre celles-ci peuvent se produit, et fournir une estimation des  $c_{i,j}(n)$  semblable à celle donnée ci-dessus n'est pas si simple. En effet, si on reprend une fois encore l'exemple II.2.9 et que l'on s'intéresse à la suite formée des nombres de chemins de longueur  $n$  joignant 1 à 3 pour  $n = 0,1,2,\ldots$ , on obtient

$$
0, 0, 1, 0, 0, 1, 0, 0, 1, \dots
$$

qui est clairément une suite divergente. Ainsi, la limite

$$
\lim  _ {n \to \infty} \frac {c _ {1 , 3} (n)}{\lambda_ {A} ^ {n}}
$$

n'existe pas! Ceci s'explique par le fait que des combinaisons convenables de puissances des racines de l'unité s'annulent $^{12}$ :

$$
\frac {(e ^ {2 i \pi / 3}) ^ {n} + (e ^ {4 i \pi / 3}) ^ {n} + 1}{3} = 0, \mathrm {s i} n \equiv 1, 2 \pmod {3}.
$$

$M^k)_{i,j} = \sum_{t=1}^{p} P_{i,j}^{(t)} \lambda_t^k$

II.2. Théorie de Perron-Frobenius

2.3. Cas d'un graphe ayant plusieurs composantes f. connexes. Nous pouvons obtenir des résultats plus fins que ceux obtenus ci-dessus. On considère le condensé  $\mathcal{C}$  d'un graphe  $G$  (ou graphe acyclique des composantes cf. définition I.4.27) dont les sommets sont les composantes f. connexes de  $G$ . Puisque le condensé est sans cycle, on peut ordonner ses sommets par tri topologique. Supposons disposer d'un tel ordonnancement. La matrice d'adjacence de  $\mathcal{C}$  a alors une forme triangulaire supérieure et si on ordonne les éléments de  $G$  en considérant les sommets des composantes f. connexes prises une à une en respectant le tri topologique, la matrice d'adjacence de  $G$  est alors une matrice bloc triangulaire supérieure.

Example II.2.27. Poursuivons l'exemple I.4.29. Pour celui-ci, on obtient la matrice suivante si on considère l'ordre donné à la figure I.38.

|  A(G)= | 0 1 0 | 0 0 0 | 0 0 0 | 0 0 0 | 0 0 0 | 0 0 0  |
| --- | --- | --- | --- | --- | --- | --- |
|   |  0 0 1 | 0 0 0 | 0 0 0 | 0 0 0 | 0 0 0 | 0 0 0  |
|   |  1 0 0 | 1 0 0 | 0 1 0 | 0 0 0 | 0 0 0 | 0 0 0  |
|   |  0 0 0 | 0 1 0 | 1 0 0 | 0 0 0 | 0 0 0 | 0 0 0  |
|   |  0 0 0 | 0 0 1 | 0 0 0 | 0 0 0 | 0 0 0 | 0 0 0  |
|   |  0 0 0 | 0 0 1 | 0 0 0 | 0 0 0 | 0 0 1 | 0 0 0  |
|   |  0 0 0 | 0 0 0 | 0 0 0 | 0 1 0 | 0 0 0 | 0 0 0  |
|   |  0 0 0 | 0 0 0 | 0 0 0 | 0 0 1 | 0 0 0 | 0 0 0  |
|   |  0 0 0 | 0 0 0 | 0 0 0 | 0 0 0 | 1 0 0 | 0 0 0  |
|   |  0 0 0 | 0 0 0 | 0 0 0 | 0 0 0 | 1 1 0 | 0 0 0  |

Remarque II.2.28. Au vu de l'exemple précédent, il est immédiat d'observer que le spectre d'un graphe est l'union des spectres de ses composantes connexes.

Nous nous restreignons une fois encore au cas où les composantes f. connexes sont des sous-graphes primitifs.

Soit un graphe fini possèdant deux composantes f. connexes primitives  $A$  et  $B$  et deux sommets  $a \in A$  et  $b \in B$  tels que  $a \to b$ . Les chemins joignant un sommet de  $A$  à un sommet de  $B$  sont en nombre fini et de longueur bornée. Si  $\lambda_A$  et  $\lambda_B$  sont les valeurs propres de Perron de  $A$  et de  $B$  respectivement, on en déduit que le nombre  $c_{a,b}(n)$  de chemins de longueur  $n$  joignant  $a$  à  $b$  est proportionnel $^{13}$  à

$$
\sum_ {i = 0} ^ {n} \lambda_ {A} ^ {i} \lambda_ {B} ^ {n - i} = \lambda_ {B} ^ {n} \sum_ {i = 0} ^ {n} \left(\frac {\lambda_ {A}}{\lambda_ {B}}\right) ^ {i}.
$$

Chapitre II. Un peu de théorie algébrique des graphes

![img-103.jpeg](img-103.jpeg)
FIGURE II.9. Deux composantes f. connexes.

- Si  $\lambda_{A} = \lambda_{B}$ , alors
$c_{a,b}(n)\asymp n\lambda^n$

- Sinon,  $\lambda_A \neq \lambda_B$  et

$$
\sum_ {i = 0} ^ {n} \lambda_ {A} ^ {i} \lambda_ {B} ^ {n - i} = \frac {\lambda_ {A} ^ {n + 1} - \lambda_ {B} ^ {n + 1}}{\lambda_ {A} - \lambda_ {B}} \asymp [ \max (\lambda_ {A}, \lambda_ {B}) ] ^ {n}.
$$

Nous détaillons à présent le cas d'un graphe fini ayant trois composantes f. connexes  $A$ ,  $B$  et  $C$ . Nous désirons estimer le nombre  $c_{a,B,c}(n)$  de chemins de longueur  $n$  joignant  $a \in A$  à  $c \in C$  en passant par un sommet quelconque de  $B$ . Le lecteur pourra aisément adapter les raisonnements au cas général.

![img-104.jpeg](img-104.jpeg)
FIGURE II.10. Trois composantes f. connexes.

Avec les mêmes notations que ci-dessus, puisque le nombre de sommets de  $B$  est fini, le nombre de chemins recherché est proportionnel à

$$
\sum_ {i = 0} ^ {n} \sum_ {j = 0} ^ {n - i} \lambda_ {A} ^ {i} \lambda_ {B} ^ {j} \lambda_ {C} ^ {n - i - j}. \tag {4}
$$

Nous traitons trois cas.

- Si  $\lambda_{A} = \lambda_{B} = \lambda_{C}$ , alors (4) devient

$$
\lambda_ {A} ^ {n} \sum_ {i = 0} ^ {n} (n + 1 - i) = \lambda_ {A} ^ {n} \left[ (n + 1) ^ {2} - \frac {n (n + 1)}{2} \right] \asymp n ^ {2} \lambda_ {A} ^ {n}.
$$

II.3. Une application : l'algorithm de PageRank

- Si  $\lambda_{A} = \lambda_{B} \neq \lambda_{C}$  (par symétrie, les autres cas se traitent de la même façon), alors (4) devient

$$
\sum_ {i = 0} ^ {n} \lambda_ {A} ^ {i} \underbrace {\sum_ {j = 0} ^ {n - i} \lambda_ {A} ^ {j} \lambda_ {C} ^ {n - i - j}} _ {\substack {\lambda_ {A} ^ {n - i + 1} - \lambda_ {C} ^ {n - i + 1} \\ \lambda_ {A} - \lambda_ {C}}} = \frac {(n + 1) \lambda_ {A} ^ {n + 1}}{\lambda_ {A} - \lambda_ {C}} - \frac {\lambda_ {C}}{\lambda_ {A} - \lambda_ {C}} \underbrace {\sum_ {i = 0} ^ {n} \lambda_ {A} ^ {i} \lambda_ {C} ^ {n - i}} _ {\substack {\lambda_ {A} ^ {n + 1} - \lambda_ {C} ^ {n + 1} \\ \lambda_ {A} - \lambda_ {C}}}.
$$

Ainsi, si  $\lambda_{A} = \lambda_{B} &gt; \lambda_{C}$ , on trouve

$$
c _ {a, B, c} (n) \asymp n \lambda_ {A} ^ {n}
$$

et si  $\lambda_{A} = \lambda_{B} &lt;   \lambda_{C}$  , on trouve

$$
c _ {a, B, c} (n) \asymp \lambda_ {C} ^ {n}.
$$

- Enfin, si  $\lambda_A, \lambda_B$  et  $\lambda_C$  sont 2 à 2 distincts, alors (4) devient

$$
\sum_ {i = 0} ^ {n} \underbrace {\lambda_ {A} ^ {i} \sum_ {j = 0} ^ {n - i} \lambda_ {B} ^ {j} \lambda_ {C} ^ {n - i - j}} _ {\substack {\lambda_ {B} ^ {n - i + 1} - \lambda_ {C} ^ {n - i + 1} \\ \lambda_ {B} - \lambda_ {C}}} = \frac {1}{\lambda_ {B} - \lambda_ {C}} \left(\lambda_ {B} \sum_ {i = 0} ^ {n} \lambda_ {A} ^ {i} \lambda_ {B} ^ {n - i} - \lambda_ {C} \sum_ {i = 0} ^ {n} \lambda_ {A} ^ {i} \lambda_ {C} ^ {n - i}\right)
$$

et on obtient

$$
c _ {a, B, c} (n) \asymp [ \max (\lambda_ {A}, \lambda_ {B}, \lambda_ {C}) ] ^ {n}.
$$

En conclusion, il suffit de détecter la plus grande valeur de Perron  $\lambda$  des différentes composantes connexes par lesquelles passent les chemins d'intérêt et de compter le nombre  $k$  de composantes ayant cette valeur propre comme valeur dominante. Le nombre de chemins de longueur  $n$  se comporte alors asymptotiquement comme  $n^{k-1}\lambda^n$ . Le lecteur adaptera facilement les développements effectués ci-dessus au cas d'un nombre arbitraire de composantes primitives. On pourrait aussi regarder la forme de Jordan de la matrice d'adjacence associée, un bloc de taille  $k$  associé à la valeur propre  $\lambda$  fait intervenir un polynôme de degré  $k-1$  multiplé par une exponentielle  $\lambda^n$ .

# 3. Une application : l'algorithm de PageRank

Dans cette section, nous allons illustrer certainement l'une des applications les plus célibres du théorème de Perron-Frobenius: le succès de Google basé sur leur algorithme de classement des pages web! Volontairement, plusieurs aspects ne sont pas pris en compte dans cette presentation (développement et efficacité des méthodes numériques, choix de la constante  $\alpha$ , mise en pratique, sensibilité aux perturbations, modèle plus évolué, ...). Pour plus de détails, consulter par exemple l'excellent ouvrage de C. Meyer et A. Langville consacre entièrement à ce sujet.

Chapitre II. Un peu de théorie algébrique des graphes

Lorsqu'on effectue une recherche sur un mot clé donné, Google trie les pages contenant ce mot clé en se basant (notamment $^{15}$ ) sur une mesure, appelée “PageRank”, destinée à quantifier la qualité des pages et à déterminer si elles font ou non autorité dans le domaine envisagé. L'obtention d'un “bon” classement des pages sur un sujet donné est d'ailleurs l'une des raisons pour lesquelles Google est si populaire.

L'idée originale de L. Page et S. Brin $^{16}$  est très simple:

- on accorde plus d'importance, i.e., un score de “PageRank” plus élevé, aux pages référencées par des pages qui font elles-mêmes autorité dans le domaine, c'est-à-dire qui ont un PageRank élevé;
- on accorde d'autant moins de crédit à une citation si elle provient d'une page qui dispose de nombreux liens (on ne peut qu'accorder moins de poids aux sites qui galvaudent leurs recommendations).

On suppose-disposer d'un graphe simple  $G = (V, E)$  ou les sommets notés  $1, \ldots, n$  représentent les pages de l'Internet (en 2005, on recensait plus de huit milliards de pages) et où l'on dispose d'un arc  $(i, j)$  si et seulement si la page  $i$  possède un lien pointant vers la page  $j$ . Au vu du modele proposé par Page et Brin, le PageRank  $\pi_j \geq 0$  de la page  $j \in \{1, \ldots, n\}$  serait donné par

(5)  $\pi_j = \sum_{i\in \mathrm{pred}(j)}\frac{\pi_i}{d^+ (i)}$

qui est une formule récursive pour laquelle on ne dispose pas a priori de méthode permettant d'assurer l'existence, l'unicité ou le calcul efficace d'une solution  $\pi = (\pi_1,\dots ,\pi_n)$  non triviale. On peut de plus supposer que les scores recherchés sont normalisés,

$$
\sum_ {i = 1} ^ {n} \pi_ {i} = 1.
$$

Le problème peut se réécrir sous forme matricielle ("  $H$  " comme "hyperlien"),

(6)  $\pi = \pi H$

ou

$$
H _ {i j} = \left\{ \begin{array}{l l} A (G) _ {i j} / d ^ {+} (i) &amp; \mathrm {s i} d ^ {+} (i) &gt; 0 \\ 0 &amp; \mathrm {s i} d ^ {+} (i) = 0 \end{array} \right.
$$

avec  $A(G)$  la matrice d'adjacence du graphe  $G$  et rappelons que  $\pi$  est un vecteur ligne. Sous cette forme, il n'est pas clair que le problème possède une solution (ni qu'elle soit unique et on dispose encore moins d'une méthode

II.3. Une application : l'algorithm de PageRank

|  α | nombre d'itérations  |
| --- | --- |
|  0,5 | 34  |
|  0,75 | 81  |
|  0,8 | 104  |
|  0,85 | 142  |
|  0,9 | 219  |
|  0,95 | 449  |
|  0,99 | 2292  |
|  0,999 | 23015  |

TABLE II.1. Rôle du paramètre  $\alpha$

de calcul). Autrement dit, il n'y a pas de raison evidente pour laquelle 1 est valeur propre de  $H$  et qui plus est, valeur propre simple.

L'astuce pour pouvoir appliquer la théorie de Perron-Frobenius est double. Tout d'abord, pour se débarrasser des "puits", i.e., des pages ne pointant vers aucune autre page et pour obtenir une matrice stochastique, on introduit une matrice  $S$  (" $S$ " comme "stochastique") définie par

$$
S _ {i j} = \left\{ \begin{array}{l l} A (G) _ {i j} / d ^ {+} (i) &amp; \mathrm {s i} d ^ {+} (i) &gt; 0 \\ 1 / n &amp; \mathrm {s i} d ^ {+} (i) = 0. \end{array} \right.
$$

Ensuite, pour assurer la forte connexité du graphe (i.e., le caractère irreductible), on construit une matrice  $G$  (" $G$ " comme Google) donnée par la combinaison affine (et même convexe) suivante avec un réel  $\alpha \in [0,1]$  fixé,

$$
G = \alpha S + (1 - \alpha) J / n
$$

ou  $J = (1)_{1\leq i,j\leq n}$  . L'equation initiale (6) est replacée par

$$
\pi = \pi G.
$$

(La matrice  $J / n$  est parfois appelée matrice de téléportation, cf. remarque suivante.) Le lecteur préférant manipuler des vecteurs propres à droite écrire  $\widetilde{\pi} = \widetilde{G}\widetilde{\pi}$ . Google attribue à  $\alpha$  une valeur de 0,85. Ce besoin n'est pas arbitraire. Au plus  $\alpha$  est proche de 1, au moins on approche le modele "naturel" (6) proposé initialement : on diminue le role artificiel de la matrice de téléportation. Cependant, on peut montré que ce paramètre  $\alpha$  contrôle la vitesse de convergence de la méthode de calcul développée et donc le nombre d'iterations à effectuer pour obtenir une estimation du vecteur  $\pi$  (par calcul de puissances successives de  $G$  qui contient plus de  $8.10^{9}$  entrées). Quand  $\alpha$  tend vers 1, ce nombre devient prohibitif comme le montre la table II.1 (tirée de l'ouvrage de C. Meyer et A. Langville). Ainsi,  $\alpha = 0,85$  semble un bon compromis entre le caractère artificiel introduit par la matrice de téléportation et la masse de calculs à réaliser. De plus, on peut

$G$  est un point du segment  $[G, J / n]$  de l'espace des matrices.

Monsieur Spock!

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

II.3. Une application : l'algorithm de PageRank

Démonstration. Soit  $M \in \mathbb{Q}_r^r$ . En multipliant tous les éléments de  $M$  par le p.p.c.m.  $\gamma$  des dénominateurs des éléments de  $M$ , la matrice

$$
M ^ {\prime} = \gamma M
$$

obtenue est telle que la somme des éléments de chaque ligne vaut  $\gamma \in \mathbb{N}$ . Il s'agit donc de la matrice d'adjacence d'un graphe  $\gamma$ -régulier. Comme nous le verrons, il suffit d'appliquer la proposition II.4.3 pour voir que  $\gamma M$  possède  $\gamma$  comme valeur propre dominante (i.e., toute autre valeur propre  $\mu$  est telle que  $|\mu| \leq \gamma$ ). La conclusion suit aisément en divisant par  $\gamma$ .

Par construction, la matrice  $G$  est primitive puisque toutes ses entrées sont strictement positives. On peut appliquer le théorème de Perron, or par le lemme précédent, nous savons déjà que 1 est valeur propre dominante de  $G$ . Par conséquent, la valeur propre dominante 1 est simple et il existe un unique vecteur colonne  $x &gt; 0$  (resp. ligne  $y &gt; 0$ ) tel que

$$
\sum_ {i = 1} ^ {n} x _ {i} = 1 \left(\text {r e s p .} \sum_ {i = 1} ^ {n} y _ {i} = 1\right) \quad \text {e t} \quad G x = x \left(\text {r e s p .} y G = y\right).
$$

Ainsi, déterminer le vecteur  $\pi$  des "PageRanks" revient à chercher le vecteur propre  $y$  de Perron à gauche de  $G$  (ou le vecteur propre de Perron à droite de  $\widetilde{G}$ ). En appliquant le résultat asymptotique (3) énoncé à la page 84, puisque  $e = (1\cdots 1)$  est un vecteur propre à droite de  $G$  de valeur propre 1 ( $G$  est stochastique) et que  $\pi.e = 1$  (puisque les scores sont normalisés), on a que

(7)  $G^{k} = e\pi + o(1)$  autrement dit,  $\lim_{k \to \infty} G^{k} = e\pi = \begin{pmatrix} 1 \\ \vdots \\ 1 \end{pmatrix} \left( \begin{array}{ccc} \pi_{1} &amp; \dots &amp; \pi_{n} \end{array} \right)$ .

Nous pouvons à présent obtenir aisément une méthode itérative pour estimer  $\pi$ . Soit  $p^{(0)} = \left(p_1^{(0)} \cdots p_n^{(0)}\right) &gt; 0$  un vecteur tel que  $\sum_{i} p_i^{(0)} = 1$ . Pour tout  $k \geq 1$ , on pose  $p^{(k)} = p^{(0)} G^k = p^{(k-1)} G$ . Il nous reste à montré que

$$
\lim  _ {k \rightarrow \infty} p ^ {(k)} = \pi
$$

ainsi, il suffit des lors de partir d'une distribution initiale et d'appliquer  $G$  de manière itérative jusqu'à obtenir la précision voulue mesurée par  $p^{(k)} - p^{(k-1)}$  (différence en norme, bien entendu). C'est immédiat, au vu de (7),

$$
\lim  _ {k \to \infty} G ^ {k} = \left( \begin{array}{c c c c} \pi_ {1} &amp; \pi_ {2} &amp; \dots &amp; \pi_ {n} \\ \vdots &amp; \vdots &amp; &amp; \vdots \\ \pi_ {1} &amp; \pi_ {2} &amp; \dots &amp; \pi_ {n} \end{array} \right) =: P
$$

et

$$
[ p ^ {(0)} P ] _ {j} = \sum_ {i = 1} ^ {n} p _ {i} ^ {(0)} \pi_ {j} = \pi_ {j} \underbrace {\sum_ {i = 1} ^ {n} p _ {i} ^ {(0)}} _ {= 1} = \pi_ {j}.
$$

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

II.4. Algèbre d'adjacence

linéairement indépendants et que  $\mathcal{A}_G$  contient donc au moins  $k + 1$  éléments linéairement indépendants.

Soit  $G = (V, E)$ , un multi-graphe non orienté connexe ayant  $A(G)$  comme matrice d'adjacence. Il est clair que puisque  $A(G)$  est diagonalisable, son polynôme minimum ne possède que des zéros simples et son degré est donc égal au nombre de valeurs propres distinctes de  $G$ . Remarquons aussi que si le polynôme minimum de  $A(G)$  est de degré  $k$ , il existe alors une relation linéaire liant  $A(G)^k, A(G)^{k-1}, \ldots, I$ . Par conséquent, la dimension de  $\mathcal{A}_G$  ne peut dépasser  $k$ .

Corollaire II.4.2. Soit  $G = (V, E)$ , un multi-graphe non orienté connexe ayant  $A(G)$  comme matrice d'adjacence et  $\mathcal{A}_G$  comme algèbre d'adjacence. Si  $\mathrm{diam}(G) = k$ , alors  $G$  a au moins  $k + 1$  valeurs propres distinctes.

Rappelons qu'un graphe non orienté est  $k$ -régulier si pour tout sommet  $v$ ,  $\deg(v) = k$ . Une fois encore, ces graphes possèdent des propriétés algébriques importantes.

Proposition II.4.3. Soit  $G = (V, E)$  un multi-graphe non orienté  $k$ -régulier. Alors

-  $k$  est une valeur propre de  $G$ ,
- pour toute valeur propre  $\lambda$  de  $G$ , on a  $|\lambda| \leq k$ ,
- si  $G$  est connexe,  $k$  est valeur propre simple (i.e., les multiplicités géométrique et algébrique valent 1).

Remarque II.4.4. Cette proposition peut aussi se réexprimer en termes de multi-graphes orientés  $k$ -réguliers. Il suffit de préciser au troisième point que  $G$  est fortement connexe.

Démonstration. Le premier point est immédiat. Il est clair que le vecteur  $(1, \ldots, 1)$  est un vecteur propre de  $A(G)$  de valeur propre  $k$ .

Passons au deuxième point et considérons une valeur propre  $\lambda$  de  $A(G)$  ayant  $y \neq 0$  comme vecteur propre. Soit  $y_{j}$  une composante de  $y$  de module maximum. On a

$$
| \lambda | | y _ {j} | = | [ A (G) y ] _ {j} | \leq \sum_ {i = 1} ^ {n} [ A (G) ] _ {j, i} | y _ {i} | \leq | y _ {j} | \sum_ {i = 1} ^ {n} [ A (G) ] _ {j, i} = k | y _ {j} |
$$

et donc,  $|\lambda| \leq k$ . On a utilisé le fait que les coefficients  $[A(G)]_{j,i}$  sont  $\geq 0$ , qu'au plus  $k$  d'entre eux sont non nuls et que  $\sum_{i=1}^{n} [A(G)]_{j,i} = k$ .

Pour le dernier point, puisque  $G$  est connexe,  $A(G)$  est irreductible et on peut donc utiliser le théorème de Perron-Frobenius. La matrice  $A(G)$  possède une unique valeur propre réelle dominante et au vu du point précédent, il s'agit de  $k$ .

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

II.5. Arbres couvrants

alors

$$
J = \frac {n}{q (k)} q (A (G)).
$$

**Démonstration.** Au vu de la preuve précédente, $q(A(G)) = \alpha J$. Les valeurs propres de $q(A)$ sont²² les $q(\lambda_i)$, $i = 1, \ldots, s$. Si $i \neq 1$, alors $q(\lambda_i) = 0$. Ainsi, $q(A(G))$ a comme seule valeur propre non nulle $q(k)$. De plus, $\alpha J$ a pour valeur propre $\alpha n$ (il suffit de considérer le vecteur propre $(1, \ldots, 1)$). On en conclut que $q(k) = \alpha n$ ou encore que $1 / \alpha = n / q(k)$.

---

## 5. Arbres couvrants

Revenons à présent au problème présenté dans l'exemple I.3.8. Il est clair que ce problème revient à chercher dans un graphe (simple²³ et non orienté) connexe un sous-arbre couvrant de poids minimal. (Etant entendu que le poids d'un graphe pondéré vaut la somme des poids de ses arêtes.)

Tout d'abord, nous allons considérer le problème plus simple consistant à rechercher un sous-arbre couvrant. Ce problème est immédiatement réglé par une recherche en profondeur²⁴. L'idée est de rechercher autant que faire se peut, des voisins successifs d'un sommet donné.

**Algorithme II.5.1** (Sous-arbre couvrant). La donnée de l'algorithme est un graphe connexe non orienté $G = (V, E)$.

```txt
Choisir un sommet $v_0 \in V$
Visite: $= \{v_0\}$, A:=∅
t:=0, j:=1
Tant que Visite ≠ V, répéter
Peut-on choisir un élément $v_j \in \nu(v_t) \backslash \text{Visite}$ ?
Si un tel choix est impossible,
il existe $k &lt; t$ maximum tel que $\{v_k, v_t\} \in \mathbb{E}$ et poser $t := k$
Si un tel choix est possible,
Visite: $= \{v_0, \ldots, v_j\}$, A:=A∪{$\{v_t, v_j\}$}, t:=j et j:=j+1
```

Ainsi, la variable Visite mémorise les sommets déjà visités et la variable A mémorise quant à elle les arêtes sélectionnées. Lorsque l'algorithme s'achève, A correspond à un sous-arbre couvrant de $G$. Les justifications sont laissées au lecteur.

**Exemple II.5.2.** Voici une application de l'algorithme précédent au graphe représenté à la figure II.11.

Pour un graphe donné $G$, nous savons trouver un sous-arbre couvrant mais bien évidemment un tel arbre n'est pas nécessairement unique. Notre

---

²² Se rappeler les résultats sur les polynômes d'endomorphismes.

²³ En effet, inutile de considérer des arêtes multiples ou des boucles dans ce type de problème.

²⁴ *Depth First Search.*

Chapitre II. Un peu de théorie algébrique des graphes

![img-105.jpeg](img-105.jpeg)
FIGURE II.11. Recherche d'un sous-arbre couvrant.

|  Visite | A | t | j  |
| --- | --- | --- | --- |
|  {v0=1} | ∅ | 0 | 1  |
|  {...,v1=5} | {1,5} | 1 | 2  |
|  {...,v2=3} | {1,5}, {5,3} | 2 | 3  |
|  {...,v3=4} | {1,5}, {5,3}, {3,4} | 3 | 4  |
|  {...,v3=4} | {1,5}, {5,3}, {3,4} | 2 | 4  |
|  {...,v4=2} | {1,5}, {5,3}, {3,4}, {3,2} | 4 | 5  |
|  {...,v4=2} | {1,5}, {5,3}, {3,4}, {3,2} | 2 | 5  |
|  {...,v5=6} | {1,5}, {5,3}, {3,4}, {3,2}, {3,6} | 5 | 6  |

TABLE II.2. Application de l'algorithmme II.5.1.

but est à présent de pouvoir compter le nombre  $\tau(G)$  de tels sous-arbres. Pour répondre à ce problème de dénombrement, plusieurs résultats intermédiaires sont nécessaires. Néanmoins, on peut d'abord obtenir assez facilement une formule récursive permettant de répondre (de manière assez fastidieuse) à la question.

5.1. Une formule de Cayley. Définissons tout d'abord une opération de contraction d'un sommet. Celle-ci va nous permettre de donner une formule récursive pour obtenir  $\tau(G)$ .

Définition II.5.3. Soient  $G = (V, E)$  un multi-graphe (non orienté) et  $e$  une arête de  $G$ . Le graphe obtenu en supprimant l'arête  $e$  et en identifiant les extrémités de celle-ci est appelé contraction de  $G$  (pour l'arête  $e$ ) et se note  $G \cdot e$ .

Si  $G$  est connexe et si  $e$  est une arête qui n'est pas une boucle, il en va de même de sa contraction  $G \cdot e$  qui contient une arête et un sommet de moins que  $G$ .

Proposition II.5.4. Si  $e$  est une arête (qui n'est pas une boucle) d'un multi-graphe connexe (non orienté), alors

$\tau (G) = \tau (G - e) + \tau (G\cdot e).$

II.5. Arbres couvrants

![img-106.jpeg](img-106.jpeg)
FIGURE II.12. Contraction d'un graphe  $G$  en  $G \cdot e$ .

Ainsi, cette formule permet d'exprimer  $\tau(G)$  à l'aide de deux graphes plus simples. On arrête la procédure lorsque la suppression d'une arête rend le graphe non connexe (autrement dit, lorsqu'on a un "pseudo"-arbre ne tenant pas compte des évientuelles boucles).

Démonstration. Tout sous-arbre couvrant de  $G$  qui ne contient pas  $e$  est aussi un sous-arbre couvrant de  $G - e$ . Ainsi,  $\tau(G - e)$  compte tous les sous-arbres couvrants de  $G$  qui ne contiennent pas  $e$ .

A chaque sous-arbre couvrant  $A$  de  $G$  qui contient  $e$ , il correspond un sous-arbre couvrant  $A \cdot e$  de  $G \cdot e$  et cette correspondance est une bijection. Ainsi, le nombre de sous-arbres couvrants de  $G$  qui contiennent  $e$  vaut exactement  $\tau(G \cdot e)$ .

Exemple II.5.5. Si on applique la formule de Cayley au graphe suivant de gauche sur la figure II.12, on obtient les décompositions reprises à la figure II.13. A chaque fois, on a représenté en traits pointillés, l'arête à laquelle est appliquée la formule. Ainsi, on s'aperçoit que le nombre d'arbres couvrants pour le graphe de départ vaut 8.

![img-107.jpeg](img-107.jpeg)
FIGURE II.13. Applications successives de la formule de Cayley.

Chapitre II. Un peu de théorie algébrique des graphes

Cet exemple montre bien que cette méthode est difficile àmettre en pratique pour des graphes de grande taille.

5.2. Une preuve bijective. En combinatoire et dans les problèmes de dénombrement, il est parfois aisé de compter le nombre d'éléments d'un ensemble en montrant que cet ensemble (fini) est en bijection avec un autre ensemble plus simple à énumérer ou à dénombreur. Cette méthode assez élégante est illustrée par le dénombrement des sous-arbres couvrants du graphe complet  $K_{n}$ .

Proposition II.5.6 (Cayley (1897)). Le nombre de sous-arbres couvrants du graphe complet  $K_{n}$  vaut  $n^{n - 2}$ . (Le nombre d'arbres à n sommets de labels distincts  $\{1, \ldots, n\}$  vaut  $n^{n - 2}$ .)

Remarque II.5.7. Dans l'énoncé précédent, il faut comprendre que chaque sommet du graphe est pourvu d'un label. A titre d'exemple,  $K_{3}$  possède 3 sous-arbres couvrants comme indiqué à la figure II.14. En effet, bien

![img-108.jpeg](img-108.jpeg)
FIGURE II.14. Nombre de sous-arbres couvrants.

qu'il s'agisse de trois arbres isomorphes, on les considère comme des arbres distincts à cause des labels portés par les différents sommets. Ainsi, notre formule de comptage va prendre en compte cette distinction!

La preuve donnée ici est à l'origine due à Prüfer. Ainsi, le codage des arbres que nous allonsprésenter est parfois appelé codage de Prüfer.

Démonstration. Numérotons les sommets de  $K_{n}$  de 1 à  $n$ . Il est clair que le nombre de sous-arbres couvrant  $K_{n}$  est égal au nombre d'arbres distincts que l'on peut construire avec des sommets numérotés de 1 à  $n$ . Il nous suffit donc de compter ces arbres.

Nous presentons un encodage (i.e., une bijection) d'un arbre  $A$  par une suite  $s$  de  $n - 2$  symboles appartenant à  $\{1, \ldots, n\}$ . Le nombre de telles suites est bien sur égal à  $n^{n - 2}$ .

Pour obtenir le  $j$ -ème élément de  $s$ , on supprime de  $A$  le plus petit sommet  $a_{j}$  de degré 1 et le  $j$ -ème élément de  $s$  est donné par le sommet adjacent à  $a_{j}$ . Considérons par exemple, l'arbre donné à la figure II.15. Le premier sommet supprimé est 2, son voisin est 1. Ensuite on supprime 5 de voisin 4, puis 4 de voisin 3, puis 3 de voisin 1, puis 1 de voisin 6 et enfin 6 de voisin 7. Cela nous donne la suite  $(1,4,3,1,6,6)$ . De par l'encodage, il est clair que les sommets apparaissant dans  $s$  sont exactement les sommets de degré au moins 2.

II.5. Arbres couvrants

![img-109.jpeg](img-109.jpeg)
FIGURE II.15. Un arbre encodé par (1,4,3,1,6,6).

![img-110.jpeg](img-110.jpeg)
FIGURE II.16. Un arbre encodé par  $(1,1,1)$ .

Réciproquement, si on se donne une suite  $s = (s_1, \ldots, s_{n-2})$  de  $n-2$  éléments de  $V = \{1, \ldots, n\}$ , il lui correspond un arbre  $A$  à  $n$  sommets dont  $s$  est l'encodage. Les sommets de  $V \setminus \{s_1, \ldots, s_{n-2}\}$  sont exactement les sommets de degré 1. Soit  $i_1$  le plus petit élément de  $V \setminus \{s_1, \ldots, s_{n-2}\}$ . L'arête  $\{i_1, s_1\}$  appartient à  $A$ . On recommence: soit  $i_2$  le plus petit élément de  $(V \setminus \{i_1\}) \setminus \{s_2, \ldots, s_{n-2}\}$ . L'arête  $\{i_2, s_2\}$  appartient à  $A$ . A la fin de la procédure,  $V$  contient deux sommets et on ajoute  $A$  l'arête formée par ceux-ci.

Ce résultat peut aussi s'obtenir comme corollaire d'un résultat de dénombrement plus général : compte le nombre d'arbres dont les sommets ont des labels distincts et les degrés sont fixés.

Théorème II.5.8. Le nombre d'arbres ayant  $n$  sommets de label respectif  $x_{1},\ldots ,x_{n}$  et dont les degrés sont disponibles par  $\deg (x_1) = d_1,\dots ,\deg (x_n) = d_n$  vaut le coefficient multinomial

$$
T _ {n, d _ {1}, \ldots , d _ {n}} := \left( \begin{array}{c} n - 2 \\ d _ {1} - 1, \dots , d _ {n} - 1 \end{array} \right) = \frac {(n - 2) !}{(d _ {1} - 1) ! \cdots (d _ {n} - 1) !},
$$

à condition qu'un arbre ayant de telles spécificités existe25.

Exemple II.5.9. Pour les arbres donnés à la figure II.14 de la remarque II.5.7, il y a un seul arbre à 3 sommets de label 1,2,3 et de degré respectif 2,1,1 (il y en a aussi un pour les degrés 1,2,1 et 1,1,2). La formule donne

Chapitre II. Un peu de théorie algébrique des graphes

bien

$$
\left( \begin{array}{c} 3 - 2 \\ 2 - 1, 1 - 1, 1 - 1 \end{array} \right) = \frac {1 !}{1 ! 0 ! 0 !} = 1.
$$

Démonstration. Commençons par quelques remarques préliminaires. Au vu de la remarque I.2.3,

$$
d _ {1} + \dots + d _ {n} = 2 \# E = 2 (n - 1).
$$

Donc,  $T \neq 0$  (i.e., il y a au moins un arbre correspondant aux données de l'énoncé) si et seulement si

$$
\sum_ {i = 1} ^ {n} (d _ {i} - 1) = 2 (n - 1) - n = n - 2.
$$

De plus, pour tout  $i$ ,  $1 \leq d_i \leq n - 1$ . Il existe une permutation  $\nu$  des sommets qui est telle que  $d_{\nu_1} \geq \dots \geq d_{\nu_n}$ . Pour simplifier l'écriture et ne rien changer au raisonnement, nous allons supposer que  $d_1 \geq d_2 \geq \dots \geq d_n$ . Au vu de l'égalité précédente, on en conclus que  $d_n = 1$ . En effet, si  $d_n \geq 2$ , alors  $\sum_{i=1}^{n} (d_i - 1) \geq 2n - n \geq n$ , ce qui est impossible au vu de la formule ci-dessus.

On a

$$
T _ {n, d _ {1}, \dots , d _ {n}} = \sum_ {i: d _ {i} \geq 2} T _ {n - 1, d _ {1}, \dots , d _ {i - 1}, d _ {i} - 1, d _ {i + 1}, \dots , d _ {n - 1}}.
$$

En effet, puisque  $x_{n}$  est de degré 1, on considère la famille  $C_i$  des arbres ayant  $x_{1},\ldots ,x_{n}$  pour sommets de degré respectif  $d_1,\dots ,d_n$  et dont le sommet  $x_{i}$  est de degré au moins 2 et pour lequel  $\{x_i,x_n\}$  est une arête de l'arbre. (Une illustration est donnée à la figure II.17.) Pour un  $i$  fixé, il suffit donc pour

![img-111.jpeg](img-111.jpeg)
FIGURE II.17. Deux arbres de  $C_i$  pour lesquels  $d_i = 2$  et  $d_i = 3$ .

énumérer les arbres de  $C_i$  d'énumérer les arbres ayant  $n - 1$  sommets de label  $x_1, \ldots, x_{i-1}, \mathbf{x_i}, x_{i+1}, \ldots, x_{n-1}$  et de degré  $d_1, \ldots, d_{i-1}, \mathbf{d_i} - \mathbf{1}, d_{i+1}, \ldots, d_{n-1}$ .

Nous pouvons à présent démontré le résultat annoncé en procédant par récurrence. Le cas de base  $n = 2$  est immédiat. Supposons donc  $n \geq 3$  et la

II.5. Arbres couvrants

formule vérifiée pour $n - 1$. Au vu de l'argumentation développée ci-dessus, il vient

$$
\begin{array}{l}
T_{n,d_1,\dots,d_n} = \sum_{i:d_i \geq 2} T_{n-1,d_1,\dots,d_{i-1},d_i-1,d_{i+1},\dots,d_{n-1}} \\
= \sum_{i:d_i \geq 2} \binom{n-3}{d_1-1,\dots,d_{i-1}-1,d_i-2,d_{i+1}-1,\dots,d_{n-1}-1} \\
= \binom{n-2}{d_1-1,d_2-1,\dots,d_{n-1}-1} \\
= \binom{n-2}{d_1-1,d_2-1,\dots,d_{n-1}-1,d_n-1}
\end{array}
$$

où, à la deuxième ligne, on utilise l'hypothèse de récurrence et pour obtenir l'avant-dernière ligne, nous avons utilisé le résultat suivant (nous sommes dans les conditions d'application, $d_1 - 1 + \dots + d_n - 1 = n - 2$ car $d_n = 1$.)

**Lemme II.5.10.** Pour tous $n, n_1, \ldots, n_p \in \mathbb{N}$ tels que $n_1 + \dots + n_p = n$, on a

$$
\binom{n}{n_1,\dots,n_p} = \sum_{i: n_i \geq 1} \binom{n-1}{n_1,\dots,n_{i-1},n_i-1,n_{i+1},\dots,n_p}.
$$

**Démonstration.** En effet, il est clair que

$$
(a_1 + \dots + a_p)^n = (a_1 + \dots + a_p) (a_1 + \dots + a_p)^{n-1}.
$$

En se rappelant le théorème multinomial$^{26}$, le terme général du membre de gauche est

$$
\binom{n}{n_1,\dots,n_p} a_1^{n_1} \cdots a_p^{n_p}
$$

et celui du membre de droite est

$$
\sum_{i:n_i \geq 1} a_i \binom{n-1}{n_1,\dots,n_{i-1},n_i-1,n_{i+1},\dots,n_p} a_1^{n_1} \cdots a_i^{n_i-1} \cdots a_p^{n_p}.
$$

La conclusion s'ensuit directement.

**Remarque II.5.11.** Ainsi, la proposition II.5.6 est un corollaire immédiat du théorème II.5.8. En effet, dans la proposition II.5.6, on compte juste

$$
(a_1 + \dots + a_p)^n = \sum_{\substack{n_1,\dots,n_p \in \mathbb{N} \\ n_1 + \dots + n_p = n}} \binom{n}{n_1,\dots,n_p} a_1^{n_1} \cdots a_p^{n_p}.
$$

Chapitre II. Un peu de théorie algébrique des graphes

le nombre d'arbres ayant  $n$  sommets (quel que soit leur degré). Ainsi, cela revient à calculer la somme suivante

$$
\begin{array}{l} \sum_{\substack{d_{1},\ldots ,d_{n}\geq 1\\ d_{1} - 1 + \dots +d_{n} - 1 = n - 2}}\binom {n - 2}{d_{1} - 1,\ldots ,d_{n} - 1} = \sum_{\substack{i_{1},\ldots ,i_{n}\geq 0\\ i_{1} + \dots +i_{n} = n - 2}}\binom {n - 2}{i_{1},\ldots ,i_{n}} \\ = \underbrace {(1 + \cdots + 1) ^ {n - 2}} _ {n \times} = n ^ {n - 2}. \\ \end{array}
$$

Corollaire II.5.12 (Clarke (1958)). Le nombre d'arbres ayant  $n$  sommets de label respectif  $x_{1},\ldots ,x_{n}$  et dont les degrés sont disponibles par  $\deg (x_1) = d_1 = k$ , ...,  $\deg (x_n) = d_n$  vaut

$$
\mathrm {C} _ {n - 2} ^ {k - 1} (n - 1) ^ {n - k - 1}.
$$

Démonstration. En effet, vu le théorème II.5.8, il suffit de calculer

$$
\sum_{\substack{d_{2},\ldots ,d_{n}\geq 1\\ k - 1 + d_{2} - 1 + \dots +d_{n} - 1 = n - 2}}\binom {n - 2}{k - 1,d_{2} - 1,\ldots ,d_{n} - 1}.
$$

Cette somme se réécrit

$$
\frac{(n - 2)!}{(k - 1)!(n - k - 1)!}\sum_{\substack{d_{2},\ldots ,d_{n}\geq 1\\ d_{2} - 1 + \dots +d_{n} - 1 = n - k - 1}}\binom {n - k - 1}{d_{2} - 1,\ldots ,d_{n} - 1}.
$$

D'ou la conclusion en procédant comme dans la remarque précédente.

5.3. Nombre de sous-arbres couvrants. Nous allons tout d'abord étendre le problème posé initialement et compter les sous-arbres couvrants, pointés et orientés depuis la racine dans un multi-graphe orienté  $G = (V, E)$  où  $V = \{v_1, \ldots, v_n\}$ . Nous supposerons de plus que ce multi-graphe est sans boucle. Ceci ne constitue pas une veritable restriction par rapport au problème envisagé ici27.

Un arbre pointé est orienté depuis la racine si les arcs constituant celui-ci sont tous orientés des sommets de niveau  $i$  vers les sommets de niveau  $i + 1$  (pour  $i$  allant de 0 à la hauteur de l'arbre). Bien évidemment, le graphe non orienté sous-jacent est lui-même un arbre.

Exemple II.5.13. A la figure II.18, on a représenté un graphe orienté et un sous-arbre couvrant orienté depuis la racine 1.

Définition II.5.14. Soit  $G = (V, E)$  un multi-graphe orienté sans boucle dont les sommets sont ordonnés par  $V = \{v_1, \ldots, v_n\}$ . La matrice  $D(G)$  de demi-degré entrant est définie par

$$
[ D (G) ] _ {j, j} = d ^ {-} (v _ {j}) \quad \text {e t} \quad [ D (G) ] _ {i, j} = - (\# (\omega^ {+} (v _ {i}) \cap \omega^ {-} (v _ {j}))), \quad \text {s i} i \neq j.
$$

II.5. Arbres couvrants

![img-112.jpeg](img-112.jpeg)
FIGURE II.18. Un sous-arbre couvrant pointé et orienté.

![img-113.jpeg](img-113.jpeg)

Autrement dit,  $[D(G)]_{i,j}$  est l'opposé du nombre d'arcs joignant  $v_{i}$  à  $v_{j}$ , si  $i \neq j$ . En conséquence de cette définition, il est clair que la somme des éléments de toute colonne de  $D(G)$  est nulle.

Example II.5.15. Pour le graphe de la figure II.18, on trouve

$$
D (G) = \left( \begin{array}{c c c c} 1 &amp; - 1 &amp; 0 &amp; - 1 \\ - 1 &amp; 2 &amp; - 1 &amp; - 1 \\ 0 &amp; 0 &amp; 1 &amp; 0 \\ 0 &amp; - 1 &amp; 0 &amp; 2 \end{array} \right).
$$

Remarque II.5.16. La matrice de demi-degré entrant associée à un sous-arbre couvrant pointé et orienté possède, à l'exception de la colonne correspondant à la racine, exactement un “-1” dans chaque colonne et une diagonale formée de 1 (on aboutit exactement une fois dans chaque sommet). La colonne correspondant à la racine est nulle puisqu'aucun arc n'aboutit à la racine. Ainsi, pour le sous-arbre de la figure II.18, on a la matrice

$$
M = \left( \begin{array}{c c c c} 0 &amp; - 1 &amp; 0 &amp; 0 \\ 0 &amp; 1 &amp; - 1 &amp; - 1 \\ 0 &amp; 0 &amp; 1 &amp; 0 \\ 0 &amp; 0 &amp; 0 &amp; 1 \end{array} \right).
$$

Nous allons introduire des sous-graphes particuliers de  $G$ . Soient  $i \in \{1, \ldots, n\}$  et  $G^{(i)} = (V, E \setminus \omega^{-}(v_i))$ , i.e., le sous-graphe obtenu en supprimant les arcs de  $G$  entrant dans  $v_i$ . Un tel sous-graphe revient à sélectionner une racine pour un arbre couvrant potentiel (en effet, aucun arc n'aboutit à la racine).

Example II.5.17. Pour le graphe de la figure II.18, on trouve par exemple

$$
D (G ^ {(3)}) = \left( \begin{array}{c c c c} 1 &amp; - 1 &amp; 0 &amp; - 1 \\ - 1 &amp; 2 &amp; 0 &amp; - 1 \\ 0 &amp; 0 &amp; 0 &amp; 0 \\ 0 &amp; - 1 &amp; 0 &amp; 2 \end{array} \right).
$$

Si  $[D(G^{(i)})]_{k,k} = r_k^{(i)}\geq 2$  , alors on peut écrire, de maniere unique, la  $k$  -ieme colonne de  $D(G^{(i)})$  comme une somme de  $r_k^{(i)}$  vecteurs-colonnes

$$
C _ {k, 1} ^ {(i)}, \ldots , C _ {k, r _ {k} ^ {(i)}} ^ {(i)}
$$

Chapitre II. Un peu de théorie algébrique des graphes

dont la  $k$ -ième composante vaut 1 et dont toutes les autres composantes sont nulles, exceptée l'une d'entre elles valant  $-1$ . Une telle décomposition revient à sélectionner pour chaque sommet un seul arc entrant, toujours dans l'optique de construire un potentiel arbre couvrant.

Exemple II.5.18. En continuant notre exemple,

$$
\underbrace {\left( \begin{array}{c} - 1 \\ 2 \\ 0 \\ - 1 \end{array} \right)} _ {C _ {2, 1} ^ {(3)}} = \underbrace {\left( \begin{array}{c} - 1 \\ 1 \\ 0 \\ 0 \end{array} \right)} _ {C _ {2, 2} ^ {(3)}} + \underbrace {\left( \begin{array}{c} 0 \\ 1 \\ 0 \\ - 1 \end{array} \right)} _ {C _ {2, 2} ^ {(3)}} \quad \text {e t} \quad \underbrace {\left( \begin{array}{c} - 1 \\ - 1 \\ 0 \\ 2 \end{array} \right)} _ {C _ {4, 1} ^ {(3)}} = \underbrace {\left( \begin{array}{c} - 1 \\ 0 \\ 0 \\ 1 \end{array} \right)} _ {C _ {4, 2} ^ {(3)}} + \underbrace {\left( \begin{array}{c} 0 \\ - 1 \\ 0 \\ 1 \end{array} \right)} _ {C _ {4, 2} ^ {(3)}}.
$$

Soient  $k_{1}^{(i)}, \ldots, k_{p}^{(i)}$  les indices des colonnes de  $D(G^{(i)})$  pour lesquelles  $[D(G^{(i)})]_{k,k} = r_{k}^{(i)} \geq 2$ . Si on désisit pour chacune de ces colonnes un des vecteurs  $C_{k_j,n}^{(i)}$ , pour  $j = 1, \ldots, p$  et  $n = 1, \ldots, r_{k_j}^{(i)}$ . Le nombre total de tels choix possibles vaut donc  $m^{(i)} = r_{k_1}^{(i)} \cdots r_{k_p}^{(i)} \geq 2^p$  et à chacun de ces choix, il correspond une matrice

$$
D ^ {(i)} _ {1}, \ldots , D ^ {(i)} _ {m ^ {(i)}}.
$$

En particulier, il est clair, par linéarité du déterminant par rapport aux colonnes, que

$$
\det  (D (G ^ {(i)})) = \sum_ {j = 1} ^ {m ^ {(i)}} \det  (D ^ {(i)} _ {j}). \tag {8}
$$

Remarquons aussi que pour un multi-graphe, il se peut que plusieurs matrices  $D^{(i)}_j$  soient identiques. Cela n'est nullement génant et il faut en tener compte pour compter exactement le nombre de sous-arbres couvrants.

Example II.5.19. Si on poursuit l'exemple,  $[D(G^{(3)})]_{2,2} = [D(G^{(3)})]_{4,4} = 2$  et on a donc  $m^{(3)} = 4$  matrices

$$
D _ {1} ^ {(3)} = \left( \begin{array}{c c c c} 1 &amp; - 1 &amp; 0 &amp; - 1 \\ - 1 &amp; 1 &amp; 0 &amp; 0 \\ 0 &amp; 0 &amp; 0 &amp; 0 \\ 0 &amp; 0 &amp; 0 &amp; 1 \end{array} \right),   D _ {2} ^ {(3)} = \left( \begin{array}{c c c c} 1 &amp; - 1 &amp; 0 &amp; 0 \\ - 1 &amp; 1 &amp; 0 &amp; - 1 \\ 0 &amp; 0 &amp; 0 &amp; 0 \\ 0 &amp; 0 &amp; 0 &amp; 1 \end{array} \right)
$$

et

$$
D _ {3} ^ {(3)} = \left( \begin{array}{c c c c} 1 &amp; 0 &amp; 0 &amp; - 1 \\ - 1 &amp; 1 &amp; 0 &amp; 0 \\ 0 &amp; 0 &amp; 0 &amp; 0 \\ 0 &amp; - 1 &amp; 0 &amp; 1 \end{array} \right),   D _ {4} ^ {(3)} = \left( \begin{array}{c c c c} 1 &amp; 0 &amp; 0 &amp; 0 \\ - 1 &amp; 1 &amp; 0 &amp; - 1 \\ 0 &amp; 0 &amp; 0 &amp; 0 \\ 0 &amp; - 1 &amp; 0 &amp; 1 \end{array} \right)
$$

telles que  $\operatorname{det}(D(G^{(3)})) = \sum_{i=1}^{4} \operatorname{det}(D_i^{(3)})$ .

A ce stade, à chaque matrice  $D(G_{\ell}^{(k)})$  correspond une sélection d'une racine et une sélection d'un arc entrant pour chaque sommet distinct de la racine. Pour un sélection convenable, il s'agira d'un arbre couvrant.

II.5. Arbres couvrants

![img-114.jpeg](img-114.jpeg)

![img-115.jpeg](img-115.jpeg)

![img-116.jpeg](img-116.jpeg)
FIGURE II.19. Graphes correspondant aux matrices  $D_1^{(3)}, D_2^{(3)}, D_3^{(3)}$  et  $D_4^{(3)}$ .

![img-117.jpeg](img-117.jpeg)

Exemple II.5.20. Aux matrices  $D_1^{(3)}, D_2^{(3)}, D_3^{(3)}$  et  $D_4^{(3)}$  de l'exemple précédent, correspondent les graphes de la figure II.19.

Remarque II.5.21. Supposons qu'un sous-arbre couvrant pointé en  $v_{i}$  et orienté existe. Alors, au vu de la remarque II.5.16, la matrice associée à cet arbre est exactement égale à une des matrices  $D^{(i)}_1, \ldots, D^{(i)}_{m^{(i)}}$ . Pour compter le nombre de sous-arbres couvrants pointés en  $v_{i}$ , il suffit donc de pouvoir désigner les matrices  $D^{(i)}_j$  qui correspondent à un sous-arbre couvrant de celles qui n'y correspondent pas.

Toute matrice  $D^{(i)}_j$  est une matrice de demi-degré entrant pour un certain sous-graphe de  $G$ , note  $G^{(i)}_j$ , dont chaque sommet a un demi-degré entrant valant au plus 1.

Théorème II.5.22. Soit  $G = (V, E)$  un graphe orienté dont le demi-degré entrant de chaque sommet vaut au plus 1. Le mineur  $M_{t,t}(G)$  de la matrice  $D(G)$  obtenu en supprimant la ligne et la colonne correspondant à  $v_t$  vaut

1, si  $G$  contient un sous-arbre couvrant pointé en  $v_{t}$  et orienté; 0, sinon.

Démonstration. Supposons que  $G$  contienne un sous-arbre couvrant  $A$  orienté et pointé en  $v_{t}$ . Puisque par hypothèse, le demi-degré entrant de chaque sommet vaut au plus 1, alors  $G = A$  ou bien  $G = A + e$  où  $e$  est une arête entrant dans  $v_{t}$ . On peut renumerer les sommets de  $A$  (donc de  $G$ ) par un parcours en largeur de l'arbre. Ainsi, l'arête  $v_{t}$  devient  $v_{1}$  et  $[D(G)]_{1,1} \leq 1$ . Pour  $i \geq 2$ ,  $[D(G)]_{i,i} = 1$  et

$$
\mathrm {s i} i &gt; j \geq 2, \mathrm {a l o r s} [ D (G) ] _ {i, j} = 0
$$

(c'est une conséquence de la renumeration des sommets). Si  $G = A + e$ , alors  $D(G)$  contient un “-1” dans la première colonne, mais cela n'a guère d'importance pour la suite. On conclut que la matrice  $D(G)$  privée de

Chapitre II. Un peu de théorie algébrique des graphes

sa première ligne et de sa première colonne est une matrice triangulaire supérieure de déterminant 1.

![img-118.jpeg](img-118.jpeg)
FIGURE II.20. Un arbre pointé parcouru en largeur (avec un arc supplémentaire).

![img-119.jpeg](img-119.jpeg)

Supposons à présent que  $G$  ne contienne pas de sous-arbre couvrant orienté et pointé en  $v_{t}$ . Si pour  $j \neq t$ ,  $d^{-}(v_{j}) = 0$ , alors la  $j$ -ème colonne de  $D(G)$  est nulle et on en conclus que  $M_{t,t}(G) = 0$ . Nous pouvons donc supposer que pour tout  $j \neq t$ ,  $d^{-}(v_{j}) = 1$ .

Soit  $j_1 \neq t$ . Puisque  $d^{-}(v_{j_1}) = 1$ , il existe un indice  $j_2$  et un arc joignant  $v_{j_2}$  à  $v_{j_1}$ . En continuant de la sorte, on obtient des sommets tous distincts  $v_{j_k}, \ldots, v_{j_1}$  tels que

$$
v _ {j _ {k}} \longrightarrow v _ {j _ {k - 1}} \longrightarrow \dots \longrightarrow v _ {j _ {2}} \longrightarrow v _ {j _ {1}}.
$$

Si en effectuant cette construction, on rencontres le sommet  $v_{t}$ , on considère alors un autre sommet que  $v_{j_1}$  pour initiaiser la construction. Nous affirmons qu'il existe au moins un sommet de  $G$  pour lequel la construction ne rencontres pas  $v_{t}$ . En effet, si ce n'était pas le cas, on aurait identifié à chaque fois un sous-arbre pointé en  $v_{t}$  et on obtiendrait des lors un sous-arbre couvrant pointé en  $v_{t}$ , ce qui n'est pas possible dans la situation envisagée ici. Dans la construction de  $v_{j_k},\ldots ,v_{j_1}$ , puisque le graphe contient un nombre fini de sommets, on finit par identifier un cycle. Les colonnes de  $D(G)$  correspondant aux sommets de ce cycle sont linéairement dépendantes car leur somme fait zéro. De là, on en conclus que  $M_{t,t}(G) = 0$ .

![img-120.jpeg](img-120.jpeg)
FIGURE II.21. Un cycle et des colonnes dont la somme est nulle.

![img-121.jpeg](img-121.jpeg)

Les graphes  $G^{(i)}_j$  satisfont l'hypothèse du théorème II.5.22. Au vu de ce théorème et de la remarque II.5.21, le nombre de sous-arbres couvrant  $G^{(i)}$  pointés en  $v_i$  vaut

$$
\sum_ {j = 1} ^ {m ^ {(i)}} M _ {i, i} (G ^ {(i)} _ {j}) = M _ {i, i} (G ^ {(i)}).
$$

II.5. Arbres couvrants

On a utilisé un raisonnement analogue à (8) appliqué ici au calcul d'un mineur. Ce nombre est aussi, bien évidemment, égal au nombre de sous-arbres couvrant  $G$  tout entier et pointés en  $v_{i}$ . Les matrices  $D(G^{(i)})$  et  $D(G)$  étant égales à l'exception de la  $i$ -ème colonne, on obtient le résultat suivant.

Théorème II.5.23 (Bott-Mayberry (1954)). Soit  $G$  un multi-graphe orienté sans boucle. Le nombre de sous-arbres couvrant  $G$  pointés au sommet  $v_{i}$  et orientés est égal au mineur  $M_{i,i}(G)$  de la matrice de demi-degré entrant de  $G$ .

Nous pouvons à présent reconsiderer notre problème initial concernant des graphes non orientés. A un graphe non orienté  $G = (V, E)$  correspond un graphe orienté  $G = (V, E')$  où chaque arête  $\{u, v\}$  de  $G$  donne lieu aux arcs  $(u, v)$  et  $(v, u)$  dans  $G'$ . Il est clair qu'à tout arbre couvrant dans  $G$ , il correspond dans  $G'$  exactement un arbre couvrant pointé en  $a$  et orienté depuis  $a$  et ce, pour tout sommet  $a$  de  $G'$ . La reciproque est également vraie. A tout arbre couvrant  $G'$  (pointé en un quelconque sommet et orienté), il correspond un arbre couvrant  $G$ .

Corollaire II.5.24. Le nombre de sous-arbres couvrant un multi-graphe  $G = (V, E)$  non orienté sans boucle vaut  $M_{i,i}(G')$  quel que soit  $i$ , où  $G'$  est le graphe symétrique orienté déduit de  $G$ .

Exemple II.5.25. Sur la figure II.22, on a représenté un graphe non orienté  $G$ , le graphe orienté symétrique  $G'$  correspondant à  $G$ , ainsi qu'un arbre couvrant  $G$  et les arbres couvrants correspondants dans  $G'$  avec comme racines respectives les différents sommets de  $G'$ .

![img-122.jpeg](img-122.jpeg)
FIGURE II.22. Nombre de sous-arbres couvrants pour  $G$  et  $G'$ .

$$
D (G ^ {\prime}) = \left( \begin{array}{c c c c} 3 &amp; - 1 &amp; - 1 &amp; - 1 \\ - 1 &amp; 2 &amp; 0 &amp; - 1 \\ - 1 &amp; 0 &amp; 2 &amp; - 1 \\ - 1 &amp; - 1 &amp; - 1 &amp; 3 \end{array} \right).
$$

Chapitre II. Un peu de théorie algébrique des graphes

On vérifie que tous les mineurs principaux valent 8. On a ainsi 8 arbres couvrants distincts ( comme le montre la figure II.23). Insistons une fois

![img-123.jpeg](img-123.jpeg)
FIGURE II.23. Les sous-arbres couvrants du graphe  $G$ .

encore sur le fait que nous avons compté l'ensemble des sous-arbres d'un graphe dont les sommets ont été numérotés. Ainsi, parmi les 8 arbres trouvés ci-dessus, seulement 3 sont 2 à 2 non isomorphes.

# 6. Arbres couvrants de poids minimal

Nous allons à présent considérer le problème de trouver non seulement un sous-arbre couvant mais un sous-arbre couvant de poids minimal. Rappelons que l'on se restreint au cas de graphes (non orientés) simples. (En effet, si on disposait de plusieurs arêtes joignant deux sommets, il suffirait de conserver l'arête de plus petit poids.) L'algorithmme de Prim répond à cette question.

Soit  $G = (V, E)$  un graphe connexe non orienté dont les arêtes sont pondérées par une fonction  $p: E \to \mathbb{R}^+$ . L'idée de cet algorithme est de construire le sous-arbre de proche en proche. Si on dispose déjà d'un sous-graphe  $G' = (V', A)$  connexe d'un arbre de poids minimal couvant  $G$ , on lui ajoute une arête de poids minimal parmi les arêtes joignant un sommet de  $V'$  à un sommet de  $V \setminus V'$ . Puisqu'il faut couvrir l'arbre tout entier, on débute la procédure avec un sous-graphe restreint à une quelconque arête de  $G$ .

Nous utilisons les mêmes conventions qu'à la remarque I.4.9. Ainsi,  $p(\{u,v\}) = +\infty$  si  $\{u,v\} \notin E$ .

Algorithm II.6.1 (Prim). La donnée de l'algorithmie est un graphe connexe non orienté  $G = (V, E)$ .

```latex
Choisir un sommet  $v_{0}\in V$ $\mathrm{V}^{\prime}:=\{v_{0}\}$  ，A:=0
Pour tout  $v\in V\setminus V^{\prime}$  L(v):=p({v0,v})
Tant que  $\mathrm{V}^{\prime}\neq V$  ，repeeter
Choisir  $u\in V\setminus V^{\prime}$  tel que L(u) soit minimal et e l'arête réalisant ce poids
$\mathrm{V}^{\prime}:=\mathrm{V}^{\prime}\cup\{u\}$

II.6. Arbres couvrants de poids minimal

```txt
A:=AU{e}
Pour tout  $v\in V\setminus V^{\prime}$  si  $p(\{u,v\}) &lt;   \mathrm{L}(v)$  , alors L(v):=p({u,v})
```

![img-124.jpeg](img-124.jpeg)
Exemple II.6.2. Appliquons l'algorithmme au graphe de la figure II.24.

![img-125.jpeg](img-125.jpeg)
FIGURE II.24. Application de l'algorithmme de Prim.

|  V' | A | L(a) | L(b) | L(c) | L(d) | L(e) | L(f) | L(g)  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  {a} | ∅ | - | 2 | 2 | 6 | +∞ | +∞ | 8  |
|  {a,b} | {{a,b}} | - | - | 2 | 4 | 3 | +∞ | 8  |
|  {a,b,c} | {..., {a,c}} | - | - | - | 4 | 2 | 3 | 4  |
|  {a,b,c,e} | {..., {c,e}} | - | - | - | 2 | - | 1 | 4  |
|  {a,b,c,e,f} | {..., {e,f}} | - | - | - | 2 | - | - | 1  |
|  {a,b,c,e,f,g} | {..., {f,g}} | - | - | - | 2 | - | - | -  |
|  {a,b,c,e,f,g,d} | {..., {d,e}} | - | - | - | - | - | - | -  |

TABLE II.3. Application de l'algorithmme de Prim.

Démonstration. On procède par récurrence sur  $\# V'$ , le cas de base  $\# V' = 1$  étant immédiat. Le choix de l'arête  $e$  joignant un sommet de  $V'$  à un sommet de  $V \setminus V'$  assure que l'on obtienne un arbre. En effet, si  $G' = (V', A)$  est un arbre, il ne contient pas de cycle et l'arête  $e$  ajoutée ne saurait créé un cycle. Ainsi  $G' + e$  est encore un arbre.

Par hypothèse de récurrence,  $G'$  est un sous-arbre d'un arbre  $T$  couvrant  $G$  et de poids minimal. Supposons que  $e$  ne soit pas une arête de  $T$ . Ainsi,  $T + e$  contient un cycle contenant  $e$  mais aussi une autre arête  $e'$  joignant un sommet de  $V'$  à un sommet de  $V \setminus V'$ . Considérons l'arbre  $S = (T + e) - e'$ . Il s'agit encore d'un arbre couvrant. De par les可以选择 réalisés dans l'algorithmme, son poids

$$
p (S) = p (T) + p (e) - p \left(e ^ {\prime}\right)
$$

Chapitre II. Un peu de théorie algébrique des graphes

est tel que  $p(S) \leq p(T)$  (car  $p(e) \leq p(e')$ ). Or  $T$  est de poids minimal. Donc  $p(S) = p(T)$  et on en conclus que  $e$  appartient bien à un arbre couvrant  $G$  et de poids minimal.

Remarque II.6.3. L'algorithmme s'adapte aisément pour rechercher un sous-arbre couvrant de poids maximal. Il est facile de se convaincre que l'algorithmme de Prim a une complexité temporelle en  $\mathcal{O}((\# V)^2)$ .

L'algorithmme de Prim n'est pas le seul répondant à cette question. On rencontres souvent l'algorithmme de Kruskal brievement décrit ci-après.

Algorithm II.6.4 (Kruskal). La donnée de l'algorithmme est un graphe connexe non orienté  $G = (V, E)$ .

```txt
Enumérer les arêtes  $e_1, e_2, \ldots$  de  $G$  de telle sorte que si  $p(e_i) &gt; p(e_j)$ , alors  $i &gt; j$  A:=∅
Pour i allant de 1 à #E
Si  $\mathsf{A} \cup \{e_i\}$  est acyclique alors A:=A∪{ei}.

# CHAPITRE III

# Graphes planaires

# 1. Introduction

Dans ce chapitre, on considère uniquement des graphes non orientés. En effet, l'orientation ne joue aucun role par rapport aux questions de planarité. Attirons de plus l'attention du lecteur sur le fait que la plupart des résultats ( comme la formule d'Euler) ne sont applicables qu'à des graphes finis.

Definition III.1.1. Un multi-graphe  $G$  est planaire (ou planaire topologique) s'il est possible de le représentier dans le plan affin euclidien de manière telle que les sommets distincts de  $G$  soient des points distincts du plan et les arêtes soient des courbes simples $^{1}$ . On impose en plus que deux arêtes distinctes ne se rencontres pas en dehors de leurs extrémités. On parlera alors de représentation planaire de  $G$ .

On ne considérera pas comme distinctes deux représentations planaires que l'on peut amener à coïncider par "déformation élastique du plan"2

Remarque III.1.2. Un même graphe planaire peut avoir plusieurs représentations planaires distinctes (cf. figure III.4).

Remarque III.1.3. Pour rappel, le théorème de Jordan stipule que pour toute courbe simple fermée  $\Gamma$ ,  $\mathbb{R}^2 \setminus \Gamma$  possède exactement deux composantes connexes. Cela va donc nous permettre de définir la notion de face de manière rigoureuse. En effet, un circuit simple détermine une courbe simple.

![img-126.jpeg](img-126.jpeg)
FIGURE III.1. Théorème de Jordan

Définition III.1.4. Soit  $G$  un multi-graphe planaire. Une face de  $G$  est une région du plan délimitée par des arêtes formant un circuit simple. Par conséquent, deux points arbitraires de cette région peuvent être joints par un

Chapitre III. Graphes planaires

trait continu ne rencontres aucun sommet ni arête de  $G$ . La frontière  $\partial F$  d'une face  $F$  est l'ensemble des arêtes qui "touchent" la face (on omet donc les sommets). Par extension et pour simplifier notre exposé, on s'autorisera à parler des sommets et des arêtes appartenant à une face (si les arêtes appartiennent à la frontière de la face considérée ou si un sommet est une extrémité d'une telle arête). Deux faces  $F$  et  $F'$  sont adjacentes si leur frontière ont au moins une arête commune, i.e.,  $\partial F \cap \partial F' \neq \emptyset$ . En particulier, deux faces ne se touchant que par un sommet ne sont pas considérées comme adjacentes. Un multi-graphe planaire (fini) contient toujours une et une seule face infinie, i.e., non bornée. On peut néanmoins toujours parler de la frontière de la face infinie.

![img-127.jpeg](img-127.jpeg)
FIGURE III.2. Un graphe planaire et ses faces.

Remarque III.1.5. A titre indicatif, citons le théorème de Steinitz (1922). Rappelons que le squelette d'un polyèdre  $P$  est le graphe dont les sommets sont les sommets de  $P$  et dont les arêtes sont aussi celles de  $P$ . Par exemple, le graphe de gauche à la figure I.13 est le squelette d'un cube. Le résultat s'énonce comme suit. Un graphe est le squelette d'un polyèdre convexe (borné) de  $\mathbb{R}^3$  si et seulement si c'est un graphe planaire au moins 3-connexe (i.e., ne pouvant pas être disconnecté en retirant moins de trois sommets,  $\kappa(G) \geq 3$ ).

Proposition III.1.6. Si  $G$  est un multi-graphe planaire et  $\Delta$  une face de  $G$ . Il est possible d'obtenir une représentation planaire du graphe  $G$  dans le plan affin euclidien de manière telle que  $\Delta$  soit la face infinie. En particulier, pour tout sommet  $x$  de  $G$ , on peut obtenir une représentation planaire du graphe  $G$  telle que  $x$  appartienne à la face infinie.

Démonstration. Par projection stéréographique (cf. figure IV.3), on se ramène à une représentation du graphe sur la sphère et inversement. Par rotation (cf. figure III.3), on peut toujours amener la face  $\Delta$  à contenir le pôle de la projection. Il s'agit d'un argument purement géométrique.

III.2. Formule d'Euler

![img-128.jpeg](img-128.jpeg)
FIGURE III.3. Rotation sur la sphère

Exemple III.1.7. A la figure III.4, on a représenté un même graphe et à chaque fois, la face infinie est différente. Dans le premier cas, la frontière de la face infinie est donnée par  $\partial F = \{\{1,4\}, \{4,3\}, \{3,1\}\}$  et dans les deux autres cas par  $\partial F = \{\{1,3\}, \{3,2\}, \{2,1\}\}$  et  $\partial F = \{\{1,2\}, \{2,4\}, \{4,1\}\}$ .

![img-129.jpeg](img-129.jpeg)
FIGURE III.4. Choix de la face infinie.

# 2. Formule d'Euler

Le théorème suivant peut parfois être utilisé pour vérifier qu'un graphe donné n'est pas planaire. En effet, c'est principalement grâce à ce résultat que nous montrerons que ni  $K_{5}$  ni  $K_{3,3}$  ne sont planaires.

Théorème III.2.1 (Formule d'Euler $^4$ ). Dans un multi-graphe planaire connexe (fini) possédant s sommets, a arêtes et  $f$  faces, on a

$$
\boxed {s - a + f = 2.}
$$

Démonstration. On procède par récurrence sur le nombre  $f$  de faces. Si  $f = 1$ , le graphe possède uniquement une face infinie. Par conséquent, le graphe connexe ne possède aucun cycle, il s'agit d'un arbre. Ainsi,  $s = a + 1$  et la formule est vérifiée.

Supposons à présent la formule d'Euler satisfaite pour les valeurs  $&lt; f$  et démontrons-la pour  $f \geq 2$ . Soit  $e$  une arête d'un cycle du graphe. Cette

Chapitre III. Graphes planaires

arête appartient à la frontière de deux faces  $A$  et  $B$ . Si on supprime l'arête  $e$ , on obtient un nouveau graphe ayant  $a - 1$  arêtes, le même nombre  $s$  de sommets et  $f - 1$  faces (en effet,  $A$  et  $B$  forment une face de ce nouveau graphe). Par hypothèse de récurrence, on a  $s - (a - 1) + f - 1 = 2$  ce qui suffit.

Exemple III.2.2. Pour le graphe de la figure III.2, on a  $s = 10$ ,  $a = 17$  et  $f = 9$ .

Le simple argument de comptage utilisé dans la preuve du résultat suivant nous sera utile à plusieurs reprises dans la suite de ce chapitre.

Corollaire III.2.3. Dans un graphe  $G = (V, E)$  simple et planaire, il existe un sommet  $x$  tel que  $\deg(x) \leq 5$ .

Démonstration. Quitte à considérer séparément chaque composante connexe de  $G$ , nous allons supposer  $G$  connexe. On peut tout d'abord éliminer les arêtes ne délimitant pas de face (celles-ci ont une extrémité de degré 1 et le résultat est alors immédiat, cf. figure III.5). Cela ne fait que renforcer encore le résultat.

![img-130.jpeg](img-130.jpeg)
FIGURE III.5. Suppression d'arêtes.

Puisque  $G$  est simple, la frontière de toute face contient au moins 3 arêtes. Donc en comptant le nombre de faces, on compte à chaque fois au moins 3 arêtes, ainsi,  $3f \leq a$ . On peut même dire plus car, dans ce comptage, chaque arête est comptée deux fois puisqu'elle apparait dans la frontière de deux faces. On en conclus donc que

(9)

Procedons par l'absurde et supposons que pour tout sommet  $x$  de  $G$ ,  $\deg(x) \geq 6$ . Dans ce cas,

$$
s \leq \frac {2 a}{6}.
$$

Si on applique la formule d'Euler,

$$
2 = s - a + f \leq \frac {a}{3} - a + \frac {2 a}{3} = 0
$$

et on obtient une absurdité.

III.3. Graphes homéomorphes

# 3. Graphes homéomorphes

Nous définissons ici la notion de graphes homéomorphes nécessaire à l'énoncé du théorème de Kuratowski.

Definition III.3.1. La subdivision d'une arête  $e = \{a, b\}$  d'un graphe  $G = (V, E)$  consiste à replacer cette arête par deux nouvelles arêtes  $e_1 = \{a, u\}$  et  $e_2 = \{u, b\}$  où  $u$  est un nouveau sommet n'appartenant pas à  $V$ . Le

![img-131.jpeg](img-131.jpeg)
FIGURE III.6. Subdivision d'une arête.

graphe obtenu est alors  $G' = (V \cup \{u\}, (E \cup \{e_1, e_2\}) \setminus \{e\})$ .

Definition III.3.2. Deux graphes sont homéomorphes s'ils peuvent être obtenus par une suite finie (voir vide) de subdivisions à partir d'un même graphe. En particulier, si un graphe  $G'$  résultat de subdivisions d'arêtes de  $G$ , alors  $G$  et  $G'$  sont homéomorphes.

Example III.3.3. La figure III.7 reprend deux graphes homéomorphes construits à partir d'un même graphe.

![img-132.jpeg](img-132.jpeg)
FIGURE III.7. Graphes homéomorphes.

Remarque III.3.4. On se convainc assez facilement que la relation “être homéomorphe” est une relation d'équivalence sur l'ensemble des multi-graphes finis (non orientés). Dans une classe d'équivalence, on peut considérer le graphe ayant un nombre minimum d'arêtes (ou de sommets).

Chapitre III. Graphes planaires

![img-133.jpeg](img-133.jpeg)
FIGURE III.8. Graphes homéomorphes.

# 4. Théorème de Kuratowski

Les graphes  $K_{5}$  et  $K_{3,3}$  sont l'archétype même des graphes non planaires.

Lemma III.4.1. Le graphe  $K_{5}$  n'est pas planaire.

Démonstration. On utilise les notations de la formule d'Euler. D'une manière générale, dans un graphe simple et planaire, de la relation  $3f \leq 2a$  démontrée en (9) et de la formule d'Euler ( $3a - 3f = 3s - 6$ ), on tire que

$$
a \leq 3 s - 6.
$$

Or,  $K_{5}$  est un graphe simple qui possède 5 sommets et 10 arêtes et  $10 \not\leq 3.5 - 6$ . On en conclus que  $K_{5}$  ne peut être planaire.

Lemma III.4.2. Le graphe  $K_{3,3}$  n'est pas planaire.

Démonstration. On utilise une fois encore le même raisonnement que celui ayant permis d'obtenir la relation (9), mais cette fois dans un graphe simple, planaire et biparti. Dès lors, chaque face a une frontière déterminée par au moins 4 arêtes et on en tire que  $4f \leq 2a$ , i.e.,  $2f \leq a$ . De la formule d'Euler,  $2a - 2f = 2s - 4$ , on en tire que

$$
a \leq 2 s - 4.
$$

Or,  $K_{3,3}$  est un graphe biparti simple qui possède 6 sommets et 9 arêtes. Il ne peut donc pas être planaire car  $9 \not\leq 2.6 - 4$ .

Théorème III.4.3. Un multi-graphe (non orienté) est planaire si et seulement si il ne contient pas de sous-graphe homéomorphe à  $K_{5}$  ou à  $K_{3,3}$ .

La preuve de ce résultat est due de quatrelemmes suivants (et principalement deslemmes III.4.4 et III.4.7). La presentation adoptée ici est due originellement à Thomassen (1981). Nous dirons qu'un graphe possède la propriété  $(\mathbf{K})$  s'il contient un sous-graphe homéomorphe à  $K_{5}$  ou à  $K_{3,3}$ . En fait, de nombreuses recherches actuelles et hautement non triviales essaient de déterminer des classes de graphes ( comme les graphes planaires) qui seraient caractérisées par l'exclusion de certains graphes précis. Le théorème de Kuratowski est le premier exemple d'un tel théorème d'exclusion.

III.4. Théorème de Kuratowski

Lemma III.4.4. Soit  $G = (V, E)$  a graph non planaire ne satisfaisant pas (K) et pour lequel  $\# E$  est minimal pour ces propriétés. Alors  $G$  est 3-connexe.

Pour rappel, l'opération de contraction a été donnée à la définition II.5.3.

Lemma III.4.5. Soit  $G = (V, E)$  un graphe 3-connexe ayant au moins 5 sommets. Il existe une arête  $e \in E$  telle que la contraction  $G \cdot e$  soit encore 3-connexe.

Lemma III.4.6. Soit  $G = (V, E)$  un graphe. Si pour une arête  $e \in E$ ,  $G \cdot e$  satisfait (K), alors  $G$  aussi.

Lemma III.4.7. Tout graphe 3-connexe et qui ne satisfait pas (K) est planaire.

Démonstration. (Théorème de Kuratowski) Au vu deslemmes III.4.1 et III.4.2, il est clair que si  $G$  satisfait (K), alors  $G$  n'est pas planaire.

Il suffit donc de vérifier que si  $G$  n'est pas planaire, alors  $G$  satisfait (K). Procedons par l'absurde et supposons que  $G$  est non planaire et satisfait (K). Quitte à replacer  $G$  par un graphe qui lui est homéomorphe (cela ne change en rien la (non-)planarité), on peut supposer que  $G$  contient un nombre minimal d'arêtes. Ainsi, par le lemme III.4.4,  $G$  est 3-connexe. Du lemme III.4.7, on tire que  $G$  est planaire. Ceci achève la preuve.

Démonstration. (Lemme III.4.4) Montrons d'abord que  $G$  est 2-connexe en procédant par l'absurde et en supposant que  $G$  possède un point d'articulation  $v$ . Soient  $A_{1},\ldots ,A_{k}$  les composantes connexes de  $G - v$ ,  $k\geq 2$ . Pour tout  $i$ , le sous-graphe  $G_{i}$  de  $G$  induit par les sommets de  $A_{i}$  et  $v$  possède un nombre d'arêtes strictement inférieur au nombre d'arêtes de  $G$ . Par hypothèse,  $G$  est un grahe non planaire ne satisfaisant pas  $(\mathbf{K})$  et ayant un nombre minimal d'arêtes pour ces propriétés. On en conclus que  $G_{i}$  est planaire et par la proposition III.1.6, on peut même supposer que  $v$  se trouve sur la frontière extérieure de  $G_{i}$ . De là, on en conclus que  $G$  est

![img-134.jpeg](img-134.jpeg)
FIGURE III.9. Si  $G$  n'était pas 2-connexe.

Chapitre III. Graphes planaires

planaire, une contradiction.

Supposons à présent que  $G$  possède un ensemble d'articulation formé de deux sommets:  $S = \{u, v\}$ . (On ne peut pas faire moins car  $G$  est au moins 2-connexe, vu la première partie de la preuve). Soient  $G_{1} = (E_{1}, V_{1})$  et  $G_{2} = (E_{2}, V_{2})$  deux sous-graphes de  $G$  tels que  $E = E_{1} \cup E_{2}$ ,  $V_{1} \cap V_{2} = S$  et contenant chacun une composante connexe de  $G - S$ . Il existe dans  $G_{1}$  (resp. dans  $G_{2}$ ) un chemin joignant  $u$  et  $v$  (car  $u$  comme  $v$  sont adjacents à un sommet de chaque composante connexe de  $G - S$ ).

On utilise le même raisonnement que précédemment. Soit  $H_{i}$  le sous-graphe de  $G$  induit par les sommets de  $G_{i}$  et de  $S$ ,  $i = 1,2$ , auquel on ajoute éventuellement l'arête  $\{u,v\}$  (cette arête peut ou non apparénir à  $E$ ). Si  $H_{1}$  et  $H_{2}$  étaient tous les deux planaires, on pourrait en conclude que  $G$  lui-même est planaire en plaçant l'arête  $\{u,v\}$  à sur la frontière extérieure de  $H_{1}$  et  $H_{2}$  (ce qui est possible vu la proposition III.1.6).

![img-135.jpeg](img-135.jpeg)
FIGURE III.10.  $H_{1}$ ,  $H_{2}$  et l'arête  $\{u, v\}$ .

Ainsi, supposons que  $H_{1}$  est non planaire. Puisqu'il contient moins d'arêtes que  $G$ , alors  $H_{1}$  satisfait (K). Cependant, il nous faut encore retirer l'arête  $\{u,v\}$  si cette dernière n'appartient pas à  $G$ . Néanmoins,  $G$  va lui aussi satisfaire (K) car on dispose d'un chemin dans  $G_{2}$  joignant  $u$  et  $v$ . On peut donc substituer ce chemin à  $\{u,v\}$  et assurer que  $G$  satisfait (K) sans utiliser l'arête  $\{u,v\}$ . On a donc obtenu une contradiction et  $G$  ne peut contenir un ensemble d'articulation contenant uniquement 2 sommets.

Démonstration. (Lemme III.4.5) Procedons par l'absurde et supposons que, qu'elle que soit l'arête  $e \in E$ , le graphe  $G \cdot e$  possède un ensemble de coupure  $S$  tel que  $\# S = 2$  (on suppose donc que  $G \cdot e$  n'est plus 3-connexe). Soient  $e = \{u, v\}$  et  $x$  le sommet obtenu après contraction de  $e$ . Ce sommet  $x$  appartient à  $S$  car sinon,  $G$  lui-même possèderait  $S$  comme ensemble de coupure (or, par hypothèse  $G$  est 3-connexe). Ainsi, on a  $S = \{x, z\}$  pour un certain sommet  $z$ . On peut en conclude que

$$
T = \{u, v, z \}
$$

est un ensemble de coupure de  $G$ . Puisqu'un tel ensemble existe, on désisit  $^7 e$  et  $z$  de manière telle que  $G - T$  possède une composante connexe  $A$  contenant un nombre minimal de sommets.

III.4. Théorème de Kuratowski

Il existe dans  $A$  un sommet  $y$  tel que  $\{y, z\}$  est une arête de  $E$  car sinon,  $\{u, v\}$  séparerait  $A$  du reste du graphe  $G$  (ce qui n'est pas possible car  $G$  est 3-connexe). Si on contracte l'arête  $\{y, z\}$  dans le graphe  $G$ , au vu de notre

![img-136.jpeg](img-136.jpeg)
FIGURE III.11. Lemme III.4.5

supposition, le graphe  $G \cdot \{y, z\}$  n'est pas 3-connexe. On applique donc à nouveau le même raisonnement et on en tire qu'il existe un sommet  $w$  tel que

$$
R = \{y, z, w \}
$$

sépare  $G$  ( $w$  peut très bien être égal à  $u$  ou a  $v$ , ce n'est pas interdit).

Si on regarde le graphe  $G - R$ , il possède une composante connexe  $B$  ne contenant ni  $u$  ni  $v$ . En effet,  $\{u, v\}$  est une arête de  $G$  (donc soit les deux sommets appartiennent à une même composante distincte de  $B$ , soit, lorsque  $w = u$  ou  $w = v$ , un de ces deux sommets est supprimé et l'autre appartient à une composante distincte de  $B$ ). Cette composante connexe  $B$  contient un sommet  $y'$  tel que  $\{y, y'\} \in E$ . C'est le même raisonnement que précédemment, si ce n'était pas le cas, alors  $\{z, w\}$  séparerait  $G$ .

Le sous-graphe de  $G - T$  induit par les sommets de  $B$  est connexe. Puisque  $y \in A$ ,  $y' \in B$  et  $\{y, y'\} \in E$ , on en conclus que  $B$  est inclus dans  $A$  et cette inclusion est propre car  $y$  n'appartient pas à  $B$ . De là  $\# B &lt; \# A$  et on obtient une contradiction, vu le choix de  $A$ .

Démonstration. (Lemme III.4.6) Soit  $H$ , un sous-graphe de  $G \cdot e$  homéomorphe à  $K_{5}$  ou  $K_{3,3}$  dont  $x$  est un sommet. On suppose de plus que ce sommet  $x$  de  $G \cdot e$  est obtenu par contraction de  $e = \{u, v\}$ . Si  $x$  est de degré 2, alors il est immédiat que  $G$  possède la propriété (K). Il reste à envisager les cas  $\deg(x) = 3$  et  $\deg(x) = 4$  (car  $K_{3,3}$  est 3-régulier et  $K_{5}$  est 4-régulier). S'il existe au plus une arête  $\{x, y\}$  dans  $H$  telle que  $\{u, y\}$  ou  $\{v, y\}$  appartiennent à  $E$  (ou exclusif), alors il est clair que  $G$  satisfait (K).

![img-137.jpeg](img-137.jpeg)
FIGURE III.12. Si on dispose d'au plus une arête.

![img-138.jpeg](img-138.jpeg)

Chapitre III. Graphes planaires

Il reste donc le cas où à la fois  $u$  et  $v$  ont 3 voisins dans le sous-graphe de  $G$  correspondant à  $H$ . En particulier,  $H$  est donc homéomorphe à  $K_{5}$ . Grâce à la figure III.13, on se convainc que  $G$  contient un sous-graphe homéomorphe à  $K_{3,3}$ .

![img-139.jpeg](img-139.jpeg)
FIGURE III.13.  $G$  contient un sous-graphe homéomorphe à  $K_{3,3}$ .

![img-140.jpeg](img-140.jpeg)

Démonstration. (Lemme III.4.7) On procède par récurrence sur  $\# V$ . Le seul graphe 3-connexe à 4 sommets est  $K_{4}$  et il est planaire. On peut donc supposer  $\# V \geq 5$ . Par le lemme III.4.5, il existe une arête  $e = \{u, v\}$  telle que  $G \cdot e$  (ou l'arête contractée donne le sommet  $x$ ) est 3-connexe. Par le lemme III.4.6, puisque  $G$  ne vérifie pas  $(\mathbf{K})$ , il en est de même pour  $G \cdot e$ . Puisque  $G \cdot e$  possède un sommet de moins que  $G$ , on peut appliquer l'hypothèse de récurrence et en conclude que  $G \cdot e$  est planaire.

Considérons une représentation planaire de  $G \cdot e$  et  $C$  la frontière de la face de  $(G \cdot e) - x$  qui contiendrait  $x$ . Puisque  $G$  est 3-connexe,  $C$  est un cycle.

Il est clair que  $G - \{u, v\} = (G \cdot e) - x$ . Par conséquent,  $\nu_{G}(u) \subset C \cup \{v\}$  et  $\nu_{G}(v) \subset C \cup \{u\}$  (ou l'on a identifié le cycle  $C$  et l'ensemble de ses sommets). Supposons que, dans  $G$ ,  $\deg(v) \leq \deg(u)$  (on pourrait par symétrie traiter l'autre situation). Ordonnons les sommets de  $\nu_{G}(v) \setminus \{u\} = \{v_{1}, \ldots, v_{k}\}$  en respectant le cycle  $C$ . Soit  $P_{i,j}$  le chemin le long de  $C$  joignant  $v_{i}$  à  $v_{j}$  par indices croissants. En se basant sur la représentation planaire de  $G - \{u, v\}$ , on obtient une représentation planaire de  $G - u$  en traçant les segments de droites  $\{v, v_{i}\}$ .

![img-141.jpeg](img-141.jpeg)
FIGURE III.14. Illustration du lemme III.4.7.

![img-142.jpeg](img-142.jpeg)

III.4. Théorème de Kuratowski

(a) Si  $\nu_{G}(u)\setminus \{v\} \subseteq P_{i,i + 1}$  (ou  $i + 1$  est interprete modulo  $k$  ) pour un certain  $i$  , alors  $G$  est planaire. Il suffit de positionner  $u$  dans le triangle  $v,v_{i},v_{i + 1}$

![img-143.jpeg](img-143.jpeg)
FIGURE III.15. Illustration du lemme III.4.7, cas (a).

![img-144.jpeg](img-144.jpeg)

(b) S'il existe  $y, z \in \nu_{G}(u) \setminus \{u\}$  tels que  $y \in P_{i,j}$  et  $z \notin P_{i,j}$  avec  $y, z \notin \{v_i, v_j\}$ . Dans ce cas,  $\{u, v_i, v_{i+1}\}$  et  $\{v, z, y\}$  forment une copie homéomorphe à  $K_{3,3}$ , ce qui est impossible.
(c) Le dernier cas à envisager est donc  $\nu_{G}(u)\setminus \{v\} \subset \nu_{G}(v)$ . Or nous avons supposer que  $\deg (v)\leq \deg (u)$ , de là,

$\nu_{G}(u)\setminus \{v\} = \nu_{G}(v)\setminus \{v\}$  et  $\deg (u) = \deg (v)$

Si  $\deg (u) = \deg (v)\leq 3$  , le graphe  $G$  est planaire. Sinon, il contient une copie homéomorphe a  $K_{5}$

![img-145.jpeg](img-145.jpeg)
FIGURE III.16. Illustration du lemme III.4.7, cas (b) et (c).

![img-146.jpeg](img-146.jpeg)



“哈，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，你是个小伙子，

# CHAPITRE IV

# Coloriage

Dans ce chapitre, on considère le problème de coloriage de sommets d'un graphe, le théorème des cinq couleurs et une introduction à la théorie de Ramsey. Nous supposerons rencontres des multi-graphes non orientés (les boucles n'ont ici que peu d'importance).

# 1. Nombre chromatique

Definition IV.1.1. Soit  $c: V \to \Sigma$  un étiquetage des sommets du graphe où l'on suppose que  $\Sigma$  est fini. Dans le contexte qui nous intéresse, on parlera plutôt de coloriage et pour tout sommet  $u$ , on dira que  $c(u)$  est la couleur de  $u$ . Un coloriage  $c$  des sommets d'un graphe est dit propre si deux sommets voisins ont des couleurs distinctes pour  $c$ .

S'il existe un coloriage propre  $c: V \to \Sigma$  de  $G$  tel que  $\# \Sigma = k$ , on dira que  $G$  est  $k$ -colorable. La valeur minimum de  $k$  pour laquelle  $G$  est  $k$ -colorable est appelée le nombre chromatique de  $G$  et est noté  $\chi(G)$ .

Remarque IV.1.2. Pour obtenir un coloriage propre, on peut attribuer à des sommets indépendants (définition I.5.1) une même couleur. Ainsi, le nombre chromatique d'un graphe  $G = (V, E)$  est le nombre minimum de sous-ensembles de sommets indépendants nécessaires pour partitionner  $V$ .

Lemma IV.1.3. Le nombre chromatique d'un graphe  $G = (V, E)$  (non orienté et simple) est le plus petit entier n tel qu'il existe un homomorphisme de  $G$  dans  $K_{n}$ .

![img-147.jpeg](img-147.jpeg)
FIGURE IV.1. Une illustration du lemme IV.1.3.

Démonstration. Soient  $H = (V', E')$  un graphe simple et  $f: V \to V'$  un homomorphisme de graphes. Pour tout  $y \in V'$ ,

$$
f ^ {- 1} (y) = \{x \in V: f (x) = y \}.
$$

Chapitre IV. Coloriage

Puisque  $H$  est simple, il ne contient aucune boucle et donc,  $f^{-1}(y)$  est formé de sommets indépendants (cf. définition I.5.1).

![img-148.jpeg](img-148.jpeg)
FIGURE IV.2. Si  $H$  n'est pas simple.

Par conséquent, s'il existe un homomorphisme de  $G$  vers un graphe simple à  $n$  sommets ( comme  $K_{n}$  par exemple), alors  $G$  est  $n$ -colorable  $(\chi(G) \leq n)$ . Réciproquement, si  $G$  est  $n$ -colorable, alors l'application qui envoie les sommets de  $G$  d'une même couleur sur un sommet de  $K_{n}$  est un homomorphisme.

Proposition IV.1.4. Un graphe  $G = (V, E)$  est bipartis si et seulement si il est 2-colorable.

Demonstration. C'est immédiat.

Remarque IV.1.5. Un graphe  $G = (V, E)$  est 1-colorable si et seulement si  $E = \emptyset$ .

# 2. Le théorème des cinq couleurs

On peut reprendre le problème de cartographie énoncé à l'exemple I.3.4. Tout d'abord, on peut remarquer que par projection stéréographique, tout problème de planarité ou de coloriage de graphes sur la sphère revient à un problème analogue dans le plan.

Le lecteur attentif aura remarqué que le problème de cartographie demande de colorier des faces adjacentes d'un graphe planaire avec des couleurs distinctes. Pourtant, dans la section précédente, nous nous sommes intéressés au coloriage des sommets d'un graphe. On pourrait donc penser qu'il s'agit d'un nouveau type de problème. En fait, il n'en est rien comme nous allons le voir en introduisant le dual d'un graphe planaire.

Définition IV.2.1. Soit  $G = (V, E)$  un multi-graphe planaire. Le dual  $G^*$  de  $G$  est le graphe dont les sommets sont les faces de  $G$ . A chaque arête de  $G$  appartenant à la frontière de deux faces  $F$  et  $F'$ , il correspond une arête dans  $G^*$  joignant les sommets correspondants aux faces  $F$  et  $F'$ . Il est clair que le dual d'un graphe planaire est encore planaire et que  $(G^*)^* = G$ .

IV.2. Le théorème des cinq couleurs

![img-149.jpeg](img-149.jpeg)
FIGURE IV.3. Projection stéréographique.

![img-150.jpeg](img-150.jpeg)
FIGURE IV.4. Un graphe et son dual.

Ainsi grâce au dual, colorier les faces d'un graphe planaire, revient à colorier les sommets de son dual et inversement.

Remarque IV.2.2. La trace la plus ancienne du "problème des quatre couleurs" remonte à une lettre de De Morgan datée du 23 octobre 1852 et destinée à Hamilton: "A student of mine asked me today to give him reason for a fact which I did not know was a fact and do not yet. He says that if a figure be anyhow divided and the compartments differently colored, so that figures with any portion of common boundary line are differently coloured — four colours may be wanted but no more." L'étudiant dont il est question dans la lettre est Frederick Guthrie. Futur physicien, il avait pris connaissance de ce problème par son frère Francis, futur mathématicien, qui avait fait cette conjecture en observant la carte d'Angleterre. Un résumé interressant de l'histoire du théorème des quatre couleurs peut être trouve dans "Four Colours Suffice, or how to colour a map, R. Wilson, European Math. Society Newsletter 46 December 2002, 15-19."

Chapitre IV. Coloriage

![img-151.jpeg](img-151.jpeg)
FIGURE IV.5. Francis Guthrie.

Dans la suite, nous nous intéressons donc au coloriage des faces d'un multi-graphe planaire de manière telle que deux faces adjacentes (y compris la face infinie) recoivent des couleurs distinctes. On parlera alors simplement de coloriage, les contraintes précitées étant sous-entendues.

Par convention, les couleurs seront désignées par des lettres grecques minuscules. Il est tout d'abord clair qu'il existe des graphes pour lesquels quatre couleurs sont nécessaires. Il suffit de considérer le graphe de la figure III.4.

Nous allons ici démontré une version plus faible que le véritable théorème des quatre couleurs. Notre presentation est basée sur celle d'Ore [20]. En effet, nous allons démontré le théorème suivant. Ce résultat contient les idées essentielles du théorème des quatre couleurs et présente l'avantage de pouvoir être démontré de manière directe.

Théorème IV.2.3. Cinq couleurs suffisent pour colorier les faces d'un multi-graphe planaire de manière telle que deux faces adjacentes recoivent des couleurs distinctes.

Pour démontré ce résultat, on peut tout d'abord se restreindre à des multi-graphes 3-réguliers. Tout d'abord, les sommets de degré 1 peuvent être éliminés. Ils n'interviennent pas dans la définition d'une face. Ensuite, on peut modifier le graphe comme à la figure IV.6 pour ne plus avoir de sommets de degré 2. Il est clair que cette opération ne modifie en rien un

FIGURE IV.6. Suppression des sommets de degré 2.

coloriage valide. Ainsi, le graphe résultat possède un coloriage valide si et seulement si le graphe de départ en possède un.

Nous pouvons enfin modifier le graphe comme à la figure IV.7 pour ne plus avoir de sommets de degré  $\geq 4$ . Si le graphe résultat possède un coloriage valide avec le graphe de départ aussi.

IV.2. Le théorème des cinq couleurs

![img-152.jpeg](img-152.jpeg)
FIGURE IV.7. Suppression des sommets de degré  $\geq 4$ .

Ainsi, nous pouvons considérer avoir un graphe planaire 3-régulier  $G$ . Si nous sommes capables de colorier les graphes planaires 3-réguliers, nous serons capables de colorier n'importe quel graphe planaire. Denotons par  $\varphi_{i}$ , le nombre de faces de  $G$  dont la frontière est déterminée par exactement  $i$  arêtes (ou de manière équivalente, par  $i$  sommets),  $i \geq 2$ . Le graphe étant 3-régulier, chaque sommet appartient à 3 faces. Si pour chaque face, nous en comptons les sommets², on obtient

Notations de la formule d'Euler.

$3s = 2\varphi_{2} + 3\varphi_{3} + 4\varphi_{4} + 5\varphi_{5} + \dots$

Si pour chaque face, nous en comptons les arêtes³, on obtient

$2a = 2\varphi_{2} + 3\varphi_{3} + 4\varphi_{4} + 5\varphi_{5} + \dots$

Enfin, il est clair que

$f = \varphi_{2} + \varphi_{3} + \varphi_{4} + \varphi_{5} + \dots$

Si on substitue les termes apparaissant dans la formule d'Euler  $12 = 6s - 6a + 6f$  par les valeurs obtenues ci-dessus, on obtient la relation

$12 = 4\varphi_{2} + 3\varphi_{3} + 2\varphi_{4} + \varphi_{5} - \varphi_{7} - 2\varphi_{8} - \dots$

Or les  $\varphi_{i}$  sont positifs ou nuls. Dès lors, on en tire le résultat suivant.

Lemme IV.2.4. Tout multi-graphe planaire 3-régulier contient une face dont la frontière est délimitée par 2,3,4 ou 5 arêtes.

Notre but est de considérer, dans le graphe  $G$ , une face délimitée par 2,3,4 ou 5 arêtes et d'en supprimer une arête. (L'existence d'une telle face est assurée par le lemme IV.2.4.) De cette façon, on obtient un graphe  $G'$  ayant au moins une face de moins que le graphe  $G$  de départ. Les constructions que nous allons développer auront deux propriétés.

Chapitre IV. Coloriage

R.1 La construction préserve le caractère 3-régulier (on pourra donc l'appliquer itérativement).
R.2 Si le graphe  $G'$  peut être colorié avec au plus 5 couleurs, alors il en est de même pour le graphe de départ  $G$ .

On parle en général de réduction. On s'est ramené à un problème pour un graphe ayant moins de faces que le graphe initial.

Ainsi, grâce à la propriété R.1, chaque étape fournit un graphe 3-régulier. Au vu du lemme IV.2.4, on peut opérer une nouvelle réduction et diminuer le nombre de faces du graphe. En répétant la procédure, on obtiendra un graphe ayant au plus 5 faces qu'il est donc toujours possible de colorier! La propriété R.2 assure alors que le graphe de départ peut, lui aussi, être correctement colorié.

- Réduction d'une face délimitée par deux arêtes

Considérons la situation donnée à la figure IV.8 et sa réduction. Il est facile

![img-153.jpeg](img-153.jpeg)
FIGURE IV.8. Réduction d'une face délimitée par deux arêtes.

de se convaincre que les propriétés R.1 et R.2 sont satisfaites.

- Réduction d'une face délimitée par trois arêtes

Considérons la situation donnée à la figure IV.9 et sa réduction. Il est facile de se convaincre que les propriétés R.1 et R.2 sont satisfaites.

- Réduction d'une face délimitée par quatre arêtes

Ici, la situation est un peu plus délicate. Si une face  $F$  est délimitée par quatre arêtes, comme on le voit à la figure IV.10, quatre faces  $A, B, C, D$  peuvent lui être adjacentes. Dans le premier cas,  $A$  et  $C$  ne sont pas adjacentes, dans le deuxième cas (gardez à l'esprit que le graphe doit être 3-régulier), elles ont une arête commune. Mais dans tous les cas,  $B$  et  $D$  ne font pas partie d'une même face et ne sont pas adjacents (ils pourront donc receivevoir la même couleur). Enfin, dans le dernier cas,  $A$  et  $C$  constituent une seule face. Il faut se convaincre qu'il s'agit des seules situations envisageables (quitte à renommer les différentes faces). En effet, il n'est pas possible dans le plan que simultanément, les faces  $A$  et  $C$  soient adjacentes de même que  $B$  et  $D$ . On peut supprimer certains sommets et arêtes pour

IV.2. Le théorème des cinq couleurs

![img-154.jpeg](img-154.jpeg)
FIGURE IV.9. Réduction d'une face délimitée par deux arêtes.

![img-155.jpeg](img-155.jpeg)
FIGURE IV.10. Configurations pour la réduction d'une face rectangulaire.

obtenir les configurations de la figure IV.11. Ces dernières peuvent être coloriées et la propriété R.1 est bien satisfaite. Ensuite, il reste à se convaincre,

![img-156.jpeg](img-156.jpeg)
FIGURE IV.11. Réductions d'une face rectangulaire.

Chapitre IV. Coloriage

grace à la figure IV.12 que R.2 est aussi satisfait.

![img-157.jpeg](img-157.jpeg)
FIGURE IV.12. Satisfaction de R.2.

# - Réduction d'une face délimitée par cinq arêtes

Le raisonnement est semblable au cas précédent. Il existe toujours deux faces opposées, disons  $A$  et  $C$ , adjacentes à une face pentagonale  $F$  telles que  $A$  et  $C$  ne font pas partie d'une même face et ne sont pas adjacentes. On modifie le graphe comme indiqué à la figure IV.13. Les faces  $B$ ,  $D$  et  $E$  peuvent nécessiter 3 couleurs distinctes  $\alpha$ ,  $\beta$  et  $\gamma$ . Ayant 5 couleurs à notre disposition, il est possible d'assurer R.2. Cela ne serait pas le cas avec uniquement 4 couleurs. Ceci démontre entièrement le théorème des cinq

![img-158.jpeg](img-158.jpeg)
FIGURE IV.13. Réductions d'une face pentagonale.

couleurs. Comme le montre la figure IV.14, dans le cas de face pentagonale, l'utilisation de 5 couleurs est inévitable (en utilisant la méthode préconisée ci-dessus, à savoir donné la même couleur aux deux faces non adjacentes). Bien sur, il est possible de réaliser un coloriage ne nécessitant pour cela que 4 couleurs, mais des lors, on sort de la méthode de réduction proposée.

Enonçons enfin le théorème des quatre couleurs. La démonstration de ce dernier nécessite le même type de réductions que celles effectuées pour le théorème des cinq couleurs mais ici, le nombre de configurations à considérer est de l'ordre de 600 pour la preuve fournie par N. Robertson, D. Sanders, P. Seymour et R. Thomas (1996). La preuve originale de K. Appel et W. Haken utilisait quant à elle pres de 2000 configurations inévitables. Par ailleurs, cette dernière preuve a en fait été partiellement réalisée par un

IV.2. Le théorème des cinq couleurs

![img-159.jpeg](img-159.jpeg)
FIGURE IV.14. Cinq couleurs effectives.

![img-160.jpeg](img-160.jpeg)

programme informatique $^4$  passant en revue et testant l'ensemble de ces configurations.

Théorème IV.2.5 (K. Appel, W. Haken (1976)). Quatre couleurs suffisent pour colorier les faces d'un multi-graphe planaire de manière telle que deux faces adjacentes recoivent des couleurs distinctes.

Remarque IV.2.6. On pourrait s'intéresser à des questions de coloriage (ou de planarité) pour des graphes tracés sur d'autres surfaces que la sphère ou le plan. On dispose par exemple du théorème des sept couleurs : Sept couleurs sont suffisantes pour colorier tout multi-graphe planaire sur un tore. De plus, il existe un multi-graphe planaire sur un tore nécessitant sept couleurs.

Remarque IV.2.7. Le genre d'une surface est un invariant topologique. Il s'agit du nombre maximum de courbes simples fermées que l'on peut tracer sur la surface sans la disconnector. De manière grossière, cela correspond

![img-161.jpeg](img-161.jpeg)
FIGURE IV.15. Une surface de genre 2.

au nombre de “trous” que la surface comporte. Ainsi, le genre d'une sphère vaut 0 et celui d'un tore vaut 1. On dispose de la formule d'Heawood (1980) qui stipule que si un graphe peut être représenté de manière planaire sur

Chapitre IV. Coloriage

|  g | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  cg | 4 | 7 | 8 | 9 | 10 | 11 | 12 | 12 | 13 | 13 | 14  |

TABLE IV.1. Les première valeurs de  $c_{g}$

une surface de genre  $g$ , alors ses faces peuvent être colorées avec  $c_{g}$  couleurs où

$$
c _ {g} = \left\lfloor \frac {1}{2} (7 + \sqrt {1 + 4 8 g}) \right\rfloor .
$$

La partie délicate consiste à montré qu'il existe effectivement un graphe nécessitant ce nombre de couleurs. En particulier, pour  $g = 0,1$ , on retrouve le théorème des 4 et des 7 couleurs.

# 3. Polynôme chromatique

Jusqu'à présent, nous nous sommes intéressés à la possibilité de colorier ou non un graphe donné à l'aide de  $k$  couleurs. La question suivante serait de déterminer le nombre de facons distinctes de réaliser un tel coloriage.

Définition IV.3.1. Soit  $G = (V, E)$  un multi-graphe non orienté ayant  $n$  sommets. On dénote par  $m_{k,G}$ , le nombre de coloriages propres distincts de  $G$  utilisant exactement  $k$  couleurs et par  $z^k$ , le polynôme en  $z \in \mathbb{C}$  de degré  $k$

$$
z ^ {\underline {{k}}} = z (z - 1) \dots (z - k + 1).
$$

Avec ces notations, le polynôme chromatique de  $G$  est donné par

$$
\begin{array}{l} \pi_ {G} (z) = \sum_ {i = 1} ^ {n} \frac {m _ {i , G}}{i !} z ^ {\underline {{i}}} \\ = \frac {m _ {1 , G}}{1 !} z + \frac {m _ {2 , G}}{2 !} z (z - 1) + \frac {m _ {3 , G}}{3 !} z (z - 1) (z - 2) + \dots \\ \dots + \frac {m _ {n , G}}{n !} z (z - 1) \dots (z - n + 1). \\ \end{array}
$$

Il s'agit d'un polynôme de degré  $n$  en la variable  $z$ .

Remarque IV.3.2. La quantité

$$
\frac {m _ {k , G}}{k !}
$$

est le nombre de partitions de  $V$  en  $k$  sous-ensembles (disjoints et non vides) donnant lieu à tous les coloriages propres possibles de  $G$  utilisant exactement  $k$  couleurs. En fait, cette quantité équivaut au nombre de partitions de  $V$  en  $k$  sous-ensembles (disjoints et non vides) de sommets indépendants.

Exemple IV.3.3. Soit le graphe de la figure IV.16. Considérons les par

IV.3. Polynôme chromatique

![img-162.jpeg](img-162.jpeg)
FIGURE IV.16. Illustration de  $m_{k,G}$ .

![img-163.jpeg](img-163.jpeg)

![img-164.jpeg](img-164.jpeg)

titions de  $V$  en  $k$  sous-ensembles (disjoints et non vides) donnant lieu à tous les coloriages propres possibles de  $G$  utilisant exactement  $k$  couleurs,  $k = 2,3$ . Pour  $k = 2$ , on a la partition  $V = \{v_{1}, v_{3}\} \cup \{v_{2}, v_{4}\}$  et les coloriages  $c_{1}: v_{1}, v_{3} \mapsto 1$ ,  $v_{2}, v_{4} \mapsto 2$  et  $c_{2}: v_{1}, v_{3} \mapsto 2$ ,  $v_{2}, v_{4} \mapsto 1$ . Ainsi,  $m_{2,G} = 2$  et  $m_{2,G}/2! = 1$  correspond bien à la seule partition convenable de  $V$ . Pour  $k = 3$ , on a deux partitions possibles de  $V$  en  $\{v_{1}, v_{3}\} \cup \{v_{2}\} \cup \{v_{4}\}$  ou bien  $\{v_{1}\} \cup \{v_{3}\} \cup \{v_{2}, v_{4}\}$ . Chaque partition donne lieu à 3! coloriages propres distincts de  $G$ . Ainsi,  $m_{3,G} = 12$  et  $m_{3,G}/3! = 2$ . Pour  $k = 4$ , il y a une seule partition de  $V$  en quatre singletons donc  $m_{4,G}/4! = 1$ . Par conséquent, le polynôme chromatique du graphe est donné par

$$
\underbrace {m _ {1 , G}} _ {= 0} z + \underbrace {\frac {m _ {2 , G}}{2 !}} _ {= 1} z (z - 1) + \underbrace {\frac {m _ {3 , G}}{3 !}} _ {= 2} z (z - 1) (z - 2) + \underbrace {\frac {m _ {4 , G}}{4 !}} _ {= 1} z (z - 1) (z - 2) (z - 3)
$$

ou encore

$$
\pi_ {G} (z) = z ^ {4} - 4 z ^ {3} + 6 z ^ {2} - 3 z.
$$

Exemple IV.3.4. Pour le graphe complet  $K_{n}$ , les seuls ensembles de sommets indépendants sont les singletons. Ainsi, pour tout  $k &lt; n$

$$
\frac {m _ {k , K _ {n}}}{k !} = 0 \quad \text {e t} \quad \pi_ {K _ {n}} (z) = z ^ {\underline {{n}}}.
$$

Remarque IV.3.5. Quelques remarques immédiates.

- Si  $G$  possède  $n$  sommets, alors  $m_{n,G} = n!$  car on assigne une couleur par sommet. On en déduit que le polynôme chromatique est monique.
Si  $G$  n'est pas connexe mais possede 2 composantes  $G_{1}$  et  $G_{2}$ , alors

$$
\pi_ {G} (z) = \pi_ {G _ {1}} (z). \pi_ {G _ {2}} (z).
$$

Cela résultat du fait que les sommets de  $G_{1}$  peuvent être colorés indépendamment de ceux de  $G_{2}$ .

- Il est clair que  $\pi_G(0) = 0$  pour tout graphe  $G$ .
- Il est impossible de colorier un graphe non vide avec aucune couleur.

Proposition IV.3.6. Soit  $k \in \mathbb{N}$ . Le nombre  $\pi_G(k)$  est le nombre de coloriages propres de  $G$  utilisant au plus  $k$  couleurs.

Chapitre IV. Coloriage

Exemple IV.3.7. Avec le graphe de la figure IV.16, si on calcule  $\pi_G(4)$ , on trouve  $\pi_G(4) = 84$  et donc, il y est possible de colorier proprement  $G$  avec au plus 4 couleurs de 84 façon distinctes.

Démonstration. Tout d'abord, il est inutile de considérer dans l'expression de  $\pi_G(k)$ , les termes d'exposant  $&gt;k$ . En effet, si  $i &gt; k$ , alors  $z^i$  évalué en  $k$  est nul. Ainsi, on a

$$
\pi_ {G} (k) = \sum_ {i = 1} ^ {\mathbf {k}} \frac {m _ {i , G}}{i !} k ^ {\underline {{i}}}.
$$

Ensuite, pour tout  $i \leq k$ , on dispose de  $\frac{m_{i,G}}{i!}$  partitions de  $V$  en  $i$  sous-ensembles de sommets indépendants. Considérons ces partitions et autorisons-nous cette fois àCHOISIR  $i$  couleurs parmi  $k$ , pour chaque partition  $C_1 \cup \dots \cup C_i$  de  $V$ , à  $C_1$ , on peut assigner  $k$  couleurs, pour  $C_2$ , on a  $k - 1$  choix possibles, ... et pour  $C_i$ , on a  $k - i + 1$  choix possibles. Ainsi, chacune des  $\frac{m_{i,G}}{i!}$  partitions de  $V$  en  $i$  sous-ensembles donnent lieu à  $k^i$  coloriages utilisant exactement  $i$  des  $k$  couleurs disponibles. La conclusion en découle.

Example IV.3.8. En continuant l'exemple précédent, le nombre de coloriages du graphe repris à la figure IV.16 utilisant au plus 3 couleurs vaut

$$
\pi_ {G} (3) = \underbrace {m _ {1 , G}} _ {= 0} 3 + \frac {m _ {2 , G}}{2 !} 3. 2 + \frac {m _ {3 , G}}{3 !} 3. 2. 1 = 0 + 6 + 2. 6 = 1 8.
$$

Ces coloriages sont donnés par

|  v1 | v2 | v3 | v4  |
| --- | --- | --- | --- |
|  1 | 2 | 1 | 2  |
|  2 | 1 | 2 | 1  |
|  1 | 3 | 1 | 3  |
|  3 | 1 | 3 | 1  |
|  2 | 3 | 2 | 3  |
|  3 | 2 | 3 | 2  |
|  v1 | v2 | v3 | v4  |
| --- | --- | --- | --- |
|  1 | 2 | 1 | 3  |
|  1 | 3 | 1 | 2  |
|  2 | 1 | 2 | 3  |
|  2 | 3 | 2 | 1  |
|  3 | 1 | 3 | 2  |
|  3 | 2 | 3 | 1  |
|  v1 | v2 | v3 | v4  |
| --- | --- | --- | --- |
|  2 | 1 | 3 | 1  |
|  3 | 1 | 2 | 1  |
|  1 | 2 | 3 | 2  |
|  3 | 2 | 1 | 2  |
|  1 | 3 | 2 | 3  |
|  2 | 3 | 1 | 3  |

Corollaire IV.3.9. Le nombre chromatique de  $G$  est le plus petit entier  $k$  tel que  $\pi_G(k) \neq 0$ .

Ainsi, la détermination du nombre chromatique d'un graphe à  $n$  sommets peut se ramener à l'estimation des zéros d'un polynôme de degré  $n$ . Par exemple, avec le graphe repris à la figure IV.16, on a  $\pi_G(1) = 0$  et  $\pi_G(2) \neq 0$ .

Le polynôme chromatique d'un graphe peut être obtenu par une technique semblable à celle développée pour obtenir la formule de Cayley comptant le nombre de sous-arbres couvrants (proposition II.5.4).

Proposition IV.3.10. Si  $e$  est une arête de  $G$  (qui n'est pas une boucle), alors le polynôme chromatique satisfait la relation

$$
\pi_ {G} (z) = \pi_ {G - e} (z) - \pi_ {G \cdot e} (z).
$$

IV.4. Coloriage d'arêtes et théorème de Ramsey

Démonstration. Si on considère tous les coloriages propres de $G - e$ avec exactement $k$ couleurs. Il y en a de deux types : ceux pour lesquels on assigne aux extrémités de $e$ deux couleurs distinctes (resp. une même couleur). Ceux du premier type sont en bijection avec les coloriages propres de $G$ utilisant $k$ couleurs. Pour conclure, on remarque que ceux du second type sont en bijection avec les coloriages propres de $G \cdot e$ utilisant $k$ couleurs.

Corollaire IV.3.11. Le polynôme chromatique d'un arbre $T$ à $n$ sommets vaut

$$
\pi_T(z) = z(z - 1)^{n-1}.
$$

Démonstration. On procède par récurrence sur $n$. Le cas $n = 1$ est immédiat. Supposons le résultat acquis pour $n \geq 1$ et vérifions-le pour $n + 1$. Si un arbre possède $n + 1$ sommets, il a au moins un sommet de degré 1 et soit $e$, l'arête incidente à ce sommet. Ainsi, $T - e$ possède deux composantes : un sommet isolé dont le polynôme chromatique vaut $z$ et un arbre à $n$ sommets (qui n'est autre que la contraction $T \cdot e$) de polynôme chromatique $z(z - 1)^{n-1}$ (par hypothèse de récurrence). On en conclut que

$$
\pi_{T - e}(z) = z \pi_{T \cdot e}(z).
$$

Par la proposition précédente,

$$
\pi_T(z) = \pi_{T - e}(z) - \pi_{T \cdot e}(z) = (z - 1) \pi_{T \cdot e}(z) = z(z - 1)^n.
$$

## 4. Coloriage d'arêtes et théorème de Ramsey

Bien évidemment, on peut considérer le problème de coloriage "dual" consistant à vouloir attribuer à chaque arête d'un graphe une couleur de façon telle que deux arêtes adjacentes reçoivent des couleurs distinctes. Nous préférons dans cette section introduire le célèbre théorème de Ramsey qui possède de nombreuses applications dans diverses branches des mathématiques$^{8}$; aussi bien en théorie des nombres qu'en analyse harmonique ou qu'en géométrie.

La question que l'on se pose est la suivante. Etant donné un entier $s$, existe-t-il un entier $n$ (dépendant de $s$) tel pour tout coloriage des arêtes de $K_n$ avec deux couleurs (par convention, le rouge et le bleu), $K_n$ contient un sous-graphe $K_s$ formé d'arêtes d'une même couleur ? Il n'est pas évident qu'à priori cette question possède une solution. En effet, on pourrait penser que certains coloriages permettent de mettre en défaut la propriété désirée quel que soit $n$. En fait, il n'en est rien, pour $n$ suffisamment grand, il existe toujours dans $K_n$ une copie de $K_s$ monochromatique et ce, quel que soit le coloriage envisagé.

$^{8}$V. Rosta, *Ramsey Theory Applications*, www.combinatorics.org/Surveys/ds13.pdf

Chapitre IV. Coloriage

En fait, nous allons définir le nombre  $R(s,t)$  comme étant le plus petit  $n$  tel que  $K_{n}$  contienne une copie de  $K_{s}$  rouge ou une copie de  $K_{t}$  bleue. Il nous faudra bien sur montrer que ces nombres existent (cf. théorème IV.4.3). Mais si de tels nombres existent, par définition même de  $R(s,t)$ , on peut énoncer le résultat suivant.

Théorème IV.4.1 (Ramsey (1930)). Il existe un plus petit entier  $R(s, t)$  tel que pour tout  $n \geq R(s, t)$ , tout coloriage de  $K_{n} = (V, E)$ ,  $c: E \to \{\mathcal{R}, \mathcal{B}\}$ , est tel que  $G$  contient une copie de  $K_{s}$  colorée en  $\mathcal{R}$  ou une copie de  $K_{t}$  colorée en  $\mathcal{B}$ .

Il est tout d'abord clair que  $R(s,t) = R(t,s)$  pour tous  $s,t\geq 2$ . En effet, il suffit d'inverser les couleurs rouge et bleue attribuées aux différentes arêtes dans les coloriages envisagés.

De plus,

(10)  $R(s,2) = R(2,s) = s.$

En effet, dans tout coloriage des arêtes de  $K_{s}$ , au moins une arête est bleue ou alors elles sont toutes rouges. De plus, il est clair qu'une valeur inférieure à  $s$  ne peut convenir.

Example IV.4.2. Un rapide exemple montre que  $R(3,3) &gt; 5$ . En effet, il suffit d'exhiber un coloriage des arêtes de  $K_{5}$  ne contenant aucune copie de  $K_{3}$  monochromatique. A la figure IV.17, on désigne les deux couleurs grâce à des traits pleins et des traits pointillés. Pour vérifier que  $R(3,3) = 6$ , il faut

![img-165.jpeg](img-165.jpeg)
FIGURE IV.17.  $R(3,3) &gt; 3,4,5$

passer en revue tous les coloriages de  $K_{6}$  et s'assurer qu'ils contiennent tous une copie de  $K_{3}$  monochromatique! Ainsi,  $K_{n}$  contient  $\mathrm{C}_n^2$  arêtes pouvant chacune être coloriée en rouge ou en bleu. Le tableau suivant reprend le nombre de coloriages possibles des arêtes de  $K_{n}$  pour les première valeurs

IV.4. Coloriage d'arêtes et théorème de Ramsey

de  $n$

|  n | 2Cn2  |
| --- | --- |
|  3 | 8  |
|  4 | 64  |
|  5 | 1024  |
|  6 | 32768  |
|  7 | 2097152  |
|  8 | 268435456  |
|  9 | 68719476736  |
|  10 | 35184372088832  |
|  11 | 36028797018963968  |
|  12 | 73786976294838206464  |
|  13 | 302231454903657293676544  |
|  14 | 2475880078570760549798248448  |

Il nous suffit de démontré le résultat suivant.

Théorème IV.4.3 (Erdős, Szekeres (1935)). Pour tous  $s, t \geq 2$ , le nombre  $R(s, t)$  existe. De plus, on a

$$
R (s, t) \leq \mathrm {C} _ {s + t - 2} ^ {s - 1}
$$

et si  $s,t\geq 3$  ，alors

$$
R (s, t) \leq R (s - 1, t) + R (s, t - 1).
$$

Démonstration. Pour la première partie de la preuve, on procède par récurrence sur  $s + t$ . Au vu de (10), on sait que  $R(2,t)$  et  $R(s,2)$  existent toujours. Donc, en particulier,  $R(s,t)$  existe si  $s + t \leq 5$ . Nous allons supposer que  $R(s,t)$  existe pour tous  $s,t$  tels que  $s + t &lt; N$ . Nous allons montré que  $R(s,t)$  existe encore pour  $s + t = N$ , avec  $N \geq 6$ . Si  $s$  ou  $t$  valent 2, le résultat est entièrement démontré. Nous pouvons donc supposer  $s,t \geq 3$ .

Il nous suffit de trouver un entier  $n$  tel que tout coloriage de  $K_{n}$  contient toujours une copie de  $K_{s}$  rouge ou de  $K_{t}$  bleue. De cette manière, on aura majoré  $R(s,t)$ . Pour cela, nous allons montré que tout graphe  $G = (V,E)$  ayant  $n = R(s - 1,t) + R(s,t - 1)$  sommets contient un sous-graphe isomorphe à  $K_{s}$  ou un ensemble de  $t$  sommets indépendants.

Chapitre IV. Coloriage

![img-166.jpeg](img-166.jpeg)
FIGURE IV.18. Un graphe et un coloriage de  $K_{n}$ .

![img-167.jpeg](img-167.jpeg)

En effet, par hypothèse de récurrence, les nombres  $R(s - 1,t)$  et  $R(s,t - 1)$  existent. De plus, prendre un graphe arbitraire  $G$  ayant  $n$  sommets revient exactement à considérer un sous-graphe monochromatique de  $K_{n}$  (grace à  $G$ , on sélectionne certaines arêtes qui recoivent toutes la même couleur, les autres arêtes de  $K_{n}$  non presents dans  $G$  receivevant la seconde couleur $^{9}$ .)

Soit  $v$  un sommet de  $G$ . Désignons par  $A_v = V \setminus (\nu(v) \cup \{v\})$ , l'ensemble des sommets distincts de  $v$  et qui ne sont pas voisins de  $v$ . Puisque  $G$  a

$$
\# V \setminus \{v \} = R (s - 1, t) + R (s, t - 1) - 1
$$

sommets distincts de  $v$ , alors

$$
\# \nu (v) \geq R (s - 1, t) \text { ou } \# A _ {v} \geq R (s, t - 1).
$$

En effet,  $\nu (v)$  et  $A_{v}$  partitionnent  $V\setminus \{v\}$  (donc  $\# \nu (v) + \# A_v = \# V\setminus \{v\})$

Supposons tout d'abord que  $\# \nu (v)\geq R(s - 1,t)$ . Par définition du nombre de Ramsey  $R(s - 1,t)$ , le sous-graphe de  $G$  induit par  $\nu (v)$  contient un sous-graphe  $B$  isomorphe à  $K_{s - 1}$  ou un ensemble de sommets indépendants de taille  $t$ . Dans le premier cas, le sous-graphe induit par  $B\cup \{v\}$  est isomorphe à  $K_{s}$  (en effet,  $v$  est adjacent à tous les sommets de  $B$ ). Dans le second cas, on dispose directement d'un sous-ensemble de sommets indépendants de taille  $t$ .

Supposons à présent que  $\# A_v \geq R(s, t - 1)$ . Par définition du nombre de Ramsey  $R(s, t - 1)$ , le sous-graphe de  $G$  induit par  $A_v$  contient un sous-graphe isomorphe à  $K_s$  (ce qui suffit) ou un ensemble  $C$  de sommets indépendants de taille  $t - 1$ . Dans ce dernier cas,  $C \cup \{v\}$  est un ensemble de sommets indépendants de taille  $t$  (en effet,  $v$  n'est adjacent à aucun sommet de  $C$ ).

L'inégalité est facile à vérifier. On procède une fois encore par récurrence sur  $s + t$ . Pour  $s = 2$  (ou  $t = 2$ ), on a bien

$$
\underbrace {R (2 , t)} _ {= t} \leq \underbrace {\mathrm {C} _ {t} ^ {1}} _ {= t}.
$$

Supposons que  $R(s,t) \leq \mathrm{C}_{s + t - 2}^{s - 1}$  pour tous  $s, t$  tels que  $s + t &lt; N$  et vérifièn-le pour  $s + t = N$ . Une fois encore, nous pouvons supposer  $s, t \geq 3$ . Par

IV.4. Coloriage d'arêtes et théorème de Ramsey

la première partie de la preuve et en utilisant l'hypothèse de récurrence, il vient

$$
R (s, t) \leq R (s - 1, t) + R (s, t - 1) \leq \mathrm {C} _ {s + t - 3} ^ {s - 2} + \mathrm {C} _ {s + t - 3} ^ {s - 1}.
$$

On conclus par la formule du triangle de Pascal.

Example IV.4.4. La détermination précise de  $R(s, t)$  est extrémement complexe (il est difficile d'un point de vue combinatoire d'énumérer tous les coloriages de  $K_{n}$ ). Par exemple, la détermination de  $R(3, 3)$  date de 1955, alors que l'estimation de  $R(3, 12)$  reprise ci-contre date de 2001.

|  j | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  R(3,j) | 6 | 9 | 14 | 18 | 23 | 28 | 36 | [40,43] | [46,51] | [52,59]  |

On consultera par exemple l'article "Small Ramsey Number, S. P. Radziszowski, http://www.combinatorics.org/Surveys/ds1.pdf" qui donne un état de l'art en la matière.

.

141

# CHAPITRE V

## Annexe : implémentation en C

### 1. Pointeurs, allocation mémoire et listes chaînées

Dans cette courte section, nous rappelons quelques fondements essentiels du langage C dans le but d'appréhender au mieux les structures données permettant de coder les graphes. Nos rappels concernent principalement la gestion et l'allocation dynamique de la mémoire. Nous supposons connus¹ les boucles, les fonctions ou encore la gestion des entrées/sorties.

#### 1.1. Adresses. Toute variable possède une adresse mémoire. Cette adresse est accessible grâce à l'opérateur &amp;. Autrement dit, si x est l'identificateur d'une variable, alors &amp;x représente l'adresse de x.

**Exemple V.1.1.** Après compilation et exécution du court programme suivant,

```c
#include <stdio.h>
main()
{
int i;
printf("&gt; %u \n",&amp;i);
}
```

vous devriez obtenir un affichage ressemblant à

&gt; 3221219076

Ce programme affiche simplement (sous forme décimale) l'adresse mémoire² de la variable i.

On peut accéder à une variable non seulement grâce à son identificateur (si elle en possède un), mais aussi grâce à son adresse. Cela est rendu possible au moyen de l'opérateur *. L'adresse d'une variable est une donnée qui peut être stockée dans une variable. Une telle variable est appelée *pointeur*.

**Exemple V.1.2.** Dans cet exemple, on définit deux variables : un entier i et un pointeur vers un entier. Cette dernière variable va être utilisée pour stocker l'adresse de la première variable. Connaissant l'adresse de i, on peut modifier le contenu de i en se référant au pointeur vers ce dernier.

---

¹Consulter si nécessaire un ouvrage de référence sur le C comme [13].

²La gestion de la mémoire est laissée au système d'exploitation. Vous pouvez dès lors obtenir des résultats différents suivant la machine employée et même d'une exécution à l'autre sur une machine fixée.</stdio.h>

Chapitre V. Annexe : implémentation en C

```c
include<stdio.h>
main()
{
int i;
int *pointeur;
i=10;
pointeur=&amp;i
*pointeur=55;
printf("&gt; %d\n",*pointeur);
}
```

Ce programme affiche : 55.

![img-168.jpeg](img-168.jpeg)
FIGURE V.1. Un pointeur et une variable.

Exemple V.1.3. Puisqu'un pointeur est lui-même une variable (qui contient l'adresse d'une autre variable), on peut définir un pointeur vers un pointeur (i.e., une variable contenant l'adresse d'un pointeur).

```c
#include<stdio.h>
main()
{
int a=1;
int *p;
int **q;
printf("%d, ",a);
p=&amp;a
*p=2;
printf("%d, ",a);
q=&amp;p
**q=3;
printf("%d\n",a);
}
```

Affichage: 1, 2, 3.

L'utilisation des pointeurs présente deux intérêts majeurs : passer l'adresse d'une variable à une fonction et allouer dynamiquement de la mémoire.

Exemple V.1.4. Un exemple archi-classique consiste à définir une fonction inversant le contenu de deux variables. Voici d'abord un programme erroné.

```c
/* Exemple errone --- */
#include <stdio.h>
void swap(int x, int y);
main()
{
int a=1,b=2;</stdio.h></stdio.h></stdio.h>

V.1. Pointeurs, allocation mémoire et listes chainées

```c
printf("&gt; a=%d, b=%d \n",a,b);
swap(a,b);
printf("&gt; a=%d, b=%d \n",a,b);
}
void swap(int x, int y)
{
int temp;
temp=x;
x=y;
y=temp;
}
```

Lorsqu'on compile et exécute ce programme, on obtient l'affichage suivant et on n'a donc pas le résultat escompté.

```txt
&gt; a=1, b=2
&gt; a=1, b=2
```

La raison en est toute simple: lorsqu'une variable est passée à une fonction, C effectue une copie de cette variable et la fonction ne travaille donc pas sur les variables transmises mais sur une copie de celles-ci³. Lorsque la fonction s'achève, les copies sont détruites. On peut régler ce problème en transmettant l'adresse des variables à considérer. En plus de résoudre notre problème, cela permet de faire des économies de mémoire qui peuvent s'avérer importantes lorsqu'on transmet des variables de grande taille (dans ce cas, on transmet uniquement une adresse et ce n'est pas toute la variable qui est dupliquée). Voici à présent le programme corrigé. Le lecteur remarque la présence des opérateurs &amp; et *.

```c
/* Exemple correct --- */
#include <stdio.h>

void swap(int *x, int *y);
main()
{
int a=1,b=2;
printf("&gt; a=%d, b=%d \n",a,b);
swap(&amp;a,&amp;b);
printf("&gt; a=%d, b=%d \n",a,b);
}

void swap(int *x, int *y)
{
int temp;
temp=*x;
*x=*y;
*y=temp;
}
```

Cette fois, le programme a bien le comportement souhaité !

³On parle parfois de transmission par valeurs.</stdio.h>

Chapitre V. Annexe : implémentation en C

&gt; a=1, b=2
&gt; a=2, b=1

En effet, la fonction swap a accès au contenu des variables a et b grâce à leur adresse. On a schématiquement la situation représentée à la figure V.2.

![img-169.jpeg](img-169.jpeg)
FIGURE V.2. L'échange de deux variables.

1.2. Allocation dynamique. Il se peut qu'en cours d'exécution d'un programme, il soit nécessaire d'affecter une valeur à une nouvelle variable. Si cette dernière n'a pas été déclarée avant la compilation, le programme ne peut réaliser cette affection. On peut pallier à cet inconvenient grâce à une instruction comme malloc. Cette fonction create un espace mémoire pour y stocker une donnée. Il faut donc lui préciser la taille de l'espace mémoire à réserver (En effet, chaque type de variable consomme une quantité plus ou moins grande de mémoire). De plus, malloc renvoie l'adresse de la zone mémoire qui vient d'être réservée. Cette dernière sera donc stockée par un pointeur. Si l'allocation mémoire n'est pas possible, malloc renvoie NULL.

Example V.1.5. Remarquer que, dans l'exemple qui suit, l'unique variable définie p est un pointeur vers un entier.

```c
include<stdio.h>
main()
{
int *p;
p=(int *) malloc(sizeof(int));
*p=3;
printf("adresse : %u contenu : %d \n",p,*p);
}
```

En fait, malloc renvoie un pointeur "générique". Ainsi, au lieu de taper simplement

p=malloc(sizeof(int));

on a effectué ce qu'on appelle un transtypage ("casting"), i.e., on transforme le pointeur "générique" en un pointeur vers un entier. Sans ce transtypage, le compilateur (ici gcc) produit en général un message d'alerte comme cédessous

test.c:5: warning: assignment makes pointer from integer without a cast</stdio.h>

V.1. Pointeurs, allocation mémoire et listes chainées

Remarque V.1.6. Allouer dynamiquement la mémoire nécessite aussi de rendre la mémoire au système d'exploitation lorsque le programme n'a plus besoin d'une zone mémoire affectée précédemment. Pour se faire, on utilise l'instruction free. Ainsi, dans l'exemple précédent, on devrait ajouter en fin de programme la ligne suivante.

free(p);

1.3. Structures. On peut définir des variables plus complexes en utilisant la notion de structure. Cela permet de regrouper un ensemble de variables en une seule et même variable.

Exemple V.1.7. Par cet exemple, nous rappelons comment définir une structure et comment acceder aux différents champs qui la constituent. Ici, nous déclarons simplement une variable et initialisons les deux champs correspondant.

```c
struct couple {
int x;
int y;
};
main()
{
struct couple a;
a.x=3;
a.y=5;
}
```

En utilisant l'instruction typedef, nous pouvons définir des synonymes. Cela permet d'alleger quelque peu le code (par exemple, on ne doit plus réécrire struct dans les déclarations de variables et les en-têtes de fonctions). Ainsi, le programme précédent peut se réécrire comme suit.

```c
struct couple {
int x;
int y;
};
typedef struct couple PAIRE;

main()
{
PAIRE a;
a.x=3;
a.y=5;
}
```

Nous pouvons bien évidemment considérer un pointeur vers une structure. Avec la définition de PAIRE donnée dans l'exemple précédent, on a par exemple les affectations licites suivantes.

main()

Chapitre V. Annexe : implémentation en C

```c
{ PAIRE \*a; (\*a).x=3; a-&gt;y=5; }
```

Ici, a est un pointeur vers une PAIRE. Ainsi, il est normal d'accéder au premier champ de la paire avec (*a).x. Enfin, nous en profitons pour rappeler que a-&gt;y est un synonyme de (*a).y.

1.4. Listes chaînées. Nous allonsprésent cette construction à l'aide d'un exemple. Imaginons pouvoir stocker les éléments d'une liste de taille  $n$  d'entiers. Si  $n$  varie, il n'est pas possible de prévoir à l'avance la taille d'un tableau devant recevoir les  $n$  entiers. Ainsi, un tel tableau doit être créé dynamiquement, i.e., au moment où le programme est en cours d'exécution. Nous définissons une structure.

```c
struct elt {
int x;
struct elt *suivant;
};
```

Celle-ci est constituée d'un premier champ entier  $\mathbf{x}$  et d'un second champ suivant qui est un pointeur vers une structure du même type. Pour ne pas alourdir l'exposé, notre exemple ne va pas inclure d'interaction avec un utilisateur devant introduire une liste de nombres. Nous construisons simplement une liste chaine contenant 6 éléments à l'aide de deux pointeurs.

Remarque V.1.8. Nous en profitons également pour vous rappeler que le langage C est "case sensitive", i.e., la distinction est faite entre minuscules et majuscules. Ainsi, "elt" diffère de "ELT" (ou de "Elt").

```c
#include <stdio.h>
struct elt{
int x;
struct elt *suivant;
};
typedef struct elt ELT;

main()
{
int i;
ELT *p, *q;

/* construction de la liste chainee --- */
q=(ELT *) malloc(sizeof(ELT));
q-&gt;x=0;
q-&gt;suivant=NULL;

for (i=1; i&lt;=5; i++)</stdio.h>

V.1. Pointeurs, allocation mémoire et listes chainées

```c
{ p=(ELT *) malloc(sizeof(ELT)); p-&gt;x=i; p-&gt;suivant=q; q=p; } /* parcours de la liste chainee - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -  $\text{一}$  while (p-&gt;suivant !=NULL) { printf("  $\text{一} ^ { \text{一} }$  %d -- adresse %u \n",p-&gt;x,p); p=p-&gt;suivant; } printf("  $\text{一} ^ { \text{一} }$  %d -- adresse %u \n",p-&gt;x,p); /*libere la memoire - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -  $\text{一}$  Schématiquement, la création de la liste chainée se passse comme indiqué à la figure V.3. ![img-170.jpeg](img-170.jpeg)
FIGURE V.3. Création d'une liste chainee.

Le programme affiche un résultat semblable à :

&gt;5--adresse134520920
&gt;4--adresse134520904
&gt;3--adresse134520888
&gt;2--adresse134520872
&gt;1--adresse134520856
&gt;0--adresse134520840

Pour connaître le nombre d'octets nécessaires pour stocker un type de données sur une architecture donnée, on peut utiliser une instruction comme

Chapitre V. Annexe : implémentation en C

```c
printf("%d\n",sizeof(int));
printf("%d\n",sizeof(ELT));
```

## 2. Liste d'adjacence

Dans ce chapitre, on va principalement considérer l'implémentation des graphes orientés (i.e., avec boucle et sans multi-arcs). Le cas des multi-graphes se traite de manière sensiblement analogue, le cas échéant, nous attirerons l'attention du lecteur sur les éventuelles différences apparaissant dans le traitement des multigraphes. Bien sûr, les graphes non orientés peuvent être vus comme un cas particulier de graphes orientés symétriques. (On pourrait dès lors présenter une structure de données plus fine nécessitant un espace mémoire moins important, ici la structure de données générales code deux fois trop d'information).

Nous supposerons également disposer de deux fonctions de poids, l'une pour les arcs, l'autre pour les sommets. Ainsi, si $G = (V, E)$ est un graphe, alors on dispose des fonctions

$$
f: V \to \mathbb{N} \quad \text{et} \quad g: E \to \mathbb{N}.
$$

Un moyen commode de représenter un graphe est de considérer des listes d'adjacence. Cette structure de données est particulièrement bien adaptée aux graphes "peu denses"⁶ (c'est-à-dire comportant peu d'arcs par sommet). En effet, si un graphe comporte de nombreux arcs par sommet, il est plus efficace de coder le graphe par une matrice (on évite alors la manipulation plus lourde des listes chaînées).

Une représentation par liste d'adjacence est une liste chaînée de structures de liste d'adjacence. Ces dernières sont formées de deux champs : un sommet et une liste chaînée des sommets adjacents à celui-ci. Trois structures sont utilisées pour coder un graphe : ELTADJ, SOMMET et GRAPHE. La figure V.4 reprend schématiquement un graphe et son codage.

La structure la plus fondamentale permet de lister les sommets adjacents à un sommet donné. Autrement dit, elle sert à construire une liste d'adjacence des arcs issus d'un sommet donné. Le champ dest contient l'extrémité de l'arc, info contient un entier pouvant recevoir une quelconque information (un poids par exemple) à propos de l'arc codé et enfin, suivant est un pointeur vers l'élément suivant de la liste chaînée. La figure V.5 reprend schématiquement une liste d'adjacence.

Ensuite, la structure suivante permet de définir une liste chaînée destinée à coder les sommets du graphe. Chaque sommet possède un label (un indice) ainsi qu'une information qui lui est propre (par exemple, une couleur ou une valeur permettant de retenir si le sommet a déjà été visité). Les deux derniers champs de la structure sont plus spécifiques, suivant pointe simplement vers l'élément suivant de la liste chaînée. Enfin, adj est un pointeur vers une variable de type eltadj. Cela permet donc de lier un sommet à la liste

C'est un choix : simplifier la présentation ! De même, la structure d'arbre ne sera pas envisagée en tant que telle.

Gardez ce fait à l'esprit !

⁶En anglais, "sparse graphs".

V.2. Listed adjacency

![img-171.jpeg](img-171.jpeg)

![img-172.jpeg](img-172.jpeg)
FIGURE V.4. Implementation d'un graphe.

```txt
struct eltadj
{
int dest;
int info;
struct eltadj *suivant;
};
```

![img-173.jpeg](img-173.jpeg)
FIGURE V.5. Le type eltadj.

des sommets qui lui sont adjacents. La figure V.6 reprend schématiquement une liste chainee de sommets.

```txt
struct sommet {
int label;
int info;
struct sommet *suivant;
struct eltadj *adj;
};
```

![img-174.jpeg](img-174.jpeg)
FIGURE V.6. Le type sommet.

Enfin, la structure graphe est principalement un pointeur vers un sommet (premierSommet). Les autres champs de la structure permettent de stocker quelques informations utiles comme le nombre de sommets ou d'arcs du graphe ainsi qu'un pointeur vers le dernier sommet de la liste chainee. Dans de nombreuses procédures, les sommets recoivent un indice. La variable maxS stocke le plus grand indice qui a ete attribué a un sommet du graphe. Ainsi, en attribuant a un nouveau sommet l'indice maxS+1, on est

Chapitre V. Annexe : implémentation en C

certain que cet indice n'est pas déjà employé. La figure V.7 reprend schématiquement la structure de graphe.

```c
struct graphe {
int nbS;
int nbA;
int maxS;
struct sommet *premierSommet;
struct sommet *dernierSommet;
};
```

![img-175.jpeg](img-175.jpeg)
FIGURE V.7. Le type graphe.

2.1. Une manipulation simplifiée. Nous mettons à la disposition du lecteur une série de fonctions préprogrammées pour faciliter la manipulation des graphes et se concentrer principalement sur les algorithmes relevant de ce cours.

void initialiserGraphe(GRAPHE\*);

On transmet à cette fonction l'adresse d'une variable de type GRAPHE. La variable correspondante sera initialisée au graphe vide.

int ajouterSommet(GRAPHE *, int info);

On transmet à cette fonction l'adresse d'une variable de type GRAPHE et un entier. Le champ maxS est incrémenté d'une unité, un sommet de label maxS (après incrémentation) est ajouté au graphe en fin de liste et ce sommet porte l'information info fournie en argument.

|  Erreur | -1 | si l'allocation de mémoire n'est pas possible  |
| --- | --- | --- |

int ajouterArc(GRAPHE *, int a, int b, int info);

Crée un nouvel arc du sommet de label a vers le sommet de label b avec l'information supplémentaire donnée par info. Les arcs sont rangés dans la liste d'adjacence par indice croissant. Si un arc entre a et b existe déjà, le champ info est mis à jour sans créé de nouvel arc dans la liste d'adjacence. Cette fonction ne renvoie rien.

|  Erreur | -1 | si a n'existe pas  |
| --- | --- | --- |
|   |  -2 | si b n'existe pas  |
|   |  -3 | si l'allocation de mémoire n'est pas possible  |

V.2. Liste d'adjacence

```txt
- int supprimerSommet(GRAPHE *, int label);
Supprime un sommet et les arcs correspondants (i.e., les arcs ayant leur origine ou leur extrémité correspondant à label). Cette fonction libre la mémoire correspondante.
```

|  Erreur | -1 | si le sommet n'existe pas  |
| --- | --- | --- |

```txt
- int supprimerArc(GRAPHE *, int a, int b);
Supprime l'arc joignant a à b. Cette fonction libre la mémoire correspondante.
```

|  Erreur | -1 | si l'arc n'existe pas  |
| --- | --- | --- |

```txt
- void supprimerGraphe(GRAPHE *);
Supprime entièrement un graphe en libérant la mémoire correspondante. En fin de procédure, le graphe est automatiquement réinitialisé par la fonction initialiserGraphe.
```

```txt
- void afficherGraphe(GRAPHE *);
Il s'agit exclusivement d'une fonction d'affichage. Elle affiche à l'écran la valeur des champs nbS, nbA et maxS ainsi que l'ensemble des sommets, des arcs et des informations qu'ils portent.
```

```txt
- int lireFichier(char *nomf, GRAPHE *);
```

On fournit à cette fonction le nom d'un fichier de données et un pointeur vers une variable de type GRAPHE. Ce fichier doit contenir des champs séparés par des virgules (et peut contenir arbitrairement des tabulations et des espaces, ceux-ci ne sont pas pris en compte). Un fichier valide contient des champs formés de nombres ou du seul caractère x. Chaque ligne (terminée par un retour chariot) doit contenir le même nombre de champs (les lignes vides sont ignorées). Le  $k$ -ème champ de la  $j$ -ème ligne du fichier contient le poids de l'arc entre les sommets  $j$  et  $k$  d'un graphe (on se limite ici à des poids positifs ou nuls). Si ce champ est x, il n'y a pas d'arcs entre les sommets  $j$  et  $k$ . Il est inutile d'initialiser le graphe avant d'appeler cette fonction. En effet, un tel appel est effectué en début de fonction.

|  Erreur | -1 | si le fichier n'est pas valide  |
| --- | --- | --- |

Exemple V.2.1. Voici un fichier codant le graphe donné à la figure V.4. Ici, les labels  $a, b, \ldots, f$  ont été remplacés par les entiers de 1 à 6.

Chapitre V. Annexe : implémentation en C

```txt
x,1,x,4
x,2,3,x
x,x,x,6
5,x,x,x
```

Remarque V.2.2. L'utilisation d'un tableau peut s'avérer commode pour l'encodage d'un graphe dans un fichier. En effet, après avoir placé les divers éléments dans les cases d'un tableau, le tableau propose une option du type "Save as" dans laquelle on peut spécifier que le type du fichier en sortie sera un (simple) texte. Le tableau demande alors à l'utilisateur de préciser un délimiteur pour les champs (il convient deCHOISIR la virgule) et un délimiteur pour le texte (il convient de ne prendre aucun délimiteur).

# 2.2. Utilisation des fonctions.

Example V.2.3. Voici deux variantes de programmes permettant d'encoder le graphe de la figure V.4.

```c
/* FICHIER main.c --- */
#include <stdio.h>
#include "graphes.h"
int main()
{
GRAPHE g;
int i;
initialiserGraphe(&amp;g);
for (i=1; i&lt;=4; i++) ajouterSommet(&amp;g,0);
ajouterArc(&amp;g,1,2,1);
ajouterArc(&amp;g,1,4,4);
ajouterArc(&amp;g,2,2,2);
ajouterArc(&amp;g,2,3,3);
ajouterArc(&amp;g,3,4,6);
ajouterArc(&amp;g,4,1,5);
afficherGraphe(&amp;g);
supprimerGraphe(&amp;g);
}
```

Dans ce second fichier, on utilise un fichier de données externes comme celui donné dans l'exemple V.2.1.

```c
/* FICHIER main.c --- */
#include <stdio.h>
#include "graphes.h"
int main()
{
GRAPHE g;
int i;
lireFichier("data.gr",&amp;g);
afficherGraphe(&amp;g);
supprimerGraphe(&amp;g);</stdio.h></stdio.h>

V.2. Liste d'adjacence
153

}

Ainsi, pour compiler le fichier et obtenir un fichier exécutable, vous avez besoin dans le répertoire courant non seulement du fichier source main.c mais aussi du fichier graphes.c et du fichier d'en-tête graphes.c. La compilation peut être lancée par la commande suivante⁷.

&gt; gcc graphes.c main.c ~o nom_du_programme

Bien évidemment, il vous faut également un fichier de données nommé data.gr pour utiliser la seconde variante.

2.3. Détail de l'implémentation. Le fichier d'en-tête est on ne peut plus classique. Il sert uniquement à la déclaration des différents types de données, des fonctions et d'une constante MAX utilisée pour la lecture de fichiers.

```c
/* graphes.h --- */
#if defined(GRAPHES_H)
#else
#define GRAPHES_H

#define MAX 10000

struct eltadj {
int dest;
int info;
struct eltadj *suivant;
};

struct sommet {
int label;
int info;
struct sommet *suivant;
struct eltadj *adj;
};

struct graphe {
int nbS;
int nbA;
int maxS;
struct sommet *premierSommet;
struct sommet *dernierSommet;
};

typedef struct graphe GRAPHE;
typedef struct sommet SOMMET;
typedef struct eltadj ELTADJ;

void initialiserGraphe(GRAPHE *);
int ajouterSommet(GRAPHE *, int info);
```

⁷Ou une instruction similaire suivant le système employé.

Chapitre V. Annexe : implémentation en C

```c
int ajouterArc(GRAPHE *, int a, int b, int info);
int supprimerSommet(GRAPHE *, int label);
int supprimerArc(GRAPHE *, int a, int b);
void supprimerGraphe(GRAPHE *);
void afficherGraphe(GRAPHE *);
int lireFichier(char *nomf, GRAPHE *);
```

#endif

Nous allons à présent détailler l'implémentation des fonctions du module graphes.c permettant la gestion de la structure de données de type graphe.

- La fonction `initialiserGraphe` est très simple, il s'agit uniquement d'affectations élémentaires.

```c
void initialiserGraphe(GRAPHE *g)
{
g-&gt;nbS=0;
g-&gt;nbA=0;
g-&gt;maxS=0;
g-&gt;premierSommet=NULL;
g-&gt;dernierSommet=NULL;
}
```

- La fonction `ajouterSommet` ajoute un élément de type `sommet` comme `dernier élément` de la liste châineé contenant les sommets du graphe d'adresse *g. Le champ `maxS` de g (contenant le plus grand label attribué en cours d'exécution à un sommet du graphe) est incrémenté d'une unité et cette valeur est aussi le label du sommet à ajouter. Il est de plus nécessaire de mettre à jour les champs `dernierSommet`, `nbS` et éventuellement `premierSommet` (s'il s'agit du premier sommet créé dans le graphe) de g. On notera l'intérêt d'avoir à sa disposition le champ `dernierSommet` permettant un accès immédiat à l'extrémité de la liste châineé.

```c
int ajouterSommet(GRAPHE *g, int info)
{
SOMMET *pointeur;
g-&gt;maxS++;
pointeur=(SOMMET *) malloc(sizeof(SOMMET));
if (pointeur == NULL)
{
printf("Erreur! Memoire insuffisante pour creer un sommet\n");
return -1;
}
else
{
pointeur-&gt;label=g-&gt;maxS;
pointeur-&gt;info=info;
pointeur-&gt;suivant=NULL;
pointeur-&gt;adj=NULL;
if (g-&gt;nbS == 0)
{
g-&gt;premierSommet=pointeur;
g-&gt;dernierSommet=pointeur;
}
else
{
g-&gt;dernierSommet-&gt;suivant=pointeur;
g-&gt;dernierSommet=pointeur;

V.2. Liste d'adjacence

```c
}
g-&gt;nbS++;
return pointeur-&gt;label;
}
```

- La fonction `ajouterArc` détermine tout d'abord si le graphe contient bien les sommets de label donnés par `a` et par `b`. Si l'un d'eux n'existe pas, la fonction renvoie un code d'erreur égal respectivement à `-1` ou `-2`. Sinon, deux situations peuvent se présenter. L'arc existe, le champ `info` est simplement mis à jour. L'arc n'existe pas. Il est créé au sein de la liste chaînée des sommets adjacents au sommet de label `a`. Il est positionné dans cette liste de telle sorte que les labels des sommets présents sont ordonnés par ordre croissant.

```c
int ajouterArc(GBAPHE *g, int a, int b, int info)
{
SOMMET *psommet, *psommet2;
ELTADJ *padj, *precedent;
psommet=g-&gt;premierSommet;

/* on parcourt les sommets jusqu'à trouver a */
while (psommet != NULL)
{
if (psommet-&gt;label == a) break;
psommet=psommet-&gt;suivant;
}
if (psommet == NULL)
{
printf("Erreur! Creation d'un arc dont l'origine n'existe pas\n");
return -1;
}
else /* on a trouver a */
{
padj=psommet-&gt;adj;

/* on parcourt les sommets pour trouver b */
psommet2=g-&gt;premierSommet;
while (psommet2 != NULL)
{
if (psommet2-&gt;label == b) break;
psommet2=psommet2-&gt;suivant;
}
if (psommet2 == NULL)
{
printf("Erreur! Creation d'un arc dont l'extremite n'existe pas\n");
return -2;
}
else /* on a trouver a et b */
{
if (padj == NULL) /* la liste d'adjacence est vide */
{
padj=(ELTADJ *) malloc(sizeof(ELTADJ));
if (padj == NULL)
{
printf("Erreur! Memoire insuffisante pour créer un sommet\n");
return -3;
}
else
{
psommet-&gt;adj=padj; /* premier élément de la liste d'adjacence */
padj-&gt;suivant=NULL;
}
}
else /* la liste d'adjacence est non vide, on la parcourt pour voir si b s'y trouve */
}

Chapitre V. Annexe : implémentation en C

```c
{
if (padj-&gt;dest &gt; b)
{
padj = (ELTADJ *) malloc(sizeof(ELTADJ));
if (padj == NULL)
{
printf("Erreur! Mémoire insuffisante pour créer un sommet\n");
return -3;
}
else
{
padj-&gt;suivant = psommet-&gt;adj;
psommet-&gt;adj = padj;
}
}
else
{
while (padj != NULL)
{
if (padj-&gt;dest == b) {padj-&gt;info = info; break;} /* l'arc existe, update info */
if (padj-&gt;dest &gt; b) {padj = NULL; break;} /* on dépasse b sans le trouver */
precedent = padj;
padj = padj-&gt;suivant;
}
if (padj == NULL) /* l'arc n'existe pas, il faut le créer */
{
padj = (ELTADJ *) malloc(sizeof(ELTADJ));
if (padj == NULL)
{
printf("Erreur! Mémoire insuffisante pour créer un sommet\n");
return -3;
}
else
if (precedent-&gt;suivant == NULL) /* élément ajouter à la fin */
{
precedent-&gt;suivant = padj;
padj-&gt;suivant = NULL;
}
else /* élément ajouter "au milieu" pour garder ordre */
{
padj-&gt;suivant = precedent-&gt;suivant;
precedent-&gt;suivant = padj;
}
}
}
}
padj-&gt;dest = b;
padj-&gt;info = info;
g-&gt;nbA++;
}
return 0;
}
```

- La fonction `supprimerSommet` teste si le sommet de label a existe. Si tel est le cas, ce sommet est supprimé de la liste chaînée constituée des sommets du graphe (ce qui nécessite la mise à jour de pointeurs de cette liste). On supprime aussi la liste chaînée des sommets adjacents à a et les occurrences du sommet a dans les listes d'adjacence des autres sommets (avec les mises à jour ad hoc des listes chaînées).

```c
int supprimerSommet(GRAPHE *g, int a)
{
SOMMET *psommet, *precedent;
ELTADJ *padj, *suivant, *precedent_adj;
int flag_premier_sommet, flag_premier_arc;
if (g-&gt;premierSommet == NULL)

V.2. Liste d'adjacence

```c
{
printf("Erreur! Graphe vide, suppression impossible\n");
return -1;
}
else
{
psommet = g-&gt;premierSommet;
flag_premier_sommet = 1;
while (psommet != NULL)
{
if (psommet-&gt;label == a) break;
else
{
flag_premier_sommet = 0;
precedent = psommet;
psommet = psommet-&gt;suivant;
}
}
if (psommet == NULL)
{
printf("Erreur! Le sommet a supprimer n'existe pas\n");
return -1;
}
else
{
if (psommet-&gt;suivant == NULL) g-&gt;dernierSommet = precedent;
if (flag_premier_sommet == 1) g-&gt;premierSommet = psommet-&gt;suivant;
else precedent-&gt;suivant = psommet-&gt;suivant;
padj = psommet-&gt;adj;
free(psommet);
g-&gt;nbS--;
while (padj != NULL)
{
suivant = padj-&gt;suivant;
free(padj);
g-&gt;nbA--;
padj = suivant;
}
}
}
```

/* il faut aussi supprimer les arcs ayant le sommet a supprimer comme extremite */
psommet = g-&gt;premierSommet;
while (psommet != NULL)
{
padj = psommet-&gt;adj;
while (padj != NULL)
{
flag_premier_arc = 1;
if (padj-&gt;dest == a) break;
else
{
flag_premier_arc = 0;
precedent_adj = padj;
padj = padj-&gt;suivant;
}
}
if (padj != NULL)
{
if (flag_premier_arc == 1) psommet-&gt;adj = padj-&gt;suivant;
else precedent_adj-&gt;suivant = padj-&gt;suivant;
free(padj);
g-&gt;nbA--;
}
psommet = psommet-&gt;suivant;
}
return 0;
}

Chapitre V. Annexe : implémentation en C

- La fonction supprimerArc supprime, si possible, de la liste d'adjacence du sommet de label a l'arc joignant les sommets de label a et b. Une erreur est renvoie sinon.

```c
int supprimerArc(GRAPHE *g, int a, int b)
{
SOMMET *psommet;
ELTADJ *padj, *precedent_adj;
int flag_premier_arc;
if (g-&gt;premierSommet == NULL)
{
printf("Erreur! Graphe vide, suppression impossible\n");
return -1;
}
else
{
psommet = g-&gt;premierSommet;
while (psommet != NULL)
{
if (psommet-&gt;label == a) break;
else psommet = psommet-&gt;suivant;
}
if (psommet == NULL)
{
printf("Erreur! L'extrémité de l'arc a supprimer n'existe pas !!!!\n");
return -1;
}
else
{
padj = psommet-&gt;adj;
flag_premier_arc = 1;
while (padj != NULL)
{
if (padj-&gt;dest == b) break;
else
{
flag_premier_arc = 0;
precedent_adj = padj;
padj = padj-&gt;suivant;
}
}
if (padj != NULL)
{
if (flag_premier_arc == 1) psommet-&gt;adj = padj-&gt;suivant;
else precedent_adj-&gt;suivant = padj-&gt;suivant;
free(padj);
g-&gt;nhA--;
}
else
{
printf("Erreur! L'extrémité de l'arc a supprimer n'existe pas !!!!\n");
return -1;
}
}
return 0;
}
}
```

- La fonction supprimerGraphe libre l'ensemble de la mémoire utilisée par un graphe et rend simplement un graphe "vide".

```c
void supprimerGraphe(GRAPHE *g)
{
SOMMET *psommet, *temps;
ELTADJ *padj, *tempadj;
psommet = g-&gt;premierSommet;
while (psommet != NULL)
{
padj = psommet-&gt;adj;

V.2. Liste d'adjacence

```c
while (padj != NULL)
{
tempadj = padj;
padj = padj-&gt;suivant;
free(tempadj);
}
temps = psommet;
psommet = psommet-&gt;suivant;
free(temp);
}
initialiserGraphe(g);
```

- La fonction `afficherGraphe` affiche tout d'abord plusieurs informations de base sur le graphe d'adresse `*g` (nombre de sommets, nombre d'arêtes, ...). Ensuite la liste chaînée des sommets est parcourue et pour chacun de ces sommets, on parcourt l'entièreté de la liste d'adjacence correspondante.

```c
void afficherGraphe(GRAPHE *g)
{
SOMMET *psommet;
ELTADJ *padj;
if (g-&gt;premierSommet == NULL) printf("graphe vide\n");
else
{
printf("nbS=%d, nbA=%d, label max.=%d\n", g-&gt;nbS, g-&gt;nbA, g-&gt;maxS);
psommet = g-&gt;premierSommet;
do
{
printf("\n");
printf("Sommet de label: %d, info: %d\n", psommet-&gt;label, psommet-&gt;info);
if (psommet-&gt;adj == NULL) printf(" -&gt; ce sommet n'a aucun arc sortant\n");
else
{
padj = psommet-&gt;adj;
do
{
printf(" -&gt; arc de %d vers %d avec l'info. %d\n", psommet-&gt;label, padj-&gt;dest, padj-&gt;info);
padj = padj-&gt;suivant;
}
while (padj != NULL);
}
printf("\n");
psommet = psommet-&gt;suivant;
}
while (psommet != NULL);
}
}
```

- La fonction `lireFichier` ouvre en lecture un fichier texte. Ce fichier, s'il satisfait les conventions énoncées précédemment (cf. exemple V.2.1), est utilisé pour construire le graphe correspondant.

```c
int lireFichier(char *nomf, GRAPHE *g)
{
FILE *fp;
char ligne[MAX+1];
int temp, i, j, nbS1, nbLigne, sommet, nbElt, creerArc;

initialiserGraphe(g);
fp = fopen(nomf, "r"); /* ouvre un fichier en lecture */
nbLigne = 0; /* compte les lignes du fichier */
sommet = 0; /* label du sommet en cours */
nbS1 = 0; /* compte les sommets de la 1ère ligne */
while (fgets(ligne, MAX, fp) != NULL)

Chapitre V. Annexe : implémentation en C

```c
{ nbLigne++; /* compte le nombre de lignes du fichier */
if (ligne[0] != '\n') /* on passe les lignes vides */
{
i=0;
if (nbS1 == 0) /* compte les sommets de la 1ere ligne */
{
nbS1=1;
while (ligne[i] != '\n')
{
if (ligne[i] == ',') nbS1++;
i++;
}
for (j=1; j&lt;=nbS1; j++)
{
ajouterSommet(g,0);
}
i=0; /* on relit la 1ere ligne */
}
sommet++; /* origine des arcs */
nbElt=0; /* controle le nombre d'arcs crees */
while (ligne[i] != '\n')
{
temp=0; /* va contenir l'extremite */
creerArc=1;
while (ligne[i] != ', &amp;&amp; ligne[i] != '\n')
{
while (ligne[i] == ' || ligne[i] == '\t') {i++;}
if ((ligne[i] &gt; '9' || ligne[i] &lt; '0') &amp;&amp; ligne[i] != 'x')
{
printf("Erreur à la ligne %d !\n",nbLigne);
supprimerGraphe(g);
return -1; /* pas des chiffres ! */
}
if (ligne[i] == 'x') creerArc=0;
temp=10*temp+ligne[i] - '0';
i++;
while (ligne[i] == ' || ligne[i] == '\t') {i++;}
}
if (ligne[i] == ',') i++;
nbElt++;
if (nbElt&lt;=nbS1 &amp;&amp; creerArc==1) ajouterArc(g,sommet,nbElt,temp); /* ligne pas trop longue */
}
if (nbElt != nbS1) /* pas le bon nombre de champs sur ligne */
{
printf("Erreur à la ligne %d !\n",nbLigne);
supprimerGraphe(g);
return -1; /* pas le bon nombre de champs */
}
}
fclose(fp);
return 0;
}

Bibliographie

[1] J.-P. Allouche, J. Shallit, Automatic sequences, Theory, Applications, Generalizations, Cambridge Univ. Press, Cambridge (2004).

[2] C. Berge, Graphes et hypergraphes, Dunod, Paris, (1970).

[3] N. Biggs, Algebraic Graph Theory, 2nd Edition, Cambridge Math. Library, (1993).

[4] B. Bollobás, Graph Theory, An Introductory Course, Graduate Text in Math. 63, Springer, (1979).

[5] J.A. Bondy, U.S.R. Murty, Graph Theory with Applications, North-Holland, New-York, (1976).

[6] V. Bouchitté, Graphes et algorithmique des graphes, notes de cours, École normale supérieure de Lyon.

[7] C.P. Bruter, Les matroïdes, nouvel outil mathématique, Initiation aux nouveautés de la science 21, Dunod, Paris, (1970).

[8] R. Diestel, Graph Theory, 3rd Edition, Graduate Text in Math. 173, Springer, (2005).

[9] F.R. Gantmacher, The theory of matrices, Chelsea, (1960).

[10] A. Gibbons, Algorithmic Graph Theory, Cambridge University Press, Cambridge University Press, (1994).

[11] C. Godsil, G. Royle, Algebraic Graph Theory, Graduate Text in Math. 207, Springer, (2001).

[12] T. Harju, Lecture Notes on Graph Theory, University of Turku, Spring 2002.

[13] B.W. Kernighan, D.M. Ritchie, The C Programming Language, 2nd Ed., Prentice Hall, (1988).

[14] D. Lind, B. Marcus, Symbolic Dynamics and Coding, An Introduction, Cambridge University Press, (1995).

[15] P. Lopez, Cours de GRAPHES, notes de cours (2005).
www.laas.fr~lopez/cours/GRAPHES/graphes.html.

[16] K. Loudon, Maîtrise des algorithmes en C, Ed. française, O'Reilly, Paris, (2000)

[17] C. Meyer, A. Langville, Updating the PageRank Vector, SIAM Annual Meeting, New Orleans, 2005.

[18] C. Meyer, A. Langville, Google's PageRank and Beyond: The Science of Search Engine Rankings, Princeton University Press, (2006).

[19] Ministère de l'éducation nationale, Introduction d'éléments de la théorie des graphes, accompagnent de la mise en oeuvre des programmes, (2001).

[20] O. Ore, Graphs and their uses, version révisée par R. J. Wilson, New Mathematical Library 34, Math. Association of America.

[21] J. Oxley, On the interplay between graphs and matroids, Surveys in combinatorics, 2001 (Sussex), 199–239, London Math. Soc. Lecture Note Ser. 288, Cambridge Univ. Press, (2001).

[22] L. Page, S. Brin, R. Motwani, T. Winograd, The PageRank Citation Ranking: Bringing Order to the Web, Technical Report, Stanford University, 1998.

[23] J. Miles Prystowsky, L. Gill, Calculating Web Page Authority Using the PageRank Algorithm, http://online.redwoods.cc.ca.us/instruct/darnold/LAPROJ/.

[24] S. P. Radziszowski, Small Ramsey Number, Dynamic surveys, The Electronic Journal of Combinatorics, http://www.combinatorics.org/Surveys/ds1.pdf

161

Chapitre V. Bibliographie

- [25] V. Rosta, Ramsey Theory Applications, Dynamic surveys, The Electronic Journal of Combinatorics, www.combinatorics.org/Surveys/ds13.pdf
- [26] E. Seneta, Non-Negative Matrices, An Introduction to Theory and Applications, George Allen and Unwin Ltd, London, (1973).
- [27] W.T. Tutte, Graph Theory, Cambridge Mathematical Library, Encyclopedia of mathematics and its applications 21, (1984).
- [28] D.J.A. Welsh, Matroid Theory, Academic Press, London, (1976).
- [29] R. Wilson, Four Colours Suffice, or how to colour a map, European Math. Society Newsletter 46 December 2002, 15–19.

# Liste des figures

I.1 Un arc reliant deux sommets, une boucle. 6
I.2 Un exemple de graphe. 6
I.3 Un exemple de multi-graphe. 7
I.4 Un exemple de graphe simple. 8
I.5 Un graphe (dirigé) simple, un graphe et un multi-graphe. 9
I.6 Un graphe (non dirigé) simple, un graphe et un multi-graphe. 9
I.7 Des graphes non orientés 3 et 4-reguliers. 9
I.8 Un multi-graphe orienté 2-regulier. 10
I.9 Un graphe biparti (non complet). 10
I.10Graphe étiqueté par les distances entre villes (itinéraire “express” calculé par www.mappy.fr). 11
I.11Un exemple d'hyper-graphe. 11
I.12Les sept ponts de Königsberg. 13
I.13Deux représentations d'un cube. 13
I.14La carte des USA. 14
I.15Graphe d'incompatibilité. 14
I.16Graphe de Cayley de  $S_{3}$  15
I.17Un autre graphe de Cayley de  $S_{3}$  16
I.18Sous-graphe couvrant. 16
I.19Orientation d'un graphe. 17
I.20Un graphe planaire. 18
I.21Un graphe non planaire. 18
$\mathrm{I.22}K_{3,3}$  sur un tore. 18
I.23Graphe d'intervalles. 19
I.24Problème d'affectation. 19
I.25Une application du tri topologique. 20
I.26Le graphe de De Bruijn d'ordre 3 de ababab... 20
I.27Un problème de flot maximum. 21
I.28Chemin critique dans la planification de tâches. 22
I.29Une piste (ou chemin élémentaire) et un chemin simple 23

Liste des figures

I.30Un circuit, un circuit élémentaire (ou piste fermée) et un circuit simple. 23
I.31Un graphe orienté. 25
I.32Un digraphesimplepondéré. 26
I.33Illustration de l'algorithmme de Dijkstra. 28
I.34Illustration de l'algorithmme de Dijkstra. 28
I.35Construction d'un circuit eulérien. 30
I.36Une maison à tracer d'un seul trait. 31
I.37Un graphe simple (connexe). 33
I.38digraphe et graphe des composantes. 35
I.39Un graphe et deux sous-graphes. 37
I.40Des sommets indépendants. 38
I.41Un graphe et ses points d'articulation. 39
I.42Un graphe au moins 2-connexe. 39
I.43Un graphe 2-connexe. 39
I.44Un graphe et ses composantes 2-connexes. 40
I.45Un graphe et ses arêtes de coupure. 40
I.46Illustration de la proposition I.6.6 41
I.47Mise à sens unique 42
I.48Un graphe et une coupure. 42
I.49Un multi-graphe tel que  $\lambda (G) = 2$  et  $\kappa (G) = 1$  42
I.50Un multi-graphe tel que  $\lambda (G) = k$  et  $\kappa (G) = 1$  43
I.51Un graphe simple tel que  $\lambda (G) = 4$  et  $\kappa (G) = 1$  43
I.52Une clique de taille 4,  $\omega (G) = 4$  44
I.53Une illustration du théorème de Menger : 3 chemins indépendants joignent  $u$  et  $v$ , 3 sommets séparant  $u$  et  $v$ . 45
I.54Une illustration de la preuve corollaire de Menger. 46
I.55Un arbre. 50
I.56Un arbre pointé. 51
I.57Un arbre à parcourir. 52
I.58Exemple de parcours en profondeur d'un graphe. 53
I.59Homomorphisme de  $G$  dans  $H$  53
I.60Homomorphisme de  $G$  dans  $H$  53
I.61Deux graphes isomorphes. 54
I.62Un arbre lexicographique infini. 55
I.63Un arbre infini pondéré représentant le dictionnaire de  $A_{L}$  56
I.64Un arbre infini pondéré représentant le dictionnaire de  $A_{M}$  56

Liste des figures

I.65Squelette d'un dodécaèdre et circuit hamiltonien. 57
I.66Déplacement d'un cavalier. 58
I.67Une illustration de la preuve de la proposition I.11.3. 58
I.68Un graphe non hamiltonien. 59
I.69Le graphe de Petersen. 59
I.70Théorème d'Ore, circuit dans  $G$  62
I.71Construction de la fermeture d'un graphe. 63
I.72Deux circuits hamiltoniens disjoints de  $K_{5}$  66
I.73Deux autres circuits hamiltoniens disjoints de  $K_{5}$  66
I.74Un tentative de partition de  $K_{7}$  67
I.75Chemins hamiltoniens disjoints de  $K_{6}$  et  $K_{8}$  68
II.1 Un graphe  $G$  et sa matrice d'adjacence. 70
II.2 Deux graphes cospectraux. 71
II.3 Disposition des valeurs propres d'une matrice irréductible. 75
II.4 Une illustration des graphes  $G$  et  $G^{\prime}$  75
II.5 Disposition des valeurs propres d'une matrice primitive. 77
II.6 Un graphe avec une matrice d'adjacence primitive. 78
II.7 Un graphe avec une matrice d'adjacence irreductible. 78
II.8 Trois graphes f. connexes. 83
II.9 Deux composantes f. connexes. 86
II.10 Trois composantes f. connexes. 86
II.1 Recherche d'un sous-arbre couvrant. 96
II.12Contraction d'un graphe  $G$  en  $G\cdot e$  97
II.13 Applications successives de la formule de Cayley. 97
II.1Nombre de sous-arbres couvrants. 98
II.15Un arbre encodé par (1,4,3,1,6,6). 99
II.16Un arbre encodé par  $(1,1,1)$  99
II.17Deux arbres de  $C_i$  pour lesquels  $d_{i} = 2$  et  $d_{i} = 3$  100
II.18Un sous-arbre couvrant pointe et oriente. 103
II.19Graphes correspondant aux matrices  $D_1^{(3)},D_2^{(3)},D_3^{(3)}$  et  $D_4^{(3)}$  105
II.20Un arbre pointé parcouru en largeur (avec un arc supplémentaire). 106
II.2Un cycle et des colonnes dont la somme est nulle. 106
II.2Nombre de sous-arbres couvrants pour  $G$  et  $G^{\prime}$  107
II.23Les sous-arbres couvrants du graphe  $G$  108
II.24Application de l'algorithmme de Prim. 109
III.IIhéorème de Jordan 111

Liste des figures

III.2 Un graphe planaire et ses faces. 112
III. Rotation sur la sphère 113
III.Choix de la face infinie. 113
III. Suppression d'arêtes. 114
III.6 Subdivision d'une arête. 115
III. Graphes homéomorphes. 115
III.8 Graphes homéomorphes. 116
III.9 Si  $G$  n'etait pas 2-connexe. 117
III.  $\mathcal{M}_1$ ,  $H_2$  et l'arête  $\{u, v\}$ . 118
III. 1lemme III.4.5 119
III.  $\mathbb{S}$  on dispose d'au plus une arête. 119
III.  $\mathcal{G}$  contient un sous-graphe homéomorphe à  $K_{3,3}$ . 120
III. Illustration du lemme III.4.7. 120
III. Illustration du lemme III.4.7, cas (a). 121
III. Illustration du lemme III.4.7, cas (b) et (c). 121
IV. Une illustration du lemme IV.1.3. 123
IV.2Si  $H$  n'est pas simple. 124
IV.3Projection stéréographique. 125
IV.4Un graphe et son dual. 125
IV.Francis Guthrie. 126
IV.6 Suppression des sommets de degré 2. 126
IV. Suppression des sommets de degré  $\geq 4$ . 127
IV.8 Reduction d'une face délimitée par deux arêtes. 128
IV.9 Reduction d'une face délimitée par deux arêtes. 129
IV.10 Configurations pour la réduction d'une face rectangulaire. 129
IV.12 Reductions d'une face rectangulaire. 129
IV. Satisfaction de R.2. 130
IV. Reductions d'une face pentagonale. 130
IV.  $\mathcal{G}$  inq couleurs effectives. 131
IV.  $\mathbb{S}$  ne surface de genre 2. 131
IV. Illustration de  $m_{k,G}$ . 133
IV.  $\mathcal{R}(3,3) &gt; 3,4,5$  136
IV.  $\mathbb{S}$  n graphe et un coloriage de  $K_{n}$ . 138
V.1 Un pointeur et une variable. 142
V.2 L'échange de deux variables. 144
V.3 Création d'une liste chañée. 147

Liste des figures
167

V.4 Implémentation d'un graphe. 149
V.5 Le type eltadj. 149
V.6 Le type sommet. 149
V.7 Le type graphe. 150

.

# Index

## Notations

$(v_{i},v_{j})$ (arc) ... 5
$A(G)$ (matrice d'adjacence) ... 69, 72
$Aut(G)$ (groupe des automorphismes) ... 54
$D(G)$ (matrice de demi-degré entrant) ... 102
$G - e$ (sous-graphe) ... 37
$G - v$ (sous-graphe) ... 37
$K_{n}$ (graphe complet) ... 9
$K_{m,n}$ (graphe biparti complet) ... 10
$\alpha(G)$ (sommets indépendants) ... 37
$\chi(G)$ (nombre chromatique) ... 123
$\kappa(H)$ (taille min. ens. articulation) ... 39
$\lambda(H)$ (taille min. coupe) ... 42
$\leftrightarrow$ (sommets f. connectés) ... 24
$\mathcal{A}_G$ (algèbre des polynômes de $A(G)$) ... 92
$\mathcal{C}_S(G)$ (graphe de Cayley) ... 15
$\mathcal{F}(G_0)$ (fermeture) ... 62
$\nu(v)$ (ens. des voisins) ... 8
$\omega(G)$ (clique maximale) ... 43
$\omega(v)$ (arcs adjacents) ... 6
$\omega^{+}(v)$ (arcs sortants) ... 6
$\omega^{-}(v)$ (arcs entrants) ... 6
$\sim$ (sommets connectés) ... 23
$\tau(G)$ (nombre de sous-arbres couvrants) ... 96
$\{v_i,v_j\}$ (arête) ... 8
$d^{+}(v)$ (demi-degré sortant) ... 6
$d^{-}(v)$ (demi-degré entrant) ... 6
$p$-graphe ... 7
$\operatorname{pred}(v)$ (prédécesseur) ... 6
$\operatorname{succ}(v)$ (successeur) ... 6
$\operatorname{diam}(G)$ (diamètre) ... 24

$\mathrm{d}(a,b)$ (distance) ... 24
$\operatorname{pred}^*(v)$ ... 34
$\operatorname{succ}^*(v)$ ... 34

## A

adjacent
arc ... 6
sommet ... 8
algèbre d'adjacence ... 92

algorithme
Dijkstra ... 26
Fleury ... 31
Kruskal ... 110
Prim ... 108
sous-arbre couvrant ... 95
test connexité ... 32
test cycle ... 47
test cycle (2) ... 48

ancêtre ... 51
Appel K. ... 131
arête ... 5
adjacente ... 8
incidente ... 8

arête
coupure ... 40
subdivision ... 115
arbre ... 49
$n$-aire ... 49, 51
ancêtre ... 51
complet ... 51
descendant ... 51
feuille ... 51
fils ... 51
lexicographique ... 55
orienté ... 102
père ... 51
parcours

169

170
Index

en profondeur ... 51
en largeur ... 52
infixe ... 52
préfixe ... 51
suffixe ... 51
pointé ... 50
orienté ... 102
régulier ... 56
racine ... 50
arc ... 5
adjacent ... 6
destination ... 5
entrant ... 6
extrémité ... 5
incident ... 6
origine ... 5
sortant ... 6
articulation
ensemble ... 38
point ... 38

B
Bott-Mayberry ... 107
boucle ... 5

C
Cayley ... 15, 98
chemin ... 22, 24
élémentaire ... 22
eulérien ... 29
fermé ... 22
hamiltonien ... 57
ouvert ... 22
simple ... 22
Chvátal (théorème) ... 64
circuit ... 22
eulérien ... 29
hamiltonien ... 57
Clarke ... 102
clique ... 43
collapse ... 35
coloriage ... 123
propre ... 123
component graph ... 35
composante
connexe ... 23
f. connexe ... 24
fortement connexe ... 24

s. connexe ... 24
simplement connexe ... 24
condensé ... 35
contraction ... 96
coupe ... 41
coupure ... 41
coupure (arête) ... 40
cycle ... 22

D
De Bruijn (graphe) ... 20
degré ... 6, 8
demi-degré
entrant ... 6
sortant ... 6
descendant ... 51
destination ... 5
diamètre ... 24
digraphe ... 8
Dijkstra (algorithme) ... 26
Dirac (théorème) ... 60
distance ... 24

E
edge ... 5
ensemble
articulation ... 38
Erdős ... 66, 137
etiquette ... 10
Euler
formule ... 113
extrémité ... 5

F
face ... 111
adjacente ... 112
infinie ... 112
fermeture ... 62
feuille ... 51
fils ... 51
Fleury
algorithme ... 31
forêt ... 49
formule
Euler ... 113
frontière ... 112

G

Index

graphe...5
2-connexe...38
k-connexe...39
k-connexe (pour les arêtes)...42
étiqueté...10
asymétrique...54
biparti...10
biparti complet...10
Cayley...15
comple...9
connexe...23
cospectraux...71
De Bruijn...20
des composantes...35
diamètre...24
dirigé...5,8
eulérien...29
f. connexe...24
fermeture...62
fini...5
fortement connexe...24
hamiltonien...57
isomorphe...54
non dirigé...8
non orienté...8
orienté...5
parcours en profondeur...52
Petersen...59
planaire...111
planaire topologique...111
pondéré...10
primitif...74
régulier...9
s. connexe...24
simple...7
simplement connexe...24
sous-graphe...37
couvrant...38
induit...37
propre...37
strict...7
graphe d'intervalles...19
Guthrie F...125

H
Haken W...131
handshaking lemma...8
Heawood (formule)...131

Hoffman (théorème)...94
homéomorphisme...115
homomorphisme...53
hyper-arête...11
hyper-graphe...11
fini...11

I
incident
arête...8
arc...6
isomorphisme...54

K
Kruskal (algorithme)...110

L
label...10
liste
adjacence...148

M
matrice
acyclique...80
adjacence...69,72
cyclique...80
demi-degré entrant...102
incidence...70
irréductible...73
primitive...73
matroïde...11
normal...12
Mayberry, Bott...107
Menger
théorème...44
multi-ensemble...7
multi-graphe...7
fini...7

N
noeud...5
nombre chromatique...123

O
Ore (théorème)...62
origine...5

P
période...79

172
Index

père ... 51
parcours
en largeur ... 52
en profondeur ... 51, 52
infixe ... 52
préfixe ... 51
suffixe ... 51
passerelle ... 29
Perron
théorème ... 77
valeur propre ... 74
Perron-Frobenius (théorème) ... 74
Petersen (graphe) ... 59
piste ... 22
poids ... 10
point d'articulation ... 38
polynôme chromatique ... 132
Prüfer (codage) ... 98
prédécesseur ... 6
Prim (algorithme) ... 108

R
racine ... 50
Robbins (théorème de) ... 42
rooted tree ... 50
routage ... 28
routeur ... 29

S
sommet ... 5
adjacent ... 6, 8
connecté ... 23
distance ... 24
fortement connecté ... 24
indépendant ... 37
voisin ... 6, 8
sous-graphe ... 37
clique ... 43
couvrant ... 38
propre ... 37
sous-graphe induit ... 37
SPF (Shortest Path First) ... 29
squelette ... 13, 112
Steinitz (théorème) ... 112
subdivision ... 115
successeur ... 6
Szekeres ... 137

théorème
Chvátal ... 64
Chvátal-Erdös ... 66
Dirac ... 60
exclusion ... 116
Hoffman ... 94
Menger ... 44
Ore ... 62
quatre couleurs ... 131
Robbins ... 42
tri topologique ... 48
triangle ... 70
TSP ... 12

V
vertex ... 5
voyageur de commerce ... 12