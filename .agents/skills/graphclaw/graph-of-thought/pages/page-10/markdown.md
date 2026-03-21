puter operation tasks *[39]*. In GoT, we partially rely on self-evaluation when taking decisions on how to expand the graph of thoughts within a prompt.

### 8.3 LLMs & Planning

There are many works recently on how to plan complex tasks with LLMs *[36, 37, 68, 76, 78, 81]*. GoT could be seen as a generic framework that could potentially be used to enhance such schemes, by offering a paradigm for generating complex graph-based plans.

### 8.4 Graphs and Graph Computing

Graphs have become an immensely popular and important part of the general computing landscape *[31, 32, 44, 46, 56]*. Recently, there has been a growing interest in domains such as graph databases *[2, 3, 4, 7, 55]*, graph pattern matching *[8, 10, 11, 18, 25, 62]*, graph streaming *[1, 22, 26]*, and graph machine learning as well as graph neural networks *[5, 6, 12, 16, 30, 33, 33, 57, 74, 82, 83]*. The graph abstraction has been fruitful for many modern research domains, such as social sciences (e.g., studying human interactions), bioinformatics (e.g., analyzing protein structures), chemistry (e.g., designing chemical compounds), medicine (e.g., drug discovery), cybersecurity (e.g., identifying intruder machines), healthcare (e.g., exposing groups of people who submit fraudulent claims), web graph analysis (e.g., providing accurate search services), entertainment services (e.g., predicting movie popularity), linguistics (e.g., modeling relationships between words), transportation (e.g., finding efficient routes), physics (e.g., understanding phase transitions and critical phenomena), and many others *[15, 20, 35, 38, 44]*. In this work, we harness the graph abstraction as a key mechanism that enhances prompting capabilities in LLMs.

## 9 Conclusion

Prompt engineering is one of the central new domains of the large language model (LLM) research. It enables using LLMs efficiently, without any model updates. However, designing effective prompts is a challenging task.

In this work, we propose Graph of Thoughts (GoT), a new paradigm that enables the LLM to solve different tasks effectively without any model updates. The key idea is to model the LLM reasoning as an arbitrary graph, where thoughts are vertices and dependencies between thoughts are edges. This enables novel transformations of thoughts, such as aggregation. Human’s task solving is often non-linear, and it involves combining intermediate solutions into final ones, or changing the flow of reasoning upon discovering new insights. GoT reflects this with its graph structure.

GoT outperforms other prompting schemes, for example ensuring 62% increase in the quality of sorting over ToT, while simultaneously reducing costs by $>$31%. We also propose a novel metric for a prompting scheme, the volume of a thought, to indicate the scope of information that a given LLM output could carry with it, where GoT also excels. This provides a step towards more principled prompt engineering.

The graph abstraction has been the foundation of several successful designs in computing and AI over last decades, for example AlphaFold for protein predictions. Our work harnesses it within the realm of prompt engineering.

## Acknowledgements

We thank Hussein Harake, Colin McMurtrie, Mark Klein, Angelo Mangili, and the whole CSCS team granting access to the Ault and Daint machines, and for their excellent technical support. We thank Timo Schneider for help with infrastructure at SPCL. This project received funding from the European Research Council (Project PSAP, No. 101002047), and the European High-Performance Computing Joint Undertaking (JU) under grant agreement No. 955513 (MAELSTROM). This project was supported by the ETH Future Computing Laboratory (EFCL), financed by a donation from Huawei Technologies. This project received funding from the European Union’s HE research and innovation programme under the grant agreement No. 101070141 (Project GLACIATION).

## References

- [1] Besta, M.; Fischer, M.; Kalavri, V.; Kapralov, M.; and Hoefler, T. 2023. Practice of Streaming Processing of Dynamic Graphs: Concepts, Models, and Systems. IEEE Transactions on Parallel and Distributed Systems, 34(6): 1860–1876.
- [2] Besta, M.; Gerstenberger, R.; Blach, N.; Fischer, M.; and Hoefler, T. 2023. GDI: A Graph Database Interface Standard. https://github.com/spcl/GDI-RMA. Accessed: 2023-09-05.
- [3] Besta, M.; Gerstenberger, R.; Fischer, M.; Podstawski, M.; Blach, N.; Egeli, B.; Mitenkov, G.; Chlapek, W.; Michalewicz, M.; Niewiadomski, H.; Müller, J.; and Hoefler, T. 2023. The Graph Database Interface: Scaling Online Transactional and Analytical Graph Workloads to Hundreds of Thousands of Cores. In Proceedings of the International Conference for High Performance Computing, Networking, Storage and Analysis, SC ’23. ACM.
- [4] Besta, M.; Gerstenberger, R.; Peter, E.; Fischer, M.; Podstawski, M.; Barthels, C.; Alonso, G.; and Hoefler, T. 2023. Demystifying Graph Databases: Analysis and Taxonomy of Data Organization, System Designs, and Graph Queries. ACM Comput. Surv., 56(2).
- [5] Besta, M.; Grob, R.; Miglioli, C.; Bernold, N.; Kwaśniewski, G.; Gjini, G.; Kanakagiri, R.; Ashkboos, S.; Gianinazzi, L.; Dryden, N.; and Hoefler, T. 2022. Motif Prediction with Graph Neural Networks. In Proceedings of the 28th ACM SIGKDD Conference on Knowledge Discovery and Data Mining, KDD ’22, 35–45.
- [6] Besta, M.; and Hoefler, T. 2022. Parallel and Distributed Graph Neural Networks: An In-Depth Concurrency Analysis. arXiv:2205.09702.
- [7] Besta, M.; Iff, P.; Scheidl, F.; Osawa, K.; Dryden, N.; Podstawski, M.; Chen, T.; and Hoefler, T. 2022. Neural Graph Databases. In Proceedings of the First Learning on Graphs Conference, volume 198 of Proceedings of Machine Learning Research, 31:1–31:38. PMLR.
-