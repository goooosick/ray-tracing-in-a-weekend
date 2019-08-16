use crate::{Ray, Vec3, HitRecord};
use crate::random_in_unit_sphere;
use crate::{reflect, refract};

/// object material trait
pub trait Material {
    /// material scatters incident ray
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}

/// scatter record with scattered ray and material attenuation
pub struct ScatterRecord {
    /// scattered ray
    pub scattered: Ray,
    /// material attenuation
    pub attenuation: Vec3,
}

/// lambertian material, scattering ray to random direction and attenuating
#[derive(Clone)]
pub struct Lambertian {
    /// lambertian material attenuation
    pub albedo: Vec3
}

impl Lambertian {
    /// construct new lambertian material
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let target = rec.point + rec.normal + random_in_unit_sphere();
        
        Some(ScatterRecord {
            scattered: Ray::new(rec.point, target - rec.point),
            attenuation: self.albedo,
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
            fuzz: fuzz.min(1.0)
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(ray.direction.normalize(), rec.normal) +
                        self.fuzz * random_in_unit_sphere();

        if reflected.dot(rec.normal) > 0.0 {
            Some(ScatterRecord {
                scattered: Ray::new(rec.point, reflected),
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
                false => (rec.normal, self.ref_index.recip(), -cos)
            }
        };

        if let Some(refracted) = refract(ray.direction, out_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.ref_index);

            if thread_rng().gen::<f32>() > reflect_prob {
                return Some(ScatterRecord {
                    scattered: Ray::new(rec.point, refracted),
                    attenuation
                });
            }
        }

        Some(ScatterRecord {
            scattered: Ray::new(rec.point, reflected),
            attenuation
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
