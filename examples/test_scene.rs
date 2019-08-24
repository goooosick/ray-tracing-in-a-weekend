use rand::prelude::*;
use rayon::prelude::*;
use rtw::material::*;
use rtw::shape::{transform::*, *};
use rtw::{accel::BVH, *};

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

fn final_scene(time_start: f32, time_end: f32) -> HitableList<'static> {
    let mut list = HitableList::default();

    let white = Lambertian::new(ConstantTexture(Vec3::new(0.73, 0.73, 0.73)));
    let ground = Lambertian::new(ConstantTexture(Vec3::new(0.48, 0.83, 0.53)));
    let light = DiffuseLight::new(ConstantTexture(Vec3::new(7.0, 7.0, 7.0)));

    // light
    list.push(XzRect::new((123.0, 423.0), (147.0, 412.0), 554.0, light));

    // ground
    {
        let mut box_list = HitableList::default();
        for i in 0..20 {
            for j in 0..20 {
                let w = 100.0;
                let min = Vec3::new(-1000.0 + i as f32 * w, 0.0, -1000.0 + j as f32 * w);
                let max = Vec3::new(min.x + w, 100.0 * (randf() + 0.01), min.z + w);
                box_list.push(BBox::new(min, max, ground.clone()));
            }
        }
        list.push(BVH::from_list(box_list, time_start, time_end));
    }

    // large spheres
    {
        let center = Vec3::new(400.0, 400.0, 400.0);
        list.push(MovingSphere::new(
            center,
            Vec3::new(30.0, 0.0, 0.0),
            time_start,
            time_end,
            50.0,
            Lambertian::new(ConstantTexture(Vec3::new(0.8, 0.1, 0.1))),
        ));

        list.push(Sphere::new(
            Vec3::new(260.0, 150.0, 45.0),
            50.0,
            Dielectric::new(1.5),
        ));
        list.push(Sphere::new(
            Vec3::new(0.0, 150.0, 145.0),
            50.0,
            Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.0),
        ));
        list.push(Sphere::new(
            Vec3::new(220.0, 280.0, 300.0),
            80.0,
            Lambertian::new(NoiseTexture(0.05)),
        ));

        // venus
        let image = load_image("res/venusmap.jpg");
        list.push(Sphere::new(
            Vec3::new(400.0, 200.0, 400.0),
            100.0,
            Lambertian::new(ImageTexture::new(image)),
        ));
    }

    // with medium
    {
        let boundary = Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
        list.push(Sphere::new(
            Vec3::new(360.0, 150.0, 145.0),
            70.0,
            Dielectric::new(1.5),
        ));
        list.push(ConstantMedium::new(
            boundary,
            0.2,
            ConstantTexture(Vec3::new(0.2, 0.4, 0.9)),
        ));
        // fog
        let boundary = Sphere::new(Vec3::zero(), 5000.0, Dielectric::new(1.5));
        list.push(ConstantMedium::new(
            boundary,
            0.0001,
            ConstantTexture(Vec3::unit()),
        ));
    }

    // shperes cube
    {
        let mut box_list = HitableList::default();
        for _ in 0..1000 {
            box_list.push(Sphere::new(
                Vec3::new(randf(), randf(), randf()) * 165.0,
                10.0,
                white.clone(),
            ));
        }
        let box_list = BVH::from_list(box_list, time_start, time_end);
        list.push(Translate(
            Rotate::around(Axis::Y, box_list, 15.0),
            Vec3::new(-100.0, 270.0, 395.0),
        ));
    }

    list
}

fn randf() -> f32 {
    thread_rng().gen()
}

fn main() {
    let nx = 400;
    let ny = 400;
    let ns = 100;

    let time_start = 0.0;
    let time_end = 1.0;
    let apture = 0.0;
    let focus_dist = 10.0;

    let look_from = Vec3::new(478.0, 278.0, -600.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);

    let cam = Camera::new(look_from, look_at, view_up, 40.0, nx as f32 / ny as f32)
        .apture(apture, focus_dist)
        .period(time_start, time_end);

    let world = BVH::from_list(final_scene(time_start, time_end), time_start, time_end);

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
