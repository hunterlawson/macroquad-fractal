use std::ops::Div;

use crate::complex::C64;

#[derive(Clone, Copy, Debug)]
pub struct Dimension(pub f64, pub f64);

impl Div<f64> for Dimension {
    type Output = Dimension;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

/// Store some translation values that are only updated when the view changes
#[derive(Debug, Default)]
struct ScreenToComplexMapping {
    base_re: f64,
    scale_re: f64,
    base_im: f64,
    scale_im: f64,
}

#[derive(Debug)]
pub struct ComplexView {
    pixel_dimensions: Dimension,
    offset_start: C64,
    offset: C64,
    /// The complex region is scaled to be the right aspect ratio
    r_region: Dimension,
    c_region: Dimension,

    zoom: f64,
    // zoomed regions
    r_region_z: Dimension,
    c_region_z: Dimension,

    // regenerate after every view change
    view_translation: ScreenToComplexMapping,
}

impl ComplexView {
    pub fn new(w_pixels: usize, h_pixels: usize, r_region: Dimension, offset: C64) -> Self {
        let i_ratio = h_pixels as f64 / w_pixels as f64;
        let c_bounds = (r_region.1 - r_region.0) * i_ratio / 2.;
        let c_region = Dimension(-1. * c_bounds, c_bounds);
        let mut view = Self {
            pixel_dimensions: Dimension(w_pixels as f64, h_pixels as f64),
            offset_start: offset,
            offset: offset,
            r_region,
            c_region,
            zoom: 1.,
            r_region_z: r_region,
            c_region_z: c_region,
            view_translation: ScreenToComplexMapping::default(),
        };

        view.update_view_translation();

        view
    }

    fn update_view_translation(&mut self) {
        self.view_translation.base_re = self.r_region_z.0;
        self.view_translation.scale_re =
            (self.r_region_z.1 - self.r_region_z.0) / self.pixel_dimensions.0;
        self.view_translation.base_im = self.c_region_z.1;
        self.view_translation.scale_im =
            (self.c_region_z.0 - self.c_region_z.1) / self.pixel_dimensions.1;
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
    pub fn scaled_offset(&mut self, offset: &C64) {
        self.offset += *offset / self.zoom;
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

    /// Get a complex number from the view for the given pixel
    pub fn get_pixel_value(&self, x: usize, y: usize) -> C64 {
        let a = self.view_translation.base_re + x as f64 * self.view_translation.scale_re;
        // Flip the complex plane because y = 0 is the top of the screen
        let b = self.view_translation.base_im + y as f64 * self.view_translation.scale_im;

        C64(a, b) + self.offset
    }
}
