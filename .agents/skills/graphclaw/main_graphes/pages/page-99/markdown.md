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