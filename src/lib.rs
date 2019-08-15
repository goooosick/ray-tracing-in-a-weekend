/// RGB color ([0.0-1.0] each channel)
pub type Color = vec::Vec3;

pub use ray::Ray;
pub use vec::Vec3;
pub use hitable::*;
pub use sphere::Sphere;

mod vec;
mod ray;
mod hitable;
mod sphere;

/// convert Vec3 to RGB8([u8; 3])
pub fn vec_to_rgb(c: Color) -> [u8; 3] {
    let c = c.clamp(0.0, 1.0) * 255.99;
    [c[0] as u8, c[1] as u8, c[2] as u8]
}
