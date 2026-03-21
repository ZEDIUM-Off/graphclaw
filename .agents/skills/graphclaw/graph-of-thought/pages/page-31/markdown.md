Table 18: Few-shot examples for count prompt used for the keyword counting task; some paragraphs and dictionaries are truncated and formatting is slightly adjusted for brevity.

|  count_prompt:  |
| --- |
|  <Examples>  |
|  Input: Alexandra boarded the first flight of her grand journey, starting from Canada. With a globe-trotting ... (Omitted)  |
|  Paragraphs:  |
|  Alexandra boarded the first flight of her grand journey, starting from Canada. With a globe-trotting itinerary ... (Omitted)  |
|  Her first stop was Mexico, where she marveled at the Mayan ruins. From there, she explored the rainforests ... (Omitted)  |
|  Sublist frequencies:  |
|  { "Canada": 1 }}  |
|  { "Mexico": 1, "Brazil": 1, "Argentina": 1 }}  |
|  Output: { "Canada": 1, "Mexico": 1, "Brazil": 1, "Argentina": 1 }}  |
|  Input: The adventure led him to the peaks of Peru where he trekked to see the mysteries of Machu Picchu ... (Omitted)  |
|  Paragraphs:  |
|  The adventure led him to the peaks of Peru where he trekked to see the mysteries of Machu Picchu. He then ... (Omitted)  |
|  A quick detour to Uruguay and Paraguay allowed him to experience the vibrancy of the local cultures before ... (Omitted)  |
|  Sublists:  |
|  { "Peru": 1, "Chile": 1 }}  |
|  { "Uruguay": 1, "Paraguay": 1, "Canada": 1, "Peru": 1, "Brazil": 1, "Mexico": 1 }}  |
|  Output: { "Peru": 2, "Chile": 1, "Uruguay": 1, "Paraguay": 1, "Canada": 1, "Brazil": 1, "Mexico": 1 }}  |
|  Input: Journeying westward, she admired the art in Italy and sipped coffee in France. The music of ... (Omitted)  |
|  Paragraphs:  |
|  Journeying westward, she admired the art in Italy and sipped coffee in France.  |
|  The music of Spain and the history of Greece deepened her love for Europe. The Nordic beauty of Norway, ... (Omitted)  |
|  She danced in Ireland, explored castles in Scotland, and marveled at the architecture in Germany and Russia.  |
|  Italy, Norway, Sweden and Germany will always stay her favourite destinations to visit.  |
|  Sublists:  |
|  { "Italy": 1, "France": 1 }}  |
|  { "Spain": 1, "Greece": 1, "Norway": 1, "Sweden": 1, "Finland": 1, "Denmark": 1 }}  |
|  { "Ireland": 1, "Scotland": 1, "Germany": 1, "Russia": 1 }}  |
|  { "Italy": 1, "Norway": 1, "Sweden": 1, "Germany": 1 }}  |
|  Output: { "Italy": 2, "France": 1, "Spain": 1, "Greece": 1, "Norway": 2, "Sweden": 2, ... (Omitted) }}  |