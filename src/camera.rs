use crate::Ray;
use crate::Vec3;
use crate::random_in_unit_disk;

/// simple camera
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    /// construct new camera
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, fov: f32, aspect: f32, apture: f32, focus_dist: f32) -> Self {
        let half_height = (fov.to_radians() / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        
        Camera {
            origin: look_from,
            lower_left_corner: look_from - (half_width * u + half_height * v + w) * focus_dist,
            horizontal: 2.0 * half_width * u * focus_dist,
            vertical: 2.0 * half_height * v * focus_dist,
            u, v, w,
            lens_radius: apture / 2.0,
        }
    }

    /// get camera ray, from camera's orgin to uv point on film
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset,
            self.lower_left_corner
            + u * self.horizontal + v * self.vertical
            - self.origin - offset
        )
    }
}
