58]. Set intersection of two sets is implemented similarly as the sorting. The second input set is split into subsets and the intersection of those subsets with the first input set is determined with the help of the LLM. Afterwards the resulting intersection sets are aggregated for the final results. For the evaluation we use different set sizes of 32, 64 and 128 elements and we vary the number of elements found in both sets to be between  $25\%$  and  $75\%$ .

Our score indicates the total number of missing or incorrectly included elements in the final intersection. Specifically, denote two input sets with  $A = [a_{1}, a_{2}, \dots, a_{n}]$  and  $B = [b_{1}, b_{2}, \dots, b_{n}]$ , and the output set with  $C = [c_{1}, c_{2}, \dots, c_{m}]$ . Then,

error-scope  $= X_{1} + X_{2} + X_{d}$

where  $X_{1} = |C \setminus (A \cap B)|$  are the number of elements in  $C$  that are not supposed to be there,  $X_{2} = |(A \cap B) \setminus C|$  are the number of elements missing from  $C$ , and  $X_{d}$  is the number of duplicates in  $C$  (because the LLM expresses the set as a list in natural language). Finally, to use a "positive score" describing "the scope of correctly computed" elements, one can use the value  $\max(n - \text{error-scope}, 0)$ .

# 5.3 Keyword Counting

Keyword counting finds the frequency of keywords in a given category (countries in our example implementation) within the input text. GoT splits the input text into multiple passages, counts the keywords in each passage and aggregates the subresults. The number of passages is configurable and can also be left to the LLM, making it possible to treat each sentence as a separate passage. Here, to score a thought, we first – for each keyword – derive the absolute difference between the computed count and the correct one. We then sum all these differences to get the final score.

# 5.4 Document Merging

Finally, we also provide document merging. Here, the goal is to generate a new Non-Disclosure Agreement (NDA) document based on several input ones that partially overlap in terms of their contents. The goal is to ensure minimal amount of duplication, while maximizing information retention. Document merging is broadly applicable in, e.g., legal procedures, where multiple sources of information have to be combined into a single document or article. To score a solution, we query the LLM for two values (3 times for each value, and take the average). The first value corresponds to the solution redundancy (10 indicates no redundancy, 0 implies at least half the information is redundant), the second value stands for information retention (10 indicates all information is retained, 0 says that none is retained). We compute the harmonic mean of these values.

# 6 The Latency-Volume Tradeoff

We now show that GoT improves upon previous prompting schemes in terms of the tradeoff between latency (number of hops in the graph of thoughts to reach a given final thought) and volume. We define volume - for a given thought  $t$  - as

the number of preceding LLM thoughts that could have impacted  $t$ . Formally, the volume of  $t$  is the number of thoughts from which there exists a path to  $t$  in the graph of thoughts. We assume that outputting a single thought costs  $O(1)$  time and fix the total cost to  $\Theta(n)$  for each prompting scheme.

The structure of the schemes is as follows. CoT-SC consists of  $k$  independent chains originating from a single starting thought. ToT is a complete  $k$ -ary tree. Finally, in GoT, a complete  $k$ -ary tree is joined at its leaves with a "mirrored"  $k$ -ary tree of the same size but with its edges reversed.

The analysis is detailed in Table 2. CoT offers a large volume of up to  $N$ , but at the cost of a high latency of  $N$ . CoT-SC reduces the latency by a factor of  $k$  (which corresponds to its branching factor), but it simultaneously decreases the volume by  $k$  as well. ToT offers a latency of  $\log_k N$  but also has low volume. GoT is the only scheme to come with both a low latency of  $\log_k N$  and a high volume  $N$ . This is enabled by the fact that GoT harnesses aggregations of thoughts, making it possible to reach the final thought from any other intermediate thought in the graph decomposition.

|  Scheme | Latency | Volume  |
| --- | --- | --- |
|  Chain-of-Thought (CoT) | N | N  |
|  Self-Consistency with CoT (CoT-SC) | N/k | N/k  |
|  Tree of Thoughts (ToT) | logkN | O(logkN)  |
|  Graph of Thoughts (GoT) | logkN | N  |

Table 2: Comparison of prompting schemes, with respect to their fundamental tradeoff between latency and volume. GoT offers the best tradeoff.

# 7 Evaluation

We show the advantages of GoT over the state of the art. We focus on comparing GoT to ToT, as it was shown to consistently outperform other schemes. Still, for a broad comparison, we also experiment with IO, CoT, and CoT-SC. As our analysis results in a large evaluation space, we present representative results and omit data that does not bring relevant insights (e.g., CoT-SC).

# 7.1 Evaluation Methodology

We use 100 input samples for each task and comparison baseline. We set the temperature to 1.0 and use a 4k context size unless stated otherwise. For each experiment, we fix the numbers of thoughts in respective schemes to achieve similar costs in each experiment.

Parameters We experiment extensively with the branching factor  $k$  and the number of levels  $L$  to ensure that we compare GoT to cost-effective and advantageous configurations. We plot two variants of ToT: one with higher  $k$  and lower depth (ToT), the other with lower  $k$  but higher  $L$  (ToT2). We usually aim to achieve a sweet spot in the tradeoff between sparser generation rounds (lower  $k$ ) vs. more rounds (larger  $L$ ). Usually more responses per round is more expensive (e.g., 80 vs. 60 total responses for Figure 7 but $6 vs. $3 costs). We also try different problem sizes  $P$  (e.g., in sorting,  $P$  states how many numbers are to be sorted).