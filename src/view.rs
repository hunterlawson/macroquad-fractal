use std::ops::Div;

use crate::{complex::C64};

#[derive(Clone, Copy, Debug)]
pub struct Dimension(pub f64, pub f64);

impl Div<f64> for Dimension {
    type Output = Dimension;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

#[derive(Debug)]
pub struct ComplexView {
    pixel_dimensions: Dimension,
    offset_start: C64,
    offset: C64,
    /// The complex region is scaled to be the right aspect ratio
    r_region: Dimension,
    c_region: Dimension,

    // zoomed regions
    r_region_z: Dimension,
    c_region_z: Dimension,
}

impl ComplexView {
    pub fn new(w_pixels: usize, h_pixels: usize, r_region: Dimension, offset: C64) -> Self {
        let i_ratio = h_pixels as f64 / w_pixels as f64;
        let c_bounds= (r_region.1 - r_region.0) * i_ratio / 2.;
        let c_region = Dimension(-1. * c_bounds, c_bounds);
        Self {
            pixel_dimensions: Dimension(w_pixels as f64, h_pixels as f64),
            offset_start: offset,
            offset: offset,
            r_region,
            c_region,
            r_region_z: r_region,
            c_region_z: c_region,
        }
    }

    /// Apply a zoom factor.
    pub fn zoom(&mut self, factor: f64) {
        self.r_region_z = self.r_region_z / factor;
        self.c_region_z = self.c_region_z / factor;
    }

    /// Apply a complex offset
    pub fn offset(&mut self, offset: &C64) {
        self.offset = *offset + self.offset_start
    }

    /// Reset the view
    pub fn reset(&mut self) {
        self.r_region_z = self.r_region;
        self.c_region_z = self.c_region;
        self.offset = C64::new();
    }

    /// map a value x [a, b] to a value y [a2, b2]
    fn map_value(x: f64, xmin: f64, xmax: f64, omin: f64, omax: f64) -> f64 {
        let x_scale = (x - xmin) / (xmax - xmin);
        omin + (omax - omin) * x_scale
    }

    /// Get a complex number from the view for the given pixel
    pub fn get_pixel_value(&self, x: usize, y: usize) -> C64 {
        let a = ComplexView::map_value(
            x as f64,
            0.,
            self.pixel_dimensions.0,
            self.r_region_z.0,
            self.r_region_z.1,
        );
        let b = ComplexView::map_value(
            y as f64,
            0.,
            self.pixel_dimensions.1,
            self.c_region_z.0,
            self.c_region_z.1,
        );

        // debug!("({}, {}) -> ({}, {})", x, y, a, b);

        C64(a, b) + self.offset
    }
}
