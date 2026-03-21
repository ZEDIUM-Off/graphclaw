Chapitre V. Annexe : implémentation en C

```c
{ nbLigne++; /* compte le nombre de lignes du fichier */
if (ligne[0] != '\n') /* on passe les lignes vides */
{
i=0;
if (nbS1 == 0) /* compte les sommets de la 1ere ligne */
{
nbS1=1;
while (ligne[i] != '\n')
{
if (ligne[i] == ',') nbS1++;
i++;
}
for (j=1; j&lt;=nbS1; j++)
{
ajouterSommet(g,0);
}
i=0; /* on relit la 1ere ligne */
}
sommet++; /* origine des arcs */
nbElt=0; /* controle le nombre d'arcs crees */
while (ligne[i] != '\n')
{
temp=0; /* va contenir l'extremite */
creerArc=1;
while (ligne[i] != ', &amp;&amp; ligne[i] != '\n')
{
while (ligne[i] == ' || ligne[i] == '\t') {i++;}
if ((ligne[i] &gt; '9' || ligne[i] &lt; '0') &amp;&amp; ligne[i] != 'x')
{
printf("Erreur à la ligne %d !\n",nbLigne);
supprimerGraphe(g);
return -1; /* pas des chiffres ! */
}
if (ligne[i] == 'x') creerArc=0;
temp=10*temp+ligne[i] - '0';
i++;
while (ligne[i] == ' || ligne[i] == '\t') {i++;}
}
if (ligne[i] == ',') i++;
nbElt++;
if (nbElt&lt;=nbS1 &amp;&amp; creerArc==1) ajouterArc(g,sommet,nbElt,temp); /* ligne pas trop longue */
}
if (nbElt != nbS1) /* pas le bon nombre de champs sur ligne */
{
printf("Erreur à la ligne %d !\n",nbLigne);
supprimerGraphe(g);
return -1; /* pas le bon nombre de champs */
}
}
fclose(fp);
return 0;
}