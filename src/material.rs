use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub(crate) trait Material: Clone {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Vec3, Ray, bool);
}

#[derive(Clone, Default)]
pub(crate) struct Lambertian {
    attenuation: Vec3,
}

impl Lambertian {
    pub(crate) fn new(attenuation: Vec3) -> Self {
        Self { attenuation }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Vec3, Ray, bool) {
        let reflected_direction = &hit_record.point + Vec3::random_in_unit_sphere();
        let scattered_ray = Ray::new(hit_record.point.clone(), reflected_direction);
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

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Vec3, Ray, bool) {
        let reflected = ray.direction().unit_vector().reflect(&hit_record.normal);
        let scattered = Ray::new(hit_record.point.clone(), reflected);
        let m = scattered.direction().dot(&hit_record.normal);
        (self.attenuation.clone(), scattered, m > 0.)
    }
}
