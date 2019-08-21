use std::ops::{
    Neg,
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Index, IndexMut
};

/// three dimension vector 
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// construct new vector
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    /// zero vector of [0.0, 0.0, 0.0]
    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// unit vector of [1.0, 1.0, 1.0]
    pub fn unit() -> Self {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    /// L2 norm of vector (length)
    pub fn norm(&self) -> f32 {
        self.norm_squared().sqrt()
    }

    /// squared L2 norm of vector (squared length)
    pub fn norm_squared(&self) -> f32 {
        self.x * self.x +
        self.y * self.y +
        self.z * self.z
    }

    /// normalize vector to unit length
    pub fn normalize(&self) -> Self {
        *self / self.norm()
    }

    /// in-place normalize vector to unit length
    pub fn normalize_mut(&mut self) {
        *self /= self.norm();
    }

    /// clamp element-wise
    pub fn clamp(&self, a: f32, b: f32) -> Vec3 {
        Vec3 {
            x: self.x.max(a).min(b),
            y: self.y.max(a).min(b),
            z: self.z.max(a).min(b),
        }
    }

    /// dot product
    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + 
        self.y * other.y +
        self.z * other.z
    }

    /// cross product
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// max component
    pub fn max_component(&self) -> f32 {
        self.x.max(self.y).max(self.z)
    }

    /// min component
    pub fn min_component(&self) -> f32 {
        self.x.min(self.y).min(self.z)
    }

    /// maximum component-wise
    pub fn max(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    /// minimum component-wise
    pub fn min(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bound"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bound"),
        }
    }
}

impl std::iter::Sum for Vec3 {
    fn sum<I: Iterator<Item=Vec3>>(iter: I) -> Vec3 {
        iter.fold(Vec3::zero(), |a, b| a + b)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        let r  = Vec3::new(3.0, 5.0, 7.0);
        assert_eq!(r, v1 + v2);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        let r  = Vec3::new(-1.0, -1.0, -1.0);
        assert_eq!(r, v1 - v2);
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        let r1 = Vec3::new(2.0, 6.0, 12.0);
        assert_eq!(r1, v1 * v2);

        let r2 = Vec3::new(4.0, 12.0, 24.0);
        assert_eq!(r2, r1 * 2.0);
        assert_eq!(r2, 2.0 * r1);
    }

    #[test]
    fn test_clamp() {
        let v1 = Vec3::new(-1.0, 0.99, 2.3);
        let v2 = Vec3::new(0.0, 0.99, 1.0);
        assert_eq!(v2, v1.clamp(0.0, 1.0));
    }

    #[test]
    fn test_index() {
        let v1 = Vec3::new(-1.0, 0.99, 2.3);
        assert_eq!(v1[0], -1.0);
        assert_eq!(v1[1], 0.99);
        assert_eq!(v1[2], 2.3);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(20.0, v1.dot(v2));
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(0.0, 0.0, 1.0);
        let v2 = Vec3::new(1.0, 0.0, 0.0);
        let v3 = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(v3, v1.cross(v2));
    }
}
