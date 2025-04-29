use crate::{
    hitable::{HitRecord, Hitable},
    vec3::Vec3,
};
use rand::Rng;

pub(crate) struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub(crate) fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Returns a random point in some unit sphere
    pub(crate) fn random_in_unit() -> Vec3 {
        let mut rng = rand::rng();
        loop {
            let p = Vec3::new(
                rng.random::<f32>(),
                rng.random::<f32>(),
                rng.random::<f32>(),
            );
            if p.squared_length() >= 1. {
                break p;
            }
        }
    }
}

impl Hitable for Sphere {
    // equation of a sphere centered at the origin is given by:
    //  $x^2 + y^2 + z^2 = r^2$
    // if centered at some other point c = (cx, cy, cz) we have:
    //  $(x - cx)^2 + (y - cy)^2 + (z - cz)^2$
    // we want to determine if a ray hits the sphere
    // recall that a ray represents all positions on some line
    // the equation is given as follows:
    //  $p(t) = origin + t * dir = A + t * B$
    // we are trying to see if the following equation holds for any t
    // $dot(p(t) - c, p(t) - c)$ both p(t) and c are three dim vectors
    // $dot(A + t * B - C, A + t * B - C)$
    // via vector algebra we have the following simplification
    // $t^2 \cdot dot(B, B) + 2 \cdot t \cdot dot(B, A - C) + dot(A - C, A - C) - r^2 = 0$
    // this is a quadratic equation on the variable t
    // 3 possible cases:
    //  no roots: the ray doesn't hit the sphere
    //  1 root: the ray hit the sphere once (tanget point on the sphere)
    //  2 roots: the ray passed through the sphere (entry + exit)
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().dot(ray.direction());
        let b = 2. * oc.dot(ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4. * a * c;

        if discriminant > 0. {
            // has roots
            let root1 = (-b - discriminant.sqrt()) / (2. * a);
            let root2 = (-b + discriminant.sqrt()) / (2. * a);

            // find the root within the t range (min and max)
            let root = if root1 < t_max && root1 > t_min {
                root1
            } else if root2 < t_max && root2 > t_min {
                root2
            } else {
                return None;
            };

            let point_at_t = ray.point_at(root);
            let point_normal = &(&point_at_t - &self.center) / self.radius;

            return Some(HitRecord::new(root, point_at_t, point_normal));
        }

        None
    }
}
