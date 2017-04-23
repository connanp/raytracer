use collision::*;
use material::MaterialKind;
use ray::Ray;
use vec::*;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: (V3, V3),
    pub radius: f32,
    pub material: MaterialKind,
    pub time: V2,
}

impl Sphere {
    pub fn new(c0: V3, c1: V3, r: f32, time: V2, m: MaterialKind) -> Self {
        Sphere {
            center: (c0, c1),
            radius: r,
            material: m,
            time: time,
        }
    }
}

impl Moveable for Sphere {
    fn center(&self, time: f32) -> V3 {
        self.center.0 + ((time - self.time.0) / (self.time.1 - self.time.0))*(self.center.1 - self.center.0)
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let center = self.center(r.time());
        let oc = *r.origin() - center;
        let a = dot(r.direction(), r.direction());
        let b = dot(&oc, r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;

        let do_hit = |t| {
            if t < t_max && t > t_min {
                let mut rec = HitRecord::new();
                rec.t = t;
                rec.p = r.point_at(t);
                rec.normal = (rec.p - center) / self.radius;
                rec.material = self.material;
                Some(rec)
            } else {
                None
            }
        };

        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            if let Some(res) = do_hit((-b - discriminant.sqrt()) / a) {
                return Some(res);
            }
            // other direction
            if let Some(res) = do_hit((-b + discriminant.sqrt()) / a) {
                return Some(res);
            }
        }

        None
    }
}