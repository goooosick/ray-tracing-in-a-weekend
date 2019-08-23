use rand::prelude::*;
use rayon::prelude::*;
use rtw::{accel::BVH, material::*, shape::*, *};

use std::time::Instant;

fn color(ray: &Ray, hitable: &dyn Hitable, depth: u32) -> Color {
    if let Some(rec) = hitable.hit(&ray, 0.001, std::f32::MAX) {
        let emitted = rec.material.emitted(rec.uv.0, rec.uv.1, rec.point);

        if depth < 50 {
            if let Some(srec) = rec.material.scatter(ray, &rec) {
                return emitted + srec.attenuation * color(&srec.scattered, hitable, depth + 1);
            }
        }

        emitted
    } else {
        Vec3::zero()
    }
}

fn cornell_box() -> HitableList<'static> {
    let mut list = HitableList::default();

    let light = DiffuseLight::new(ConstantTexture(Vec3::new(15.0, 15.0, 15.0)));
    let white = Lambertian::new(ConstantTexture(Vec3::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(ConstantTexture(Vec3::new(0.12, 0.45, 0.15)));
    let red = Lambertian::new(ConstantTexture(Vec3::new(0.65, 0.05, 0.05)));

    list.push(XzRect::new((213.0, 343.0), (227.0, 332.0), 554.0, light));
    list.push(XzRect::new((0.0, 555.0), (0.0, 555.0), 0.0, white.clone()));
    list.push(YzRect::new((0.0, 555.0), (0.0, 555.0), 0.0, red));
    list.push(FlipNormal(YzRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        green,
    )));
    list.push(FlipNormal(XzRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    )));
    list.push(FlipNormal(XyRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    )));

    list
}

fn main() {
    let nx = 800;
    let ny = 800;
    let ns = 100;

    let time_start = 0.0;
    let time_end = 0.0;
    let apture = 0.0;
    let focus_dist = 10.0;

    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);

    let cam = Camera::new(look_from, look_at, view_up, 40.0, nx as f32 / ny as f32)
        .apture(apture, focus_dist)
        .period(time_start, time_end);

    let world = BVH::from_list(cornell_box(), time_start, time_end);

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
