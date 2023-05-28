mod vec3;
mod color;
mod ray;

use color::write_color;
use vec3::{Vec3, unit_vector, dot};
use ray::Ray;

use Vec3 as Color;
use Vec3 as Point3;

struct Image {
    width: i32,
    height: i32,
}

fn hit_sphere(centre: &Point3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - *centre;

    let a = r.direction().length_squared();
    let half_b = dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;

    let discriminant:f32  = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt())/ a;
    }
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0,0.0,-1.0), 0.5, &r);
    
    if t > 0.0 {
        let n: Vec3 = unit_vector(r.at(t) - Vec3::new(0.0,0.0,-1.0));
        return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
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