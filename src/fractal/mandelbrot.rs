use macroquad::{
    material::Material,
    miniquad::{UniformDesc, UniformType},
};

use crate::{complex::C64, fractal::Fractal};

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

    fn orbit(&self, point: C64) -> Vec<C64> {
        let mut output = vec![];
        let mut z = point;
        for _ in 0..self.max_iter {
            output.push(z);
            if z.len_squared() > 4. {
                break;
            }
            z = z * z + point;
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
