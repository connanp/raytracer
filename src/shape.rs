use collision::*;
use material::MaterialKind;
use ray::Ray;
use vec::*;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: V3,
    pub radius: f32,
    pub material: MaterialKind,
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
                rec.material = self.material;
                return Some(rec);
            }
            // other direction
            let t2 = (-b + discriminant.sqrt()) / a;
            if t2 < t_max && t2 > t_min {
                let mut rec = HitRecord::new();
                rec.t = t2;
                rec.p = r.point_at(t2);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = self.material;
                return Some(rec);
            }
        }

        None
    }
}
