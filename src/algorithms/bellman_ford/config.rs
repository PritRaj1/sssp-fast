use crate::algorithms::{HasSsspConfig, SsspConfig};

#[derive(Clone, Debug)]
pub struct BellmanFordConfig {
    base: SsspConfig,
    pub early_termination: bool, // Stop early if no relaxations occur in iter
}

impl Default for BellmanFordConfig {
    fn default() -> Self {
        Self {
            base: SsspConfig::default(),
            early_termination: true,
        }
    }
}

impl BellmanFordConfig {
    pub fn with_target(target: usize) -> Self {
        Self {
            base: SsspConfig::with_target(target),
            early_termination: true,
        }
    }

    pub fn without_early_termination(mut self) -> Self {
        self.early_termination = false;
        self
    }
}

impl HasSsspConfig for BellmanFordConfig {
    fn sssp_config(&self) -> &SsspConfig {
        &self.base
    }
}
