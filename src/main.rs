use std::fs::File;
use std::io::{BufWriter, Write};

mod ray;
mod vec3;

use ray::Ray;
use vec3::{Point3, Vec3};

type Color = Vec3;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let oc = center - r.origin;
    let a = Vec3::dot(r.direction, r.direction);
    let b = -2.0 * Vec3::dot(r.direction, oc);
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_dir = Vec3::unit_vector(r.direction);
    let a = 0.5 * (unit_dir.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), std::io::Error> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let height = ((image_width as f64) / aspect_ratio) as i32;
    let image_height = if height > 1 { height } else { 1 };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let viewport_v = Vec3 {
        x: 0.0,
        y: -viewport_height,
        z: 0.0,
    };

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        }
        - viewport_u / 2.0
        - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Open output file
    let file = File::create("output.ppm")?;
    let mut writer = BufWriter::new(file);

    // Render
    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", image_width, image_height)?;
    writeln!(writer, "255")?;

    for j in 0..image_height {
        print!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r);

            // write color
            let ir = (255.999 * pixel_color.x) as i32;
            let ig = (255.999 * pixel_color.y) as i32;
            let ib = (255.999 * pixel_color.z) as i32;
            writeln!(writer, "{} {} {}", ir, ig, ib)?;
        }
    }
    println!("\rDone!                      \n");

    Ok(())
}
