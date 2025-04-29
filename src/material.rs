use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub(crate) trait Material<'a> {
    fn scatter(ray: &'a Ray, hit_record: &'a HitRecord) -> (Vec3, Ray<'a>, bool);
}
