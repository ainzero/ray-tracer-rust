use std::fs::File;
use std::io::prelude::*;

mod math;
use math::{Vector3, Ray};

mod shapes;
mod hittable;

use shapes::Sphere;
use crate::hittable::{Hittable,HittableList};
use std::f32::MAX;

type RGB = Vector3;

#[derive(Debug, Copy, Clone)]
struct HittingGrid {
    width: Vector3,
    height: Vector3,
    origin: Vector3,
}
impl HittingGrid {
    fn with_height_and_width(height: f32, width: f32, z_depth: f32) -> HittingGrid {
        HittingGrid {
            height: Vector3::new(0.0,height, z_depth),
            width: Vector3::new(width, 0.0, z_depth),
            origin: Vector3::new(0.0,0.0,z_depth)
        }
    }
    fn get_height(&self) -> &Vector3 {
        &self.height
    }
    fn get_width(&self) -> &Vector3 {
        &self.width
    }
    fn get_origin(&self) -> Vector3 {
        self.origin
    }
}

fn get_color_at_pixel(r: &Ray, scene: &HittableList) -> RGB {
    let (did_hit, record) = scene.did_hit(r, 0.0,MAX);

    if did_hit {
        // TODO(Z): How doe this change over time? why 1.0?
        // dbg!(record.get_normal().x(),record.get_normal().y(),record.get_normal().z());
        return RGB::new(record.get_normal().x() + 1.0, record.get_normal().y() + 1.0, record.get_normal().z() + 1.0) * 0.5;
    } else {
        let unit_direction = math::unit_vector(r.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        RGB::new(1.0, 1.0, 1.0) * (1.0 - t) + RGB::new(0.5, 0.7, 1.0) * t
    }
}

fn update_output_image(pixel: RGB, mut image: &File) {
    let red = scale_by_value(255.99, pixel.r());
    let green = scale_by_value(255.99,pixel.g());
    let blue = scale_by_value(255.99,pixel.b());

    image.write(&format!("{} {} {} \n", red, green, blue).into_bytes()).expect("Could not write additional bytes to the ray traced image");
}

fn scale_by_value(scale_by: f32, value: &f32) -> i32 {
    (scale_by * value) as i32
}

fn prepare_image(mut image: &File, height: u32, width: u32) {
    let ppm_format_header = format!("P3\n{} {}\n255\n", width, height);
    image.write(&ppm_format_header.into_bytes()).expect("Could not ");
}

fn main() -> std::io::Result<()> {
    let image =  File::create("test.ppm")?;

    let image_width = 800;
    let image_height = 400;
    
    prepare_image(&image, image_height, image_width);

    let grid = HittingGrid::with_height_and_width(2.0, 4.0, 0.0);

    // positive z is coming towards the camera
    let grid_lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);

    // Objects in the scene
    let mut scene = HittableList::new(2);
    //scene.add(Sphere::new(Vector3::new(0.0,0.0,-0.0),0.25));
    scene.add(Sphere::new(Vector3::new(0.0,0.0,-1.0),0.5));
    scene.add(Sphere::new(Vector3::new(0.0,-100.5,-1.0),100.0));


    for j in (0..image_height - 1).rev() {
        for i in 0..image_width {

            // u and v are bounded between 0 and 1
            let u = i as f32 / image_width as f32;
            let v = j as f32 / image_height as f32;

            let ray = Ray {
                origin: grid.get_origin(),
                direction: grid_lower_left_corner + (grid.get_width() * u) + (grid.get_height() * v),
            };

            let color_at_pixel = get_color_at_pixel(&ray, &scene);

            update_output_image(color_at_pixel, &image);
        }
    }

    Ok(())
}
