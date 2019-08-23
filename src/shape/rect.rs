use super::{HitRecord, Hitable};
use crate::accel::AABB;
use crate::{Material, Ray, Vec3};

/// rectangle in xy plane
pub struct XyRect<T> {
    x: (f32, f32),
    y: (f32, f32),
    z: f32,
    material: Box<T>,
}

impl<T> XyRect<T> {
    /// construct new rectangle with x(min, max), y(min, max), and z
    pub fn new(x: (f32, f32), y: (f32, f32), z: f32, mat: T) -> Self {
        XyRect {
            x,
            y,
            z,
            material: Box::new(mat),
        }
    }
}

impl<T> Hitable for XyRect<T>
where
    T: Material,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.z - ray.origin.z) / ray.direction.z;
        if t >= t_min && t <= t_max {
            let x = ray.origin.x + t * ray.direction.x;
            let y = ray.origin.y + t * ray.direction.y;

            if (x >= self.x.0 && x <= self.x.1) && (y >= self.y.0 && y <= self.y.1) {
                return Some(HitRecord {
                    t,
                    point: ray.point_at(t),
                    normal: Vec3::new(0.0, 0.0, 1.0),
                    material: self.material.as_ref(),
                    uv: (
                        (x - self.x.0) / (self.x.1 - self.x.0),
                        (x - self.y.0) / (self.y.1 - self.y.0),
                    ),
                });
            }
        }

        None
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x.0, self.y.0, self.z - 0.0001),
            Vec3::new(self.x.1, self.y.1, self.z + 0.0001),
        ))
    }
}

/// rectangle in xz plane
pub struct XzRect<T> {
    x: (f32, f32),
    y: f32,
    z: (f32, f32),
    material: Box<T>,
}

impl<T> XzRect<T> {
    /// construct new rectangle with x(min, max), z(min, max), and y
    pub fn new(x: (f32, f32), z: (f32, f32), y: f32, mat: T) -> Self {
        XzRect {
            x,
            y,
            z,
            material: Box::new(mat),
        }
    }
}

impl<T> Hitable for XzRect<T>
where
    T: Material,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.y - ray.origin.y) / ray.direction.y;
        if t >= t_min && t <= t_max {
            let x = ray.origin.x + t * ray.direction.x;
            let z = ray.origin.z + t * ray.direction.z;

            if (x >= self.x.0 && x <= self.x.1) && (z >= self.z.0 && z <= self.z.1) {
                return Some(HitRecord {
                    t,
                    point: ray.point_at(t),
                    normal: Vec3::new(0.0, 1.0, 0.0),
                    material: self.material.as_ref(),
                    uv: (
                        (x - self.x.0) / (self.x.1 - self.x.0),
                        (z - self.z.0) / (self.z.1 - self.z.0),
                    ),
                });
            }
        }

        None
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x.0, self.y - 0.0001, self.z.0),
            Vec3::new(self.x.1, self.y + 0.0001, self.z.1),
        ))
    }
}

/// rectangle in yz plane
pub struct YzRect<T> {
    x: f32,
    y: (f32, f32),
    z: (f32, f32),
    material: Box<T>,
}

impl<T> YzRect<T> {
    /// construct new rectangle with y(min, max), z(min, max), and x
    pub fn new(y: (f32, f32), z: (f32, f32), x: f32, mat: T) -> Self {
        YzRect {
            x,
            y,
            z,
            material: Box::new(mat),
        }
    }
}

impl<T> Hitable for YzRect<T>
where
    T: Material,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.x - ray.origin.x) / ray.direction.x;
        if t >= t_min && t <= t_max {
            let y = ray.origin.y + t * ray.direction.y;
            let z = ray.origin.z + t * ray.direction.z;

            if (y >= self.y.0 && y <= self.y.1) && (z >= self.z.0 && z <= self.z.1) {
                return Some(HitRecord {
                    t,
                    point: ray.point_at(t),
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    material: self.material.as_ref(),
                    uv: (
                        (y - self.y.0) / (self.y.1 - self.y.0),
                        (z - self.z.0) / (self.z.1 - self.z.0),
                    ),
                });
            }
        }

        None
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x - 0.0001, self.y.0, self.z.0),
            Vec3::new(self.x + 0.0001, self.y.1, self.z.1),
        ))
    }
}
