use line_clipping::{LineSegment, Point, Window, cohen_sutherland::clip_line};
use macroquad::{
    material::Material,
    math::{Vec2, vec2},
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
            UniformDesc::new("pixel_dimensions", UniformType::Float2),
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

    /// Return whether or not the given screen-space pixel coordinate is inside this view
    pub fn in_view(&self, pos: &Vec2) -> bool {
        (pos.x > self.screen_pixel_position.x
            && pos.x < self.screen_pixel_position.x + self.pixel_dimensions.x)
            && (pos.y > self.screen_pixel_position.y
                && pos.y < self.screen_pixel_position.y + self.pixel_dimensions.y)
    }

    /// Clip a line to fall inside the view
    pub fn clip_line(&self, line: (Vec2, Vec2)) -> Option<(Vec2, Vec2)> {
        let window = Window::new(
            self.screen_pixel_position.x as f64,
            self.screen_pixel_position.x as f64 + self.pixel_dimensions.x as f64,
            self.screen_pixel_position.y as f64,
            self.screen_pixel_position.y as f64 + self.pixel_dimensions.y as f64,
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

    /// Get a complex number in the view from the given screen-space coordinate
    pub fn screen_to_complex(&self, pos: &Vec2) -> Option<C64> {
        // check that the mouse isn't within bounds of the view's pixel space
        if !self.in_view(&pos) {
            return None;
        }

        // convert from screen-space to the view's pixel space
        let scale_x = ((pos.x - self.screen_pixel_position.x) / self.pixel_dimensions.x) as f64;
        let scale_y = ((pos.y - self.screen_pixel_position.y) / self.pixel_dimensions.y) as f64;

        Some(C64(
            self.view_translation.base_re + scale_x * self.view_translation.scale_re,
            self.view_translation.base_im + scale_y * self.view_translation.scale_im,
        ))
    }

    /// Convert from the view's complex space to screen-space pixel coordinates
    pub fn complex_to_screen(&self, c: &C64) -> Vec2 {
        // normalize the complex number
        let re_norm = (c.0 - self.view_translation.base_re) / self.view_translation.scale_re;
        let im_norm = (c.1 - self.view_translation.base_im) / self.view_translation.scale_im;

        // scale to pixels
        let mut pix_x = re_norm as f32 * self.pixel_dimensions.x;
        let mut pix_y = im_norm as f32 * self.pixel_dimensions.y;

        // offset to the fractal position
        pix_x += self.screen_pixel_position.x;
        pix_y += self.screen_pixel_position.y;

        vec2(pix_x, pix_y)
    }
}
