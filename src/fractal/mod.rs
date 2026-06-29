mod julia;
mod mandelbrot;

pub use julia::*;
use macroquad::{material::Material, math::vec2, miniquad::UniformDesc};
pub use mandelbrot::*;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::{DEFAULT_MAX_ITER, complex::C64};

#[derive(EnumIter, Display, PartialEq, Clone, Copy, EnumString)]
pub enum FractalType {
    #[strum(serialize = "Mandelbrot Set")]
    Mandelbrot,
    #[strum(serialize = "Julia Set")]
    Julia,
}

impl FractalType {
    pub fn make(&self) -> Box<dyn Fractal> {
        match *self {
            FractalType::Mandelbrot => Box::new(Mandelbrot {
                max_iter: DEFAULT_MAX_ITER,
            }),
            FractalType::Julia => Box::new(Julia {
                max_iter: DEFAULT_MAX_ITER,
                c: vec2(-0.5125, 0.5213), // Nice looking default starting point
            }),
        }
    }

    pub fn next(self) -> Self {
        FractalType::iter()
            .cycle()
            .skip_while(|&t| t != self)
            .nth(1)
            .unwrap()
    }
}

/// Describes a fractal that can be rendered at runtime
pub trait Fractal {
    /// Get the fragment shader for this fractal
    fn fragment_shader(&self) -> &'static str;
    /// Get the uniform descriptions for the fragment shader
    fn uniform_descs(&self) -> Vec<UniformDesc>;
    /// Set the uniform values to the provided material
    ///
    /// Must set the uniform descriptions on the material first otherwise
    /// this will not work
    fn set_uniforms(&self, material: &Material);
    /// Return the orbit for the given complex point
    fn orbit(&self, point: C64) -> Vec<C64>;
    /// Input the given complex value into the fractal
    ///
    /// Some fractals use an input/starting value (like the Julia set)
    fn input_parameter(&mut self, _point: C64) {}
    /// Set the maximum iteration value (for escape time fractals)
    fn set_max_iter(&mut self, max_iter: u32);
    /// Get the maximum iteration value (for escape time fractals)
    fn max_iter(&self) -> u32;
}
