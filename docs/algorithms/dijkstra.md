# Dijkstra's Algorithm

## Overview

Dijkstra's algorithm finds shortest paths from a source vertex to all other vertices in a weighted graph with non-negative edge weights.

|  | Complexity |
|--------|-------|
| Time | O((V + E) log V) |
| Space | O(V) distances + O(V) heap |

- All edge weights must be **non-negative**
- Graph can be directed or undirected

## Configuration

```rust
DijkstraConfig {
    base: SsspConfig,      // inherited (early_stop, etc.)
    lazy_deletion: bool,   // default: true
}
```

## Usage

```rust
use sssp_fast::{AdjListGraph, SsspBuffers, dijkstra, Dyn};

let mut graph: AdjListGraph<f64> = AdjListGraph::new(4);
graph.add_edge(0, 1, 1.0);
graph.add_edge(1, 2, 2.0);

let mut buffers: SsspBuffers<f64, Dyn> = SsspBuffers::new_inf(Dyn(4));
let result = dijkstra(&graph, 0, &mut buffers);

// buffers.dist[v] = shortest distance from source to v
// buffers.path_to(v) = reconstructed path
```

## References

- [The OG himself](https://ir.cwi.nl/pub/9256/9256D.pdf)
