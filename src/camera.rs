use vec::V3;
use ray::Ray;

pub struct Camera {
    origin: V3,
    starting_pos: V3,
    horizontal: V3,
    vertical: V3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            origin: V3(0.0, 0.0, 0.0),
            starting_pos: V3(-2.0, -1.0, -1.0),
            horizontal: V3(4.0, 0.0, 0.0),
            vertical: V3(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            a: self.origin,
            b: self.starting_pos + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
