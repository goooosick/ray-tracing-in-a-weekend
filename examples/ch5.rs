use rtw::*;

fn color(ray: &Ray, hitable: &HitableList) -> Color {
    if let Some(rec) = hitable.hit(&ray, 0.0, std::f32::MAX) {
        0.5 * (rec.normal + Vec3::unit())
    } else {
        let unit_dir = ray.direction.normalize();
        let t = 0.5 * (unit_dir.y + 1.0);
        
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let nx = 200;
    let ny = 100;

    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let corner = Vec3::new(-2.0, -1.0, -1.0); 
    let horiz = Vec3::new(4.0, 0.0, 0.0);
    let verti = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut list = HitableList::default();
    list.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5,
                Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)))));
    list.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0,
                Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)))));

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = x as f32 / nx as f32;
        let v = (ny - y) as f32 / ny as f32;

        let r = Ray::new(origin, corner + u * horiz + v * verti);
        let c = color(&r, &list);

        *pixel = image::Rgb(vec_to_rgb(c));
    }

    imgbuf.save("ch5.png").unwrap();
}
