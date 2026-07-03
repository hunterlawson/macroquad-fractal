use macroquad::{
    material::Material,
    math::{Vec2, vec2},
    miniquad::{UniformDesc, UniformType},
};
use rug::{Assign, Complex, Float};

use crate::{PRECISION, fractal::Fractal};

const JULIA_FRAGMENT_SHADER: &'static str = include_str!("../../shaders/julia.frag");

pub struct Julia {
    pub max_iter: u32,
    /// Starting c value
    pub c: Complex,
}

impl Fractal for Julia {
    fn fragment_shader(&self) -> &'static str {
        JULIA_FRAGMENT_SHADER
    }

    fn uniform_descs(&self) -> Vec<UniformDesc> {
        vec![
            UniformDesc::new("max_iter", UniformType::Int1),
            UniformDesc::new("c", UniformType::Float2),
        ]
    }

    fn set_uniforms(&self, material: &Material) {
        let c_f32 = (self.c.real().to_f32(), self.c.imag().to_f32());
        material.set_uniform("max_iter", self.max_iter as i32);
        material.set_uniform("c", c_f32);
    }

    fn orbit(&self, point: &Complex) -> Vec<Vec2> {
        let mut output = vec![];
        let mut z = point.clone();
        let mut norm_sqr = Float::with_val(PRECISION, 0.);

        // let mut iter = 0;
        for _ in 0..self.max_iter {
            // push the current z (starts at z(0) = the cursor point) before iterating
            output.push(vec2(z.real().to_f32(), z.imag().to_f32()));

            z.square_mut();
            z += &self.c;

            norm_sqr.assign(z.norm_ref());

            if norm_sqr > 4. {
                break;
            }
        }

        output
    }

    fn input_parameter(&mut self, c: &Complex) {
        self.c = c.clone();
    }

    fn set_max_iter(&mut self, max_iter: u32) {
        self.max_iter = max_iter;
    }

    fn max_iter(&self) -> u32 {
        self.max_iter
    }

    fn fractal_type(&self) -> super::FractalType {
        super::FractalType::Julia
    }
}
