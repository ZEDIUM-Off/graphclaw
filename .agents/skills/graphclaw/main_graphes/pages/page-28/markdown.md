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