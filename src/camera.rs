use ray::Ray;

use std::f32;
use vec::*;

extern crate rand;

pub struct Camera {
    origin: V3,
    starting_pos: V3,
    horizontal: V3,
    vertical: V3,
    u: V3,
    v: V3,
    w: V3,
    lens_radius: f32,
    time: V2, // shutter open/close times
}

impl Camera {
    // fov_deg is top to bottom in degrees
    pub fn new(look_from: V3,
               look_at: V3,
               v_up: V3,
               fov_deg: f32,
               aspect: f32,
               aperture: f32,
               focus_dist: f32,
               time: V2)
               -> Self {
        let theta = fov_deg * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(&v_up, &w));
        let v = cross(&w, &u);
        Camera {
            origin: look_from,
            starting_pos: look_from - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0,
            time: time,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let disk_radius = self.lens_radius * random_in_unit_disk();
        let offset = self.u * disk_radius.0 + self.v * disk_radius.1;
        let time_step = self.time.0 + rand::random::<f32>() * (self.time.1 - self.time.0);
        Ray::new(self.origin + offset,
                 self.starting_pos + s * self.horizontal + t * self.vertical - self.origin - offset,
                 time_step)
    }
}

// https://github.com/rust-lang/rust/issues/28570
#[allow(unused_assignments)]
fn random_in_unit_disk() -> V3 {
    let mut p = V3(0.0, 0.0, 0.0);
    // do-while loop
    while {
              p = 2.0 * V3(rand::random::<f32>(), rand::random::<f32>(), 0.0) - V3(1.0, 1.0, 0.0);
              dot(&p, &p) >= 1.0
          } {}
    p
}
