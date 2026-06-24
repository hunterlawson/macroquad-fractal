use crate::complex::*;

pub trait Fractal: Send + Sync {
    /// Run the fractal iteration function.
    /// Return None if the function does not diverge.
    /// Return Some(x) where x is the number of iterations before diverging
    fn iterate(&self, c: C64) -> Option<f64>;
}

/// mandelbrot set is the set of all complex values c where the function
/// `f(z) = z^2 + c`
/// does not diverge to infinity when iterated starting at z = 0
pub struct Mandelbrot {
    pub max_iter: u32,
}

impl Fractal for Mandelbrot {
    fn iterate(&self, c: C64) -> Option<f64> {
        let mut iter = 0;
        let mut z = C64(0., 0.);
        while iter < self.max_iter{
            z = z.squared() + c;
            iter += 1;
            if z.len_squared() > 4. {
                // smoothed coloring
                let mag = z.len_squared().sqrt();
                let smooth = iter as f64 - mag.log2().log2();
                return Some(smooth.max(0.0))
            }
        }

        None
    }
}
