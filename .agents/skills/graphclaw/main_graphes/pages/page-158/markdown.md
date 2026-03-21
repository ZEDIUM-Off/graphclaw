Chapitre V. Annexe : implémentation en C

```c
int ajouterArc(GRAPHE *, int a, int b, int info);
int supprimerSommet(GRAPHE *, int label);
int supprimerArc(GRAPHE *, int a, int b);
void supprimerGraphe(GRAPHE *);
void afficherGraphe(GRAPHE *);
int lireFichier(char *nomf, GRAPHE *);
```

#endif

Nous allons à présent détailler l'implémentation des fonctions du module graphes.c permettant la gestion de la structure de données de type graphe.

- La fonction `initialiserGraphe` est très simple, il s'agit uniquement d'affectations élémentaires.

```c
void initialiserGraphe(GRAPHE *g)
{
g-&gt;nbS=0;
g-&gt;nbA=0;
g-&gt;maxS=0;
g-&gt;premierSommet=NULL;
g-&gt;dernierSommet=NULL;
}
```

- La fonction `ajouterSommet` ajoute un élément de type `sommet` comme `dernier élément` de la liste châineé contenant les sommets du graphe d'adresse *g. Le champ `maxS` de g (contenant le plus grand label attribué en cours d'exécution à un sommet du graphe) est incrémenté d'une unité et cette valeur est aussi le label du sommet à ajouter. Il est de plus nécessaire de mettre à jour les champs `dernierSommet`, `nbS` et éventuellement `premierSommet` (s'il s'agit du premier sommet créé dans le graphe) de g. On notera l'intérêt d'avoir à sa disposition le champ `dernierSommet` permettant un accès immédiat à l'extrémité de la liste châineé.

```c
int ajouterSommet(GRAPHE *g, int info)
{
SOMMET *pointeur;
g-&gt;maxS++;
pointeur=(SOMMET *) malloc(sizeof(SOMMET));
if (pointeur == NULL)
{
printf("Erreur! Memoire insuffisante pour creer un sommet\n");
return -1;
}
else
{
pointeur-&gt;label=g-&gt;maxS;
pointeur-&gt;info=info;
pointeur-&gt;suivant=NULL;
pointeur-&gt;adj=NULL;
if (g-&gt;nbS == 0)
{
g-&gt;premierSommet=pointeur;
g-&gt;dernierSommet=pointeur;
}
else
{
g-&gt;dernierSommet-&gt;suivant=pointeur;
g-&gt;dernierSommet=pointeur;