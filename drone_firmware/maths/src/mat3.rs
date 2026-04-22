use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    // 3 columns of 3 elements each 
    pub m: [[f32; 3]; 3],
}

impl Mat3 {
    pub const IDENTITY: Self = Self {
        m: [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ],
    };

    pub const fn new(m: [[f32; 3]; 3]) -> Self {
        Self { m }
    }

    pub fn from_cols(
        col0: impl Into<Vec3>,
        col1: impl Into<Vec3>,
        col2: impl Into<Vec3>,
    ) -> Self {
        let c0 = col0.into();
        let c1 = col1.into();
        let c2 = col2.into();
        
        Self {
            m: [
                c0.to_array(),
                c1.to_array(),
                c2.to_array(),
            ],
        }
    }
 
}

impl Add for Mat3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::IDENTITY;
        for i in 0..3 {
            for j in 0..3 {
                result.m[i][j] = self.m[i][j] + rhs.m[i][j];
            }
        }
        result
    }
}

impl Sub for Mat3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::IDENTITY;
        for i in 0..3 {
            for j in 0..3 {
                result.m[i][j] = self.m[i][j] - rhs.m[i][j];
            }
        }
        result
    }
}

impl Mul for Mat3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self::IDENTITY;
        for i in 0..3 {
            for j in 0..3 {
                result.m[i][j] = self.m[i][0] * rhs.m[0][j]
                    + self.m[i][1] * rhs.m[1][j]
                    + self.m[i][2] * rhs.m[2][j];
            }
        }
        result
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.m[0][0] * rhs.x + self.m[0][1] * rhs.y + self.m[0][2] * rhs.z,
            self.m[1][0] * rhs.x + self.m[1][1] * rhs.y + self.m[1][2] * rhs.z,
            self.m[2][0] * rhs.x + self.m[2][1] * rhs.y + self.m[2][2] * rhs.z,
        )
    }
}