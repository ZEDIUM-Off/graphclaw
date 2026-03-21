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