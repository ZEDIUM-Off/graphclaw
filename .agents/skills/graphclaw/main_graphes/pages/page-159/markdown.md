V.2. Liste d'adjacence

```c
}
g-&gt;nbS++;
return pointeur-&gt;label;
}
```

- La fonction `ajouterArc` détermine tout d'abord si le graphe contient bien les sommets de label donnés par `a` et par `b`. Si l'un d'eux n'existe pas, la fonction renvoie un code d'erreur égal respectivement à `-1` ou `-2`. Sinon, deux situations peuvent se présenter. L'arc existe, le champ `info` est simplement mis à jour. L'arc n'existe pas. Il est créé au sein de la liste chaînée des sommets adjacents au sommet de label `a`. Il est positionné dans cette liste de telle sorte que les labels des sommets présents sont ordonnés par ordre croissant.

```c
int ajouterArc(GBAPHE *g, int a, int b, int info)
{
SOMMET *psommet, *psommet2;
ELTADJ *padj, *precedent;
psommet=g-&gt;premierSommet;

/* on parcourt les sommets jusqu'à trouver a */
while (psommet != NULL)
{
if (psommet-&gt;label == a) break;
psommet=psommet-&gt;suivant;
}
if (psommet == NULL)
{
printf("Erreur! Creation d'un arc dont l'origine n'existe pas\n");
return -1;
}
else /* on a trouver a */
{
padj=psommet-&gt;adj;

/* on parcourt les sommets pour trouver b */
psommet2=g-&gt;premierSommet;
while (psommet2 != NULL)
{
if (psommet2-&gt;label == b) break;
psommet2=psommet2-&gt;suivant;
}
if (psommet2 == NULL)
{
printf("Erreur! Creation d'un arc dont l'extremite n'existe pas\n");
return -2;
}
else /* on a trouver a et b */
{
if (padj == NULL) /* la liste d'adjacence est vide */
{
padj=(ELTADJ *) malloc(sizeof(ELTADJ));
if (padj == NULL)
{
printf("Erreur! Memoire insuffisante pour créer un sommet\n");
return -3;
}
else
{
psommet-&gt;adj=padj; /* premier élément de la liste d'adjacence */
padj-&gt;suivant=NULL;
}
}
else /* la liste d'adjacence est non vide, on la parcourt pour voir si b s'y trouve */
}