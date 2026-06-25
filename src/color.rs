use macroquad::color::{BLACK, Color, hsl_to_rgb};

use crate::fractal::IterationResult;

/// Create a simple looping colorset using hsl (0, 1)
pub fn create_colorset(num: usize) -> Vec<Color> {
    if num == 0 {
        return vec![];
    }

    (0..num)
        .map(|i| hsl_to_rgb(i as f32 / num as f32, 1., 0.5))
        .collect::<Vec<Color>>()
}

pub enum FractalColorType<'a> {
    /// Banded coloring with the given colorset - colors wrap around the colorset
    Banded(&'a [Color]),
    /// Smooth coloring with a range of the given size
    Smooth(f64),
}

impl<'a> FractalColorType<'a> {
    pub fn get_color(&self, res: &IterationResult) -> Color {
        match *self {
            FractalColorType::Banded(colorset) => FractalColorType::color_banded(res, colorset),
            FractalColorType::Smooth(range) => FractalColorType::color_smooth(res, range),
        }
    }
}

impl<'a> FractalColorType<'a> {
    fn color_banded(res: &IterationResult, colorset: &'a [Color]) -> Color {
        let Some(iter) = res.iterations else {
            return BLACK;
        };

        let scale = iter as f32 / res.max_iterations as f32;
        let i = ((colorset.len() - 1) as f32 * scale) as usize;
        colorset[i]
    }

    fn color_smooth(res: &IterationResult, range: f64) -> Color {
        let Some(iter) = res.iterations else {
            return BLACK;
        };

        // smoothed coloring
        let mag = res.final_z.len_squared().sqrt();
        let smooth = iter as f64 - mag.log2().log2();

        let h = (smooth % range) / range;;

        hsl_to_rgb(h as f32, 1., 0.5 as f32)
    }
}
