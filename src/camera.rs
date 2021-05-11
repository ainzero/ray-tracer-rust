use crate::math::{Vector3, Ray};

pub struct Camera {
    camera_plane: Plane
}

impl Camera {
    pub fn centered_in_plane_with_height_and_width(height: f32, width: f32) -> Camera {
        Camera {
            camera_plane: Plane::with_height_and_width(height, width)
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.camera_plane.get_origin(),
            // TODO(Z): look into getting borrows straight with Vector3
            direction: self.camera_plane.get_lower_left_corner() + &((self.camera_plane.get_width() * u) + (self.camera_plane.get_height() * v) - self.camera_plane.get_origin())
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Plane {
    width: Vector3,
    height: Vector3,
    origin: Vector3,
    lower_left_corner: Vector3,
}
impl Plane {
    fn with_height_and_width(height: f32, width: f32) -> Plane {
        Plane {
            height: Vector3::new(0.0,height, 0.0),
            width: Vector3::new(width, 0.0, 0.0),
            origin: Vector3::new(0.0,0.0,0.0),
            lower_left_corner: Vector3::new(width / -2.0, height / -2.0, -1.0)
        }
    }
    fn get_height(&self) -> &Vector3 {
        &self.height
    }
    fn get_width(&self) -> &Vector3 {
        &self.width
    }
    fn get_origin(&self) -> Vector3 {
        self.origin
    }
    fn get_lower_left_corner(&self) -> &Vector3 { &self.lower_left_corner }
}