use super::variations::*;
use std::ops::{Add, AddAssign, Mul};

/// Representation of a Point
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point(pub f32, pub f32);

impl Point {
    pub fn new() -> Point {
        Point(0.0, 0.0)
    }

    /// Applies affine transformation on a point
    pub fn affine(&self, coeffs: (f32, f32, f32, f32, f32, f32)) -> Point {
        let Point(x, y) = self;
        let (a, b, c, d, e, f) = coeffs;
        Point(a * x + b * y + c, d * x + e * y + f)
    }

    /// Applies a function on a point given the weights and variations that define such function
    pub fn apply_variation(p: &PreProc, weights: &Vec<f32>, vars: &Vec<fn(&PreProc) -> Point>) -> Point {
        let mut res = Point::new();
        for i in 0..vars.len() {
            res += weights[i] * vars[i](p);
        }
        return res;
    }
}

// Addition and multiplication by scalar to simplify notation

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            0: self.0 + other.0,
            1: self.1 + other.1,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Point(scalar * self.0, scalar * self.1)
    }
}

impl Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, point: Point) -> Self::Output {
        Point(self * point.0, self * point.1)
    }
}
