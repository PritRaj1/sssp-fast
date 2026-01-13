use crate::trait::FloatNum;
use nalgebra::{
    allocator::Allocator,
    DefaultAllocator,
    Const,
    Dim,
    OVector,
    usize,
    U1
};

// Sentinel value representing "no parent".
pub const PARENT_NONE: usize = usize::MAX;

#[derive(Clone, Debug)]
pub struct SsspBuffers<T, N>
where
    T: FloatNum,
    N: Dim,
    DefaultAllocator: Allocator<T, N> + Allocator<usize, N>,
{
    pub dist: OVector<T, N>,
    pub parent: OVector<usize, N>,
}

impl<T, N> SsspBuffers<T, N>
where
    T: FloatNum,
    N: Dim,
    DefaultAllocator: Allocator<T, N> + Allocator<usize, N>,
{   
    // Init infty + no parent
    pub fn new_inf(n: N) -> Self {
        let dist = OVector::<T, N>::from_element_generic(
            N::from_usize(1), 
            U1, 
            T::infinity()
        );
        let parent = OVector::<usize, N>::from_element_generic(
            N::from_usize(1), 
            U1, 
            PARENT_NONE
        );
        Self { dist, parent }
    }

    /// Reset infty + no-parent.
    pub fn reset_inf(&mut self) {
        self.dist.fill(T::infinity());
        self.parent.fill(PARENT_NONE);
    }

    #[inline]
    pub fn set_source(&mut self, s: usize) {
        self.dist[s] = T::zero();
        self.parent[s] = PARENT_NONE;
    }

    #[inline]
    pub fn parent_of(&self, v: usize) -> Option<usize> {
        let p = self.parent[v];
        (p != PARENT_NONE).then_some(p)
    }
}
