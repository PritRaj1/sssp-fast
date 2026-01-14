use super::graph::FloatNumber;
use nalgebra::{allocator::Allocator, DefaultAllocator, Dim, OVector, U1};

/// Sentinel value for no parent in tree.
pub const PARENT_NONE: usize = usize::MAX;

/// SSSP result buffers (distances + parents).
#[derive(Clone, Debug)]
pub struct SsspBuffers<T, N>
where
    T: FloatNumber,
    N: Dim,
    DefaultAllocator: Allocator<N>,
{
    pub dist: OVector<T, N>,       // Source to vertex
    pub parent: OVector<usize, N>, // Parent vertex
}

impl<T, N> SsspBuffers<T, N>
where
    T: FloatNumber,
    N: Dim,
    DefaultAllocator: Allocator<N>,
{
    /// Create new buffers with inf distances and no parents.
    pub fn new_inf(n: N) -> Self {
        let dist = OVector::<T, N>::from_element_generic(n, U1, T::infinity());
        let parent = OVector::<usize, N>::from_element_generic(n, U1, PARENT_NONE);
        Self { dist, parent }
    }

    /// Reset all distances to inf and parents to none.
    pub fn reset_inf(&mut self) {
        self.dist.fill(T::infinity());
        self.parent.fill(PARENT_NONE);
    }

    /// Init source vertex with distance 0.
    #[inline]
    pub fn set_source(&mut self, s: usize) {
        self.dist[s] = T::zero();
        self.parent[s] = PARENT_NONE;
    }

    /// Get parent of vertex v, or None if no parent.
    #[inline]
    pub fn parent_of(&self, v: usize) -> Option<usize> {
        let p = self.parent[v];
        (p != PARENT_NONE).then_some(p)
    }

    /// Reconstruct path from source to vertex v.
    pub fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        if self.dist[v].is_infinite() {
            return None; // Unreachable
        }
        let mut path = Vec::new();
        let mut curr = v;
        while curr != PARENT_NONE {
            path.push(curr);
            curr = self.parent[curr];
        }
        path.reverse();
        Some(path)
    }
}
