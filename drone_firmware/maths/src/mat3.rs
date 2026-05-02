use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    // m[column][row] 
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

    pub const ZERO: Self = Self {
        m: [
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
        ],
    };

    pub const fn new(m: [[f32; 3]; 3]) -> Self {
        Self { m }
    }

    // NOTE: these "impl Into<Vec3>" parameter types are saying "a type that has this trait implemented" 
    // so we can pass both Vec3 and [f32; 3] to this function. 
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

    pub fn row(&self, index: usize) -> Vec3 {
        Vec3::new(self.m[0][index], self.m[1][index], self.m[2][index])
    }

    pub fn col(&self, index: usize) -> Vec3 {
        Vec3::from(self.m[index])
    }
 
}

impl Add for Mat3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::ZERO;
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
        let mut result = Self::ZERO;
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
        let c0 = self * rhs.col(0);
        let c1 = self * rhs.col(1);
        let c2 = self * rhs.col(2);
        Self::from_cols(c0, c1, c2)
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let x = self.row(0).dot(rhs);
        let y = self.row(1).dot(rhs);
        let z = self.row(2).dot(rhs);

        Vec3::new(x,y,z)
    }
}