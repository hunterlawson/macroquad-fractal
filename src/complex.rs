#![allow(dead_code)]

use std::{fmt::Display, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub}};

use macroquad::math::Vec2;

#[derive(Clone, Copy, Debug, Default)]
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

    pub fn a_squared(&self) -> f64 {
        self.0 * self.0
    }

    pub fn b_squared(&self) -> f64 {
        self.1 * self.1
    }

    /// Returns (z², |z|²). Both derive from the same a²/b² subexpressions,
    /// so this is the fast form for an escape-time loop (3 real multiplies).
    pub fn square_and_norm(self) -> (C64, f64) {
        let a2 = self.0 * self.0;
        let b2 = self.1 * self.1;
        (C64(a2 - b2, 2. * self.0 * self.1), a2 + b2)
    }
}

impl From<Vec2> for C64 {
    fn from(value: Vec2) -> Self {
        Self(value.x as f64, value.y as f64)
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
        let mut c2 = self;
        c2 *= rhs;
        c2
    }
}

impl DivAssign<f64> for C64 {
    fn div_assign(&mut self, rhs: f64) {
        (*self).0 = self.0 / rhs;
        (*self).1 = self.1 / rhs;
    }
}

impl Div<f64> for C64 {
    type Output = C64;

    fn div(self, rhs: f64) -> Self::Output {
        let mut c2 = self;
        c2 /= rhs;
        c2
    }
}

impl Mul<C64> for C64 {
    type Output = C64;

    fn mul(self, rhs: C64) -> Self::Output {
        // ((a + bi)(c + di) = (ac - bd) + (ad + bc)i
        Self(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl Display for C64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.0, self.1)
    }
}
