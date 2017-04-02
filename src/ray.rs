use vec::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub a: V3,
    pub b: V3,
}

impl Ray {
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
