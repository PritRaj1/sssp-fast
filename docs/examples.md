# Examples

GIFs showing SSSP algorithms finding shortest paths.

## Run

```bash
cargo run --example dijkstra_maze
cargo run --example dijkstra_euclid
```

GIFs in `examples/gifs/`.

## Colours

| Colour | Meaning |
|--------|---------|
| Black | Wall (impassable) |
| Dark gray | Unvisited node |
| Yellow | Frontier (in priority queue) |
| Blue gradient | Visited (dark = early, light = late) |
| Green | Shortest path |
| Red | Start |
| Purple | Goal |

## Maze (`dijkstra_maze`)

- 2D grid, 4-directional movement (up/down/left/right)
- All edges have weight 1

## Euclidean Graph (`dijkstra_euclid`)

- 500 vertices randomly placed in 2D space
- Edges connect nearby vertices (k-nearest neighbors + proximity)
- Edge weights = Euclidean distance
