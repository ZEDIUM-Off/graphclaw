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