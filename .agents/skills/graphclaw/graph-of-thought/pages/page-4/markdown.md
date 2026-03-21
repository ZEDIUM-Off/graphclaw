less incorporation of schemes where, in order to save space within the context, one can remove parts of reasoning that do not promise improvements.

The specific form of $\mathcal{T}$ and how it impacts $G$ depends on a specific transformation. We first detail the primary graph-enabled thought transformations, and then proceed to describe how GoT embraces the transformations from the earlier schemes. Unless stated otherwise, $V^{-}=E^{-}=\emptyset$.

#### Aggregation Transformations

First, with GoT, one can aggregate arbitrary thoughts into new ones, to combine and reinforce the advantages of these thoughts, while eliminating their disadvantages. In the basic form, in which only one new vertex is created, $V^{+}=\{v^{+}\}$ and $E^{+}=\{(v_{1},v^{+}),...,(v_{k},v^{+})\}$, where $v_{1},...,v_{k}$ are the merged $k$ thoughts. More generally, this enables aggregating reasoning paths, i.e., longer chains of thoughts, beyond just individual thoughts. With the graph model, it is simply achieved by adding outgoing edges from the vertices $v_{1},...,v_{k}$, modeling final thoughts in several chains, into a single thought $v^{+}$ combining these chains.

#### Refining Transformations

Another thought transformation is the refining of a current thought $v$ by modifying its content: $V^{+}=\{\}$ and $E^{+}=\{(v,v)\}$. This loop in the graph indicates an iterated thought with the same connections as the original thought.

#### Generation Transformations

Finally, one can generate one or more new thoughts based on an existing single thought $v$. This class embraces analogous reasoning steps from earlier schemes, such as ToT or CoT-SC. Formally, we have $V^{+}=\{v_{1}^{+},...,v_{k}^{+}\}$ and $E^{+}=\{(v,v_{1}^{+}),...,(v,v_{k}^{+})\}$.

### 3.3 Scoring & Ranking Thoughts

Thoughts are scored to understand whether the current solution is good enough. A score is modeled as a general function $\mathcal{E}(v,G,p_{\theta})$, where $v$ is a thought to be evaluated. We use the state of the whole reasoning process ($G$) in $\mathcal{E}$ for maximum generality, because – for example – in some evaluation scenarios, scores may be relative to other thoughts.

GoT can also rank thoughts. We model this with a function $\mathcal{R}(G,p_{\theta},h)$ where $h$ specifies the number of highest-ranking thoughts in $G$ to be returned by $\mathcal{R}$. While the specific form of $\mathcal{R}$ depends on the use case, we most often use a simple yet effective strategy where $h$ thoughts with the highest scores are returned, i.e., $v_{1},...,v_{h}=\mathcal{R}(G,p_{\theta},h)$.

Specific forms of $\mathcal{E}$ and $\mathcal{R}$ depend on the use case. We discuss the details in Section 5. For example, the score (or rank) for sorting corresponds to the count of elements correctly sorted (or incorrectly, when using the error as a score).

## 4 System Architecture & Extensibility

The GoT architecture consists of a set of interacting modules, see Figure 3 (the blue part). These modules are the Prompter (prepares the messages for the LLM), the Parser (extracts information from LLM thoughts), the Scoring module (verifies and scores the LLM thoughts), and the Controller (coordinates the entire reasoning process, and decides on how to progress it). The Controller contains two further important elements: the Graph of Operations (GoO) and the Graph Reasoning State (GRS). GoO is a static structure that specifies the graph decomposition of a given task, i.e., it prescribes transformations to be applied to LLM thoughts, together with their order & dependencies. GRS is a dynamic structure that maintains the state of the ongoing LLM reasoning process (the history of its thoughts and their states).

### 4.1 Prompter

The Prompter prepares the prompts to be sent to the LLM. This module is responsible for the specifics of encoding the graph structure within the prompt. The GoT architecture enables the user to implement use case specific graph encodings by providing full access to the graph structure.

### 4.2 Parser

The Parser extracts information from LLM thoughts. For each such thought, the Parser constructs the thought state, which contains this extracted information. The thought state is then used to update the GRS accordingly.

### 4.3 Scoring & Validation

Here, we verify whether a given LLM thought satisfies potential correctness conditions, and then we assign it a score. Depending on how the score is derived, the module may consult the LLM. Moreover, depending on the use case, the score may also be assigned by a human. Finally, use cases such as sorting use simple local scoring functions.

### 4.4 Controller

The Controller implements a specific strategy for selecting thoughts from its GRS structure. It also selects what transformations should be applied to which thoughts, and then passes this information to the Prompter. It also decides whether the whole process should be finalized, or whether the next round of interaction with the LLM should be initiated. In our current design, this is dictated by the execution plan specified in the GoO.

### 4.5 GoO & GRS

The user constructs a GoO instance, which prescribes the execution plan of thought operations. The GoO is a static structure that is constructed once, before the execution starts. Each operation object knows its predecessor and successor operations. Then, during the execution, an instance of the GRS maintains the continually updated information about the LLM reasoning process. This includes which operation has been executed so far, the states of all the generated LLM thoughts, their validity and scores, and any other relevant information.

The above elements offer extensible APIs, enabling straightforward implementations of different prompting schemes. The APIs are outlines in the green part of Figure 3, and detailed in the documentation. We also provide examples of prompts used by these operations and a corresponding GRS in the red part of Figure 3.