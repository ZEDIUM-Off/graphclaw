V.1. Pointeurs, allocation mémoire et listes chainées

```c
printf("&gt; a=%d, b=%d \n",a,b);
swap(a,b);
printf("&gt; a=%d, b=%d \n",a,b);
}
void swap(int x, int y)
{
int temp;
temp=x;
x=y;
y=temp;
}
```

Lorsqu'on compile et exécute ce programme, on obtient l'affichage suivant et on n'a donc pas le résultat escompté.

```txt
&gt; a=1, b=2
&gt; a=1, b=2
```

La raison en est toute simple: lorsqu'une variable est passée à une fonction, C effectue une copie de cette variable et la fonction ne travaille donc pas sur les variables transmises mais sur une copie de celles-ci³. Lorsque la fonction s'achève, les copies sont détruites. On peut régler ce problème en transmettant l'adresse des variables à considérer. En plus de résoudre notre problème, cela permet de faire des économies de mémoire qui peuvent s'avérer importantes lorsqu'on transmet des variables de grande taille (dans ce cas, on transmet uniquement une adresse et ce n'est pas toute la variable qui est dupliquée). Voici à présent le programme corrigé. Le lecteur remarque la présence des opérateurs &amp; et *.

```c
/* Exemple correct --- */
#include <stdio.h>

void swap(int *x, int *y);
main()
{
int a=1,b=2;
printf("&gt; a=%d, b=%d \n",a,b);
swap(&amp;a,&amp;b);
printf("&gt; a=%d, b=%d \n",a,b);
}

void swap(int *x, int *y)
{
int temp;
temp=*x;
*x=*y;
*y=temp;
}
```

Cette fois, le programme a bien le comportement souhaité !

³On parle parfois de transmission par valeurs.</stdio.h>