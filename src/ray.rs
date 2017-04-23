use vec::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub a: V3,
    pub b: V3,
    time: f32
}

impl Ray {
    pub fn new(a: V3, b: V3, ti: f32) -> Self {
        // TODO
        Ray { a: a, b: b, time: 0.0 }
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn origin(&self) -> &V3 {
        &self.a
    }

    pub fn direction(&self) -> &V3 {
        &self.b
    }

    pub fn point_at(&self, t: f32) -> V3 {
        self.a + t * self.b
    }
}
