extern crate rand;
use std::ops::{Add, Neg, Sub, Mul, Div};
use rand::Rng;

// Vec3
#[derive(Debug, PartialEq, Clone, Copy)]
struct V3(f32, f32, f32);

impl V3 {
    fn length(self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    fn squared_length(self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    fn unit_vector(&mut self) -> () {
        let k: f32 = 1.0 / self.length();
        self.0 *= k;
        self.1 *= k;
        self.2 *= k;
    }
}

fn dot(v1: &V3, v2: &V3) -> f32 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

fn cross(v1: &V3, v2: &V3) -> V3 {
    V3(v1.1 * v2.2 - v1.2 * v2.1,
       -(v1.0 * v2.2 - v1.2 * v2.0),
       v1.0 * v2.1 - v1.1 * v2.0)
}

fn unit_vector(v: &V3) -> V3 {
    *v / v.length()
}

impl Add for V3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        V3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Neg for V3 {
    type Output = Self;

    fn neg(self) -> Self {
        V3(-self.0, -self.1, -self.2)
    }
}

impl Sub for V3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        V3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul for V3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        V3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f32> for V3 {
    type Output = V3;

    fn mul(self, rhs: f32) -> Self {
        V3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<V3> for f32 {
    type Output = V3;

    fn mul(self, rhs: V3) -> V3 {
        V3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Div for V3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        V3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl Div<f32> for V3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        V3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Div<V3> for f32 {
    type Output = V3;

    fn div(self, rhs: V3) -> V3 {
        V3(self / rhs.0, self / rhs.1, self / rhs.2)
    }
}

#[derive(Debug, Clone, Copy)]
struct Ray {
    a: V3,
    b: V3,
}

impl Ray {
    fn origin(&self) -> &V3 {
        &self.a
    }

    fn direction(&self) -> &V3 {
        &self.b
    }

    fn point_at(self, t: f32) -> V3 {
        self.a + t * self.b
    }
}

#[derive(Debug, Clone, Copy)]
struct HitRecord {
    t: f32,
    p: V3,
    normal: V3,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: 0.0,
            p: V3(0.0, 0.0, 0.0),
            normal: V3(0.0, 0.0, 0.0),
        }
    }
}

trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Debug, Clone, Copy)]
struct Sphere {
    center: V3,
    radius: f32,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let b = dot(&oc, r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let mut rec = HitRecord::new();
                rec.t = t;
                rec.p = r.point_at(t);
                rec.normal = (rec.p - self.center) / self.radius;
                return Some(rec);
            }
            // other direction
            let t2 = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let mut rec = HitRecord::new();
                rec.t = t2;
                rec.p = r.point_at(t2);
                rec.normal = (rec.p - self.center) / self.radius;
                return Some(rec);
            }
        }

        None
    }
}

#[derive(Debug)]
struct Hitables<T>(Vec<T>) where T: Hitable;

impl<T> Hitables<T>
    where T: Hitable
{
    fn new() -> Self {
        Hitables(vec![])
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec = HitRecord::new();
        let mut closest = Some(t_max as f64);
        for obj in &self.0 {
            match obj.hit(r, t_min, closest.unwrap() as f32) {
                Some(o) => {
                    closest = Some(o.t as f64);
                    rec = o
                }
                _ => (),
            }
        }
        match closest {
            Some(t) if t < t_max as f64 && t > t_min as f64 => Some(rec),
            _ => None,
        }
    }
}

#[allow(dead_code)]
fn color_normal<T: Hitable>(r: &Ray, world: &Hitables<T>) -> V3 {
    match world.hit(r, 0.0, std::f32::MAX) {
        Some(rec) => 0.5 * V3(rec.normal.0 + 1.0, rec.normal.1 + 1.0, rec.normal.2 + 1.0),
        None => {
            let unit_d = unit_vector(r.direction());
            let t = 0.5 * (unit_d.1 + 1.0);
            (1.0 - t) * V3(1.0, 1.0, 1.0) + t * V3(0.5, 0.7, 1.0)
        }
    }
}

fn color<T: Hitable>(r: &Ray, world: &Hitables<T>) -> V3 {
    // t_min at 0.001 to avoid 'shadow acne' problem
    match world.hit(r, 0.001, std::f32::MAX) {
        Some(rec) => {
            let target = rec.p + rec.normal + random_in_unit_sphere();
            0.5 *
            color(&Ray {
                       a: rec.p,
                       b: target - rec.p,
                   },
                  world)
        }
        None => {
            let unit_d = unit_vector(r.direction());
            let t = 0.5 * (unit_d.1 + 1.0);
            (1.0 - t) * V3(1.0, 1.0, 1.0) + t * V3(0.5, 0.7, 1.0)
        }
    }
}

struct Camera {
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

// https://github.com/rust-lang/rust/issues/28570
#[allow(unused_assignments)]
fn random_in_unit_sphere() -> V3 {
    let mut p = V3(0.0, 0.0, 0.0);
    // do-while loop
    while {
              p = 2.0 *
                  V3(rand::random::<f32>(),
                     rand::random::<f32>(),
                     rand::random::<f32>()) - V3(1.0, 1.0, 1.0);
              p.squared_length() >= 1.0
          } {}
    p
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let camera = Camera::new();
    let world = Hitables(vec![Sphere {
                                  center: V3(0.0, 0.0, -1.0),
                                  radius: 0.5,
                              },
                              Sphere {
                                  center: V3(0.0, -100.5, -1.0),
                                  radius: 100.0,
                              }]);
    println!("P3\n{} {}\n255", nx, ny);

    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut c = V3(0., 0., 0.);
            for _ in 0..ns {
                let u = (rand::random::<f32>() + x as f32) / nx as f32;
                let v = (rand::random::<f32>() + y as f32) / ny as f32;
                let r = camera.get_ray(u, v);
                c = c + color(&r, &world);
            }
            c = c / ns as f32;
            // gamma correction of value 2
            c = V3(c.0.sqrt(), c.1.sqrt(), c.2.sqrt());
            // clip to rgb max
            let ir: i32 = (255.99 * c.0) as i32;
            let ig: i32 = (255.99 * c.1) as i32;
            let ib: i32 = (255.99 * c.2) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
