use rtw::*;

fn color(ray: &Ray) -> Color {
    let unit_dir = ray.direction.normalize();
    let t = 0.5 * (unit_dir[1] + 1.0);
    
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn vec_to_rgb(c: Color) -> [u8; 3] {
    let c = c.clamp(0.0, 1.0) * 255.99;
    [c[0] as u8, c[1] as u8, c[2] as u8]
}

fn main() {
    let nx = 200;
    let ny = 100;

    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let corner = Vec3::new(-2.0, -1.0, -1.0); 
    let horiz = Vec3::new(4.0, 0.0, 0.0);
    let verti = Vec3::new(0.0, 2.0, 0.0);

    let origin = Vec3::new(0.0, 0.0, 0.0);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = x as f32 / nx as f32;
        let v = (ny - y) as f32 / ny as f32;
        let r = Ray::new(origin, corner + u * horiz + v * verti);

        let c = color(&r);

        *pixel = image::Rgb(vec_to_rgb(c));
    }

    imgbuf.save("test.png").unwrap();
}
