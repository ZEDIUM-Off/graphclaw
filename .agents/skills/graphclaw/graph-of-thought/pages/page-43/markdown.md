Table 30: Prompt stubs for the document merging task continued; parameters in single curly brackets will be substituted at runtime.

|  improve_prompt: The following NDA <S> merges initial NDAs <Doc1> - <Doc4>.  |
| --- |
|  Please improve the merged NDA <S> by adding more information and removing redundancy. Output only the improved NDA, placed between the tags <Merged> and </Merged>, without any additional text.  |
|  Here are NDAs <Doc1> - <Doc4>:  |
|  <Doc1> {doc1} </Doc1>  |
|  <Doc2> {doc2} </Doc2>  |
|  <Doc3> {doc3} </Doc3>  |
|  <Doc4> {doc4} </Doc4>  |
|  Here is the merged NDA <S>:  |
|  <S> {s} </S>  |