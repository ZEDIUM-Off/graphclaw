Chapitre V. Annexe : implémentation en C

```c
{ PAIRE \*a; (\*a).x=3; a-&gt;y=5; }
```

Ici, a est un pointeur vers une PAIRE. Ainsi, il est normal d'accéder au premier champ de la paire avec (*a).x. Enfin, nous en profitons pour rappeler que a-&gt;y est un synonyme de (*a).y.

1.4. Listes chaînées. Nous allonsprésent cette construction à l'aide d'un exemple. Imaginons pouvoir stocker les éléments d'une liste de taille  $n$  d'entiers. Si  $n$  varie, il n'est pas possible de prévoir à l'avance la taille d'un tableau devant recevoir les  $n$  entiers. Ainsi, un tel tableau doit être créé dynamiquement, i.e., au moment où le programme est en cours d'exécution. Nous définissons une structure.

```c
struct elt {
int x;
struct elt *suivant;
};
```

Celle-ci est constituée d'un premier champ entier  $\mathbf{x}$  et d'un second champ suivant qui est un pointeur vers une structure du même type. Pour ne pas alourdir l'exposé, notre exemple ne va pas inclure d'interaction avec un utilisateur devant introduire une liste de nombres. Nous construisons simplement une liste chaine contenant 6 éléments à l'aide de deux pointeurs.

Remarque V.1.8. Nous en profitons également pour vous rappeler que le langage C est "case sensitive", i.e., la distinction est faite entre minuscules et majuscules. Ainsi, "elt" diffère de "ELT" (ou de "Elt").

```c
#include <stdio.h>
struct elt{
int x;
struct elt *suivant;
};
typedef struct elt ELT;

main()
{
int i;
ELT *p, *q;

/* construction de la liste chainee --- */
q=(ELT *) malloc(sizeof(ELT));
q-&gt;x=0;
q-&gt;suivant=NULL;

for (i=1; i&lt;=5; i++)</stdio.h>