use crate::algorithms::{finalize_sssp, init_sssp, SsspAlgorithm, SsspAlgorithmInfo, SsspResult};
use crate::utils::{relax, FloatNumber, Graph, RelaxResult, SsspBuffers};
use nalgebra::{allocator::Allocator, DefaultAllocator, Dim};

use super::config::BellmanFordConfig;

#[derive(Debug)]
pub struct BellmanFord<T: FloatNumber> {
    config: BellmanFordConfig,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: FloatNumber> BellmanFord<T> {
    pub fn new() -> Self {
        Self {
            config: BellmanFordConfig::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_config(config: BellmanFordConfig) -> Self {
        Self {
            config,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn config(&self) -> &BellmanFordConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut BellmanFordConfig {
        &mut self.config
    }
}

impl<T: FloatNumber> Default for BellmanFord<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: FloatNumber> SsspAlgorithmInfo for BellmanFord<T> {
    fn name(&self) -> &'static str {
        "Bellman-Ford"
    }

    fn supports_negative_weights(&self) -> bool {
        true
    }
}

impl<T, N, G> SsspAlgorithm<T, N, G> for BellmanFord<T>
where
    T: FloatNumber,
    N: Dim,
    G: Graph<T>,
    DefaultAllocator: Allocator<N>,
{
    fn run(&mut self, graph: &G, source: usize, buffers: &mut SsspBuffers<T, N>) -> SsspResult<T> {
        debug_assert!(source < graph.n(), "Source vertex out of bounds");

        init_sssp(buffers, source);

        let n = graph.n();
        let mut iterations = 0usize;

        // Relax all edges |V| - 1 times
        for _ in 0..n.saturating_sub(1) {
            let mut any_improved = false;
            iterations += 1;

            for u in 0..n {
                if buffers.dist[u].is_infinite() {
                    continue;
                }

                graph.for_each_out_edge(u, |v, w| {
                    if let RelaxResult::Improved = relax(
                        buffers.dist.as_mut_slice(),
                        buffers.parent.as_mut_slice(),
                        u,
                        v,
                        w,
                    ) {
                        any_improved = true;
                    }
                });
            }

            if self.config.early_termination && !any_improved {
                break;
            }
        }

        // Negative cycle detection: one more iter
        let mut negative_cycle = false;
        for u in 0..n {
            if buffers.dist[u].is_infinite() {
                continue;
            }

            graph.for_each_out_edge(u, |v, w| {
                let new_dist = buffers.dist[u] + w;
                if new_dist < buffers.dist[v] {
                    negative_cycle = true;
                }
            });

            if negative_cycle {
                break;
            }
        }

        finalize_sssp(buffers, iterations, negative_cycle)
    }
}
