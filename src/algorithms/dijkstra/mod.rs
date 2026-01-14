mod algo;
mod config;
mod heap;

pub use algo::Dijkstra;
pub use config::DijkstraConfig;
pub use heap::{HeapEntry, MinHeap};

use crate::algorithms::{SsspAlgorithm, SsspResult};
use crate::utils::{FloatNumber, Graph, SsspBuffers};
use nalgebra::{allocator::Allocator, DefaultAllocator, Dim};

/// One-shot Dijkstra execute.
pub fn cheeky_dijkstra<T, N, G>(
    graph: &G,
    source: usize,
    buffers: &mut SsspBuffers<T, N>,
) -> SsspResult<T>
where
    T: FloatNumber,
    N: Dim,
    G: Graph<T>,
    DefaultAllocator: Allocator<N>,
{
    Dijkstra::new().run(graph, source, buffers)
}

pub fn dijkstra_to<T, N, G>(
    graph: &G,
    source: usize,
    target: usize,
    buffers: &mut SsspBuffers<T, N>,
) -> SsspResult<T>
where
    T: FloatNumber,
    N: Dim,
    G: Graph<T>,
    DefaultAllocator: Allocator<N>,
{
    Dijkstra::with_config(DijkstraConfig::with_target(target)).run(graph, source, buffers)
}
