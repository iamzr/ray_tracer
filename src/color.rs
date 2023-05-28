use crate::Vec3;

pub fn write_color(color: &Vec3) {
    let ir = (255.999 as f32 * color.x()) as i32;
    let ig = (255.999 as f32 * color.y()) as i32 ;
    let ib = (255.999 as f32 * color.z()) as i32 ;

    println!("{} {} {}", ir, ig, ib);
}