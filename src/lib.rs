#![allow(clippy::match_bool)]

/// RGB color ([0.0-1.0] each channel)
pub type Color = vec::Vec3;

pub use ray::Ray;
pub use vec::{Vec3, reflect, refract};
pub use hitable::*;
pub use sphere::Sphere;
pub use camera::Camera;
pub use material::*;

mod vec;
mod ray;
mod hitable;
mod sphere;
mod camera;
mod material;

/// convert Vec3 to RGB8([u8; 3])
pub fn vec_to_rgb(c: Color) -> [u8; 3] {
    // gamma correction
    let c = Color::new(c.x.sqrt(), c.y.sqrt(), c.z.sqrt());
    // convert to RGB 256
    let c = c.clamp(0.0, 1.0) * 255.99;
    [c[0] as u8, c[1] as u8, c[2] as u8]
}

/// generate random point in unit shpere
pub fn random_in_unit_sphere() -> Vec3 {
    use rand::prelude::*;

    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
        let p = 2.0 * p - Vec3::unit();

        if p.norm_squared() <= 1.0 {
            return p;
        }
    }
}
