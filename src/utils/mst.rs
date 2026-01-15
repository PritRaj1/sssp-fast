use super::graph::FloatNumber;
use nalgebra::{allocator::Allocator, DefaultAllocator, Dim, OVector, U1};

/// Sentinel value for no parent in MST.
pub const MST_PARENT_NONE: usize = usize::MAX;

/// MST edge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MstEdge<T: FloatNumber> {
    pub from: usize,
    pub to: usize,
    pub weight: T,
}

impl<T: FloatNumber> MstEdge<T> {
    pub fn new(from: usize, to: usize, weight: T) -> Self {
        Self { from, to, weight }
    }
}

/// MST result buffers (keys + parents + membership).
#[derive(Clone, Debug)]
pub struct MstBuffers<T, N>
where
    T: FloatNumber,
    N: Dim,
    DefaultAllocator: Allocator<N>,
{
    pub key: OVector<T, N>,        // Min edge weight to include vertex
    pub parent: OVector<usize, N>, // Parent vertex in MST
    pub in_mst: OVector<bool, N>,  // Whether vertex is in MST
}

impl<T, N> MstBuffers<T, N>
where
    T: FloatNumber,
    N: Dim,
    DefaultAllocator: Allocator<N>,
{
    /// Create new buffers with inf keys and no parents.
    pub fn new_inf(n: N) -> Self {
        let key = OVector::<T, N>::from_element_generic(n, U1, T::infinity());
        let parent = OVector::<usize, N>::from_element_generic(n, U1, MST_PARENT_NONE);
        let in_mst = OVector::<bool, N>::from_element_generic(n, U1, false);
        Self {
            key,
            parent,
            in_mst,
        }
    }

    /// Reset all buffers to initial state.
    pub fn reset_inf(&mut self) {
        self.key.fill(T::infinity());
        self.parent.fill(MST_PARENT_NONE);
        self.in_mst.fill(false);
    }

    /// Init source vertex with key 0.
    #[inline]
    pub fn set_source(&mut self, s: usize) {
        self.key[s] = T::zero();
        self.parent[s] = MST_PARENT_NONE;
    }

    /// Get parent of vertex v, or None if no parent.
    #[inline]
    pub fn parent_of(&self, v: usize) -> Option<usize> {
        let p = self.parent[v];
        (p != MST_PARENT_NONE).then_some(p)
    }

    /// Collect all MST edges from parent array.
    pub fn collect_edges(&self) -> Vec<MstEdge<T>> {
        let mut edges = Vec::new();
        for v in 0..self.parent.len() {
            if self.parent[v] != MST_PARENT_NONE && self.in_mst[v] {
                edges.push(MstEdge::new(self.parent[v], v, self.key[v]));
            }
        }
        edges
    }

    /// Compute total MST weight from collected edges.
    pub fn total_weight(&self) -> T {
        let mut total = T::zero();
        for v in 0..self.key.len() {
            if self.in_mst[v] && self.parent[v] != MST_PARENT_NONE {
                total += self.key[v];
            }
        }
        total
    }

    /// Count vertices in MST.
    pub fn vertices_in_mst(&self) -> usize {
        self.in_mst.iter().filter(|&&x| x).count()
    }
}
