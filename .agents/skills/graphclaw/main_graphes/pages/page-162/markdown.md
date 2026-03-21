Chapitre V. Annexe : implémentation en C

- La fonction supprimerArc supprime, si possible, de la liste d'adjacence du sommet de label a l'arc joignant les sommets de label a et b. Une erreur est renvoie sinon.

```c
int supprimerArc(GRAPHE *g, int a, int b)
{
SOMMET *psommet;
ELTADJ *padj, *precedent_adj;
int flag_premier_arc;
if (g-&gt;premierSommet == NULL)
{
printf("Erreur! Graphe vide, suppression impossible\n");
return -1;
}
else
{
psommet = g-&gt;premierSommet;
while (psommet != NULL)
{
if (psommet-&gt;label == a) break;
else psommet = psommet-&gt;suivant;
}
if (psommet == NULL)
{
printf("Erreur! L'extrémité de l'arc a supprimer n'existe pas !!!!\n");
return -1;
}
else
{
padj = psommet-&gt;adj;
flag_premier_arc = 1;
while (padj != NULL)
{
if (padj-&gt;dest == b) break;
else
{
flag_premier_arc = 0;
precedent_adj = padj;
padj = padj-&gt;suivant;
}
}
if (padj != NULL)
{
if (flag_premier_arc == 1) psommet-&gt;adj = padj-&gt;suivant;
else precedent_adj-&gt;suivant = padj-&gt;suivant;
free(padj);
g-&gt;nhA--;
}
else
{
printf("Erreur! L'extrémité de l'arc a supprimer n'existe pas !!!!\n");
return -1;
}
}
return 0;
}
}
```

- La fonction supprimerGraphe libre l'ensemble de la mémoire utilisée par un graphe et rend simplement un graphe "vide".

```c
void supprimerGraphe(GRAPHE *g)
{
SOMMET *psommet, *temps;
ELTADJ *padj, *tempadj;
psommet = g-&gt;premierSommet;
while (psommet != NULL)
{
padj = psommet-&gt;adj;