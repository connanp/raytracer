use std::ops::{Add, Neg, Sub, Mul, Div};

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

fn hit_sphere(center: &V3, radius: f32, r: &Ray) -> f32 {
    let oc = *r.origin() - *center;
    let a = dot(r.direction(), r.direction());
    let b = 2.0 * dot(&oc, r.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }

}

fn color(r: &Ray) -> V3 {
    let t = hit_sphere(&V3(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let normal = r.point_at(t) - V3(0.0, 0.0, -1.0);
        let n = unit_vector(&normal);
        return 0.5 * V3(n.0 + 1.0, n.1 + 1.0, n.2 + 1.0);
    }
    let unit_d = unit_vector(r.direction());
    let t = 0.5 * (unit_d.1 + 1.0);
    (1.0 - t) * V3(1.0, 1.0, 1.0) + t * V3(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;
    let bot_left = V3(-2.0, -1.0, -1.0);
    let horizontal = V3(4.0, 0.0, 0.0);
    let vertical = V3(0.0, 2.0, 0.0);
    let origin = V3(0.0, 0.0, 0.0);

    println!("P3\n{} {}\n255", nx, ny);

    for y in (0..ny).rev() {
        for x in 0..nx {
            let u = x as f32 / nx as f32;
            let v = y as f32 / ny as f32;
            let r = Ray {
                a: origin,
                b: bot_left + u * horizontal + v * vertical,
            };
            let c = color(&r);
            let ir: i32 = (255.99 * c.0) as i32;
            let ig: i32 = (255.99 * c.1) as i32;
            let ib: i32 = (255.99 * c.2) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
