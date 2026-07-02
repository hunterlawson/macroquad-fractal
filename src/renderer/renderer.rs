use macroquad::{
    Error, color::WHITE, logging::debug, material::{Material, MaterialParams, gl_use_default_material, gl_use_material, load_material}, miniquad::ShaderSource, shapes::draw_rectangle,
};

use crate::{fractal::Fractal, renderer::view::View};

/// Global vertex shader
const VERT_SHADER: &'static str = include_str!("../../shaders/fractal.vert");

/// Perturbation fractal renderer
pub struct Renderer {
    fractal: Box<dyn Fractal>,
    material: Material,
}

impl Renderer {
    pub fn new(fractal: Box<dyn Fractal>) -> Result<Self, Error> {
        Ok(Self {
            material: Self::build_fractal_material(fractal.as_ref())?,
            fractal,
        })
    }

    fn build_fractal_material(fractal: &dyn Fractal) -> Result<Material, Error> {
        let mut uniforms = View::uniform_descs();
        uniforms.extend(fractal.uniform_descs());

        debug!("Creating fractal material");
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
    }

    pub fn fractal(&self) -> &dyn Fractal {
        self.fractal.as_ref()
    }

    /// Render the fractal to the given view
    pub fn render(&self, view: &View) {
        gl_use_material(&self.material);

        // Bind
        gl_use_material(&self.material);

        // Set uniforms per-fractal, some fractals will have different uniforms
        self.fractal.set_uniforms(&self.material);

        // Set universal uniforms used by all fractals
        view.set_uniforms(&self.material);

        // Draw full size quad to get the values to render the fractal
        draw_rectangle(
            view.pos().x,
            view.pos().y,
            view.dim().x,
            view.dim().y,
            WHITE,
        );

        // Unbind
        gl_use_default_material();
    }
}
