Table 19: Few-shot examples for split, merge and improve_merge prompts used for the keyword counting task; some paragraphs and dictionaries are truncated and formatting is slightly adjusted for brevity.

|  split_prompt:  |
| --- |
|  <Examples>  |
|  Input: Journeying westward, she admired the art in Italy and sipped coffee in France. The music of Spain and the history of Greece deepened her love for Europe. The Nordic beauty of Norway, Sweden, Finland, and Denmark took her breath away. She danced in Ireland, explored castles in Scotland, and marveled at the architecture in Germany and Russia. Italy, Norway, Sweden and Germany will always stay her favourite destinations to visit.  |
|  Output:  |
|  {{ "Paragraph 1": "Journeying westward, she admired the art in Italy and sipped coffee in France.", "Paragraph 2": "The music of Spain and the history of Greece deepened her love for... (Omitted)", "Paragraph 3": "She danced in Ireland, explored castles in Scotland, and marveled... (Omitted)", "Paragraph 4": "Italy, Norway, Sweden and Germany will always stay her favourite... (Omitted)" }}  |
|  </Examples>  |
|  merge_prompt: -  |
|  improve_merge_prompt:  |
|  <Example>  |
|  Dictionary 1: {{ "Peru": 2, "Chile": 1, "Uruguay": 1, "Paraguay": 1}}  |
|  Dictionary 2: {{ "Peru": 1, "Argentina": 1, "Canada": 1, "Chile": 3, "Germany": 2}}  |
|  Incorrectly Combined Dictionary:  |
|  {{ "Peru": 3, "Chile": 2, "Uruguay": 1, "Paraguay": 1, "Argentina": 1, "Chile": 3, "Germany": 2}}  |
|  Output:  |
|  {{ "Peru": 3, "Chile": 4, "Uruguay": 1, "Paraguay": 1, "Argentina": 1, "Canada": 1, "Germany": 2}}  |
|  </Example>  |

Table 20: Keyword counting for an example 4-passage split (GoT4): Execution plan (GoO)

|  GoO:  |
| --- |
|  1. Split the input text into four paragraphs of roughly equal size (split_prompt)  |
|  2. For each paragraph: Count the occurrences of individual countries (count_prompt) 10 times; score each counting attempt; keep the best  |
|  3. Merge the country counts into one dictionary (merge_prompt) 3 times; validate and improve invalid merge attempts (improve_merge_prompt) up to 3 attempts each; score; keep the best  |