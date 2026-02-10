use std::fmt::{Debug, Formatter, Result};
use std::simd::f64x2;

pub struct Vector2(f64x2);

impl Debug for Vector2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Vector2({}, {})", self.0[0], self.0[1])
    }
}

impl Vector2 {
    #[inline]
    #[must_use]
    pub fn new(x: f64, y: f64) -> Self {
        Self(f64x2::from_array([x, y]))
    }

    #[inline]
    #[must_use]
    pub fn zeros() -> Self {
        Self(f64x2::splat(0.0))
    }

    #[inline]
    #[must_use]
    pub fn ones() -> Self {
        Self(f64x2::from_array([1.0, 1.0, 0.0, 0.0]))
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
    pub fn set_x(&mut self, x: f64) {
        self.0[0] = x;
    }

    #[inline]
    pub fn set_y(&mut self, y: f64) {
        self.0[1] = y;
    }

    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> f64 {
        let prod = self.0 * rhs.0;
        prod[0] + prod[1]
    }

    #[inline]
    #[must_use]
    pub fn magnitude_squared(self) -> f64 {
        self.dot(self)
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
    pub fn cross(self, rhs: Self) -> f64 {
        self.x() * rhs.y() - self.y() * rhs.x()
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

impl std::ops::Add for Vector2 {
    type Output = Vector2;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Vector2;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul<f64> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * f64x2::splat(rhs))
    }
}

impl std::ops::Mul<Vector2> for f64 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: Vector2) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f64> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / f64x2::splat(rhs))
    }
}

impl std::ops::Neg for Vector2 {
    type Output = Vector2;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl std::ops::AddAssign for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::SubAssign for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::ops::MulAssign<f64> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= f64x2::splat(rhs);
    }
}

impl std::ops::DivAssign<f64> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= f64x2::splat(rhs);
    }
}

impl PartialEq for Vector2 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0] && self.0[1] == other.0[1]
    }
}

impl Clone for Vector2 {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Copy for Vector2 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vector2::new(1.0, 2.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
    }

    #[test]
    fn test_zeros() {
        let v = Vector2::zeros();
        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 0.0);
    }

    #[test]
    fn test_ones() {
        let v = Vector2::ones();
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 1.0);
    }

    #[test]
    fn test_setters() {
        let mut v = Vector2::zeros();
        v.set_x(1.0);
        v.set_y(2.0);
        assert_eq!(v, Vector2::new(1.0, 2.0));
    }

    #[test]
    fn test_add() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        let c = a + b;
        assert_eq!(c, Vector2::new(4.0, 6.0));
    }

    #[test]
    fn test_sub() {
        let a = Vector2::new(4.0, 6.0);
        let b = Vector2::new(1.0, 2.0);
        let c = a - b;
        assert_eq!(c, Vector2::new(3.0, 4.0));
    }

    #[test]
    fn test_mul_scalar() {
        let a = Vector2::new(1.0, 2.0);
        let b = a * 2.0;
        assert_eq!(b, Vector2::new(2.0, 4.0));
    }

    #[test]
    fn test_mul_scalar_left() {
        let a = Vector2::new(1.0, 2.0);
        let b = 2.0 * a;
        assert_eq!(b, Vector2::new(2.0, 4.0));
    }

    #[test]
    fn test_div_scalar() {
        let a = Vector2::new(2.0, 4.0);
        let b = a / 2.0;
        assert_eq!(b, Vector2::new(1.0, 2.0));
    }

    #[test]
    fn test_neg() {
        let a = Vector2::new(1.0, -2.0);
        let b = -a;
        assert_eq!(b, Vector2::new(-1.0, 2.0));
    }

    #[test]
    fn test_dot() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        let dot = a.dot(b);
        assert_eq!(dot, 11.0);
    }

    #[test]
    fn test_magnitude_squared() {
        let a = Vector2::new(3.0, 4.0);
        let mag_sq = a.magnitude_squared();
        assert_eq!(mag_sq, 25.0);
    }

    #[test]
    fn test_magnitude() {
        let a = Vector2::new(3.0, 4.0);
        let mag = a.magnitude();
        assert_eq!(mag, 5.0);
    }

    #[test]
    fn test_normalize() {
        let a = Vector2::new(3.0, 4.0);
        let normalized = a.normalize();
        let mag = normalized.magnitude();
        assert!((mag - 1.0).abs() < 1e-10);
        assert!((normalized.x() - 0.6).abs() < 1e-10);
        assert!((normalized.y() - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_normalize_zero() {
        let a = Vector2::zeros();
        let normalized = a.normalize();
        assert_eq!(normalized, Vector2::zeros());
    }

    #[test]
    fn test_cross() {
        let a = Vector2::new(1.0, 0.0);
        let b = Vector2::new(0.0, 1.0);
        let cross = a.cross(b);
        assert_eq!(cross, 1.0);
    }

    #[test]
    fn test_cross_anticommutative() {
        let a = Vector2::new(1.0, 0.0);
        let b = Vector2::new(0.0, 1.0);
        let c1 = a.cross(b);
        let c2 = b.cross(a);
        assert_eq!(c1, -c2);
    }

    #[test]
    fn test_distance() {
        let a = Vector2::new(0.0, 0.0);
        let b = Vector2::new(3.0, 4.0);
        let dist = a.distance(b);
        assert_eq!(dist, 5.0);
    }

    #[test]
    fn test_distance_squared() {
        let a = Vector2::new(0.0, 0.0);
        let b = Vector2::new(3.0, 4.0);
        let dist_sq = a.distance_squared(b);
        assert_eq!(dist_sq, 25.0);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        a += b;
        assert_eq!(a, Vector2::new(4.0, 6.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Vector2::new(4.0, 6.0);
        let b = Vector2::new(1.0, 2.0);
        a -= b;
        assert_eq!(a, Vector2::new(3.0, 4.0));
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Vector2::new(1.0, 2.0);
        a *= 2.0;
        assert_eq!(a, Vector2::new(2.0, 4.0));
    }

    #[test]
    fn test_div_assign() {
        let mut a = Vector2::new(2.0, 4.0);
        a /= 2.0;
        assert_eq!(a, Vector2::new(1.0, 2.0));
    }

    #[test]
    fn test_equality() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(1.0, 2.0);
        let c = Vector2::new(1.0, 3.0);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_clone_and_copy() {
        let a = Vector2::new(1.0, 2.0);
        let b = a;
        let c = a.clone();
        assert_eq!(a, b);
        assert_eq!(a, c);
    }
}
