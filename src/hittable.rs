pub struct hit_record {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl hit_record {
    fn set_face_normal(self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit(r: &Ray, t_min: f32, t_max: f32, hit_record: &hit_record) -> bool;
}