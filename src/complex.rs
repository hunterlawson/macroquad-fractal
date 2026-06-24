use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

#[derive(Clone, Copy, Debug)]
pub struct C64(pub f64, pub f64);

impl C64 {
    /// Return |c|^2
    pub fn len_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1
    }

    /// Square the value in place
    pub fn squared(self) -> C64 {
        self * self
    }

    /// Create a new empy c64
    pub fn new() -> Self {
        Self(0., 0.)
    }
}

impl Add for C64 {
    type Output = C64;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<C64> for C64 {
    fn add_assign(&mut self, rhs: C64) {
        (*self).0 += rhs.0;
        (*self).1 += rhs.1;
    }
}

impl Sub for C64 {
    type Output = C64;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl MulAssign<f64> for C64 {
    fn mul_assign(&mut self, rhs: f64) {
        (*self).0 = self.0 * rhs;
        (*self).1 = self.1 * rhs;
    }
}

impl Mul<f64> for C64 {
    type Output = C64;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut s2 = self;
        s2 *= rhs;
        s2
    }
}

impl Mul<C64> for C64 {
    type Output = C64;

    fn mul(self, rhs: C64) -> Self::Output {
        // ((a + bi)(c + di) = (ac - bd) + (ad + bc)i
        Self(self.0 * rhs.0 - self.1 * rhs.1, self.0 * rhs.1 + self.1 * rhs.0)
    }
}
