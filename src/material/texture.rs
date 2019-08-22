use crate::{Color, Vec3};

/// material texture trait
pub trait Texture: Sync {
    /// return color at `point` with `u` and `v`
    fn value(&self, u: f32, v: f32, point: Vec3) -> Color;
}

/// constant color texture
#[derive(Clone)]
pub struct ConstantTexture(pub Color);

impl Texture for ConstantTexture {
    fn value(&self, _: f32, _: f32, _: Vec3) -> Color {
        self.0
    }
}

/// checker pattern texture
#[derive(Clone)]
pub struct CheckerTexture(pub ConstantTexture, pub ConstantTexture);

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Color {
        let sines = (point.x * 10.0).sin() * (point.y * 10.0).sin() * (point.z * 10.0).sin();

        if sines > 0.0 {
            self.0.value(u, v, point)
        } else {
            self.1.value(u, v, point)
        }
    }
}
