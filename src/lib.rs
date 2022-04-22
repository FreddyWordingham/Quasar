//! Library core.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#![allow(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::blanket_clippy_restriction_lints,
    clippy::default_numeric_fallback,
    clippy::else_if_without_else,
    clippy::exhaustive_structs,
    clippy::expect_used,
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::panic
)]

pub mod algebra;
pub mod geometry;
pub mod parse;
pub mod utility;
