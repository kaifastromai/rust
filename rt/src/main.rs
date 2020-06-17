//use std::io::Write;
mod csglm;
mod ray;
use csglm::vector::*;
use ray::Ray;

type Color = Vec3<f32>;
type Point = Vec3<f64>;
fn main() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 512;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    let mut v = Point::new(3.4, 5.5, 6.7);
    println!("P3\n {} {} \n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let viewport_height = 2.0f64;
    let viewport_width = (ASPECT_RATIO as f64 * viewport_height) as f64;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\r Snanlines remaining {}", j);
        // std::io::stderr()
        //     .flush()
        //     .expect("Could not flush err buffer");
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH as f32 - 1.0);
            let v = j as f32 / (IMAGE_HEIGHT as f32 - 1.0);
            let r = Ray {
                orig: origin,
                dir: lower_left_corner + horizontal * u.into() + vertical * v.into() - origin,
            };
            let pixel_color = ray_color(&r);
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone");
}
fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x) as i32,
        (255.999 * pixel_color.y) as i32,
        (255.999 * pixel_color.z) as i32
    );
}

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = Point::normal(&ray.direction());
    let t = 0.5 * (unit_direction.y + 0.5);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) as f32 + Color::new(0.5, 0.7, 1.0) * (t as f32)
}
