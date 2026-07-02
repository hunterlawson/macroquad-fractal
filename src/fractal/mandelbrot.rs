use macroquad::{
    material::Material, math::{Vec2, vec2}, miniquad::{UniformDesc, UniformType},
};
use rug::{Assign, Complex, Float, ops::CompleteRound};

use crate::{PRECISION, complex::C64, fractal::Fractal};

const MANDELBROT_FRAGMENT_SHADER: &'static str = include_str!("../../shaders/mandelbrot.frag");
pub struct Mandelbrot {
    pub max_iter: u32,
}

impl Fractal for Mandelbrot {
    fn fragment_shader(&self) -> &'static str {
        MANDELBROT_FRAGMENT_SHADER
    }

    fn uniform_descs(&self) -> Vec<UniformDesc> {
        vec![UniformDesc::new("max_iter", UniformType::Int1)]
    }

    fn set_uniforms(&self, material: &Material) {
        material.set_uniform("max_iter", self.max_iter as i32);
    }

    fn orbit(&self, mut point: Complex) -> Vec<Vec2> {
        let mut output = vec![];
        let c = point.clone();
        let mut z = point;
        let mut norm_sqr = Float::with_val(PRECISION, 0.);

        let mut iter = 0;
        for _ in 0..self.max_iter {
            iter += 1;
            z.square_mut();
            z += &c;
            output.push(vec2(z.real().to_f32(), z.imag().to_f32()));

            norm_sqr.assign(z.norm_ref());

            if norm_sqr > 4. {
                break;
            }
        }

        output
    }

    fn set_max_iter(&mut self, max_iter: u32) {
        self.max_iter = max_iter;
    }

    fn max_iter(&self) -> u32 {
        self.max_iter
    }
}
