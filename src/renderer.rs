use macroquad::{
    camera::{Camera2D, set_camera, set_default_camera},
    color::{BLACK, WHITE},
    material::{Material, MaterialParams, gl_use_default_material, gl_use_material, load_material},
    math::Rect,
    miniquad::ShaderSource,
    shapes::draw_rectangle,
    texture::{DrawTextureParams, FilterMode, RenderTarget, draw_texture_ex, render_target},
    window::clear_background,
};

use crate::{complex::C64, fractal::Fractal, view::ComplexView};

const VERT_SHADER: &'static str = include_str!("../shaders/fractal.vert");

/// Fractal shader renderer
pub struct Renderer {
    fractal: Box<dyn Fractal>,
    material: Material,
    target: Option<RenderTarget>,
    cached_view: Option<ComplexView>,
    render_dirty: bool,
}

impl Renderer {
    pub fn new(fractal: Box<dyn Fractal>) -> Self {
        Self {
            material: Self::build_material(&*fractal),
            fractal,
            target: None,
            cached_view: None,
            render_dirty: true,
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
        self.render_dirty = true;
    }

    /// Get a reference to the fractal being rendered
    pub fn fractal(&self) -> &dyn Fractal {
        self.fractal.as_ref()
    }

    /// Input a point into the underlying fractal
    ///
    /// Not all fractals actually use any input points
    pub fn set_fractal_input_parameter(&mut self, point: C64) {
        self.fractal.input_parameter(point);
        self.render_dirty = true;
    }

    /// Set the underlying escape time fractal's maximum iteration value
    pub fn set_fractal_max_iter(&mut self, max_iter: u32) {
        self.fractal.set_max_iter(max_iter);
        self.render_dirty = true;
    }

    /// Get the underlying escape time fractal's maximum iteration value
    pub fn fractal_max_iter(&self) -> u32 {
        self.fractal.max_iter()
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
            view.screen_pixel_position.x,
            view.screen_pixel_position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(view.pixel_dimensions),
                flip_y: true,
                ..Default::default()
            },
        );
    }

    /// Forced render the fractal with the given view
    pub fn render(&mut self, view: &ComplexView) {
        let dim = view.pixel_dimensions;
        let pos = view.screen_pixel_position;

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
        let target = self.target.clone().unwrap();

        // Cache the view
        self.cached_view = Some(view.clone());

        // Create camera and bind render target
        let mut camera = Camera2D::from_display_rect(Rect::new(pos.x, pos.y, dim.x, dim.y));
        camera.render_target = Some(target.clone());
        set_camera(&camera);
        clear_background(BLACK);

        // Bind
        gl_use_material(&self.material);

        // Set uniforms per-fractal, some fractals will have different uniforms
        self.fractal.set_uniforms(&self.material);

        // Set universal uniforms used by all fractals
        view.set_uniforms(&self.material);

        // Draw full size quad to get the values to render the fractal
        draw_rectangle(
            view.screen_pixel_position.x,
            view.screen_pixel_position.y,
            view.pixel_dimensions.x,
            view.pixel_dimensions.y,
            WHITE,
        );

        // Unbind
        gl_use_default_material();

        // Reset camera
        set_default_camera();
    }

    /// Render the fractal with the given view only if there are changes that need rendering:
    ///
    /// - changes to the underlying fractal
    /// - changes to the renderer
    /// - changes to the cached view
    /// 
    /// Returns `true` if it triggered a re-render of the fractal image
    pub fn render_cached(&mut self, view: &ComplexView) -> bool {
        if self.cached_view.as_ref() != Some(view) {
            self.render_dirty = true;
        }
        if self.render_dirty {
            self.render(view);
            self.render_dirty = false;
            return true;
        }

        false
    }
}
