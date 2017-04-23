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

use std::ops::Range;

extern crate rand;
extern crate rayon;

use rayon::prelude::*;
use rayon::range::Iter;


fn random_scene() -> Hitables<Sphere> {
    let n = 500;
    let mut spheres: Vec<Sphere> = Vec::with_capacity(n);
    spheres.push(Sphere::new(V3(0.0, -1000.0, -0.0),
                             V3(0.0, -1000.0, -0.0),
                             1000.0,
                             V2(0.0, 1.0),
                             Lambertian::new(&V3(0.5, 0.5, 0.5))));

    let origin = V3(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = V3(a as f32 + 0.9 * rand::random::<f32>(),
                            0.2,
                            b as f32 + 0.9 * rand::random::<f32>());
            if (center - origin).length() > 0.9 {
                match choose_mat {
                    m if m < 0.8 => {
                        // diffuse
                        spheres.push(Sphere::new(center,
                                                 center + V3(0.0, 0.5 * rand::random::<f32>(), 0.0),
                                                 0.2,
                                                 V2(0.0, 1.0),
                                                 Lambertian::new(&V3(rand::random::<f32>() * rand::random::<f32>(),
                                                                     rand::random::<f32>() * rand::random::<f32>(),
                                                                     rand::random::<f32>() * rand::random::<f32>()))))
                    }
                    m if m < 0.95 => {
                        // metal
                        spheres.push(Sphere::new(center,
                                                 center,
                                                 0.2,
                                                 V2(0.0, 1.0),
                                                 Metal::new(&V3(0.5 * (1.0 + rand::random::<f32>()),
                                                                0.5 * (1.0 + rand::random::<f32>()),
                                                                0.5 * (1.0 + rand::random::<f32>())),
                                                            0.5 * rand::random::<f32>())))

                    }
                    _ => {
                        // glass
                        spheres.push(Sphere::new(center, center, 0.2, V2(0.0, 1.0), Dielectric::new(1.5)))
                    }
                }
            }
        }
    }

    spheres.push(Sphere::new(V3(0.0, 1.0, 0.0),
                             V3(0.0, 1.0, 0.0),
                             1.0,
                             V2(0.0, 1.0),
                             Dielectric::new(1.5)));
    spheres.push(Sphere::new(V3(-4.0, 1.0, 0.0),
                             V3(-4.0, 1.0, 0.0),
                             1.0,
                             V2(0.0, 1.0),
                             Lambertian::new(&V3(0.4, 0.2, 0.1))));
    spheres.push(Sphere::new(V3(4.0, 1.0, 0.0),
                             V3(4.0, 1.0, 0.0),
                             1.0,
                             V2(0.0, 1.0),
                             Metal::new(&V3(0.7, 0.6, 0.5), 0.0)));

    Hitables(spheres)
}

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
    // let nx = 1200;
    // let ny = 800;
    let nx = 768;
    let ny = 486;
    let ns = 50;

    let look_from = V3(13.0, 2.0, 3.0);
    let look_at = V3(0.0, 0.0, 0.0);
    let focus_plane = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(look_from,
                             look_at,
                             V3(0.0, 1.0, 0.0),
                             20.0,
                             nx as f32 / ny as f32,
                             aperture,
                             focus_plane,
                             V2(0.0, 1.0)
                             );

    let world = random_scene();
    println!("P3\n{} {}\n255", nx, ny);

    let y_mid = ny / 2;
    let ly = 0..y_mid;
    let lx = 0..nx;
    let ry = y_mid..ny;
    let rx = 0..nx;

    let tracer = |columns: Range<u32>, rows: Range<u32>| {
        let mut result = Vec::new();
        for y in columns.rev() {
            for x in rows.clone() {
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
                c *= 255.99;

                result.push(format!("{} {} {}", c.0 as i32, c.1 as i32, c.2 as i32));
            }
        }
        result
    };
    let (bottom, top) = rayon::join(|| tracer(ly, lx), || tracer(ry, rx));

    for s in &top {
        println!("{}", s)
    }
    for s in &bottom {
        println!("{}", s)
    }
}
