use crate::random_in_unit_disk;
use crate::Ray;
use crate::Vec3;

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
    time_start: f32,
    time_interval: f32,
}

impl Camera {
    /// construct new camera
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, fov: f32, aspect: f32) -> Self {
        let half_height = (fov.to_radians() / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            origin: look_from,
            lower_left_corner: look_from - (half_width * u + half_height * v + w),
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,

            u,
            v,
            w,

            lens_radius: 0.0,
            time_start: 0.0,
            time_interval: 0.0,
        }
    }

    /// defocus blur effect
    pub fn apture(mut self, apture: f32, focus_dist: f32) -> Self {
        self.lens_radius = apture / 2.0;
        self.lower_left_corner +=
            (1.0 - focus_dist) * (self.horizontal / 2.0 + self.vertical / 2.0 + self.w);
        self.horizontal *= focus_dist;
        self.vertical *= focus_dist;
        self
    }

    /// motion blur effect
    pub fn period(mut self, t0: f32, t1: f32) -> Self {
        self.time_start = t0;
        self.time_interval = t1;
        self
    }

    /// get camera ray, from camera's orgin to uv point on film
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        use rand::prelude::*;

        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let time = self.time_start + self.time_interval * thread_rng().gen::<f32>();

        Ray::new(self.origin + offset, self.point_at(u, v) - offset, time)
    }

    #[inline(always)]
    fn point_at(&self, u: f32, v: f32) -> Vec3 {
        self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
    }
}
