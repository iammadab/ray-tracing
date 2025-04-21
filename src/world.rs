use crate::hitable::{HitRecord, Hitable};

#[derive(Default)]
pub(crate) struct World {
    hittable_objects: Vec<Box<dyn Hitable>>,
}

impl World {
    pub(crate) fn add_object(&mut self, obj: Box<dyn Hitable>) {
        self.hittable_objects.push(obj)
    }
}

impl Hitable for World {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut record = None;
        for obj in &self.hittable_objects {
            if let Some(hit_record) = obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                record = Some(hit_record);
            }
        }
        record
    }
}
