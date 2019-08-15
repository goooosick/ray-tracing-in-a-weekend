use crate::Ray;
use crate::Vec3;

/// simple camera
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    /// construct new camera, located at `origin`.
    /// Film is built from its lower-left `corner`,
    /// horizontal stretch and vertical stretch.
    pub fn new(origin: Vec3, corner: Vec3, horiz: Vec3, verti: Vec3) -> Self {
        Camera {
            origin: origin,
            lower_left_corner: corner,
            horizontal: horiz,
            vertical: verti,
        }
    }

    /// get camera ray, from camera's orgin to uv point on film.
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin,
            self.lower_left_corner
            + u * self.horizontal + v * self.vertical
            - self.origin
        )
    }
}
