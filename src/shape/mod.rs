use crate::accel::accumulate_aabbs;
use crate::accel::AABB;
use crate::Material;
use crate::Ray;
use crate::Vec3;

pub use bbox::BBox;
pub use medium::ConstantMedium;
pub use rect::*;
pub use sphere::{MovingSphere, Sphere};

mod bbox;
mod medium;
mod rect;
mod sphere;

pub mod transform;

/// record for ray object intersection
pub struct HitRecord<'a> {
    /// ray distance
    pub t: f32,
    /// hit point
    pub point: Vec3,
    /// surface normal on hit point
    pub normal: Vec3,
    /// material pointer
    pub material: &'a dyn Material,
    /// uv value on object
    pub uv: (f32, f32),
}

/// hitable object trait
pub trait Hitable: Sync {
    /// test ray object intersection constrained  by `t_min` and `t_max`
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    /// get aabb of object in time t0-t1
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

/// a list of hitable objects
#[derive(Default)]
pub struct HitableList<'a> {
    list: Vec<Box<dyn Hitable + 'a>>,
}

impl<'a> HitableList<'a> {
    /// add a hitable object
    pub fn push(&mut self, hitable: impl Hitable + 'a) {
        self.list.push(Box::new(hitable));
    }

    /// convert into vector of objects
    pub fn into_vec(self) -> Vec<Box<dyn Hitable + 'a>> {
        self.list
    }
}

impl<'a> Hitable for HitableList<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.list
            .iter()
            .filter_map(|h| h.hit(ray, t_min, t_max))
            // .filter(|t| !t.is_nan())
            .min_by(|h1, h2| h1.t.partial_cmp(&h2.t).unwrap())
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        accumulate_aabbs(&self.list, t0, t1)
    }
}
