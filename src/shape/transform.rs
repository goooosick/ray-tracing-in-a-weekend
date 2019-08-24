//! hitable wrappers do geometry transformations

use crate::accel::AABB;
use crate::shape::{HitRecord, Hitable};
use crate::{Ray, Vec3};

/// flipping normals
pub struct FlipNormal<T>(pub T);

impl<T> Hitable for FlipNormal<T>
where
    T: Hitable,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.0.hit(ray, t_min, t_max).map(|mut rec| {
            rec.normal = -rec.normal;
            rec
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.0.bounding_box(t0, t1)
    }
}

/// translation
pub struct Translate<T>(pub T, pub Vec3);

impl<T> Hitable for Translate<T>
where
    T: Hitable,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let ray = Ray::new(ray.origin - self.1, ray.direction, ray.time);

        self.0.hit(&ray, t_min, t_max).map(|mut rec| {
            rec.point += self.1;
            rec
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.0.bounding_box(t0, t1).map(|mut bbox| {
            bbox.min += self.1;
            bbox.max += self.1;
            bbox
        })
    }
}

/// rotation axises - x y z
#[derive(Copy, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// rotation
pub struct Rotate<T> {
    sin_theta: f32,
    cos_theta: f32,
    axis: Axis,
    hitable: T,
    bbox: Option<AABB>,
}

impl<T> Rotate<T>
where
    T: Hitable,
{
    /// rotate object around `axis`
    pub fn around(axis: Axis, hitable: T, angle: f32) -> Self {
        use std::f32::{MAX, MIN};

        let sin_theta = angle.to_radians().sin();
        let cos_theta = angle.to_radians().cos();

        // smelly code :D
        let bbox = hitable.bounding_box(0.0, 1.0).map(|mut bbox| {
            let mut min = Vec3::new(MAX, MAX, MAX);
            let mut max = Vec3::new(MIN, MIN, MIN);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let index = Vec3::new(i as f32, j as f32, k as f32);
                        let point = index * bbox.max + (Vec3::unit() - index) * bbox.min;
                        let point = rotate_point(point, sin_theta, cos_theta, axis);
                        max = max.max(point);
                        min = min.min(point);
                    }
                }
            }

            bbox.min = min;
            bbox.max = max;
            bbox
        });

        Rotate {
            sin_theta,
            cos_theta,
            axis,
            hitable,
            bbox,
        }
    }
}

impl<T> Hitable for Rotate<T>
where
    T: Hitable,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin = rotate_point(ray.origin, -self.sin_theta, self.cos_theta, self.axis);
        let direction = rotate_point(ray.direction, -self.sin_theta, self.cos_theta, self.axis);
        let ray = Ray::new(origin, direction, ray.time);

        self.hitable.hit(&ray, t_min, t_max).map(|mut rec| {
            rec.point = rotate_point(rec.point, self.sin_theta, self.cos_theta, self.axis);
            rec.normal = rotate_point(rec.normal, self.sin_theta, self.cos_theta, self.axis);
            rec
        })
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        self.bbox.clone()
    }
}

fn rotate_point(p: Vec3, sin: f32, cos: f32, axis: Axis) -> Vec3 {
    match axis {
        Axis::X => Vec3::new(p.x, cos * p.y + sin * p.z, cos * p.z - sin * p.y),
        Axis::Y => Vec3::new(cos * p.x + sin * p.z, p.y, cos * p.z - sin * p.x),
        Axis::Z => Vec3::new(cos * p.x + sin * p.y, cos * p.y - sin * p.x, p.z),
    }
}
