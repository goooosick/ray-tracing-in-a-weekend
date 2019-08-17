use rtw::*;
use rand::prelude::*;
use rayon::prelude::*;

fn color(ray: &Ray, hitable: &HitableList, depth: u32) -> Color {
    if let Some(rec) = hitable.hit(&ray, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some(srec) = rec.material.scatter(ray, &rec) {
                return srec.attenuation *
                    color(&srec.scattered, hitable, depth + 1);
            }
        }
        
        Vec3::zero()
    } else {
        let unit_dir = ray.direction.normalize();
        let t = 0.5 * (unit_dir.y + 1.0);
        
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

#[inline]
fn randf() -> f32 {
    thread_rng().gen::<f32>()
}

fn build_scene(n: i32) -> HitableList<'static> {
    assert!(n > 0);

    let mut list = HitableList::default();

    list.push(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0,
        Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))));

    for i in -n..n {
        for j in -n..n {
            let prob = randf();
            let center = Vec3::new(
                i as f32 + 0.9 * randf(),
                0.2,
                j as f32 + 0.9 * randf(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                if prob < 0.8 {
                    // diffuse
                    list.push(Sphere::new(center, 0.2,
                        Box::new(Lambertian::new(Vec3::new(
                            randf() * randf(),
                            randf() * randf(),
                            randf() * randf(),
                        )))));
                } else if prob < 0.95 {
                    // metal
                    list.push(Sphere::new(center, 0.2,
                        Box::new(Metal::new(Vec3::new(
                            0.5 * (1.0 + randf()),
                            0.5 * (1.0 + randf()),
                            0.5 * (1.0 + randf()),
                        ), 0.5 * randf()))));
                } else {
                    // glass
                    list.push(Sphere::new(center, 0.2,
                        Box::new(Dielectric::new(1.5))));
                }
            }
        }
    }

    list.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0,
                Box::new(Dielectric::new(1.5))));
    list.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0,
                Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)))));
    list.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0,
                Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))));

    list
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let list = build_scene(11);

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);

    let cam = Camera::new(look_from, look_at, view_up,
        20.0, nx as f32 / ny as f32,
        0.1, 10.0);

    let sample_range = (0..ns).collect::<Vec<_>>();

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = x as f32;
        let v = (ny - y - 1) as f32;

        let c = sample_range.par_iter()
            .map(|_| {
                let u = (u + thread_rng().gen::<f32>()) / nx as f32;
                let v = (v + thread_rng().gen::<f32>()) / ny as f32;
                let ray = cam.get_ray(u, v);


                color(&ray, &list, 0)
            })
            .sum::<Color>() / ns as f32;

        *pixel = image::Rgb(vec_to_rgb(c));
    }

    imgbuf.save("ch12.png").unwrap();
}
