IV.4. Coloriage d'arêtes et théorème de Ramsey

de  $n$

|  n | 2Cn2  |
| --- | --- |
|  3 | 8  |
|  4 | 64  |
|  5 | 1024  |
|  6 | 32768  |
|  7 | 2097152  |
|  8 | 268435456  |
|  9 | 68719476736  |
|  10 | 35184372088832  |
|  11 | 36028797018963968  |
|  12 | 73786976294838206464  |
|  13 | 302231454903657293676544  |
|  14 | 2475880078570760549798248448  |

Il nous suffit de démontré le résultat suivant.

Théorème IV.4.3 (Erdős, Szekeres (1935)). Pour tous  $s, t \geq 2$ , le nombre  $R(s, t)$  existe. De plus, on a

$$
R (s, t) \leq \mathrm {C} _ {s + t - 2} ^ {s - 1}
$$

et si  $s,t\geq 3$  ，alors

$$
R (s, t) \leq R (s - 1, t) + R (s, t - 1).
$$

Démonstration. Pour la première partie de la preuve, on procède par récurrence sur  $s + t$ . Au vu de (10), on sait que  $R(2,t)$  et  $R(s,2)$  existent toujours. Donc, en particulier,  $R(s,t)$  existe si  $s + t \leq 5$ . Nous allons supposer que  $R(s,t)$  existe pour tous  $s,t$  tels que  $s + t &lt; N$ . Nous allons montré que  $R(s,t)$  existe encore pour  $s + t = N$ , avec  $N \geq 6$ . Si  $s$  ou  $t$  valent 2, le résultat est entièrement démontré. Nous pouvons donc supposer  $s,t \geq 3$ .

Il nous suffit de trouver un entier  $n$  tel que tout coloriage de  $K_{n}$  contient toujours une copie de  $K_{s}$  rouge ou de  $K_{t}$  bleue. De cette manière, on aura majoré  $R(s,t)$ . Pour cela, nous allons montré que tout graphe  $G = (V,E)$  ayant  $n = R(s - 1,t) + R(s,t - 1)$  sommets contient un sous-graphe isomorphe à  $K_{s}$  ou un ensemble de  $t$  sommets indépendants.