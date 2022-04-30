//! Runtime settings.

/// General settings structure.
pub struct Settings {
    /// Optional limit on number of threads to use.
    num_threads: Option<usize>,
    /// Number of tracers to simulate in each thread block.
    block_size: usize,
    /// Bump distance (m).
    bump_dist: f64,
    /// Loop limit.
    loop_limit: u64,
    /// Minimum statistical weight to continue simulating.
    min_weight: f64,
}
