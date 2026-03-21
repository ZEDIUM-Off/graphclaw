Table 3: Prompt stubs for the sorting tasks; parameters in single curly brackets will be substituted at runtime.

|  sort_prompt: <Instruction> Sort the following list of numbers in ascending order. Output only the sorted list of numbers, no additional text. </Instruction>  |
| --- |
|  <Examples> See Table 4 </Examples>  |
|  Input: {input_list}  |
|  split_prompt (32 elements): <Instruction> Split the following list of 32 numbers into 2 lists of 16 numbers each, the first list should contain the first 16 numbers and the second list the second 16 numbers.  |
|  Only output the final 2 lists in the following format without any additional text or thoughts!:  |
|  {{ "List 1": [3, 4, 3, 5, 7, 8, 1, ...], "List 2": [2, 9, 2, 4, 7, 1, 5, ...] }}  |
|  </Instruction>  |
|  <Examples> See Table 4 </Examples>  |
|  Input: {input_list}  |
|  improve_prompt: <Instruction> The following two lists represent an unsorted list of numbers and a sorted variant of that list. The sorted variant is not correct. Fix the sorted variant so that it is correct. Make sure that the output list is sorted in ascending order, has the same number of elements as the input list ({length}), and contains the same elements as the input list.</Instruction>  |
|  <Approach>  |
|  To fix the incorrectly sorted list follow these steps:  |
|  1. For each number from 0 to 9, compare the frequency of that number in the incorrectly sorted list to the frequency of that number in the input list.  |
|  2. Iterate through the incorrectly sorted list and add or remove numbers as needed to make the frequency of each number in the incorrectly sorted list match the frequency of that number in the input list.  |
|  </Approach>  |
|  <Examples> See Table 4 </Examples>  |
|  Input: {input_list}  |
|  Incorrectly Sorted: {sorted_list}  |
|  merge_prompt: <Instruction> Merge the following 2 sorted lists of length {length} each, into one sorted list of length {length(combined} using a merge sort style approach. Only output the final merged list without any additional text or thoughts!: </Instruction>  |
|  <Approach>  |
|  To merge the two lists in a merge-sort style approach, follow these steps:  |
|  1. Compare the first element of both lists.  |
|  2. Append the smaller element to the merged list and move to the next element in the list from which the smaller element came.  |
|  3. Repeat steps 1 and 2 until one of the lists is empty.  |
|  4. Append the remaining elements of the non-empty list to the merged list.  |
|  </Approach>  |
|  Merge the following two lists into one sorted list:  |
|  1. {input_list1}  |
|  2. {input_list2}  |
|  Merged list:  |