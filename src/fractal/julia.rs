use crate::{
    complex::C64,
    fractal::{Fractal, IterationResult},
};

pub struct Julia {
    pub max_iter: u32,
    pub c: C64,
}

impl Fractal for Julia {
    fn iterate(&self, mut z: C64) -> IterationResult {
        let mut iter = 0;
        loop {
            let (z_sq, norm) = z.square_and_norm();
            if norm > 4. || iter == self.max_iter {
                break;
            }
            z = z_sq + self.c;
            iter += 1;
        }

        IterationResult {
            max_iterations: self.max_iter,
            iterations: (iter != self.max_iter).then_some(iter),
            final_z: z,
        }
    }
}
