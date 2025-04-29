mod camera;
mod hitable;
mod ray;
mod sphere;
mod vec3;
mod world;

use hitable::Hitable;
use sphere::Sphere;
use world::World;

use crate::ray::Ray;
use crate::vec3::Vec3;

fn color(ray: &Ray, world: &impl Hitable) -> Vec3 {
    if let Some(hit_record) = world.hit(ray, 0.0, f32::MAX) {
        return &Vec3::new(
            hit_record.normal.x() + 1.,
            hit_record.normal.y() + 1.,
            hit_record.normal.z() + 1.,
        ) * 0.5;
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

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal_len = Vec3::new(4.0, 0.0, 0.0);
    let vertical_len = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    // build the world
    let mut world = World::default();
    world.add_object(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.add_object(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));
    world.add_object(Box::new(Sphere::new(Vec3::new(-0.5, 0.5, -1.), 0.5)));

    for j in (0..=(ny - 1)).rev() {
        for i in 0..nx {
            // horizontal percent
            let u = (i as f32) / (nx as f32);
            // vertical percent
            let v = (j as f32) / (ny as f32);
            // point on the projection plane
            let point = &lower_left_corner + &horizontal_len * u + &vertical_len * v;

            // create a ray using the camera origin and the point on the projection plane
            // plane point will serve as the direction
            let ray = Ray::new(&origin, &point);

            // compute ray color entirely based on y axis dimension
            let ray_color = color(&ray, &world);

            let ir = (255.99 * ray_color.r()) as i32;
            let ig = (255.99 * ray_color.g()) as i32;
            let ib = (255.99 * ray_color.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
