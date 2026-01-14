/// 2D grid with walls and walkable cells
#[derive(Clone)]
pub struct GridMap {
    pub rows: usize,
    pub cols: usize,
    pub walls: Vec<Vec<bool>>,
}

impl GridMap {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            walls: vec![vec![false; cols]; rows],
        }
    }

    pub fn set_wall(&mut self, row: usize, col: usize) {
        if row < self.rows && col < self.cols {
            self.walls[row][col] = true;
        }
    }

    pub fn is_wall(&self, row: usize, col: usize) -> bool {
        row >= self.rows || col >= self.cols || self.walls[row][col]
    }

    pub fn is_passable(&self, row: usize, col: usize) -> bool {
        !self.is_wall(row, col)
    }

    pub fn to_vertex(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn to_coords(&self, vertex: usize) -> (usize, usize) {
        (vertex / self.cols, vertex % self.cols)
    }
}

pub fn simple_maze() -> (GridMap, (usize, usize), (usize, usize)) {
    let mut map = GridMap::new(12, 16);

    for col in 2..10 {
        map.set_wall(3, col);
    }
    for col in 6..14 {
        map.set_wall(8, col);
    }

    for row in 1..7 {
        map.set_wall(row, 12);
    }
    for row in 5..11 {
        map.set_wall(row, 4);
    }

    map.set_wall(6, 8);
    map.set_wall(6, 9);
    map.set_wall(5, 9);

    let start = (1, 1);
    let end = (10, 14);

    (map, start, end)
}

pub fn complex_maze() -> (GridMap, (usize, usize), (usize, usize)) {
    let mut map = GridMap::new(25, 35);

    // Outer walls
    for col in 0..35 {
        map.set_wall(0, col);
        map.set_wall(24, col);
    }
    for row in 0..25 {
        map.set_wall(row, 0);
        map.set_wall(row, 34);
    }

    // Horizontal walls
    for col in 2..12 {
        map.set_wall(3, col);
    }
    for col in 15..30 {
        map.set_wall(3, col);
    }
    for col in 5..20 {
        map.set_wall(7, col);
    }
    for col in 25..33 {
        map.set_wall(7, col);
    }
    for col in 2..15 {
        map.set_wall(11, col);
    }
    for col in 20..32 {
        map.set_wall(11, col);
    }
    for col in 8..25 {
        map.set_wall(15, col);
    }
    for col in 3..18 {
        map.set_wall(19, col);
    }
    for col in 22..33 {
        map.set_wall(19, col);
    }

    // Vertical walls
    for row in 3..10 {
        map.set_wall(row, 18);
    }
    for row in 7..15 {
        map.set_wall(row, 5);
    }
    for row in 11..20 {
        map.set_wall(row, 28);
    }
    for row in 15..23 {
        map.set_wall(row, 12);
    }
    for row in 3..8 {
        map.set_wall(row, 25);
    }
    for row in 19..24 {
        map.set_wall(row, 20);
    }

    let start = (1, 1);
    let end = (23, 33);

    (map, start, end)
}
