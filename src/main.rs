mod vec;
use vec::*;
mod ray;
use ray::*;
mod camera;
use camera::*;
mod material;
use material::*;
mod collision;
use collision::*;
mod shape;
use shape::*;

use std::f32;

extern crate rand;

#[allow(dead_code)]
fn color_normal<T: Hitable>(r: &Ray, world: &Hitables<T>) -> V3 {
    match world.hit(r, 0.0, std::f32::MAX) {
        Some(rec) => 0.5 * V3(rec.normal.0 + 1.0, rec.normal.1 + 1.0, rec.normal.2 + 1.0),
        None => {
            let unit_d = unit_vector(*r.direction());
            let t = 0.5 * (unit_d.1 + 1.0);
            (1.0 - t) * V3(1.0, 1.0, 1.0) + t * V3(0.5, 0.7, 1.0)
        }
    }
}

fn color<T: Hitable>(r: &Ray, world: &Hitables<T>, depth: i32) -> V3 {
    // t_min at 0.001 to avoid 'shadow acne' problem
    if let Some(rec) = world.hit(r, 0.001, std::f32::MAX) {
        if depth < 50 {
            // TODO(connanp): must be a idiomatic way to dispatch the call without
            // an exhaustive list here.
            let s_res = match rec.material {
                MaterialKind::Metal(m) => m.scatter(r, &rec),
                MaterialKind::Lambertian(m) => m.scatter(r, &rec),
                MaterialKind::Dielectric(m) => m.scatter(r, &rec),
                MaterialKind::None => (*r, V3(0.0, 0.0, 0.0), false),
            };
            // Would this be better with if/if-lets ?
            match s_res {
                (.., false) => return V3(0.0, 0.0, 0.0),
                (scatter_ray, attenuation, true) => return attenuation * color(&scatter_ray, world, depth + 1),
            }
        }
    }
    let unit_d = unit_vector(*r.direction());
    let t = 0.5 * (unit_d.1 + 1.0);
    (1.0 - t) * V3(1.0, 1.0, 1.0) + t * V3(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 400;
    let ny = 200;
    let ns = 50;

    let camera = Camera::new(V3(-2.0, 2.0, 1.0),
                             V3(0.0, 0.0, -1.0),
                             V3(0.0, 1.0, 0.0),
                             30.0,
                             nx as f32 / ny as f32);
    let R = (f32::consts::PI / 4.0).cos();
    let world = Hitables(vec![Sphere {
                                  center: V3(0.0, 0.0, -1.0),
                                  radius: 0.5,
                                  material: Lambertian::new(&V3(0.1, 0.2, 0.5)),
                              },
                              Sphere {
                                  center: V3(0.0, -100.5, -1.0),
                                  radius: 100.0,
                                  material: Lambertian::new(&V3(0.8, 0.8, 0.0)),
                              },
                              Sphere {
                                  center: V3(1.0, 0.0, -1.0),
                                  radius: 0.5,
                                  material: Metal::new(&V3(0.8, 0.6, 0.2), 0.3),
                              },
                              Sphere {
                                  center: V3(-1.0, 0.0, -1.0),
                                  radius: 0.5,
                                  material: Dielectric::new(1.5),
                              },
                              Sphere {
                                  center: V3(-1.0, 0.0, -1.0),
                                  radius: -0.45,
                                  material: Dielectric::new(1.5),
                              }]);
    println!("P3\n{} {}\n255", nx, ny);

    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut c = V3(0., 0., 0.);
            for _ in 0..ns {
                let u = (rand::random::<f32>() + x as f32) / nx as f32;
                let v = (rand::random::<f32>() + y as f32) / ny as f32;
                let r = camera.get_ray(u, v);
                c = c + color(&r, &world, 0);
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
