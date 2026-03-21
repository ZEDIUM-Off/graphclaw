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