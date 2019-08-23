use crate::random_in_unit_sphere;
use crate::shape::HitRecord;
use crate::{Ray, Vec3};

pub use texture::*;

mod noise;
mod texture;

/// reflect incident ray `v` with surface normal `n`
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

/// refract ray when possible
pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    match discriminant > 0.0 {
        true => Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt()),
        false => None,
    }
}

/// scatter record with scattered ray and material attenuation
pub struct ScatterRecord {
    /// scattered ray
    pub scattered: Ray,
    /// material attenuation
    pub attenuation: Vec3,
}

/// object material trait
pub trait Material: Sync {
    /// material scatters incident ray
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}

/// lambertian material, scattering ray to random direction and attenuating
#[derive(Clone)]
pub struct Lambertian<T> {
    /// lambertian material attenuation
    pub albedo: Box<T>,
}

impl<T> Lambertian<T> {
    /// construct new lambertian material
    pub fn new(albedo: T) -> Self {
        Lambertian {
            albedo: Box::new(albedo),
        }
    }
}

impl<T> Material for Lambertian<T>
where
    T: Texture,
{
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let target = rec.point + rec.normal + random_in_unit_sphere();

        Some(ScatterRecord {
            scattered: Ray::new(rec.point, target - rec.point, ray.time),
            attenuation: self.albedo.value(0.0, 0.0, rec.point),
        })
    }
}

/// metal material, reflecting ray deterministically with attenuation
#[derive(Clone)]
pub struct Metal {
    /// metal material attenuation
    pub albedo: Vec3,
    /// fuzziness of reflected ray
    pub fuzz: f32,
}

impl Metal {
    /// construct new metal material
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected =
            reflect(ray.direction.normalize(), rec.normal) + self.fuzz * random_in_unit_sphere();

        if reflected.dot(rec.normal) > 0.0 {
            Some(ScatterRecord {
                scattered: Ray::new(rec.point, reflected, ray.time),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

/// dielectric material like glass
#[derive(Clone)]
pub struct Dielectric {
    /// refractive index
    pub ref_index: f32,
}

impl Dielectric {
    /// construct new dielectric material
    pub fn new(ref_index: f32) -> Self {
        Dielectric { ref_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        use rand::prelude::*;

        let reflected = reflect(ray.direction, rec.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let cos = ray.direction.dot(rec.normal) / ray.direction.norm();

        let (out_normal, ni_over_nt, cosine) = {
            match ray.direction.dot(rec.normal) > 0.0 {
                true => (-rec.normal, self.ref_index, self.ref_index * cos),
                false => (rec.normal, self.ref_index.recip(), -cos),
            }
        };

        if let Some(refracted) = refract(ray.direction, out_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.ref_index);

            if thread_rng().gen::<f32>() > reflect_prob {
                return Some(ScatterRecord {
                    scattered: Ray::new(rec.point, refracted, ray.time),
                    attenuation,
                });
            }
        }

        Some(ScatterRecord {
            scattered: Ray::new(rec.point, reflected, ray.time),
            attenuation,
        })
    }
}

/// schlick approximation
/// see: https://www.youtube.com/watch?v=iKNSPETJNgo
fn schlick(cosine: f32, ref_index: f32) -> f32 {
    let r0 = (1.0 - ref_index) / (1.0 + ref_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
