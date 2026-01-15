# Examples

GIFs showing SSSP algorithms finding shortest paths.

## Run

```bash
# Dijkstra
cargo run --example dijkstra_maze
cargo run --example dijkstra_euclid

# A*
cargo run --example astar_maze
cargo run --example astar_euclid
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

## Dijkstra

Explores in all directions from source. No heuristic guidance.

### Maze (`dijkstra_maze`)
- 2D grid, 4-directional movement
- All edges have weight 1

### Euclidean Graph (`dijkstra_euclid`)
- 500 vertices randomly placed in 2D
- Edge weights = Euclidean distance

## A*

Uses heuristic to guide search toward goal. Explores fewer nodes than Dijkstra.

### Maze (`astar_maze`)
- Same maze as Dijkstra
- Heuristic: Manhattan distance to goal

### Euclidean Graph (`astar_euclid`)
- Same graph as Dijkstra
- Heuristic: Euclidean distance to goal
