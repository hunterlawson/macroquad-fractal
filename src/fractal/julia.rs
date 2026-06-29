use macroquad::{
    material::Material,
    math::{Vec2, vec2},
    miniquad::{UniformDesc, UniformType},
};

use crate::{complex::C64, fractal::Fractal};

const JULIA_FRAGMENT_SHADER: &'static str = include_str!("../../shaders/julia.frag");

pub struct Julia {
    pub max_iter: u32,
    /// Starting c value
    pub c: Vec2,
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
        material.set_uniform("max_iter", self.max_iter as i32);
        material.set_uniform("c", self.c);
    }

    fn orbit(&self, point: C64) -> Vec<C64> {
        let mut output = vec![];
        let mut z = point;
        for _ in 0..self.max_iter {
            output.push(z);
            if z.len_squared() > 4. {
                break;
            }
            z = z * z + self.c.into();
        }

        output
    }

    fn input_parameter(&mut self, c: C64) {
        self.c = vec2(c.0 as f32, c.1 as f32);
    }

    fn set_max_iter(&mut self, max_iter: u32) {
        self.max_iter = max_iter;
    }

    fn max_iter(&self) -> u32 {
        self.max_iter
    }
}
