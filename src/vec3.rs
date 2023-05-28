use std::ops::{Mul, Div, Add, Neg, Sub};

pub struct Vec3 {
    e0: f32,
    e1: f32,
    e2: f32,
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { e0: 0.0, e1: 0.0, e2: 0.0}
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z:f32) -> Self {
        Self { e0: x, e1: y, e2: z}
    }

    pub fn x(&self) -> f32 {
        self.e0
    }

    pub fn y(&self) -> f32 {
        self.e1
    }

    pub fn z(&self) -> f32 {
        self.e2
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    fn length_squared(self) -> f32 {
        self.e0 * self.e0 + self.e1 * self.e1 + self.e2 * self.e2
    }
}

impl Copy for Vec3 {}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        *self
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { e0: -self.e0, e1: -self.e1, e2: -self.e2}
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {e0: self.e0 + rhs.e0, e1: self.e1 + rhs.e1, e2: self.e2 + rhs.e2}
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self { e0: self.e0 - rhs.e0, e1: self.e1 - rhs.e1, e2: self.e2 - rhs.e2}
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, c: f32) -> Self::Output{
        Self { e0: self.e0 * c, e1: self.e1 * c , e2: self.e2 * c}
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output{
        Self { e0: self.e0 * rhs.e0 , e1: self.e1 * rhs.e1 , e2: self.e2 * rhs.e2}
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, c: f32) -> Self::Output {
       self * (1.0 /c)
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    let new_v = v.clone();
    new_v * (1.0 /new_v.length())
}