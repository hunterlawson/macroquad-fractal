use std::fmt::Display;

use line_clipping::{LineSegment, Point, Window, cohen_sutherland::clip_line};
use macroquad::{
    material::Material,
    math::{Vec2, vec2},
    miniquad::{UniformDesc, UniformType},
};
use rug::{Assign, Complex, ops::CompleteRound};

use crate::{PRECISION};

#[derive(Debug, PartialEq, Clone)]
struct ViewParams {
    pos: Vec2,
    dim: Vec2,
    c: Complex,
    re_range: Vec2,
}

/// View of the complex plane mapped onto the screen
#[derive(Debug, PartialEq, Clone)]
pub struct View {
    /// Position of the view on the screen (in pixels)
    pos: Vec2,
    /// Dimensions of the view on the screen (in pixels)
    dim: Vec2,
    /// Focus point (center) of the view. Calculated with arbitrary precision.
    c: Complex,
    /// Zoom factor of the view
    zoom: f64,
    /// Scale of the view - how wide is it (re axis)
    viewport_width: f64,
    /// Scale of the view - how tall is it (im axis)
    viewport_height: f64,
    /// Params used to initialize this view
    initial_params: ViewParams,
}

impl View {
    pub fn new(pos: Vec2, dim: Vec2, c: Complex, re_range: Vec2) -> Self {
        let initial_params = ViewParams {
            pos,
            dim,
            c: c.clone(),
            re_range,
        };
        let viewport = Self::get_viewport_width_height(&initial_params);

        Self {
            pos,
            dim,
            c,
            zoom: 1.,
            viewport_width: viewport.0,
            viewport_height: viewport.1,
            initial_params: initial_params,
        }
    }

    fn get_viewport_width_height(params: &ViewParams) -> (f64, f64) {
        let width = (params.re_range.y - params.re_range.x) as f64;
        let i_ratio = (params.dim.y / params.dim.x) as f64;

        (width, width * i_ratio)
    }

    pub fn uniform_descs() -> Vec<UniformDesc> {
        vec![
            UniformDesc::new("base_re", UniformType::Float1),
            UniformDesc::new("scale_re", UniformType::Float1),
            UniformDesc::new("base_im", UniformType::Float1),
            UniformDesc::new("scale_im", UniformType::Float1),
            UniformDesc::new("pixel_dimensions", UniformType::Float2),
            // perturbation
            UniformDesc::new("display_dimensions", UniformType::Float2),
            UniformDesc::new("viewport_dimensions", UniformType::Float2),
        ]
    }

    pub fn set_uniforms(&self, material: &Material) {
        let base_re = self.base_real() as f32;
        let base_im = self.base_imag() as f32;
        material.set_uniform("base_re", base_re);
        material.set_uniform("scale_re", self.viewport_width as f32);
        material.set_uniform("base_im", base_im);
        material.set_uniform("scale_im", (-self.viewport_height) as f32);
        material.set_uniform("pixel_dimensions", self.dim);
        // perturbation
        material.set_uniform("display_dimensions", self.dim);
        material.set_uniform(
            "viewport_dimensions",
            (self.viewport_width as f32, self.viewport_height as f32),
        );
    }

    /// Get the base real value as f64
    fn base_real(&self) -> f64 {
        (self.c.real() - (self.viewport_width / 2.))
            .complete(PRECISION)
            .to_f64()
    }

    /// Get the base imaginary value as f64
    fn base_imag(&self) -> f64 {
        (self.c.imag() + (self.viewport_height / 2.))
            .complete(PRECISION)
            .to_f64()
    }

    /// Get the screen_position of the renderer (in pixels)
    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    /// Get the dimensions of the renderer (in pixels)
    pub fn dim(&self) -> Vec2 {
        self.dim
    }

    /// Get a reference to the focus point of the renderer (c)
    pub fn c(&self) -> &Complex {
        &self.c
    }

    /// Return whether or not the given screen-space pixel coordinate is inside this view
    pub fn in_view(&self, pos: &Vec2) -> bool {
        (pos.x >= self.pos.x && pos.x <= self.pos.x + self.dim.x)
            && (pos.y >= self.pos.y && pos.y <= self.pos.y + self.dim.y)
    }

