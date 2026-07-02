use macroquad::{
    Error, camera::{Camera2D, set_camera, set_default_camera}, logging::debug, material::{Material, MaterialParams, gl_use_default_material, gl_use_material, load_material}, math::Rect, miniquad::ShaderSource, shapes::draw_rectangle, texture::{DrawTextureParams, FilterMode, RenderTarget, draw_texture_ex, render_target}, window::clear_background,
};

use crate::{fractal::Fractal, renderer::view::View};

/// Global vertex shader
const VERT_SHADER: &'static str = include_str!("../../shaders/fractal.vert");

/// Perturbation fractal renderer
pub struct Renderer {
    fractal: Box<dyn Fractal>,
    material: Material,
    target: Option<RenderTarget>,
    cached_view: Option<View>,
    dirty_render: bool,
}

impl Renderer {
    pub fn new(fractal: Box<dyn Fractal>) -> Result<Self, Error> {
        Ok(Self {
            material: Self::build_fractal_material(fractal.as_ref())?,
            fractal,
            target: None,
            cached_view: None,
            dirty_render: true,
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

    /// Forced render of the fractal to the given view
    pub fn render(&mut self, view: &View) {
        // Cache the view
        self.cached_view = Some(view.clone());
        let pos = view.pos();
        let dim = view.dim();

        // (Re)create the render target only when it's missing or the size changed.
        let size_changed = match &self.target {
            Some(target) => {
                let size = target.texture.size();
                size.x as u32 != dim.x as u32 || size.y as u32 != dim.y as u32
            }
            None => true,
        };
        if size_changed {
            let target = render_target(dim.x as u32, dim.y as u32);
            target.texture.set_filter(FilterMode::Nearest);
            self.target = Some(target);
        }
        // Clone is a cheap handle copy.
        let target = self
            .target
            .clone()
            .expect("The render target was just created");

        // Create camera and bind render target
        let mut camera = Camera2D::from_display_rect(Rect::new(pos.x, pos.y, dim.x, dim.y));
        camera.render_target = Some(target.clone());
        set_camera(&camera);
        clear_background(macroquad::color::BLACK);

        // Bind
        gl_use_material(&self.material);

        // Set uniforms per-fractal, some fractals will have different uniforms
        self.fractal.set_uniforms(&self.material);

        // Set universal uniforms used by all fractals
        view.set_uniforms(&self.material);

        // Draw full size quad to get the values to render the fractal
        draw_rectangle(pos.x, pos.y, dim.x, dim.y, macroquad::color::WHITE);

        // Unbind
        gl_use_default_material();

        // Reset camera
        set_default_camera();
    }

    /// Cached render of the fractal to the given view
    ///
    /// Only re-renders the image if the view or fractal changed.
    /// Returns `true` if a re-render happened
    pub fn cached_render(&mut self, view: &View) -> bool {
        if self.cached_view.as_ref() != Some(view) {
            self.dirty_render = true;
        }
        if self.dirty_render {
            self.render(view);
            self.dirty_render = false;
            return true;
        }

        false
    }

    /// Draw the fractal to the screen from the rendered texture and view
    pub fn draw(&self) {
        let Some(target) = &self.target else {
            return;
        };
        let Some(view) = &self.cached_view else {
            return;
        };

        draw_texture_ex(
            &target.texture,
            view.pos().x,
            view.pos().y,
            macroquad::color::WHITE,
            DrawTextureParams {
                dest_size: Some(view.dim()),
                flip_y: true,
                ..Default::default()
            },
        );
    }
}
