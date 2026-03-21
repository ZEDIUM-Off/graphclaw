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