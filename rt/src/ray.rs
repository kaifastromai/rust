use crate::csglm::vector::Vec3;
type Vector = Vec3<f64>;
pub struct Ray {
    pub orig: Vector,
    pub dir: Vector,
}

impl Ray {
    pub fn new(&self) -> Self {
        Ray {
            orig: Vector::new(0.0, 0.0, 0.0),
            dir: Vector::new(0.0, 0.0, 0.0),
        }
    }
    pub fn origin(&self) -> Vector {
        self.orig
    }
    pub fn direction(&self) -> Vector {
        self.dir
    }
    pub fn at(&self, t: f64) -> Vector {
        self.orig + self.dir * t
    }
}
