mod julia;
mod mandelbrot;
mod mandelbrot_perturb;

use macroquad::{material::Material, math::Vec2, miniquad::UniformDesc};
use rug::Complex;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::{DEFAULT_MAX_ITER, PRECISION};

#[derive(EnumIter, Display, PartialEq, Clone, Copy, EnumString)]
pub enum FractalType {
    #[strum(serialize = "Mandelbrot Set")]
    Mandelbrot,
    #[strum(serialize = "Julia Set")]
    Julia,
    #[strum(serialize = "Mandelbrot Set Perturbation")]
    MandelbrotPerturb,
}

impl FractalType {
    pub fn make(&self) -> Box<dyn Fractal> {
        match *self {
            FractalType::Mandelbrot => Box::new(mandelbrot::Mandelbrot {
                max_iter: DEFAULT_MAX_ITER,
            }),
            FractalType::Julia => Box::new(julia::Julia {
                max_iter: DEFAULT_MAX_ITER,
                c: Complex::with_val(PRECISION, (-0.8, 0.156)),
            }),
            FractalType::MandelbrotPerturb => Box::new(mandelbrot_perturb::MandelbrotPerturb {
                max_iter: DEFAULT_MAX_ITER
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
    fn fragment_shader(&self) -> String;
    /// Get the uniform descriptions for the fragment shader
    fn uniform_descs(&self) -> Vec<UniformDesc>;
    /// Set the uniform values to the provided material
    ///
    /// Must set the uniform descriptions on the material first otherwise
    /// this will not work
    fn set_uniforms(&self, material: &Material, c: &Complex);
    /// Return the orbit for the given complex point
    fn orbit(&self, point: &Complex) -> Vec<Vec2>;
    /// Input the given complex value into the fractal
    ///
    /// Some fractals use an input/starting value (like the Julia set)
    fn input_parameter(&mut self, _point: &Complex) {}
    /// Set the maximum iteration value (for escape time fractals)
    fn set_max_iter(&mut self, max_iter: u32);
    /// Get the maximum iteration value (for escape time fractals)
    fn max_iter(&self) -> u32;
    /// Get the fractal type value for this fractal
    fn fractal_type(&self) -> FractalType;
}
