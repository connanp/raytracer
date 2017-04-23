use material::MaterialKind;
use ray::Ray;
use vec::V3;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: V3,
    pub normal: V3,
    pub material: MaterialKind,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: 0.0,
            p: V3(0.0, 0.0, 0.0),
            normal: V3(0.0, 0.0, 0.0),
            material: MaterialKind::None,
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Debug)]
pub struct Hitables<T>(pub Vec<T>) where T: Hitable;

impl<T> Hitables<T>
    where T: Hitable
{
    pub fn new() -> Self {
        Hitables(vec![])
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec = HitRecord::new();
        let mut closest = Some(t_max);
        let mut hit_anything = false;
        for obj in &self.0 {
            match obj.hit(r, t_min, closest.unwrap() as f32) {
                Some(o) => {
                    closest = Some(o.t);
                    rec = o;
                    hit_anything = true
                }
                _ => (),
            }
        }
        if hit_anything {
            return Some(rec);
        }
        None
    }
}

pub trait Moveable: Hitable {
    fn center(&self, time: f32) -> V3;
}