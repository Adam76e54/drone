use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    pub s: f32,
    pub v: Vec3
}

impl Quat {
    pub const IDENTITY: Self = Self::new(1.0, Vec3::ZERO);

    pub const fn new(s: f32, v: Vec3) -> Self {
        Self { s, v }
    }

    pub fn norm(self) -> f32 {
        libm::sqrtf(self.s * self.s + self.v.dot(self.v))
    }

    pub fn normalize(self) -> Self {
        todo!()
    }
}