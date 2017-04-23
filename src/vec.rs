use std::ops::{Add, Div, Mul, MulAssign, Neg, Sub};

extern crate rand;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct V2(pub f32, pub f32);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct V3(pub f32, pub f32, pub f32);

impl V3 {
    pub fn length(self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn squared_length(self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn unit_vector(&mut self) -> () {
        let k: f32 = 1.0 / self.length();
        self.0 *= k;
        self.1 *= k;
        self.2 *= k;
    }
}

pub fn dot(v1: &V3, v2: &V3) -> f32 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

pub fn cross(v1: &V3, v2: &V3) -> V3 {
    V3(v1.1 * v2.2 - v1.2 * v2.1,
       -(v1.0 * v2.2 - v1.2 * v2.0),
       v1.0 * v2.1 - v1.1 * v2.0)
}

pub fn unit_vector(v: V3) -> V3 {
    v / v.length()
}

pub fn reflect(v: V3, n: V3) -> V3 {
    v - 2.0 * dot(&v, &n) * n
}

pub fn refract(v: &V3, n: &V3, ni_over_nt: f32) -> Option<V3> {
    let uv = unit_vector(*v);
    let dt = dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let _n = *n;
        return Some(ni_over_nt * (uv - _n * dt) - _n * discriminant.sqrt());
    }
    None
}


// https://github.com/rust-lang/rust/issues/28570
#[allow(unused_assignments)]
pub fn random_in_unit_sphere() -> V3 {
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

impl Add for V3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        V3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<f32> for V3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        V3(self.0 + rhs, self.1 + rhs, self.2 + rhs)
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

impl Sub<f32> for V3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        V3(self.0 - rhs, self.1 - rhs, self.2 - rhs)
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

impl MulAssign<f32> for V3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
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
