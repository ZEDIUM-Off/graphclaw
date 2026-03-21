Chapitre V. Annexe : implémentation en C

```c
{
if (padj-&gt;dest &gt; b)
{
padj = (ELTADJ *) malloc(sizeof(ELTADJ));
if (padj == NULL)
{
printf("Erreur! Mémoire insuffisante pour créer un sommet\n");
return -3;
}
else
{
padj-&gt;suivant = psommet-&gt;adj;
psommet-&gt;adj = padj;
}
}
else
{
while (padj != NULL)
{
if (padj-&gt;dest == b) {padj-&gt;info = info; break;} /* l'arc existe, update info */
if (padj-&gt;dest &gt; b) {padj = NULL; break;} /* on dépasse b sans le trouver */
precedent = padj;
padj = padj-&gt;suivant;
}
if (padj == NULL) /* l'arc n'existe pas, il faut le créer */
{
padj = (ELTADJ *) malloc(sizeof(ELTADJ));
if (padj == NULL)
{
printf("Erreur! Mémoire insuffisante pour créer un sommet\n");
return -3;
}
else
if (precedent-&gt;suivant == NULL) /* élément ajouter à la fin */
{
precedent-&gt;suivant = padj;
padj-&gt;suivant = NULL;
}
else /* élément ajouter "au milieu" pour garder ordre */
{
padj-&gt;suivant = precedent-&gt;suivant;
precedent-&gt;suivant = padj;
}
}
}
}
padj-&gt;dest = b;
padj-&gt;info = info;
g-&gt;nbA++;
}
return 0;
}
```

- La fonction `supprimerSommet` teste si le sommet de label a existe. Si tel est le cas, ce sommet est supprimé de la liste chaînée constituée des sommets du graphe (ce qui nécessite la mise à jour de pointeurs de cette liste). On supprime aussi la liste chaînée des sommets adjacents à a et les occurrences du sommet a dans les listes d'adjacence des autres sommets (avec les mises à jour ad hoc des listes chaînées).

```c
int supprimerSommet(GRAPHE *g, int a)
{
SOMMET *psommet, *precedent;
ELTADJ *padj, *suivant, *precedent_adj;
int flag_premier_sommet, flag_premier_arc;
if (g-&gt;premierSommet == NULL)