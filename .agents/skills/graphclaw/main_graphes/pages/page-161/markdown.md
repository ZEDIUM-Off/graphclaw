V.2. Liste d'adjacence

```c
{
printf("Erreur! Graphe vide, suppression impossible\n");
return -1;
}
else
{
psommet = g-&gt;premierSommet;
flag_premier_sommet = 1;
while (psommet != NULL)
{
if (psommet-&gt;label == a) break;
else
{
flag_premier_sommet = 0;
precedent = psommet;
psommet = psommet-&gt;suivant;
}
}
if (psommet == NULL)
{
printf("Erreur! Le sommet a supprimer n'existe pas\n");
return -1;
}
else
{
if (psommet-&gt;suivant == NULL) g-&gt;dernierSommet = precedent;
if (flag_premier_sommet == 1) g-&gt;premierSommet = psommet-&gt;suivant;
else precedent-&gt;suivant = psommet-&gt;suivant;
padj = psommet-&gt;adj;
free(psommet);
g-&gt;nbS--;
while (padj != NULL)
{
suivant = padj-&gt;suivant;
free(padj);
g-&gt;nbA--;
padj = suivant;
}
}
}
```

/* il faut aussi supprimer les arcs ayant le sommet a supprimer comme extremite */
psommet = g-&gt;premierSommet;
while (psommet != NULL)
{
padj = psommet-&gt;adj;
while (padj != NULL)
{
flag_premier_arc = 1;
if (padj-&gt;dest == a) break;
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
g-&gt;nbA--;
}
psommet = psommet-&gt;suivant;
}
return 0;
}