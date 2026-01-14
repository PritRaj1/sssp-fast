use super::maps::GridMap;
use super::rendering::CellState;

/// Maze state captured at each step
#[derive(Clone)]
pub struct VisState {
    pub grid: Vec<Vec<CellState>>,
    pub visit_order: Vec<Vec<Option<usize>>>,
    pub max_visited: usize,
}

impl VisState {
    pub fn new(map: &GridMap, start: (usize, usize), end: (usize, usize)) -> Self {
        let mut grid = vec![vec![CellState::Unvisited; map.cols]; map.rows];

        for row in 0..map.rows {
            for col in 0..map.cols {
                if map.is_wall(row, col) {
                    grid[row][col] = CellState::Wall;
                }
            }
        }

        grid[start.0][start.1] = CellState::Start;
        grid[end.0][end.1] = CellState::End;

        Self {
            grid,
            visit_order: vec![vec![None; map.cols]; map.rows],
            max_visited: 0,
        }
    }

    pub fn mark_in_queue(&mut self, row: usize, col: usize) {
        if self.grid[row][col] == CellState::Unvisited {
            self.grid[row][col] = CellState::InQueue;
        }
    }

    pub fn mark_visited(&mut self, row: usize, col: usize) {
        if self.grid[row][col] != CellState::Start && self.grid[row][col] != CellState::End {
            self.grid[row][col] = CellState::Visited;
            self.visit_order[row][col] = Some(self.max_visited);
            self.max_visited += 1;
        }
    }

    pub fn mark_path(&mut self, path: &[(usize, usize)]) {
        for &(row, col) in path {
            if self.grid[row][col] != CellState::Start && self.grid[row][col] != CellState::End {
                self.grid[row][col] = CellState::Path;
            }
        }
    }
}
