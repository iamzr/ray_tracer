pub struct HittableList {
    objects: Vec<Rc<Hittable>>
}

impl HittableList {
    pub fn new(self, object: Rc<Hittable>) {
        self.add(object);
    }

    pub fn add(self, object: Rc<Hittable>) {
        self.objects.append(object);
    }

    pub fn clear(self) {
        objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(r: &Ray, t_min: f32, t_max: f32, hit_record: &HitRecord) {
        let temp_record = HitRecord::new();
        let hit_anything: bool = false;
        let closet_so_far = t_max;

        for object in objects.iter() {
            if object.hit(r, t_min, t_max, hit_record) {
                hit_anything = true;
                closet_so_far = temp_rec.t;
                hit_record = temp_rec;
            }
        }

        hit_anything
    }
}