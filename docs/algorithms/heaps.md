# Priority Queues

> Dijkstra and A* use a priority queue to always expand the vertex with minimum distance. The heap implementation affects constant factors but not asymptotic complexity.

| Heap | push | pop | decrease_key | Notes |
|------|------|-----|--------------|-------|
| Binary | O(log n) | O(log n) | O(log n) | Cache-friendly, fast in practice |
| Fibonacci | O(1)* | O(log n)* | O(1)* | Better theory, worse practice |

\* amortized

**BinaryHeap** (default): Use this. Simpler, cache-efficient.

**FibonacciHeap**: Only if very dense graphs (E >> V log V) and want theoretical gains. Benchmark first.

## Usage

```rust
use sssp_fast::{Dijkstra, AStar, FibonacciHeap, ZeroHeuristic};

// BinaryHeap (default)
let d = Dijkstra::<f64>::new();

// FibonacciHeap via turbofish
let d = Dijkstra::<f64, FibonacciHeap<f64>>::new();

// Same for A*
let a = AStar::<f64, ZeroHeuristic>::new(target, ZeroHeuristic);
let a = AStar::<f64, ZeroHeuristic, FibonacciHeap<f64>>::new(target, ZeroHeuristic);
```

## Structure

### Binary Heap

Complete binary tree where parent ≤ children. Stored as array.

```
        1
       / \
      3   2
     / \
    5   4

Array: [1, 3, 2, 5, 4]
Parent of i: (i-1)/2
Children of i: 2i+1, 2i+2
```

Push: add at end, bubble up. Pop: swap root with last, bubble down.

### Fibonacci Heap

Forest of min-heap trees with lazy merging.

```
Roots: [T1] - [T2] - [T3]
        |
      min

Push: add new single-node tree to root list, update min. O(1).
Pop: remove min, add its children to roots, consolidate. O(log n) amortized.
```

Consolidate: merge trees of same degree until all roots have distinct degrees. Uses degree table for O(log n) max degree.

## Ref.

- [OG Fibonacci](https://dl.acm.org/doi/10.1145/28869.28874)
