/// RGB color ([0.0-1.0] each channel)
pub type Color = vec::Vec3;

pub use ray::Ray;
pub use vec::Vec3;

mod vec;
mod ray;
