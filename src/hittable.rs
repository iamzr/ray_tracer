struct hit_record {
    p: Point3,
    normal: Vec3,
    t: f32,
}

pub trait Hittable {
    fn hit(r: &Ray, t_min: f32, t_max: f32, rec: &hit_record) -> bool;
}