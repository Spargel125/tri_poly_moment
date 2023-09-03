//! [![GitHub](https://img.shields.io/github/license/Spargel125/tri_poly_moment?style=for-the-badge&logo=github)](https://github.com/H1rono/tri_poly_moment/blob/main/LICENSE)
//! [![crate](https://img.shields.io/crates/v/tri_poly_moment.svg?style=for-the-badge&logo=rust)](https://crates.io/crates/tri_poly_moment)
//! 
//! Calculate mixed-trigonometric-polynomial moment of random variables
//! 
//! reference method: [Moment-Based Exact Uncertainty Propagation Through Nonlinear Stochastic Autonomous Systems](https://arxiv.org/abs/2101.12490)
//! 
//! The distributions of the random variables are normal, uniform, and exponential.
//! ```toml
//! [dependencies]
//! tri_poly_moment = "0.1.0"
//! ```
//! 
//! # Example
//! if calculate moment of Gaussian distribusions
//!  
//! ```
//! use tri_poly_moment::Gaussian;
//! let arg = Gaussian::new(-2.1, 0.4 * 0.4);
//! arg.x(); //if calc E[x], arg.x() = -2.1
//! ```
//! 

mod uniform;
mod gaussian;
mod expotential;

pub use uniform::Uniform;
pub use gaussian::Gaussian;
pub use expotential::Expotential;