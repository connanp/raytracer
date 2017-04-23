use collision::HitRecord;
use ray::*;
use vec::*;

extern crate rand;

#[derive(Debug, Clone, Copy)]
pub enum MaterialKind {
    None,
    Metal(Metal),
    Lambertian(Lambertian),
    Dielectric(Dielectric),
}

pub trait Material {
    // (scattered, attenuation, success)
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Ray, V3, bool);
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: V3,
}

impl Lambertian {
    pub fn new(a: &V3) -> MaterialKind {
        MaterialKind::Lambertian(Self { albedo: *a })
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Ray, V3, bool) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        (Ray::new(rec.p, target - rec.p, r_in.time()), self.albedo, true)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: V3,
    fuzz: f32,
}

impl Metal {
    pub fn new(a: &V3, fuzz: f32) -> MaterialKind {
        let f = if fuzz < 1.0 { fuzz } else { 1.0 };
        MaterialKind::Metal(Self {
                                albedo: *a,
                                fuzz: f,
                            })
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Ray, V3, bool) {
        let reflected = reflect(unit_vector(*r_in.direction()), rec.normal);
        let scattered = Ray::new(rec.p,
                                 reflected + self.fuzz * random_in_unit_sphere(),
                                 r_in.time());
        let attenuation = self.albedo;
        match dot(scattered.direction(), &rec.normal) {
            x if x > 0.0 => (scattered, attenuation, true),
            _ => (scattered, attenuation, false),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    idx: f32,
}

impl Dielectric {
    pub fn new(idx: f32) -> MaterialKind {
        MaterialKind::Dielectric(Self { idx: idx })
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Ray, V3, bool) {
        let outward_normal: V3;
        let ni_over_nt: f32;
        let cosine: f32;
        let mut reflect_prob = 1.0;
        let attenuation = V3(1.0, 1.0, 1.0);
        if dot(r_in.direction(), &rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.idx;
            cosine = self.idx * dot(r_in.direction(), &rec.normal) / r_in.direction().length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.idx;
            cosine = -dot(r_in.direction(), &rec.normal) / r_in.direction().length();
        }
        // NOTE(connanp): compiler cannot reasonably determine this is safe to remain uninitialized
        // i'm probably wrong anyway
        let mut scattered;
        unsafe {
            use std::mem;
            scattered = mem::uninitialized();
        }
        if let Some(r) = refract(r_in.direction(), &outward_normal, ni_over_nt) {
            reflect_prob = schlick(cosine, self.idx);
            scattered = r;
        }
        if rand::random::<f32>() < reflect_prob {
            scattered = reflect(unit_vector(*r_in.direction()), rec.normal);
        }

        (Ray::new(rec.p, scattered, r_in.time()),
        attenuation,
        true)
    }
}

fn schlick(cos: f32, idx: f32) -> f32 {
    let mut r0 = (1.0 - idx) / (1.0 + idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
}
