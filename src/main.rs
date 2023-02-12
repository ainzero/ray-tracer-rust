use std::fs::File;
use std::io::prelude::*;
use rand::{Rng};

mod math;
use math::{Vector3, Ray};

mod shapes;
mod hittable;
mod camera;

use shapes::Sphere;
use camera::Camera;
use crate::hittable::{Hittable,HittableList};
use std::f32::MAX;
use rand::prelude::ThreadRng;

type RGB = Vector3;

fn gamma_2_correction(rgb_value: RGB) -> RGB {
    return RGB::new(rgb_value.r().sqrt(),rgb_value.g().sqrt(), rgb_value.b().sqrt());
}

fn get_random_in_unit_sphere(random_number_generator: &mut ThreadRng) -> Vector3 {
    let mut get_random_point = |random_values: (f32,f32,f32)|  -> Vector3 { Vector3::new(random_values.0, random_values.1, random_values.2) * 2.0 - Vector3::new(1.0,1.0,1.0) };
    let mut random_xyz = random_number_generator.gen::<(f32, f32, f32)>();
    let mut random_point = get_random_point(random_xyz);
    while random_point.squared_length() >= 1.0 {
        //dbg!(random_point);
        //dbg!(random_point);
        //dbg!(random_point.squared_length());
        random_xyz = random_number_generator.gen::<(f32, f32, f32)>();
        random_point = get_random_point(random_xyz);
    }
    //dbg!(random_point.squared_length());
    random_point
}

fn get_color_at_pixel(r: &Ray, scene: &HittableList, rng: &mut ThreadRng) -> RGB {
    let (did_hit, record) = scene.did_hit(r, 0.001,MAX);

    if did_hit {
        // TODO(Z): How does this change over time? why 1.0?

        let target: Vector3 = record.get_point() + record.get_normal() + get_random_in_unit_sphere(rng);
        return get_color_at_pixel( &Ray::new(record.get_point(), target - record.get_point()), scene, rng) * 0.5
    } else {

        // linear interpolation between white and blue
        // blended_value = (1 - t) * start_value + t * end_value
        let unit_direction = math::unit_vector(r.direction);

        let t = 0.5 * (unit_direction.y() + 1.0);

        RGB::new(1.0, 1.0, 1.0) * (1.0 - t) + RGB::new(0.5, 0.7, 1.0) * t
    }
}

fn update_output_image(pixel: RGB, mut image: &File) {
    let gamma_corrected_pixel = gamma_2_correction(pixel);

    let red = scale_by_value(255.99,  gamma_corrected_pixel.r());
    let green = scale_by_value(255.99, gamma_corrected_pixel.g());
    let blue = scale_by_value(255.99, gamma_corrected_pixel.b());

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
    let image =  File::create("foo.ppm")?;

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = ((image_width as f32) / aspect_ratio) as u32;
    let max_samples = 50;
    
    prepare_image(&image, image_height, image_width);

    let viewport_height = 2.0;
    let viewport_width = viewport_height / aspect_ratio;
    let camera = Camera::centered_in_plane_with_height_and_width(viewport_height, viewport_width);

    // Objects in the scene
    let mut scene = HittableList::new(2);

    scene.add(Sphere::new(Vector3::new(0.0,0.0,-1.0),0.5));
    scene.add(Sphere::new(Vector3::new(0.0,-100.5,-1.0),100.0));

    let mut random_number_generator = rand::thread_rng();

    let mut row_counter = 0;

    for j in (0..image_height - 1).rev() {
       
        for i in 0..image_width {

            let mut color_at_pixel = RGB::empty_new();
            for _k in 0..max_samples {
                // u and v are bounded between 0 and 1
                let random_val_one: f32 = random_number_generator.gen();
                let random_val_two: f32 = random_number_generator.gen();

                let u = ((i as f32) + random_val_one) / image_width as f32;
                let v = ((j as f32) + random_val_two) / image_height as f32;

                let ray = camera.get_ray(u,v);

                color_at_pixel += get_color_at_pixel(&ray, &scene,&mut random_number_generator);
            }

            update_output_image(color_at_pixel / (max_samples as f32), &image);

            
           
        }
        row_counter += 1;
    }



    Ok(())
}
