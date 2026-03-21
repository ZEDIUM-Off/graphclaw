141

# CHAPITRE V

## Annexe : implémentation en C

### 1. Pointeurs, allocation mémoire et listes chaînées

Dans cette courte section, nous rappelons quelques fondements essentiels du langage C dans le but d'appréhender au mieux les structures données permettant de coder les graphes. Nos rappels concernent principalement la gestion et l'allocation dynamique de la mémoire. Nous supposons connus¹ les boucles, les fonctions ou encore la gestion des entrées/sorties.

#### 1.1. Adresses. Toute variable possède une adresse mémoire. Cette adresse est accessible grâce à l'opérateur &amp;. Autrement dit, si x est l'identificateur d'une variable, alors &amp;x représente l'adresse de x.

**Exemple V.1.1.** Après compilation et exécution du court programme suivant,

```c
#include <stdio.h>
main()
{
int i;
printf("&gt; %u \n",&amp;i);
}
```

vous devriez obtenir un affichage ressemblant à

&gt; 3221219076

Ce programme affiche simplement (sous forme décimale) l'adresse mémoire² de la variable i.

On peut accéder à une variable non seulement grâce à son identificateur (si elle en possède un), mais aussi grâce à son adresse. Cela est rendu possible au moyen de l'opérateur *. L'adresse d'une variable est une donnée qui peut être stockée dans une variable. Une telle variable est appelée *pointeur*.

**Exemple V.1.2.** Dans cet exemple, on définit deux variables : un entier i et un pointeur vers un entier. Cette dernière variable va être utilisée pour stocker l'adresse de la première variable. Connaissant l'adresse de i, on peut modifier le contenu de i en se référant au pointeur vers ce dernier.

---

¹Consulter si nécessaire un ouvrage de référence sur le C comme [13].

²La gestion de la mémoire est laissée au système d'exploitation. Vous pouvez dès lors obtenir des résultats différents suivant la machine employée et même d'une exécution à l'autre sur une machine fixée.</stdio.h>