use macroquad::{
    material::Material,
    miniquad::{UniformDesc, UniformType},
};

use crate::fractal::Fractal;

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
}
