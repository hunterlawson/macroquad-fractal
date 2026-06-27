use macroquad::{
    material::Material,
    math::Vec2,
    miniquad::{UniformDesc, UniformType},
};

use crate::complex::C64;

/// Store some translation values that are only updated when the view changes
#[derive(Debug, Default, Clone, Copy)]
struct ViewTranslation {
    base_re: f64,
    scale_re: f64,
    base_im: f64,
    scale_im: f64,
}

#[derive(Debug, Clone)]
/// Describes a complex view mapped onto a screen-space view
pub struct ComplexView {
    pub screen_pixel_position: Vec2,
    pub pixel_dimensions: Vec2,
    offset_start: C64,
    offset: C64,
    /// The complex region is scaled to be the right aspect ratio
    r_region: C64,
    c_region: C64,

    zoom: f64,
    // zoomed regions
    r_region_z: C64,
    c_region_z: C64,

    // regenerate after every view change
    view_translation: ViewTranslation,
}

impl ComplexView {
    pub fn new(
        screen_pixel_position: Vec2,
        pixel_dimensions: Vec2,
        r_region: C64,
        offset: C64,
    ) -> Self {
        let i_ratio = pixel_dimensions.y as f64 / pixel_dimensions.x as f64;
        let c_bounds = (r_region.1 - r_region.0) * i_ratio / 2.;
        let c_region = C64(-1. * c_bounds, c_bounds);
        let mut view = Self {
            screen_pixel_position,
            pixel_dimensions,
            offset_start: offset,
            offset: offset,
            r_region,
            c_region,
            zoom: 1.,
            r_region_z: r_region,
            c_region_z: c_region,
            view_translation: ViewTranslation::default(),
        };

        view.update_view_translation();

        view
    }

    /// Get the uniform descriptions for the translation described by this view
    pub fn uniform_descs() -> Vec<UniformDesc> {
        vec![
            UniformDesc::new("base_re", UniformType::Float1),
            UniformDesc::new("scale_re", UniformType::Float1),
            UniformDesc::new("base_im", UniformType::Float1),
            UniformDesc::new("scale_im", UniformType::Float1),
            UniformDesc::new("pixel_dimensions", UniformType::Float2)
        ]
    }

    /// Set the material uniforms for the translation described by this view
    pub fn set_uniforms(&self, material: &Material) {
        let vt = self.view_translation;
        material.set_uniform("base_re", vt.base_re as f32);
        material.set_uniform("scale_re", vt.scale_re as f32);
        material.set_uniform("base_im", vt.base_im as f32);
        material.set_uniform("scale_im", vt.scale_im as f32);
        material.set_uniform("pixel_dimensions", self.pixel_dimensions);
    }

    fn update_view_translation(&mut self) {
        self.view_translation.base_re = self.r_region_z.0 + self.offset.0;
        self.view_translation.scale_re = self.r_region_z.1 - self.r_region_z.0;
        self.view_translation.base_im = self.c_region_z.1 + self.offset.1;
        self.view_translation.scale_im = self.c_region_z.0 - self.c_region_z.1;
    }

    /// Apply a zoom factor.
    /// Zoom starts at a factor of 1.0
    /// `current_zoom *= factor`
    pub fn zoom(&mut self, factor: f64) {
        self.zoom *= factor;
        self.r_region_z = self.r_region_z / factor;
        self.c_region_z = self.c_region_z / factor;
        self.update_view_translation();
    }

    /// Apply an offset scaled by the current zoom.
    /// The visible width is `region / zoom`, so a constant on-screen pan
    /// means a complex-space step proportional to `1 / zoom`.
    pub fn scaled_offset(&mut self, offset: C64) {
        self.offset += offset / self.zoom;
        self.update_view_translation();
    }

    /// Reset the view
    pub fn reset(&mut self) {
        self.r_region_z = self.r_region;
        self.c_region_z = self.c_region;
        self.offset = self.offset_start;
        self.zoom = 1.;
        self.update_view_translation();
    }
}
