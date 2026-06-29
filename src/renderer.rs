use macroquad::{
    color::WHITE,
    material::{Material, MaterialParams, gl_use_default_material, gl_use_material, load_material},
    miniquad::ShaderSource,
    shapes::draw_rectangle,
};

use crate::{fractal::Fractal, view::ComplexView};

const VERT_SHADER: &'static str = include_str!("../shaders/fractal.vert");

/// Fractal shader renderer
pub struct Renderer {
    fractal: Box<dyn Fractal>,
    material: Material,
}

impl Renderer {
    pub fn new(fractal: Box<dyn Fractal>) -> Self {
        Self {
            material: Self::build_material(&*fractal),
            fractal,
        }
    }

    fn build_material(fractal: &dyn Fractal) -> Material {
        let mut uniforms = ComplexView::uniform_descs();
        uniforms.extend(fractal.uniform_descs());

        load_material(
            ShaderSource::Glsl {
                vertex: VERT_SHADER,
                fragment: fractal.fragment_shader(),
            },
            MaterialParams {
                uniforms,
                ..Default::default()
            },
        )
        .unwrap()
    }

    /// Change the fractal and rebuild the underlying shader material
    pub fn set_fractal(&mut self, fractal: Box<dyn Fractal>) {
        self.material = Self::build_material(&*fractal);
        self.fractal = fractal;
    }

    /// Get a reference to the fractal being rendered
    pub fn fractal(&self) -> &dyn Fractal {
        self.fractal.as_ref()
    }

    /// Render the fractal to the screen using the shader
    pub fn render(&self, view: &ComplexView) {
        // bind -> set uniforms -> draw -> unbind
        // bind
        gl_use_material(&self.material);
        // set uniforms per-fractal, some fractals will have different uniforms
        self.fractal.set_uniforms(&self.material);
        // set colorer uniforms
        // set universal uniforms used by all fractals
        view.set_uniforms(&self.material);
        // draw full size quad to get the values to render the fractal
        draw_rectangle(
            view.screen_pixel_position.x,
            view.screen_pixel_position.y,
            view.pixel_dimensions.x,
            view.pixel_dimensions.y,
            WHITE,
        );
        // unbind
        gl_use_default_material();
    }
}
