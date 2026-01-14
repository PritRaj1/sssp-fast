# Examples

GIFs showing SSSP algorithm finding the shortest path through a pre-determined maze.

## Run

```bash
cargo run --release --example dijkstra_example
```

GIFs in `examples/gifs/`.

## Colours

| Colour | Meaning |
|--------|---------|
| Black | Wall (impassable) |
| Dark gray | Unvisited cell |
| Yellow | Frontier (in priority queue) |
| Blue gradient | Visited (dark = early, light = late) |
| Green | Shortest path |
| Red | Start |
| Purple | Goal |

## Maze

- 2D grid, 4-directional movement (up/down/left/right)
- All edges have weight 1
