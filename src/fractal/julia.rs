use macroquad::{
    material::Material,
    math::Vec2,
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
            if z.len_squared() > 4. {
                break;
            }
            output.push(z);
            z = z * z + self.c.into();
        }

        output
    }
}
