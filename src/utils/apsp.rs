use super::graph::FloatNumber;

/// Sentinel for no next vertex in path.
pub const APSP_NO_PATH: usize = usize::MAX;

/// APSP result buffers (V×V distance matrix + next matrix for path reconstruction).
#[derive(Clone, Debug)]
pub struct ApspBuffers<T: FloatNumber> {
    pub n: usize,
    pub dist: Vec<T>,     // Flattened V×V matrix (row-major)
    pub next: Vec<usize>, // Flattened V×V matrix for path reconstruction
}

impl<T: FloatNumber> ApspBuffers<T> {
    /// Create new buffers for n vertices.
    pub fn new(n: usize) -> Self {
        let size = n * n;
        let mut dist = vec![T::infinity(); size];
        let next = vec![APSP_NO_PATH; size];

        // Diagonal = 0
        for i in 0..n {
            dist[i * n + i] = T::zero();
        }

        Self { n, dist, next }
    }

    /// Reset to initial state (inf distances, zero diagonal).
    pub fn reset(&mut self) {
        self.dist.fill(T::infinity());
        self.next.fill(APSP_NO_PATH);
        for i in 0..self.n {
            self.dist[i * self.n + i] = T::zero();
        }
    }

    /// Get distance from i to j.
    #[inline]
    pub fn get(&self, i: usize, j: usize) -> T {
        self.dist[i * self.n + j]
    }

    /// Set distance from i to j.
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, d: T) {
        self.dist[i * self.n + j] = d;
    }

    /// Get next vertex on path from i to j.
    #[inline]
    pub fn get_next(&self, i: usize, j: usize) -> usize {
        self.next[i * self.n + j]
    }

    /// Set next vertex on path from i to j.
    #[inline]
    pub fn set_next(&mut self, i: usize, j: usize, v: usize) {
        self.next[i * self.n + j] = v;
    }

    /// Reconstruct path from i to j.
    pub fn path(&self, i: usize, j: usize) -> Option<Vec<usize>> {
        if self.get(i, j).is_infinite() {
            return None;
        }
        if i == j {
            return Some(vec![i]);
        }

        let mut path = vec![i];
        let mut curr = i;
        while curr != j {
            curr = self.get_next(curr, j);
            if curr == APSP_NO_PATH {
                return None;
            }
            path.push(curr);
        }
        Some(path)
    }

    /// Check if negative cycle exists (any diagonal < 0).
    pub fn has_negative_cycle(&self) -> bool {
        for i in 0..self.n {
            if self.get(i, i) < T::zero() {
                return true;
            }
        }
        false
    }
}
