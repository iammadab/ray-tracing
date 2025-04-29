mod camera;
mod hitable;
mod ray;
mod sphere;
mod vec3;
mod world;

use camera::Camera;
use hitable::Hitable;
use sphere::Sphere;
use world::World;

use crate::ray::Ray;
use crate::vec3::Vec3;

use rand::Rng;

fn color(ray: &Ray, world: &impl Hitable) -> Vec3 {
    if let Some(hit_record) = world.hit(ray, 0.0, f32::MAX) {
        //return &Vec3::new(
        //    hit_record.normal.x() + 1.,
        //    hit_record.normal.y() + 1.,
        //    hit_record.normal.z() + 1.,
        //) * 0.5;

        // assume diffuse material
        let reflected_direction = &hit_record.point + Sphere::random_in_unit();

        // determine the color of the new ray
        // we keep doing this until we get a miss
        // TODO: should add some kind of recursion depth limit
        return &color(&Ray::new(&hit_record.point, reflected_direction), world) * 0.5;
    }

    // get the unit direction of the ray
    // this will be a number between -1 and 1
    let unit_direction = ray.direction().unit_vector();

    // we need a t value between 0 and 1
    // -1..=1 translated to 0..=2 by + 1
    // 0..=2 translated to 0..1 by * 0.5
    let t = 0.5 * (unit_direction.y() + 1.0);

    // lerp between white and blue based on t value
    &Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + &Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    // build the world
    let mut world = World::default();
    world.add_object(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.add_object(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    let camera = Camera::default();

    // random number generator for antialiasing
    let mut rng = rand::rng();

    for j in (0..=(ny - 1)).rev() {
        for i in 0..nx {
            let mut ray_color = Vec3::new(0., 0., 0.);

            for _ in 0..ns {
                // horizontal percent + small noise
                let u = (i as f32 + rng.random::<f32>()) / (nx as f32);
                // vertical percent + small noise
                let v = (j as f32 + rng.random::<f32>()) / (ny as f32);

                // compute ray passing through current pixel
                let ray = camera.get_ray(u, v);

                // compute ray color entirely based on y axis dimension
                ray_color = ray_color + color(&ray, &world);
            }

            let ray_color = &ray_color / ns as f32;

            let ir = (255.99 * ray_color.r()) as i32;
            let ig = (255.99 * ray_color.g()) as i32;
            let ib = (255.99 * ray_color.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
