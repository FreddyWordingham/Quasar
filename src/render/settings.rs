//! Runtime settings.

use serde::Deserialize;

/// General settings structure.
#[derive(Clone, Deserialize)]
pub struct Settings {
    /// Optional limit on number of threads to use.
    pub num_threads: Option<usize>,
    /// Number of tracers to simulate in each thread block.
    pub block_size: usize,
    /// Bump distance (m).
    pub bump_dist: f64,
    /// Loop limit.
    pub loop_limit: u64,
    /// Minimum statistical weight to continue simulating.
    pub min_weight: f64,
}
