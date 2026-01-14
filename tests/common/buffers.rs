use nalgebra::{allocator::Allocator, Const, DefaultAllocator, Dim, Dyn};
use sssp_fast::{FloatNumber, SsspBuffers};

/// Create dynamic buffers.
pub fn dynamic<T: FloatNumber>(n: usize) -> SsspBuffers<T, Dyn> {
    SsspBuffers::new_inf(Dyn(n))
}

/// Create static buffers.
pub fn fixed<T: FloatNumber, const N: usize>() -> SsspBuffers<T, Const<N>> {
    SsspBuffers::new_inf(Const::<N>)
}

/// Create buffers with generic dimension.
pub fn create<T, N>(dim: N) -> SsspBuffers<T, N>
where
    T: FloatNumber,
    N: Dim,
    DefaultAllocator: Allocator<N>,
{
    SsspBuffers::new_inf(dim)
}
