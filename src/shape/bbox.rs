use super::rect::*;
use super::transform::FlipNormal;
use super::{HitRecord, HitableList};
use crate::accel::AABB;
use crate::{Hitable, Material, Ray, Vec3};

/// 3-dimension box
pub struct BBox<'a> {
    pmin: Vec3,
    pmax: Vec3,
    hitable: HitableList<'a>,
}

impl<'a> BBox<'a> {
    /// construct new box using min, max corner points
    pub fn new(pmin: Vec3, pmax: Vec3, material: impl Material + Clone + 'a) -> Self {
        let mut hitable = HitableList::default();

        // front
        hitable.push(XyRect::new(
            (pmin.x, pmax.x),
            (pmin.y, pmax.y),
            pmax.z,
            material.clone(),
        ));
        // back
        hitable.push(FlipNormal(XyRect::new(
            (pmin.x, pmax.x),
            (pmin.y, pmax.y),
            pmin.z,
            material.clone(),
        )));
        // right
        hitable.push(YzRect::new(
            (pmin.y, pmax.y),
            (pmin.z, pmax.z),
            pmax.x,
            material.clone(),
        ));
        // left
        hitable.push(FlipNormal(YzRect::new(
            (pmin.y, pmax.y),
            (pmin.z, pmax.z),
            pmin.x,
            material.clone(),
        )));
        // top
        hitable.push(XzRect::new(
            (pmin.x, pmax.x),
            (pmin.z, pmax.z),
            pmax.y,
            material.clone(),
        ));
        // bottom
        hitable.push(FlipNormal(XzRect::new(
            (pmin.x, pmax.x),
            (pmin.z, pmax.z),
            pmin.y,
            material.clone(),
        )));

        BBox {
            pmin,
            pmax,
            hitable,
        }
    }
}

impl<'a> Hitable for BBox<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hitable.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(AABB::new(self.pmin, self.pmax))
    }
}
