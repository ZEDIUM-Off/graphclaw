# F Evaluation - GoT Configurations

We detail the concrete operations that GoT was configured with to solve the set intersection and sorting use cases.

Listing 1: GoT configuration for the set intersection use case with 32 elements
```txt
1 Generate(k=1) # Split second set into two halves of 16 elements
2 foreach subset:
3 Generate(k=5) # Determine intersected subset of subset and first input set
4 Score(k=1) # Score locally the intersected subsets
5 KeepBestN(1) # Keep the best intersected subset
6 Aggregate(10) # Merge both intersected subsets
7 Score(k=1) # Score locally the intersected result sets
8 KeepBestN(1) # Keep the best result
9 GroundTruth() # Compare to precomputed result
```

Listing 2: GoT configuration for the set intersection use case with 64 elements
```txt
1 Generate  $(\mathbf{k} = 1)$  # Split second set into four parts of 16 elements
2 foreach subset:
3 Generate  $(\mathbf{k} = 5)$  # Determine intersected subset of subset and first input set
4 Score  $(\mathbf{k} = 1)$  # Score locally the intersected subsets
5 KeepBestN(1) # Keep the best intersected subset
6 merge step 1:
7 Aggregate(10) # Merge intersected subsets 1 and 2
8 Score  $(\mathbf{k} = 1)$  # Score locally the intersected result sets
9 KeepBestN(1) # Keep the best result
10 merge step 2:
11 Aggregate(10) # Merge intersected subsets 3 and 4
12 Score  $(\mathbf{k} = 1)$  # Score locally the intersected result sets
13 KeepBestN(1) # Keep the best result
14 final merge:
15 Aggregate(10) # Merge intermediate intersected subsets from merge step 1 and 2
16 Score  $(\mathbf{k} = 1)$  # Score locally the intersected result sets
17 KeepBestN(1) # Keep the best result
18 GroundTruth() # Compare to precomputed result
```

Listing 3: GoT configuration for the set intersection use case with 128 elements
```txt
1 Generate  $(\mathbf{k} = 1)$  # Split second set into eight parts of 16 elements
2 foreach subset:
3 Generate  $(\mathbf{k} = 5)$  # Determine intersected subset of subset and first input set
4 Score  $(\mathbf{k} = 1)$  # Score locally the intersected subsets
5 KeepBestN(1) # Keep the best intersected subset
6 merge step 1:
7 Aggregate(5) # Merge intersected subsets 1 and 2
8 Score  $(\mathbf{k} = 1)$  # Score locally the intersected result sets
9 KeepBestN(1) # Keep the best result
10 merge step 2:
11 Aggregate(5) # Merge intersected subsets 3 and 4
12 Score  $(\mathbf{k} = 1)$  # Score locally the intersected result sets
13 KeepBestN(1) # Keep the best result
14 merge step 3:
15 Aggregate(5) # Merge intersected subsets 5 and 6
16 Score  $(\mathbf{k} = 1)$  # Score locally the intersected result sets
17 KeepBestN(1) # Keep the best result
18 merge step 4:
19 Aggregate(5) # Merge intersected subsets 7 and 8
20 Score  $(\mathbf{k} = 1)$  # Score locally the intersected result sets
```

Listing 4: GoT configuration for the set intersection use case with 128 elements (cont.)
```txt
21 KeepBestN(1) # Keep the best result
22 merge step 5:
23 Aggregate(5) # Merge intermediate intersected subsets from merge step 1 and 2
24 Score  $(\mathbf{k} = 1)$  # Score locally the intersected result sets
25 KeepBestN(1) # Keep the best result
26 merge step 6:
27 Aggregate(5) # Merge intermediate intersected subsets from merge step 3 and 4
28 Score  $(\mathbf{k} = 1)$  # Score locally the intersected result sets
29 KeepBestN(1) # Keep the best result
30 final merge:
31 Aggregate(5) # Merge intermediate intersected subsets from merge step 5 and 6
32 Score  $(\mathbf{k} = 1)$  # Score locally the intersected result sets
33 KeepBestN(1) # Keep the best result
34 GroundTruth() # Compare to precomputed result
```

Listing 5: GoT configuration for the sorting use case with 32 elements
```txt
1 Generate  $(\mathbf{k} = 1)$  # Split list into two halves of 16 elements
2 foreach list part:
3 Generate  $(\mathbf{k} = 5)$  # Sort list part
4 Score  $(\mathbf{k} = 1)$  : # Score partially sorted list
5 KeepBestN(1): # Keep the best partially sorted list
6 Aggregate(10) # Merge both partially sorted lists
7 Score  $(\mathbf{k} = 1)$  # Score locally the sorted result lists
8 KeepBestN(1) # Keep the best result
9 Generate  $(\mathbf{k} = 10)$  # Try to improve solution
10 Score  $(\mathbf{k} = 1)$  # Score locally the sorted result lists
11 KeepBestN(1) # Keep the best result
12 GroundTruth() # Compare to precomputed result
```

Listing 6: GoT configuration for the sorting use case with 64 elements
```txt
1 Generate  $(\mathbf{k} = 1)$  # Split list into four parts of 16 elements
2 foreach list part:
3 Generate  $(\mathbf{k} = 5)$  # Sort list part
4 Score  $(\mathbf{k} = 1)$  # Score partially sorted list
5 KeepBestN(1) # Keep the best partially sorted list
6 merge step 1:
7 Aggregate(10) # Merge partially sorted lists 1 and 2
8 Score  $(\mathbf{k} = 1)$  # Score locally the partially sorted result lists
9 KeepBestN(1) # Keep the best result
10 Generate  $(\mathbf{k} = 5)$  # Try to improve the partial solution
11 Score  $(\mathbf{k} = 1)$  # Score locally the partially sorted result lists
12 KeepBestN(1) # Keep the best result
13 merge step 2:
14 Aggregate(10) # Merge partially sorted lists 3 and 4
15 Score  $(\mathbf{k} = 1)$  # Score locally the partially sorted result lists
16 KeepBestN(1) # Keep the best result
17 Generate  $(\mathbf{k} = 5)$  # Try to improve the partial solution
18 Score  $(\mathbf{k} = 1)$  # Score locally the partially sorted result lists
19 KeepBestN(1) # Keep the best result
20 final merge:
21 Aggregate(10) # Merge partially sorted lists from merge step 1 and 2
22 Score  $(\mathbf{k} = 1)$  # Score locally the sorted result lists
23 KeepBestN(1) # Keep the best result
24 Generate  $(\mathbf{k} = 10)$  # Try to improve solution
25 Score  $(\mathbf{k} = 1)$  # Score locally the sorted result lists
26 KeepBestN(1) # Keep the best result
27 GroundTruth() # Compare to precomputed result