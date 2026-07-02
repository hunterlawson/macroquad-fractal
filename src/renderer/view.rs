use std::fmt::Display;

use macroquad::{
    logging::debug, material::Material, math::Vec2, miniquad::{UniformDesc, UniformType},
};
use rug::{Assign, Complex, ops::CompleteRound};

use crate::{PRECISION, renderer::view};

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

    // pub fn uniform_descs(&self) -> Vec<UniformDesc> {
    //     vec![
    //         UniformDesc::new("display_dimensions", UniformType::Float2),
    //         UniformDesc::new("viewport_dimensions", UniformType::Float2),
    //     ]
    // }

    // pub fn set_uniforms(&self, material: &Material) {
    //     material.set_uniform("display_dimensions", self.dim);
    //     material.set_uniform(
    //         "viewport_dimensions",
    //         (self.viewport_width, self.viewport_height),
    //     );
    // }

    pub fn uniform_descs() -> Vec<UniformDesc> {
        vec![
            UniformDesc::new("base_re", UniformType::Float1),
            UniformDesc::new("scale_re", UniformType::Float1),
            UniformDesc::new("base_im", UniformType::Float1),
            UniformDesc::new("scale_im", UniformType::Float1),
            UniformDesc::new("pixel_dimensions", UniformType::Float2),
        ]
    }

    pub fn set_uniforms(&self, material: &Material) {
        let base_re = (self.c.real() - (self.viewport_width / 2.)).complete(PRECISION).to_f32();
        let base_im = (self.c.imag() - (self.viewport_height / 2.)).complete(PRECISION).to_f32();
        material.set_uniform("base_re", base_re);
        material.set_uniform("scale_re", self.viewport_width as f32);
        material.set_uniform("base_im", base_im);
        material.set_uniform("scale_im", self.viewport_height as f32);
        material.set_uniform("pixel_dimensions", self.dim);
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
    pub fn scaled_offset(&mut self, pixels: (f64, f64)) {
        let mut pixels = pixels;
        let width_scale = self.viewport_width / (self.dim.x as f64);
        let height_scale = self.viewport_height / (self.dim.y as f64);
        pixels.0 *= width_scale;
        pixels.1 *= height_scale;
        
        self.c += Complex::with_val(PRECISION, pixels);
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
        write!(f, "pos: {}, dim: {}, zoom: {}", self.pos, self.dim, self.zoom)
    }
}