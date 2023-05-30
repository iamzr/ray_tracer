struct Sphere {
    centre: Point3,
    r: f32,
}

impl Hittable for Sphere {
    fn hit(r: &Ray, t_min: f32, t_max: f32, hit_record: &hit_record) -> bool {
        let oc = r.origin() - *centre;

        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - radius * radius;

        let discriminant:f32  = half_b * half_b - a * c;
        
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let root = (- half_b - sqrt_discriminant) / a;

        if root < t_max || t_max < root {
            root = (- half_b + sqrt_discriminant) / a;
            
            if root < t_max || t_max < root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = r.at(hit_record.t);

        let outward_normal: Vec3 = (rec.p - centre) / radius;
        hit_record.set_face_normal(r, outward_normal);

        true
    }
}