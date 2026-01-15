use crate::utils::{FloatNumber, Graph, MstBuffers, SsspBuffers};
use nalgebra::{allocator::Allocator, DefaultAllocator, Dim};

// =============================================================================
// Config
// =============================================================================

#[derive(Clone, Debug, Default)]
pub struct SsspConfig {
    pub early_stop: Option<usize>,
}

impl SsspConfig {
    pub fn with_target(target: usize) -> Self {
        Self {
            early_stop: Some(target),
        }
    }

    #[inline]
    pub fn should_stop(&self, vertex: usize) -> bool {
        self.early_stop == Some(vertex)
    }
}

pub trait HasSsspConfig {
    fn sssp_config(&self) -> &SsspConfig;

    #[inline]
    fn should_stop(&self, vertex: usize) -> bool {
        self.sssp_config().should_stop(vertex)
    }
}

impl HasSsspConfig for SsspConfig {
    fn sssp_config(&self) -> &SsspConfig {
        self
    }
}

// =============================================================================
// Result
// =============================================================================

#[derive(Clone, Debug)]
pub struct SsspResult<T: FloatNumber> {
    pub iterations: usize,
    pub negative_cycle: bool,
    pub vertices_reached: usize,
    pub total_distance: T,
}

#[derive(Clone, Debug)]
pub struct MstResult<T: FloatNumber> {
    pub iterations: usize,
    pub vertices_in_mst: usize,
    pub total_weight: T,
    pub is_connected: bool,
}

// =============================================================================
// Traits
// =============================================================================

pub trait SsspAlgorithmInfo {
    fn name(&self) -> &'static str;
    fn supports_negative_weights(&self) -> bool;
}

pub trait SsspAlgorithm<T, N, G>: SsspAlgorithmInfo
where
    T: FloatNumber,
    N: Dim,
    G: Graph<T>,
    DefaultAllocator: Allocator<N>,
{
    fn run(&mut self, graph: &G, source: usize, buffers: &mut SsspBuffers<T, N>) -> SsspResult<T>;
}

pub trait MstAlgorithmInfo {
    fn name(&self) -> &'static str;
}

pub trait MstAlgorithm<T, N, G>: MstAlgorithmInfo
where
    T: FloatNumber,
    N: Dim,
    G: Graph<T>,
    DefaultAllocator: Allocator<N>,
{
    fn run(&mut self, graph: &G, source: usize, buffers: &mut MstBuffers<T, N>) -> MstResult<T>;
}

// =============================================================================
// Runner
// =============================================================================

#[inline]
pub fn init_sssp<T, N>(buffers: &mut SsspBuffers<T, N>, source: usize)
where
    T: FloatNumber,
    N: Dim,
    DefaultAllocator: Allocator<N>,
{
    buffers.reset_inf();
    buffers.set_source(source);
}

pub fn finalize_sssp<T, N>(
    buffers: &SsspBuffers<T, N>,
    iterations: usize,
    negative_cycle: bool,
) -> SsspResult<T>
where
    T: FloatNumber,
    N: Dim,
    DefaultAllocator: Allocator<N>,
{
    let (vertices_reached, total_distance) = compute_sssp_stats(buffers);
    SsspResult {
        iterations,
        negative_cycle,
        vertices_reached,
        total_distance,
    }
}

fn compute_sssp_stats<T, N>(buffers: &SsspBuffers<T, N>) -> (usize, T)
where
    T: FloatNumber,
    N: Dim,
    DefaultAllocator: Allocator<N>,
{
    let mut reached = 0usize;
    let mut total = T::zero();
    for i in 0..buffers.dist.len() {
        if !buffers.dist[i].is_infinite() {
            reached += 1;
            total += buffers.dist[i];
        }
    }
    (reached, total)
}

#[inline]
pub fn init_mst<T, N>(buffers: &mut MstBuffers<T, N>, source: usize)
where
    T: FloatNumber,
    N: Dim,
    DefaultAllocator: Allocator<N>,
{
    buffers.reset_inf();
    buffers.set_source(source);
}

pub fn finalize_mst<T, N>(buffers: &MstBuffers<T, N>, iterations: usize, n: usize) -> MstResult<T>
where
    T: FloatNumber,
    N: Dim,
    DefaultAllocator: Allocator<N>,
{
    let vertices_in_mst = buffers.vertices_in_mst();
    let total_weight = buffers.total_weight();
    let is_connected = vertices_in_mst == n;

    MstResult {
        iterations,
        vertices_in_mst,
        total_weight,
        is_connected,
    }
}
