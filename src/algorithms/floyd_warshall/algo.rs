use crate::algorithms::{ApspAlgorithm, ApspAlgorithmInfo, ApspResult};
use crate::utils::{ApspBuffers, FloatNumber, Graph};
use rayon::prelude::*;

use super::config::FloydWarshallConfig;

#[derive(Debug)]
pub struct FloydWarshall<T: FloatNumber> {
    config: FloydWarshallConfig,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: FloatNumber> FloydWarshall<T> {
    pub fn new() -> Self {
        Self {
            config: FloydWarshallConfig::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_config(config: FloydWarshallConfig) -> Self {
        Self {
            config,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn config(&self) -> &FloydWarshallConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut FloydWarshallConfig {
        &mut self.config
    }
}

impl<T: FloatNumber> Default for FloydWarshall<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: FloatNumber> ApspAlgorithmInfo for FloydWarshall<T> {
    fn name(&self) -> &'static str {
        "Floyd-Warshall"
    }

    fn supports_negative_weights(&self) -> bool {
        true
    }
}

impl<T, G> ApspAlgorithm<T, G> for FloydWarshall<T>
where
    T: FloatNumber,
    G: Graph<T> + Sync,
{
    fn run(&mut self, graph: &G, buffers: &mut ApspBuffers<T>) -> ApspResult<T> {
        let n = graph.n();
        debug_assert!(buffers.n == n, "Buffer size mismatch");

        init_from_graph(graph, buffers);
        for k in 0..n {
            update_for_k(buffers, k);
        }

        let negative_cycle = if self.config.detect_negative_cycle {
            buffers.has_negative_cycle()
        } else {
            false
        };

        finalize_apsp(buffers, n, negative_cycle)
    }
}

/// Init dist matrix from graph edges.
fn init_from_graph<T, G>(graph: &G, buffers: &mut ApspBuffers<T>)
where
    T: FloatNumber,
    G: Graph<T>,
{
    let n = graph.n();
    buffers.reset();

    for u in 0..n {
        graph.for_each_out_edge(u, |v, w| {
            let curr = buffers.get(u, v);

            // Take min in case of parallel edges
            if w < curr {
                buffers.set(u, v, w);
                buffers.set_next(u, v, v);
            }
        });
    }

    // Self-loops: next[i][i] = i for path reconstruction
    for i in 0..n {
        buffers.set_next(i, i, i);
    }
}

/// Update all (i,j) pairs for intermed vertex k.
fn update_for_k<T: FloatNumber>(buffers: &mut ApspBuffers<T>, k: usize) {
    let n = buffers.n;
    let row_k: Vec<T> = (0..n).map(|j| buffers.get(k, j)).collect();
    let col_k: Vec<T> = (0..n).map(|i| buffers.get(i, k)).collect();

    // Collect all (i, j) improvements
    let updates: Vec<(usize, usize, T, usize)> = (0..n)
        .into_par_iter()
        .flat_map(|i| {
            let d_ik = col_k[i];
            if d_ik.is_infinite() {
                return Vec::new();
            }

            let mut local_updates = Vec::new();
            for (j, &d_kj) in row_k.iter().enumerate() {
                if d_kj.is_infinite() {
                    continue;
                }

                let new_dist = d_ik + d_kj;
                let curr_dist = buffers.get(i, j);
                if new_dist < curr_dist {
                    let next_v = buffers.get_next(i, k);
                    local_updates.push((i, j, new_dist, next_v));
                }
            }
            local_updates
        })
        .collect();

    // Apply updates
    for (i, j, dist, next_v) in updates {
        buffers.set(i, j, dist);
        buffers.set_next(i, j, next_v);
    }
}

fn finalize_apsp<T: FloatNumber>(
    buffers: &ApspBuffers<T>,
    n: usize,
    negative_cycle: bool,
) -> ApspResult<T> {
    let mut pairs_reached = 0usize;

    // Count reachable pairs
    for i in 0..n {
        for j in 0..n {
            if !buffers.get(i, j).is_infinite() {
                pairs_reached += 1;
            }
        }
    }

    ApspResult::new(n, negative_cycle, pairs_reached)
}
