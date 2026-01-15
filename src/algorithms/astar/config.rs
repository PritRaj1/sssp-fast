use crate::algorithms::{HasSsspConfig, SsspConfig};
use crate::utils::FloatNumber;

/// Heuristic func for A*.
/// Must be admissible (never overestimate) for optimality.
pub trait Heuristic<T: FloatNumber>: Clone {
    fn estimate(&self, vertex: usize, target: usize) -> T;
}

/// Zero heuristic, (maps A* to Dijkstra).
#[derive(Clone, Debug, Default)]
pub struct ZeroHeuristic;

impl<T: FloatNumber> Heuristic<T> for ZeroHeuristic {
    #[inline]
    fn estimate(&self, _vertex: usize, _target: usize) -> T {
        T::zero()
    }
}

/// Function pointer heuristic wrapper.
#[derive(Clone)]
pub struct FnHeuristic<T: FloatNumber> {
    f: fn(usize, usize) -> T,
}

impl<T: FloatNumber> FnHeuristic<T> {
    pub fn new(f: fn(usize, usize) -> T) -> Self {
        Self { f }
    }
}

impl<T: FloatNumber> Heuristic<T> for FnHeuristic<T> {
    #[inline]
    fn estimate(&self, vertex: usize, target: usize) -> T {
        (self.f)(vertex, target)
    }
}

#[derive(Clone, Debug)]
pub struct AStarConfig<H> {
    base: SsspConfig,
    pub heuristic: H,
    pub lazy_deletion: bool,
}

impl<H: Default> Default for AStarConfig<H> {
    fn default() -> Self {
        Self {
            base: SsspConfig::default(),
            heuristic: H::default(),
            lazy_deletion: true,
        }
    }
}

impl<H> AStarConfig<H> {
    pub fn new(target: usize, heuristic: H) -> Self {
        Self {
            base: SsspConfig::with_target(target),
            heuristic,
            lazy_deletion: true,
        }
    }

    pub fn without_lazy_deletion(mut self) -> Self {
        self.lazy_deletion = false;
        self
    }

    pub fn target(&self) -> Option<usize> {
        self.base.early_stop
    }
}

impl<H> HasSsspConfig for AStarConfig<H> {
    fn sssp_config(&self) -> &SsspConfig {
        &self.base
    }
}
