use super::{Material, ScatterRecord, Texture};
use crate::shape::HitRecord;
use crate::{Color, Ray, Vec3};

/// diffuse light (modeled as material)
#[derive(Clone)]
pub struct DiffuseLight<T> {
    /// light emitting texture
    pub emit: T,
}

impl<T> DiffuseLight<T> {
    /// construct new diffuse light material
    pub fn new(emit: T) -> Self {
        DiffuseLight { emit }
    }
}

impl<T> Material for DiffuseLight<T>
where
    T: Texture,
{
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, u: f32, v: f32, point: Vec3) -> Color {
        self.emit.value(u, v, point)
    }
}
