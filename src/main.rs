mod vec3;
mod color;
mod ray;

use std::mem::Discriminant;

use color::write_color;
use vec3::{Vec3, unit_vector, dot};
use ray::Ray;

use Vec3 as Color;
use Vec3 as Point3;

struct Image {
    width: i32,
    height: i32,
}

fn hit_sphere(centre: &Point3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin() - *centre;

    let a = dot(r.direction(), r.direction());
    let b = 2.0 * dot(oc, r.direction());
    let c =  dot(oc, oc) - radius * radius;

    let discriminant:f32  = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn ray_color(r: Ray) -> Color {
    if hit_sphere(&Point3::new(0.0,0.0,1.0), 0.5, &r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0-t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {

    // Image

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const WIDTH: i32 = 400;
    const HEIGHT: i32 = (WIDTH as f32 / ASPECT_RATIO) as i32;

    const IMAGE: Image = Image {
        width: WIDTH,
        height: HEIGHT,
    };

    // Camera

    let viewport_height: f32= 2.0;
    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    let focal_length: f32 = 1.0;

    let origin: Point3 = Point3::default();
    let hortizontal: Vec3 = Vec3::new(viewport_width,0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Vec3 = origin - hortizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);
    
    // Render

    println!("P3
    {} {}
    255", IMAGE.width, IMAGE.height);

    for j in (0..IMAGE.height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE.width{
            let u: f32 = i as f32  / (IMAGE.width - 1) as f32;
            let v: f32 = j as f32 / (IMAGE.height - 1) as f32;

            let direction: Vec3 = lower_left_corner + hortizontal * u + vertical * v - origin;
            let ray: Ray = Ray::new(origin, direction);

            let color: Color = ray_color(ray);

            write_color(&color);
        }
    }

    eprintln!("Done");
}