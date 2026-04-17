// use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct Quat {
//     pub w: f32,
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
// }

// impl Quat {
//     pub const IDENTITY: Self = Self::new(1.0, 0.0, 0.0, 0.0);

//     pub const fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
//         Self { w, x, y, z }
//     }

//     pub fn norm(self) -> f32 {
//         libm::sqrtf(self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z)
//     }

//     pub fn normalize(self) -> Self {
//         let n = self.norm();
//         if n > 0.0 {
//             self / n
//         } else {
//             Self::IDENTITY
//         }
//     }
// }