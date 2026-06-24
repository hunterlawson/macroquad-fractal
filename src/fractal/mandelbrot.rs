use crate::{
    complex::C64,
    fractal::{Fractal, IterationResult},
};

/// mandelbrot set is the set of all complex values c where the function
/// `f(z) = z^2 + c`
/// does not diverge to infinity when iterated starting at z = 0
pub struct Mandelbrot {
    pub max_iter: u32,
}

impl Fractal for Mandelbrot {
    fn iterate(&self, c: C64) -> IterationResult {
        let mut iter = 0;
        let mut z = C64(0., 0.);
        while iter < self.max_iter {
            z = z.squared() + c;
            iter += 1;
            if z.len_squared() > 4. {
                return IterationResult {
                    max_iterations: self.max_iter,
                    iterations: Some(iter),
                    final_z: z,
                };
            }
        }

        IterationResult {
            max_iterations: self.max_iter,
            iterations: None,
            final_z: z,
        }
    }
}
