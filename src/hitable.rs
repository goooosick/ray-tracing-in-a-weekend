use crate::Vec3;
use crate::Ray;

/// record for ray object intersection
#[derive(Debug)]
pub struct HitRecord {
    /// ray distance
    pub t: f32,
    /// hit point
    pub point: Vec3,
    /// surface normal on hit point
    pub normal: Vec3,
}

/// hitable object trait
pub trait Hitable {
    /// test ray object intersection constrained  by `t_min` and `t_max`
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

/// a list of hitable objects
#[derive(Default)]
pub struct HitableList<'a> {
    list: Vec<Box<Hitable + 'a>>
}

unsafe impl<'a> Sync for HitableList<'a> {}

impl<'a> HitableList<'a> {
    /// add a hitable object
    pub fn push<T: 'a + Hitable>(&mut self, hitable: T) {
        self.list.push(Box::new(hitable));
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
}
