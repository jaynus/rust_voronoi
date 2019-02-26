#![deny(missing_docs,
        missing_debug_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

//! A Rust implementation of Fortune's Linesweep algorithm for computing Voronoi diagrams.

#[macro_use]
extern crate log;

#[cfg(feature = "serde_support")]
extern crate serde;

#[cfg(feature = "serde_support")]
#[macro_use]
extern crate serde_derive;

mod geometry;
mod point;
mod dcel;
mod beachline;
mod event;
mod voronoi;
mod lloyd;

pub use crate::voronoi::voronoi;
pub use crate::point::Point;
pub use crate::dcel::{DCEL, make_line_segments, make_polygons};
pub use crate::lloyd::{lloyd_relaxation, polygon_centroid};
