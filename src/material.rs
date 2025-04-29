use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub(crate) trait Material<'a>: Clone {
    fn scatter(&self, ray: &'a Ray, hit_record: &'a HitRecord) -> (Vec3, Ray<'a>, bool);
}

#[derive(Clone)]
pub(crate) struct Lambertian {
    attenuation: Vec3,
}

impl Lambertian {
    pub(crate) fn new(attenuation: Vec3) -> Self {
        Self { attenuation }
    }
}

impl<'a> Material<'a> for Lambertian {
    fn scatter(&self, ray: &'a Ray, hit_record: &'a HitRecord) -> (Vec3, Ray<'a>, bool) {
        let reflected_direction = &hit_record.point + Sphere::random_in_unit();
        let scattered_ray = Ray::new(&hit_record.point, reflected_direction);
        (self.attenuation.clone(), scattered_ray, true)
    }
}

#[derive(Clone)]
pub(crate) struct Metal {
    attenuation: Vec3,
}

impl Metal {
    pub(crate) fn new(attenuation: Vec3) -> Self {
        Self { attenuation }
    }
}

impl<'a> Material<'a> for Metal {
    fn scatter(&self, ray: &'a Ray, hit_record: &'a HitRecord) -> (Vec3, Ray<'a>, bool) {
        let reflected = ray.direction().unit_vector().reflect(&hit_record.normal);
        let scattered = Ray::new(&hit_record.point, reflected);
        let m = scattered.direction().dot(&hit_record.normal);
        (self.attenuation.clone(), scattered, m > 0.)
    }
}
