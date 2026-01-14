use crate::algorithms::HasSsspConfig;
use crate::algorithms::{finalize_sssp, init_sssp, SsspAlgorithm, SsspAlgorithmInfo, SsspResult};
use crate::utils::{relax_with, FloatNumber, Graph, RelaxResult, SsspBuffers};
use nalgebra::{allocator::Allocator, DefaultAllocator, Dim};

use super::config::DijkstraConfig;
use super::heap::MinHeap;

#[derive(Debug)]
pub struct Dijkstra<T: FloatNumber> {
    config: DijkstraConfig,
    heap: MinHeap<T>,
}

impl<T: FloatNumber> Dijkstra<T> {
    pub fn new() -> Self {
        Self {
            config: DijkstraConfig::default(),
            heap: MinHeap::new(),
        }
    }

    pub fn with_config(config: DijkstraConfig) -> Self {
        Self {
            config,
            heap: MinHeap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            config: DijkstraConfig::default(),
            heap: MinHeap::with_capacity(capacity),
        }
    }

    pub fn config(&self) -> &DijkstraConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut DijkstraConfig {
        &mut self.config
    }
}

impl<T: FloatNumber> Default for Dijkstra<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: FloatNumber> SsspAlgorithmInfo for Dijkstra<T> {
    fn name(&self) -> &'static str {
        "Dijkstra"
    }

    fn supports_negative_weights(&self) -> bool {
        false
    }
}

impl<T, N, G> SsspAlgorithm<T, N, G> for Dijkstra<T>
where
    T: FloatNumber,
    N: Dim,
    G: Graph<T>,
    DefaultAllocator: Allocator<N>,
{
    fn run(&mut self, graph: &G, source: usize, buffers: &mut SsspBuffers<T, N>) -> SsspResult<T> {
        debug_assert!(source < graph.n(), "Source vertex out of bounds");

        init_sssp(buffers, source);
        self.heap.clear();
        self.heap.push(T::zero(), source);

        let mut iterations = 0usize;

        while let Some(entry) = self.heap.pop() {
            let u = entry.vertex;
            let d_u = entry.dist;

            // Lazy delete, skip stale entries
            if self.config.lazy_deletion && d_u > buffers.dist[u] {
                continue;
            }

            if self.config.should_stop(u) {
                break;
            }

            iterations += 1;

            // Relax outgoing edges
            graph.for_each_out_edge(u, |v, w| {
                debug_assert!(w >= T::zero(), "Dijkstra requires non-negative weights");

                if let RelaxResult::Improved = relax_with(
                    buffers.dist.as_mut_slice(),
                    buffers.parent.as_mut_slice(),
                    u,
                    d_u,
                    v,
                    w,
                ) {
                    self.heap.push(buffers.dist[v], v);
                }
            });
        }

        finalize_sssp(buffers, iterations, false)
    }
}
