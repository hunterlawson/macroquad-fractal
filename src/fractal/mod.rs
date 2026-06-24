mod color;
pub use color::*;

mod mandelbrot;
pub use mandelbrot::*;

use crate::complex::*;

pub trait Fractal: Send + Sync {
    /// Run the fractal iteration function.
    /// Return None if the function does not diverge.
    /// Return Some(x) where x is the number of iterations before diverging
    fn iterate(&self, c: C64) -> IterationResult;
}

pub struct IterationResult {
    max_iterations: u32,
    iterations: Option<u32>,
    final_z: C64,
}
