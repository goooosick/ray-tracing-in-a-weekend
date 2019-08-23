#![allow(clippy::match_bool)]

/// RGB color ([0.0-1.0] each channel)
pub type Color = vec::Vec3;

pub use camera::Camera;
pub use ray::Ray;
pub use vec::Vec3;

#[doc(inline)]
pub use material::Material;
#[doc(inline)]
pub use shape::Hitable;

mod camera;
mod ray;
mod vec;

pub mod accel;
pub mod material;
pub mod shape;

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

/// generate random point in unit disk
pub fn random_in_unit_disk() -> Vec3 {
    use rand::prelude::*;

    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0);
        let p = 2.0 * p - Vec3::new(1.0, 1.0, 0.0);

        if p.norm_squared() <= 1.0 {
            return p;
        }
    }
}

/// load image file, convert to `Color` matrix
pub fn load_image<T: AsRef<str>>(path: T) -> Vec<Vec<Color>> {
    use image::GenericImageView;

    let img = image::open(path.as_ref()).unwrap();
    let dim = img.dimensions();

    let mut image = vec![vec![Color::zero(); dim.1 as usize]; dim.0 as usize];
    img.pixels().for_each(|(i, j, pixel)| {
        image[i as usize][j as usize] = Color::new(
            f32::from(pixel[0]),
            f32::from(pixel[1]),
            f32::from(pixel[2]),
        ) / 255.0;
    });

    image
}
