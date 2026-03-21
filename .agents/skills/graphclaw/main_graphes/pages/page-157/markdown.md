V.2. Liste d'adjacence
153

}

Ainsi, pour compiler le fichier et obtenir un fichier exécutable, vous avez besoin dans le répertoire courant non seulement du fichier source main.c mais aussi du fichier graphes.c et du fichier d'en-tête graphes.c. La compilation peut être lancée par la commande suivante⁷.

&gt; gcc graphes.c main.c ~o nom_du_programme

Bien évidemment, il vous faut également un fichier de données nommé data.gr pour utiliser la seconde variante.

2.3. Détail de l'implémentation. Le fichier d'en-tête est on ne peut plus classique. Il sert uniquement à la déclaration des différents types de données, des fonctions et d'une constante MAX utilisée pour la lecture de fichiers.

```c
/* graphes.h --- */
#if defined(GRAPHES_H)
#else
#define GRAPHES_H

#define MAX 10000

struct eltadj {
int dest;
int info;
struct eltadj *suivant;
};

struct sommet {
int label;
int info;
struct sommet *suivant;
struct eltadj *adj;
};

struct graphe {
int nbS;
int nbA;
int maxS;
struct sommet *premierSommet;
struct sommet *dernierSommet;
};

typedef struct graphe GRAPHE;
typedef struct sommet SOMMET;
typedef struct eltadj ELTADJ;

void initialiserGraphe(GRAPHE *);
int ajouterSommet(GRAPHE *, int info);
```

⁷Ou une instruction similaire suivant le système employé.