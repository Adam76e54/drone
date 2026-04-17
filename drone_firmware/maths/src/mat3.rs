use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
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

    pub fn from_cols<C0, C1, C2>(col0: C0, col1: C1, col2: C2) -> Self
    where
        C0: Into<Vec3>,
        C1: Into<Vec3>,
        C2: Into<Vec3>,
    {
        let c0 = col0.into();
        let c1 = col1.into();
        let c2 = col2.into();
        
        Self {
            m: [
                [c0.x, c1.x, c2.x],
                [c0.y, c1.y, c2.y],
                [c0.z, c1.z, c2.z],
            ],
        }
    }   
}