//! Vec3

use std::ops::{Add, Div, Mul, Neg};

#[derive(Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        let x = x.into();
        let y = y.into();
        let z = z.into();

        Self { x, y, z }
    }

    pub const fn length(&self) -> f64 {
        self.length_squared()
    }

    pub const fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(t: (f64, f64, f64)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Self { x, y, z }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, t: f64) -> Self::Output {
        let x = self.x * t;
        let y = self.y * t;
        let z = self.z * t;
        Self { x, y, z }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        Self { x, y, z }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, t: f64) -> Self::Output {
        self * (1.0 / t)
    }
}
