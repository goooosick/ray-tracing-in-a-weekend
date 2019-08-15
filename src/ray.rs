use crate::Vec3;

/// ray with origin and direction
#[derive(PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// construct new ray
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Ray {
            origin: a,
            direction: b,
        }
    }

    /// get point on the ray path respect to its direction
    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_at() {
        let r = Ray::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(2.0, 3.0, 4.0));
        assert_eq!(r.point_at(2.0), Vec3::new(4.0, 6.0, 9.0));
    }
}
