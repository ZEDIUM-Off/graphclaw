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