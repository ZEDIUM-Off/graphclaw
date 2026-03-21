many other questions, we carefully design a modular architecture for implementing GoT (contribution #2), coming with two design highlights. First, we enable a fine-grained control over individual thoughts. This enables us to fully control the ongoing conversation with the LLM, and apply advanced thought transformations, such as combining most promising thoughts from the ongoing reasoning into a new one. Second, we ensure that our architecture can be seamlessly extended with novel thought transformations, patterns of reasoning (i.e., graphs of thoughts), and LLM models. This enables rapid prototyping of novel prompting ideas using GoT, while experimenting with different models such as GPT-3.5, GPT-4, or Llama-2 [64].

We illustrate several use cases for GoT (sorting, keyword counting for summaries, set operations, document merging) and we detail how to implement them using the graph-based paradigm (contribution #3). We evaluate GoT and show its advantages over the state of the art (contribution #4). Overall, we observe that GoT is particularly well-suited for tasks that can be naturally decomposed into smaller subtasks that are solved individually and then merged for a final solution. Here, GoT outperforms other schemes, for example improving upon CoT and ToT by, respectively,  $\approx 70\%$  and  $\approx 62\%$ , in terms of the quality of sorting, while simultaneously reducing costs by  $&gt;31\%$  over ToT.

We qualitatively compare GoT to other prompting schemes in Table 1. GoT is the only one to enable arbitrary graph-based thought transformations within a prompt, such as aggregation, embracing all previously proposed schemes.

|  Scheme | Sc? | Mc? | Tr? | Ag?  |
| --- | --- | --- | --- | --- |
|  Chain-of-Thought (CoT) [71] | ■ | × | × | ×  |
|  Self-Consistency with CoT [67] | ■ | ■ | × | ×  |
|  Thought decomposition [75] | ■ | ■ | ■ | ×  |
|  Tree-of-Thought (ToT) [43] | ■ | ■ | ■ | ×  |
|  Tree of Thoughts (ToT) [77] | ■ | ■ | ■ | ×  |
|  Graph of Thoughts (GoT) | ■ | ■ | ■ | ■  |

Table 1: Comparison of prompting schemes, with respect to the supported transformations of thoughts. "Sc?": single chain of thoughts? "Mc?": multiple chains of thoughts? "Tr?": tree of thoughts? "Ag?": arbitrary graph of thoughts? "■": full support, "■": partial support, "×": no support.

Finally, we propose a new metric for evaluating a prompting strategy, the volume of a thought (contribution #5). With this metric, we aim to understand better the differences between prompting schemes. For a given thought  $v$ , the volume of  $v$  is the number of LLM thoughts, from which one can reach  $v$  using directed edges. Intuitively, these are all the LLM thoughts that have had the potential to contribute

to  $v$ . We show that GoT, by incorporating thought transformations such as aggregation, enables thoughts to have fundamentally larger volumes than other schemes.

# 2 Background &amp; Notation

We first outline background concepts and notation.

# 2.1 Language Models &amp; In-Context Learning

The conversation with the LLM consists of user messages (prompts) and LLM replies (thoughts). We follow the established notation [77] and we denote a pre-trained language model (LM) with parameters  $\theta$  as  $p_{\theta}$ . Lowercase letters such as  $x, y, z, \ldots$  indicate LLM thoughts. We purposefully do not prescribe what is a single "thought", and instead make it use-case specific. Hence, a single thought can be a paragraph (e.g., in article summary), a document (e.g., in document generation), a block of code (e.g., in code debugging or optimization), and so on.

We next describe specific prompting approaches.

Input-Output (IO) The Input-Output (IO) prompting is a straightforward approach, in which we use an LLM to turn an input sequence  $x$  into the output  $y$  directly, without any intermediate thoughts.

Chain-of-Thought (CoT) Second, in Chain-of-Thought (CoT), one introduces intermediate thoughts  $a_1, a_2, \ldots$  between  $x$  and  $y$ . This strategy was shown to significantly enhance various LM tasks over the plain IO baseline, such as mathematical puzzles [71] or general mathematical reasoning [24].

Multiple CoTs Third, one can generalize CoT into multiple CoTs by generating several (independent)  $k$  CoTs, and returning the one with the best output (according to some prescribed scoring metric). It was introduced by Wang et al. in the scheme called Self-Consistency with CoT (CoT-SC) [67]. This approach enhances CoT because it offers an opportunity to explore different reasoning paths. However, it does not offer "local exploration" within a path, such as backtracking.

Tree of Thoughts (ToT) Finally, the Tree of Thoughts (ToT) scheme was introduced independently by Yao [77] and Long [43] (where it is referred to as Tree-of-Thought); it was used implicitly to a certain degree by other schemes such as thought decomposition [75]. It enhances CoT-SC by modeling the process or reasoning as a tree of thoughts. A single tree node represents a partial solution. Based on a given node, the thought generator constructs a given number  $k$  of new nodes. Then, the state evaluator generates scores for each such new node. Depending on the use case, the evaluation could be conducted using an LLM itself, or it can harness human scores. Finally, the schedule of extending the tree is dictated by the utilized search algorithm (for example BFS or DFS).

# 3 The GoT Framework

We now detail the GoT framework. We present it in Figure 1, and compare it to other prompting strategies.