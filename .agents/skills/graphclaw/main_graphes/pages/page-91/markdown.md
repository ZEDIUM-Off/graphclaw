II.3. Une application : l'algorithm de PageRank

- Si  $\lambda_{A} = \lambda_{B} \neq \lambda_{C}$  (par symétrie, les autres cas se traitent de la même façon), alors (4) devient

$$
\sum_ {i = 0} ^ {n} \lambda_ {A} ^ {i} \underbrace {\sum_ {j = 0} ^ {n - i} \lambda_ {A} ^ {j} \lambda_ {C} ^ {n - i - j}} _ {\substack {\lambda_ {A} ^ {n - i + 1} - \lambda_ {C} ^ {n - i + 1} \\ \lambda_ {A} - \lambda_ {C}}} = \frac {(n + 1) \lambda_ {A} ^ {n + 1}}{\lambda_ {A} - \lambda_ {C}} - \frac {\lambda_ {C}}{\lambda_ {A} - \lambda_ {C}} \underbrace {\sum_ {i = 0} ^ {n} \lambda_ {A} ^ {i} \lambda_ {C} ^ {n - i}} _ {\substack {\lambda_ {A} ^ {n + 1} - \lambda_ {C} ^ {n + 1} \\ \lambda_ {A} - \lambda_ {C}}}.
$$

Ainsi, si  $\lambda_{A} = \lambda_{B} &gt; \lambda_{C}$ , on trouve

$$
c _ {a, B, c} (n) \asymp n \lambda_ {A} ^ {n}
$$

et si  $\lambda_{A} = \lambda_{B} &lt;   \lambda_{C}$  , on trouve

$$
c _ {a, B, c} (n) \asymp \lambda_ {C} ^ {n}.
$$

- Enfin, si  $\lambda_A, \lambda_B$  et  $\lambda_C$  sont 2 à 2 distincts, alors (4) devient

$$
\sum_ {i = 0} ^ {n} \underbrace {\lambda_ {A} ^ {i} \sum_ {j = 0} ^ {n - i} \lambda_ {B} ^ {j} \lambda_ {C} ^ {n - i - j}} _ {\substack {\lambda_ {B} ^ {n - i + 1} - \lambda_ {C} ^ {n - i + 1} \\ \lambda_ {B} - \lambda_ {C}}} = \frac {1}{\lambda_ {B} - \lambda_ {C}} \left(\lambda_ {B} \sum_ {i = 0} ^ {n} \lambda_ {A} ^ {i} \lambda_ {B} ^ {n - i} - \lambda_ {C} \sum_ {i = 0} ^ {n} \lambda_ {A} ^ {i} \lambda_ {C} ^ {n - i}\right)
$$

et on obtient

$$
c _ {a, B, c} (n) \asymp [ \max (\lambda_ {A}, \lambda_ {B}, \lambda_ {C}) ] ^ {n}.
$$

En conclusion, il suffit de détecter la plus grande valeur de Perron  $\lambda$  des différentes composantes connexes par lesquelles passent les chemins d'intérêt et de compter le nombre  $k$  de composantes ayant cette valeur propre comme valeur dominante. Le nombre de chemins de longueur  $n$  se comporte alors asymptotiquement comme  $n^{k-1}\lambda^n$ . Le lecteur adaptera facilement les développements effectués ci-dessus au cas d'un nombre arbitraire de composantes primitives. On pourrait aussi regarder la forme de Jordan de la matrice d'adjacence associée, un bloc de taille  $k$  associé à la valeur propre  $\lambda$  fait intervenir un polynôme de degré  $k-1$  multiplé par une exponentielle  $\lambda^n$ .

# 3. Une application : l'algorithm de PageRank

Dans cette section, nous allons illustrer certainement l'une des applications les plus célibres du théorème de Perron-Frobenius: le succès de Google basé sur leur algorithme de classement des pages web! Volontairement, plusieurs aspects ne sont pas pris en compte dans cette presentation (développement et efficacité des méthodes numériques, choix de la constante  $\alpha$ , mise en pratique, sensibilité aux perturbations, modèle plus évolué, ...). Pour plus de détails, consulter par exemple l'excellent ouvrage de C. Meyer et A. Langville consacre entièrement à ce sujet.