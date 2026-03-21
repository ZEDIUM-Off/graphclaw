V.2. Liste d'adjacence

```c
while (padj != NULL)
{
tempadj = padj;
padj = padj-&gt;suivant;
free(tempadj);
}
temps = psommet;
psommet = psommet-&gt;suivant;
free(temp);
}
initialiserGraphe(g);
```

- La fonction `afficherGraphe` affiche tout d'abord plusieurs informations de base sur le graphe d'adresse `*g` (nombre de sommets, nombre d'arêtes, ...). Ensuite la liste chaînée des sommets est parcourue et pour chacun de ces sommets, on parcourt l'entièreté de la liste d'adjacence correspondante.

```c
void afficherGraphe(GRAPHE *g)
{
SOMMET *psommet;
ELTADJ *padj;
if (g-&gt;premierSommet == NULL) printf("graphe vide\n");
else
{
printf("nbS=%d, nbA=%d, label max.=%d\n", g-&gt;nbS, g-&gt;nbA, g-&gt;maxS);
psommet = g-&gt;premierSommet;
do
{
printf("\n");
printf("Sommet de label: %d, info: %d\n", psommet-&gt;label, psommet-&gt;info);
if (psommet-&gt;adj == NULL) printf(" -&gt; ce sommet n'a aucun arc sortant\n");
else
{
padj = psommet-&gt;adj;
do
{
printf(" -&gt; arc de %d vers %d avec l'info. %d\n", psommet-&gt;label, padj-&gt;dest, padj-&gt;info);
padj = padj-&gt;suivant;
}
while (padj != NULL);
}
printf("\n");
psommet = psommet-&gt;suivant;
}
while (psommet != NULL);
}
}
```

- La fonction `lireFichier` ouvre en lecture un fichier texte. Ce fichier, s'il satisfait les conventions énoncées précédemment (cf. exemple V.2.1), est utilisé pour construire le graphe correspondant.

```c
int lireFichier(char *nomf, GRAPHE *g)
{
FILE *fp;
char ligne[MAX+1];
int temp, i, j, nbS1, nbLigne, sommet, nbElt, creerArc;

initialiserGraphe(g);
fp = fopen(nomf, "r"); /* ouvre un fichier en lecture */
nbLigne = 0; /* compte les lignes du fichier */
sommet = 0; /* label du sommet en cours */
nbS1 = 0; /* compte les sommets de la 1ère ligne */
while (fgets(ligne, MAX, fp) != NULL)