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

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let mut list = HitableList::default();
    list.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5,
                Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)))));
    list.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0,
                Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))));
    list.push(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5,
                Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3))));
    list.push(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5,
                Box::new(Dielectric::new(1.5))));
    list.push(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45,
                Box::new(Dielectric::new(1.5))));

    let cam = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0)
    );

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

    imgbuf.save("ch9.png").unwrap();
}
