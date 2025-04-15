//! # rita
//!
//! An implementation of 2D and 3D weighted delaunay triangulation via incremental algorithms.
#![forbid(unsafe_code)]
#![deny(unused, clippy::incompatible_msrv)]
#![warn(clippy::all, clippy::missing_const_for_fn)]

pub use node::VertexNode;
pub use tetrahedralization::Tetrahedralization;
pub use triangulation::Triangulation;

pub mod node;
mod tetds;
pub mod tetrahedralization;
pub mod triangulation;
mod trids;
mod utils;

#[cfg(test)]
mod test_utils {
    use std::ops::RangeInclusive;

    use rand::{distr::Uniform, prelude::Distribution};
    use rand_distr::Normal;

    /// Get `n` vertices, in `range` or `-0.5..=0.5`
    pub fn sample_vertices_2d(n: usize, range: Option<RangeInclusive<f64>>) -> Vec<[f64; 2]> {
        let mut rng = rand::rng();
        let range = range.unwrap_or(-0.5..=0.5);
        let uniform = Uniform::try_from(range).expect("Expected a valid range");

        let mut vertices: Vec<[f64; 2]> = Vec::with_capacity(n);
        for _ in 0..n {
            let x = uniform.sample(&mut rng);
            let y = uniform.sample(&mut rng);
            vertices.push([x, y]);
        }

        vertices
    }

    /// Get `n` vertices, in `range` or `-0.5..=0.5`
    pub fn sample_vertices_3d(n: usize, range: Option<RangeInclusive<f64>>) -> Vec<[f64; 3]> {
        let mut rng = rand::rng();
        let range = range.unwrap_or(-0.5..=0.5);
        let uniform = Uniform::try_from(range).expect("Expected a valid range");

        let mut vertices: Vec<[f64; 3]> = Vec::with_capacity(n);
        for _ in 0..n {
            let x = uniform.sample(&mut rng);
            let y = uniform.sample(&mut rng);
            let z = uniform.sample(&mut rng);

            vertices.push([x, y, z]);
        }

        vertices
    }

    pub fn sample_weights(n: usize, params: Option<(f64, f64)>) -> Vec<f64> {
        let mut rng = rand::rng();
        let (mean, std_dev) = params.unwrap_or((0.0, 0.005));
        let normal = Normal::new(mean, std_dev).unwrap();

        let mut weights: Vec<f64> = Vec::with_capacity(n);
        for _ in 0..n {
            let w: f64 = normal.sample(&mut rng);
            weights.push(w);
        }

        weights
    }
}
