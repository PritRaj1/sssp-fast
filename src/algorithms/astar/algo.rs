use crate::algorithms::dijkstra::MinHeap;
use crate::algorithms::HasSsspConfig;
use crate::algorithms::{finalize_sssp, init_sssp, SsspAlgorithm, SsspAlgorithmInfo, SsspResult};
use crate::utils::{relax_with, FloatNumber, Graph, RelaxResult, SsspBuffers};
use nalgebra::{allocator::Allocator, DefaultAllocator, Dim};

use super::config::{AStarConfig, Heuristic};

#[derive(Debug)]
pub struct AStar<T: FloatNumber, H: Heuristic<T>> {
    config: AStarConfig<H>,
    heap: MinHeap<T>, // Stores f(v) = g(v) + h(v)
}

impl<T: FloatNumber, H: Heuristic<T>> AStar<T, H> {
    pub fn new(target: usize, heuristic: H) -> Self {
        Self {
            config: AStarConfig::new(target, heuristic),
            heap: MinHeap::new(),
        }
    }

    pub fn with_config(config: AStarConfig<H>) -> Self {
        Self {
            config,
            heap: MinHeap::new(),
        }
    }

    pub fn with_capacity(target: usize, heuristic: H, capacity: usize) -> Self {
        Self {
            config: AStarConfig::new(target, heuristic),
            heap: MinHeap::with_capacity(capacity),
        }
    }

    pub fn config(&self) -> &AStarConfig<H> {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut AStarConfig<H> {
        &mut self.config
    }
}

impl<T: FloatNumber, H: Heuristic<T> + Default> Default for AStar<T, H> {
    fn default() -> Self {
        Self {
            config: AStarConfig::default(),
            heap: MinHeap::new(),
        }
    }
}

impl<T: FloatNumber, H: Heuristic<T>> SsspAlgorithmInfo for AStar<T, H> {
    fn name(&self) -> &'static str {
        "A*"
    }

    fn supports_negative_weights(&self) -> bool {
        false
    }
}

impl<T, N, G, H> SsspAlgorithm<T, N, G> for AStar<T, H>
where
    T: FloatNumber,
    N: Dim,
    G: Graph<T>,
    H: Heuristic<T>,
    DefaultAllocator: Allocator<N>,
{
    fn run(&mut self, graph: &G, source: usize, buffers: &mut SsspBuffers<T, N>) -> SsspResult<T> {
        debug_assert!(source < graph.n(), "Source vertex out of bounds");

        let target = self.config.target().expect("A* requires a target vertex");

        init_sssp(buffers, source);
        self.heap.clear();

        // f(source) = g(source) + h(source) = 0 + h(source)
        let h_source = self.config.heuristic.estimate(source, target);
        self.heap.push(h_source, source);

        let mut iterations = 0usize;

        while let Some(entry) = self.heap.pop() {
            let u = entry.vertex;
            let f_u = entry.dist;
            let g_u = buffers.dist[u];

            // Lazy delete, skip if found better
            // f_u should equal g_u + h(u) if entry is current
            let h_u = self.config.heuristic.estimate(u, target);
            if self.config.lazy_deletion && f_u > g_u + h_u {
                continue;
            }

            if self.config.should_stop(u) {
                break;
            }

            iterations += 1;

            // Relax outgoing
            graph.for_each_out_edge(u, |v, w| {
                debug_assert!(w >= T::zero(), "A* requires non-negative weights");

                if let RelaxResult::Improved = relax_with(
                    buffers.dist.as_mut_slice(),
                    buffers.parent.as_mut_slice(),
                    u,
                    g_u,
                    v,
                    w,
                ) {
                    let h_v = self.config.heuristic.estimate(v, target);
                    let f_v = buffers.dist[v] + h_v;
                    self.heap.push(f_v, v); // Push with f(v) = g(v) + h(v)
                }
            });
        }

        finalize_sssp(buffers, iterations, false)
    }
}
