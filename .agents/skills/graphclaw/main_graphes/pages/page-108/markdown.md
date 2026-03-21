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