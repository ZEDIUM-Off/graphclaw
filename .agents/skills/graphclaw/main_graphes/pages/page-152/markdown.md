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