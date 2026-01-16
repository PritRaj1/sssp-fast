mod algo;
mod config;

pub use algo::BellmanFord;
pub use config::BellmanFordConfig;

use crate::algorithms::{SsspAlgorithm, SsspResult};
use crate::utils::{FloatNumber, Graph, SsspBuffers};
use nalgebra::{allocator::Allocator, DefaultAllocator, Dim};

/// One-shot Bellman-Ford execute.
pub fn cheeky_bellman_ford<T, N, G>(
    graph: &G,
    source: usize,
    buffers: &mut SsspBuffers<T, N>,
) -> SsspResult<T>
where
    T: FloatNumber,
    N: Dim,
    G: Graph<T> + Sync,
    DefaultAllocator: Allocator<N>,
{
    BellmanFord::<T>::new().run(graph, source, buffers)
}

pub fn bellman_ford_to<T, N, G>(
    graph: &G,
    source: usize,
    target: usize,
    buffers: &mut SsspBuffers<T, N>,
) -> SsspResult<T>
where
    T: FloatNumber,
    N: Dim,
    G: Graph<T> + Sync,
    DefaultAllocator: Allocator<N>,
{
    BellmanFord::<T>::with_config(BellmanFordConfig::with_target(target))
        .run(graph, source, buffers)
}
