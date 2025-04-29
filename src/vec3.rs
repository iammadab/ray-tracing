#![allow(unused)]

use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Default, Clone)]
pub(crate) struct Vec3(f32, f32, f32);

impl Vec3 {
    pub(crate) fn new(a: f32, b: f32, c: f32) -> Self {
        Self(a, b, c)
    }

    pub(crate) const fn x(&self) -> f32 {
        self.0
    }
    pub(crate) const fn y(&self) -> f32 {
        self.1
    }
    pub(crate) const fn z(&self) -> f32 {
        self.2
    }

    pub(crate) const fn r(&self) -> f32 {
        self.0
    }
    pub(crate) const fn g(&self) -> f32 {
        self.1
    }
    pub(crate) const fn b(&self) -> f32 {
        self.2
    }

    pub(crate) const fn squared_length(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub(crate) fn unit_vector(&self) -> Vec3 {
        let len = self.length();
        Self::new(self.0 / len, self.1 / len, self.2 / len)
    }

    pub(crate) fn dot(&self, rhs: &Self) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}
