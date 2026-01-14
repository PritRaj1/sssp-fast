use crate::algorithms::{HasSsspConfig, SsspConfig};

#[derive(Clone, Debug)]
pub struct DijkstraConfig {
    base: SsspConfig,
    pub lazy_deletion: bool, // Skip stale entries of heap
}

impl Default for DijkstraConfig {
    fn default() -> Self {
        Self {
            base: SsspConfig::default(),
            lazy_deletion: true,
        }
    }
}

impl DijkstraConfig {
    pub fn with_target(target: usize) -> Self {
        Self {
            base: SsspConfig::with_target(target),
            lazy_deletion: true,
        }
    }

    pub fn without_lazy_deletion(mut self) -> Self {
        self.lazy_deletion = false;
        self
    }
}

impl HasSsspConfig for DijkstraConfig {
    fn sssp_config(&self) -> &SsspConfig {
        &self.base
    }
}
