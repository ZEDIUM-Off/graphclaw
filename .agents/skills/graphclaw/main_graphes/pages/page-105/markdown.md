II.5. Arbres couvrants

formule vérifiée pour $n - 1$. Au vu de l'argumentation développée ci-dessus, il vient

$$
\begin{array}{l}
T_{n,d_1,\dots,d_n} = \sum_{i:d_i \geq 2} T_{n-1,d_1,\dots,d_{i-1},d_i-1,d_{i+1},\dots,d_{n-1}} \\
= \sum_{i:d_i \geq 2} \binom{n-3}{d_1-1,\dots,d_{i-1}-1,d_i-2,d_{i+1}-1,\dots,d_{n-1}-1} \\
= \binom{n-2}{d_1-1,d_2-1,\dots,d_{n-1}-1} \\
= \binom{n-2}{d_1-1,d_2-1,\dots,d_{n-1}-1,d_n-1}
\end{array}
$$

où, à la deuxième ligne, on utilise l'hypothèse de récurrence et pour obtenir l'avant-dernière ligne, nous avons utilisé le résultat suivant (nous sommes dans les conditions d'application, $d_1 - 1 + \dots + d_n - 1 = n - 2$ car $d_n = 1$.)

**Lemme II.5.10.** Pour tous $n, n_1, \ldots, n_p \in \mathbb{N}$ tels que $n_1 + \dots + n_p = n$, on a

$$
\binom{n}{n_1,\dots,n_p} = \sum_{i: n_i \geq 1} \binom{n-1}{n_1,\dots,n_{i-1},n_i-1,n_{i+1},\dots,n_p}.
$$

**Démonstration.** En effet, il est clair que

$$
(a_1 + \dots + a_p)^n = (a_1 + \dots + a_p) (a_1 + \dots + a_p)^{n-1}.
$$

En se rappelant le théorème multinomial$^{26}$, le terme général du membre de gauche est

$$
\binom{n}{n_1,\dots,n_p} a_1^{n_1} \cdots a_p^{n_p}
$$

et celui du membre de droite est

$$
\sum_{i:n_i \geq 1} a_i \binom{n-1}{n_1,\dots,n_{i-1},n_i-1,n_{i+1},\dots,n_p} a_1^{n_1} \cdots a_i^{n_i-1} \cdots a_p^{n_p}.
$$

La conclusion s'ensuit directement.

**Remarque II.5.11.** Ainsi, la proposition II.5.6 est un corollaire immédiat du théorème II.5.8. En effet, dans la proposition II.5.6, on compte juste

$$
(a_1 + \dots + a_p)^n = \sum_{\substack{n_1,\dots,n_p \in \mathbb{N} \\ n_1 + \dots + n_p = n}} \binom{n}{n_1,\dots,n_p} a_1^{n_1} \cdots a_p^{n_p}.
$$