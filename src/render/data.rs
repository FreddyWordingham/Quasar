//! Output data.

use ndarray::Array2;
use palette::LinSrgba;
use std::path::Path;

use crate::parse::png;

/// Saveable output data.
pub struct Data {
    // Colour data.
    pub colour: Array2<LinSrgba>,
}

impl Data {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 2]) -> Self {
        let colour = Array2::from_elem(res, LinSrgba::new(0.0, 0.0, 0.0, 0.0));

        Self { colour }
    }

    /// Save the output, in it's current state, to the given output directory.
    #[inline]
    pub fn save(&self, output_dir: &Path) {
        png::save(
            self.colour.view(),
            &output_dir.join("colour").with_extension("png"),
        );
    }
}
