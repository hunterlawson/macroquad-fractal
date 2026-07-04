use macroquad::{
    material::Material,
    math::{Vec2, vec2},
    miniquad::{UniformDesc, UniformType},
};
use rug::{Assign, Complex, Float};

use crate::{PRECISION, fractal::Fractal};

const MANDELBROT_FRAGMENT_SHADER: &'static str = include_str!("../../shaders/burning_ship.frag");
pub struct BurningShip {
    pub max_iter: u32,
}

impl Fractal for BurningShip {
    fn fragment_shader(&self) -> &'static str {
        MANDELBROT_FRAGMENT_SHADER
    }

    fn uniform_descs(&self) -> Vec<UniformDesc> {
        vec![UniformDesc::new("max_iter", UniformType::Int1)]
    }

    fn set_uniforms(&self, material: &Material) {
        material.set_uniform("max_iter", self.max_iter as i32);
    }

    fn orbit(&self, point: &Complex) -> Vec<Vec2> {
        let mut output = vec![];
        let c = point.clone();
        let mut z = point.clone();
        let mut norm_sqr = Float::with_val(PRECISION, 0.);

        // let mut iter = 0;
        for _ in 0..self.max_iter {
            // push the current z (starts at z(0) = the cursor point) before iterating
            output.push(vec2(z.real().to_f32(), z.imag().to_f32()));

            z.square_mut();
            z += &c;

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

    fn fractal_type(&self) -> super::FractalType {
        super::FractalType::BurningShip
    }
}
