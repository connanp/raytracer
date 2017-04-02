use ray::Ray;

use std::f32;
use vec::*;

pub struct Camera {
    origin: V3,
    starting_pos: V3,
    horizontal: V3,
    vertical: V3,
}

impl Camera {
    // fov_deg is top to bottom in degrees
    pub fn new(look_from: V3, look_at: V3, v_up: V3, fov_deg: f32, aspect: f32) -> Self {
        let theta = fov_deg * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(&v_up, &w));
        let v = cross(&w, &u);
        Camera {
            origin: look_from,
            starting_pos: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            a: self.origin,
            b: self.starting_pos + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
