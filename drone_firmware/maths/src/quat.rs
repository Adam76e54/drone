use core::ops::{Add, Div, Mul, Sub};

use crate::vec3::Vec3;
use crate::mat3::Mat3;

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
        libm::sqrtf(self.norm_squared())
    }

    pub fn norm_squared(self) -> f32 {
        self.s * self.s + self.v.dot(self.v)
    }
    
    pub fn dot(self, other: Self) -> f32 {
        self.s * other.s + self.v.dot(other.v)
    }

    pub fn conjugate(self) -> Self {
        Self::new(self.s, -self.v)
    }

    pub fn inverse(self) -> Option<Self> {
        let norm_sq = self.norm_squared();
        if norm_sq == 0.0 {
            None
        } else {
            Some(Self::new(self.s / norm_sq, -self.v / norm_sq))
        }
    }

    pub fn normalized(self) -> Option<Self> {
        let n = self.norm();
        if n == 0.0 {
            None
        } else {
            Some(Self::new(self.s / n, self.v / n))
        }
    }

    pub fn to_matrix(self) -> Mat3 {
        todo!() 
    }
}

impl Add for Quat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            s: self.s + rhs.s,
            v: self.v + rhs.v,
        }
    }
}

impl Sub for Quat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            s: self.s - rhs.s,
            v: self.v - rhs.v,
        }
    }
}

impl Div for Quat {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse().unwrap_or(Self::IDENTITY)
    }
}

impl Div<f32> for Quat {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            s: self.s / rhs,
            v: self.v / rhs,
        }
    }
}

impl Mul for Quat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            s: self.s * rhs.s - self.v.dot(rhs.v),
            v: self.s * rhs.v + rhs.s * self.v + self.v.cross(rhs.v),
        }
    }
}

impl Mul<f32> for Quat {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            s: self.s * rhs,
            v: self.v * rhs,
        }
    }
}


impl From<Vec3> for Quat {
    fn from(vec: Vec3) -> Self {
        Self { s: 0.0, v: vec }
    }
}