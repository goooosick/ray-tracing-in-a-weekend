use super::noise::Perlin;
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

/// perlin noise texture
#[derive(Clone)]
pub struct NoiseTexture(pub f32);

impl Texture for NoiseTexture {
    fn value(&self, _: f32, _: f32, point: Vec3) -> Color {
        // Vec3::unit() * (Perlin::noise(point * self.0) + 1.0) * 0.5
        Vec3::unit() * (Perlin::turb(point * self.0) + 1.0) * 0.5
        // Vec3::unit() * (1.0 + (10.0 * Perlin::turb(point) + self.0 * point.z).sin()) * 0.5
    }
}

/// image texture
pub struct ImageTexture {
    image: Vec<Vec<Color>>,
    width: usize,
    height: usize,
}

impl ImageTexture {
    /// construct new ImageTexture fron image data
    pub fn new(image: Vec<Vec<Color>>) -> Self {
        let width = image.len();
        let height = image[0].len();
        ImageTexture {
            image,
            width,
            height,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _: Vec3) -> Color {
        let x = (u * self.width as f32) as usize;
        let y = ((1.0 - v) * self.height as f32) as usize;
        let x = x.max(0).min(self.width - 1);
        let y = y.max(0).min(self.height - 1);
        self.image[x][y]
    }
}
