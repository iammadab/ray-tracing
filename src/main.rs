mod hitable;
mod ray;
mod sphere;
mod vec3;
mod world;

use crate::ray::Ray;
use crate::vec3::Vec3;

fn color(ray: &Ray) -> Vec3 {
    // if we hit the sphere return a red color
    let sphere_center = Vec3::new(0., 0., -1.);
    let t = hit_sphere(&sphere_center, 0.5, ray);
    if t > 0.0 {
        let normal = &ray.point_at(t) - sphere_center;
        let normalized_norm = normal.unit_vector();
        // translate to 0..1 then use as rgb
        return &Vec3::new(
            normalized_norm.x() + 1.,
            normalized_norm.y() + 1.,
            normalized_norm.z() + 1.,
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
            let ray_color = color(&ray);

            let ir = (255.99 * ray_color.r()) as i32;
            let ig = (255.99 * ray_color.g()) as i32;
            let ib = (255.99 * ray_color.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
