use std::fmt::{Debug, Formatter, Result};
use std::simd::f64x4;
use std::simd::num::SimdFloat;

pub struct Vector3(f64x4);

impl Debug for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Vector3({}, {}, {})", self.0[0], self.0[1], self.0[2])
    }
}

impl Vector3 {
    #[inline]
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(f64x4::from_array([x, y, z, 0.0]))
    }

    #[inline]
    #[must_use]
    pub fn zeros() -> Self {
        Self(f64x4::default())
    }

    #[inline]
    #[must_use]
    pub fn ones() -> Self {
        Self(f64x4::from_array([1.0, 1.0, 1.0, 0.0]))
    }

    #[inline]
    #[must_use]
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    #[inline]
    #[must_use]
    pub fn y(&self) -> f64 {
        self.0[1]
    }

    #[inline]
    #[must_use]
    pub fn z(&self) -> f64 {
        self.0[2]
    }

    #[inline]
    pub fn set_x(&mut self, x: f64) {
        self.0[0] = x;
    }

    #[inline]
    pub fn set_y(&mut self, y: f64) {
        self.0[1] = y;
    }

    #[inline]
    pub fn set_z(&mut self, z: f64) {
        self.0[2] = z;
    }

    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> f64 {
        (self.0 * rhs.0).reduce_sum()
    }

    #[inline]
    #[must_use]
    pub fn magnitude_squared(self) -> f64 {
        (self.0 * self.0).reduce_sum()
    }

    #[inline]
    #[must_use]
    pub fn magnitude(self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Self::zeros()
        } else {
            self * (1.0 / mag)
        }
    }

    #[inline]
    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        let a = self.0;
        let b = rhs.0;
        let result = f64x4::from_array([
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
            0.0,
        ]);
        Self(result)
    }

    #[inline]
    #[must_use]
    pub fn distance(self, rhs: Self) -> f64 {
        (self - rhs).magnitude()
    }

    #[inline]
    #[must_use]
    pub fn distance_squared(self, rhs: Self) -> f64 {
        (self - rhs).magnitude_squared()
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * f64x4::splat(rhs))
    }
}

impl std::ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f64> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 * f64x4::splat(1.0 / rhs))
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl std::ops::AddAssign for Vector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::SubAssign for Vector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::ops::MulAssign<f64> for Vector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= f64x4::splat(rhs);
    }
}

impl std::ops::DivAssign<f64> for Vector3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= f64x4::splat(rhs);
    }
}

impl PartialEq for Vector3 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0] && self.0[1] == other.0[1] && self.0[2] == other.0[2]
    }
}

impl Clone for Vector3 {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Vector3 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_zeros() {
        let v = Vector3::zeros();
        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 0.0);
        assert_eq!(v.z(), 0.0);
    }

    #[test]
    fn test_ones() {
        let v = Vector3::ones();
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 1.0);
        assert_eq!(v.z(), 1.0);
    }

    #[test]
    fn test_setters() {
        let mut v = Vector3::zeros();
        v.set_x(1.0);
        v.set_y(2.0);
        v.set_z(3.0);
        assert_eq!(v, Vector3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_add() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let c = a + b;
        assert_eq!(c, Vector3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_sub() {
        let a = Vector3::new(4.0, 5.0, 6.0);
        let b = Vector3::new(1.0, 2.0, 3.0);
        let c = a - b;
        assert_eq!(c, Vector3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_mul_scalar() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = a * 2.0;
        assert_eq!(b, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_scalar_left() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = 2.0 * a;
        assert_eq!(b, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_div_scalar() {
        let a = Vector3::new(2.0, 4.0, 6.0);
        let b = a / 2.0;
        assert_eq!(b, Vector3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_neg() {
        let a = Vector3::new(1.0, -2.0, 3.0);
        let b = -a;
        assert_eq!(b, Vector3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_dot() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let dot = a.dot(b);
        assert_eq!(dot, 32.0);
    }

    #[test]
    fn test_magnitude_squared() {
        let a = Vector3::new(3.0, 4.0, 0.0);
        let mag_sq = a.magnitude_squared();
        assert_eq!(mag_sq, 25.0);
    }

    #[test]
    fn test_magnitude() {
        let a = Vector3::new(3.0, 4.0, 0.0);
        let mag = a.magnitude();
        assert_eq!(mag, 5.0);
    }

    #[test]
    fn test_normalize() {
        let a = Vector3::new(3.0, 4.0, 0.0);
        let normalized = a.normalize();
        let mag = normalized.magnitude();
        assert!((mag - 1.0).abs() < 1e-10);
        assert!((normalized.x() - 0.6).abs() < 1e-10);
        assert!((normalized.y() - 0.8).abs() < 1e-10);
        assert_eq!(normalized.z(), 0.0);
    }

    #[test]
    fn test_normalize_zero() {
        let a = Vector3::zeros();
        let normalized = a.normalize();
        assert_eq!(normalized, Vector3::zeros());
    }

    #[test]
    fn test_cross() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 1.0, 0.0);
        let c = a.cross(b);
        assert_eq!(c, Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_cross_anticommutative() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 1.0, 0.0);
        let c1 = a.cross(b);
        let c2 = b.cross(a);
        assert_eq!(c1, -c2);
    }

    #[test]
    fn test_distance() {
        let a = Vector3::new(0.0, 0.0, 0.0);
        let b = Vector3::new(3.0, 4.0, 0.0);
        let dist = a.distance(b);
        assert_eq!(dist, 5.0);
    }

    #[test]
    fn test_distance_squared() {
        let a = Vector3::new(0.0, 0.0, 0.0);
        let b = Vector3::new(3.0, 4.0, 0.0);
        let dist_sq = a.distance_squared(b);
        assert_eq!(dist_sq, 25.0);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        a += b;
        assert_eq!(a, Vector3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Vector3::new(4.0, 5.0, 6.0);
        let b = Vector3::new(1.0, 2.0, 3.0);
        a -= b;
        assert_eq!(a, Vector3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        a *= 2.0;
        assert_eq!(a, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_div_assign() {
        let mut a = Vector3::new(2.0, 4.0, 6.0);
        a /= 2.0;
        assert_eq!(a, Vector3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_equality() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(1.0, 2.0, 3.0);
        let c = Vector3::new(1.0, 2.0, 4.0);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_clone_and_copy() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = a;
        let c = a.clone();
        assert_eq!(a, b);
        assert_eq!(a, c);
    }
}
