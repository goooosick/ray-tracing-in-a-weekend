use rand::prelude::*;
use rayon::prelude::*;
use rtw::{accel::BVH, material::*, shape::*, *};

use std::time::Instant;

fn color(ray: &Ray, hitable: &dyn Hitable, depth: u32) -> Color {
    if let Some(rec) = hitable.hit(&ray, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some(srec) = rec.material.scatter(ray, &rec) {
                return srec.attenuation * color(&srec.scattered, hitable, depth + 1);
            }
        }

        Vec3::zero()
    } else {
        let unit_dir = ray.direction.normalize();
        let t = 0.5 * (unit_dir.y + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn two_perlin_sphere() -> HitableList<'static> {
    let mut list = HitableList::default();

    list.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(NoiseTexture(5.0)),
    ));
    list.push(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(NoiseTexture(5.0)),
    ));

    list
}

fn main() {
    let nx = 500;
    let ny = 200;
    let ns = 100;

    let time_start = 0.0;
    let time_end = 0.0;
    let apture = 0.0;
    let focus_dist = 10.0;

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);

    let cam = Camera::new(look_from, look_at, view_up, 20.0, nx as f32 / ny as f32)
        .apture(apture, focus_dist)
        .period(time_start, time_end);

    let world = BVH::from_list(two_perlin_sphere(), time_start, time_end);

    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let sample_range = (0..ns).collect::<Vec<_>>();

    let time = Instant::now();

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = x as f32;
        let v = (ny - y - 1) as f32;

        let c = sample_range
            .par_iter()
            .map(|_| {
                let u = (u + thread_rng().gen::<f32>()) / nx as f32;
                let v = (v + thread_rng().gen::<f32>()) / ny as f32;
                let ray = cam.get_ray(u, v);

                color(&ray, &world, 0)
            })
            .sum::<Color>()
            / ns as f32;

        *pixel = image::Rgb(vec_to_rgb(c));
    }

    println!(
        "rendered in {:.02} s",
        time.elapsed().as_millis() as f32 / 1000.0
    );

    imgbuf.save("scene.png").unwrap();
}
