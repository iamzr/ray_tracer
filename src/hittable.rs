pub struct hit_record {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
}

pub trait Hittable {
    fn hit(r: &Ray, t_min: f32, t_max: f32, hit_record: &hit_record) -> bool;
}