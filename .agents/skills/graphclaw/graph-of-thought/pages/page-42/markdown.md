Table 29: Prompt stubs for the document merging task; parameters in single curly brackets will be substituted at runtime.

|  merge_prompt: Merge the following 4 NDA documents <Doc1> - <Doc4> into a single NDA, maximizing retained information and minimizing redundancy. Output only the created NDA between the tags <Merged> and </Merged>, without any additional text.  |
| --- |
|  Here are NDAs <Doc1> - <Doc4>:  |
|  <Doc1> {doc1} </Doc1>  |
|  <Doc2> {doc2} </Doc2>  |
|  <Doc3> {doc3} </Doc3>  |
|  <Doc4> {doc4} </Doc4>  |
|  score_prompt: The following NDA <S> merges NDAs <Doc1> - <Doc4>.  |
|  Please score the merged NDA <S> in terms of how much redundant information is contained, independent of the original NDAs, as well as how much information is retained from the original NDAs.  |
|  A score of 10 for redundancy implies that absolutely no information is redundant, while a score of 0 implies that at least half of the information is redundant (so everything is at least mentioned twice).  |
|  A score of 10 for retained information implies that all information from the original NDAs is retained, while a score of 0 implies that no information is retained.  |
|  You may provide reasoning for your scoring, but the final score for redundancy should be between the tags <Redundancy> and </Redundancy>, and the final score for retained information should be between the tags <Retained> and </Retained>, without any additional text within any of those tags.  |
|  Here are NDAs <Doc1> - <Doc4>:  |
|  <Doc1> {doc1} </Doc1>  |
|  <Doc2> {doc2} </Doc2>  |
|  <Doc3> {doc3} </Doc3>  |
|  <Doc4> {doc4} </Doc4>  |
|  Here is the merged NDA <S>:  |
|  <S> {s} </S>  |
|  aggregate_prompt: The following NDAs <S1> - <S{num_ndas_summaries}> each merge the initial NDAs <Doc1> - <Doc4>.  |
|  Combine the merged NDAs <S1> - <S{num_ndas_summaries}> into a new one, maximizing their advantages and overall information retention, while minimizing redundancy.  |
|  Output only the new NDA between the tags <Merged> and </Merged>, without any additional text.  |
|  Here are the original NDAs <Doc1> - <Doc4>:  |
|  <Doc1> {doc1} </Doc1>  |
|  <Doc2> {doc2} </Doc2>  |
|  <Doc3> {doc3} </Doc3>  |
|  <Doc4> {doc4} </Doc4>  |
|  Here are the merged NDAs <S1> - <S{num_ndas_summaries}>:  |
|  <S1> {s1} </S1>  |
|  ...  |
|  <S{num_ndas_summaries}> {s{num_ndas_summaries}} </S{num_ndas_summaries}>  |