use crate::shapes::Sphere;
use crate::math::{Vector3, Ray};

pub trait Hittable {
    fn did_hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> (bool, HitRecord);
}

pub struct HitRecord {
    t_at_point: f32,
    point: Vector3,
    normal_from_point: Vector3,
}

impl HitRecord {
    pub fn empty_new() -> HitRecord {
        HitRecord {
            t_at_point: 0.0,
            point: Vector3::empty_new(),
            normal_from_point: Vector3::empty_new()
        }
    }

    pub fn new(t_at_point: f32,
           point: Vector3,
           normal_from_point: Vector3) -> HitRecord {
        HitRecord {
            t_at_point,
            point,
            normal_from_point,
        }
    }

    pub fn set_point(mut self, point: Vector3) {
        self.point = point;
    }

    pub fn set_t(mut self, t: f32) {
        self.t_at_point = t;
    }

    pub fn set_normal(mut self, normal: Vector3) {
        self.normal_from_point = normal;
    }

    pub fn get_normal(&self) -> Vector3 {self.normal_from_point}

    pub fn get_t(&self) -> f32 {
        self.t_at_point
    }
}

pub struct HittableList {
    list: Vec<Sphere>
}

impl HittableList {
    pub fn new(size: usize) -> HittableList {
        HittableList { list : Vec::with_capacity(size) }
    }
    pub fn add(&mut self, sphere: Sphere) {
        self.list.push(sphere);
    }
    pub fn get_list(&self) -> &Vec<Sphere> {
        &self.list
    }
}

impl Hittable for HittableList {
    fn did_hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> (bool, HitRecord) {
        // checking to see if we've hit anything
        let mut hit_anything = false;
        let mut closest_hit_so_far = max_t;
        let mut hit_record = HitRecord::empty_new();

        for shape in self.get_list() {
            let (did_hit, record) = shape.did_hit(ray, min_t, closest_hit_so_far);

            if did_hit {
                hit_anything = true;
                closest_hit_so_far = record.get_t();
                hit_record = record;
            }
        }

        (hit_anything, hit_record)
    }
}