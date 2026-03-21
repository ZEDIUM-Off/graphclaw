Chapitre V. Annexe : implémentation en C

```txt
x,1,x,4
x,2,3,x
x,x,x,6
5,x,x,x
```

Remarque V.2.2. L'utilisation d'un tableau peut s'avérer commode pour l'encodage d'un graphe dans un fichier. En effet, après avoir placé les divers éléments dans les cases d'un tableau, le tableau propose une option du type "Save as" dans laquelle on peut spécifier que le type du fichier en sortie sera un (simple) texte. Le tableau demande alors à l'utilisateur de préciser un délimiteur pour les champs (il convient deCHOISIR la virgule) et un délimiteur pour le texte (il convient de ne prendre aucun délimiteur).

# 2.2. Utilisation des fonctions.

Example V.2.3. Voici deux variantes de programmes permettant d'encoder le graphe de la figure V.4.

```c
/* FICHIER main.c --- */
#include <stdio.h>
#include "graphes.h"
int main()
{
GRAPHE g;
int i;
initialiserGraphe(&amp;g);
for (i=1; i&lt;=4; i++) ajouterSommet(&amp;g,0);
ajouterArc(&amp;g,1,2,1);
ajouterArc(&amp;g,1,4,4);
ajouterArc(&amp;g,2,2,2);
ajouterArc(&amp;g,2,3,3);
ajouterArc(&amp;g,3,4,6);
ajouterArc(&amp;g,4,1,5);
afficherGraphe(&amp;g);
supprimerGraphe(&amp;g);
}
```

Dans ce second fichier, on utilise un fichier de données externes comme celui donné dans l'exemple V.2.1.

```c
/* FICHIER main.c --- */
#include <stdio.h>
#include "graphes.h"
int main()
{
GRAPHE g;
int i;
lireFichier("data.gr",&amp;g);
afficherGraphe(&amp;g);
supprimerGraphe(&amp;g);</stdio.h></stdio.h>