    /// Get a complex value from the view from the given screen position
    ///
    /// Returns `None` if the provided position is outside the view
    pub fn screen_to_view(&self, pos: &Vec2) -> Option<Complex> {
        // check that the position is within bounds of the view
        if !self.in_view(&pos) {
            return None;
        }

        // Convert from screen-space pixel coordinates to view-space
        let real_scale = (pos.x - self.pos.x) as f64 / self.dim.x as f64;
        let imag_scale = (pos.y - self.pos.y) as f64 / self.dim.y as f64;

        // Get the value via an offset from C
        let mut output_val = self.c.clone();
        // Offset to the top left of the viewport
        *output_val.mut_real() -= self.viewport_width / 2.;
        *output_val.mut_imag() += self.viewport_height / 2.;
        // Add the total offset
        *output_val.mut_real() += self.viewport_width * real_scale;
        *output_val.mut_imag() -= self.viewport_height * imag_scale;

        Some(output_val)
    }

    /// Convert from the view's complex space to screen-space pixel coordinates
    pub fn view_to_screen(&self, c: &Vec2) -> Vec2 {
        // normalize the complex number
        let base_real = self.base_real();
        let base_imag = self.base_imag();
        let re_norm = (c.x as f64 - base_real) / self.viewport_width;
        let im_norm = (base_imag - c.y as f64) / self.viewport_height;

        // scale to pixels
        let mut pix_x = re_norm * self.dim.x as f64;
        let mut pix_y = im_norm * self.dim.y as f64;

        // offset to the fractal position
        pix_x += self.pos.x as f64;
        pix_y += self.pos.y as f64;

        vec2(pix_x as f32, pix_y as f32)
    }

    /// Clip a line to fall inside the view
    pub fn clip_line(&self, line: (Vec2, Vec2)) -> Option<(Vec2, Vec2)> {
        let window = Window::new(
            self.pos.x as f64,
            self.pos.x as f64 + self.dim.x as f64,
            self.pos.y as f64,
            self.pos.y as f64 + self.dim.y as f64,
        );
        let line = LineSegment::new(
            Point::new(line.0.x as f64, line.0.y as f64),
            Point::new(line.1.x as f64, line.1.y as f64),
        );

        clip_line(line, window).map(|l| {
            let v1 = vec2(l.p1.x as f32, l.p1.y as f32);
            let v2 = vec2(l.p2.x as f32, l.p2.y as f32);
            (v1, v2)
        })
    }

    /// Apply a zoom factor
    ///
    /// zoom *= factor
    pub fn zoom(&mut self, factor: f64) {
        self.zoom *= factor;
        self.viewport_width /= factor;
        self.viewport_height /= factor;
    }

    /// Apply an offset (in pixels)
    ///
    /// Uses arbitrary precision to keep C precise
    pub fn pixel_offset(&mut self, pixels: (f64, f64)) {
        let mut pixels = pixels;
        let width_scale = self.viewport_width / (self.dim.x as f64);
        let height_scale = self.viewport_height / (self.dim.y as f64);
        pixels.0 *= width_scale;
        pixels.1 *= height_scale;

        self.c += Complex::with_val(PRECISION, pixels);
    }

    /// Zoom with a focus pixel point that stays in the same spot on the screen
    pub fn zoom_focus(&mut self, factor: f64, focus: Vec2) {
        let focus_c_start = self.screen_to_view(&focus);
        self.zoom(factor);
        let focus_c_end = self.screen_to_view(&focus);

        match focus_c_start {
            Some(c_start) => {
                let c_end = focus_c_end.expect("Guaranteed to exist of c_start exists");
                let offset = c_start - c_end;
                self.c += offset;
            }
            None => (),
        }
    }

    /// Reset the view to the values used at initialization
    pub fn reset(&mut self) {
        self.zoom = 1.;
        self.c.assign(&self.initial_params.c);
        let view = Self::get_viewport_width_height(&self.initial_params);
        self.viewport_width = view.0;
        self.viewport_height = view.1;
    }
}

impl Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pos: {}, dim: {}, zoom: {}",
            self.pos, self.dim, self.zoom
        )
    }
}
