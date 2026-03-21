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