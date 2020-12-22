use crate::math::{Ray, Vector3};
use crate::hittable::{Hittable, HitRecord};
use crate::math;

pub struct Sphere {
    center: Vector3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    pub fn get_center(&self) -> Vector3 {
        self.center
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn was_hit(sphere: Sphere, ray: &Ray) -> f32 {
        let center = sphere.get_center();
        let radius = sphere.get_radius();

        let normal = ray.origin - center;

        // 0 = ax^2 + bx + c
        let a = math::dot(&(ray.direction), &(ray.direction));
        let b = 2.0 * math::dot(&normal, &(ray.direction));
        let c = math::dot(&normal, &normal) - radius * radius;

        // part under the sqrt in the quadratic formula
        let discriminate = b * b - 4.0 * a * c;

        // eliminate all non-real solutions
        // no real solutions if the discriminate is less than 0
        if discriminate < 0.0 {
            -1.0
        } else {
            // one or more real solutions if the discriminate is positive
            // solve the quadratic formula
            (-b - discriminate.sqrt()) / (2.0 * a)
        }
    }
}

impl Hittable for Sphere {
    fn did_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> (bool, HitRecord) {
        let sphere_center = self.get_center();
        let sphere_radius = self.get_radius();

        let normal = ray.origin - sphere_center;

        // 0 = ax^2 + bx + c
        let a = math::dot(&(ray.direction), &(ray.direction));
        let b = 2.0 * math::dot(&normal, &(ray.direction));
        let c = math::dot(&normal, &normal) - sphere_radius * sphere_radius;

        // part under the sqrt in the quadratic formula
        let discriminate = b * b - 4.0 * a * c;


        if discriminate > 0.0 {
            let t_minus = (-b - discriminate.sqrt()) / (2.0 * a);
            if t_minus > t_min && t_minus < t_max {
                let point_at_t = ray.get_point_at_time(t_minus);
                let normal_at_point = (point_at_t - sphere_center) / sphere_radius;

                let hit_record = HitRecord::new(t_minus, point_at_t, normal_at_point);
                return (true, hit_record)
            }

            let t_plus = (-b + discriminate.sqrt()) / (2.0 * a);
            if t_plus > t_min && t_plus < t_max {
                let point_at_t = ray.get_point_at_time(t_plus);
                let normal_at_point = (point_at_t - sphere_center) / sphere_radius;

                let hit_record = HitRecord::new(t_minus, point_at_t, normal_at_point);
                return (true, hit_record)
            }
            return (false, HitRecord::empty_new())
        }
        return (false, HitRecord::empty_new())
    }
}


