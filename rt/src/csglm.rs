pub mod vector {
    use num_traits::Float;
    use std::ops;
    #[derive(Debug, Clone, Copy)]
    pub struct Vec3<T> {
        pub x: T,
        pub y: T,
        pub z: T,
    }
    impl<T> Vec3<T>
    where
        T: Float + std::ops::Mul + std::ops::Add + std::ops::Div,
    {
        pub fn new(x: T, y: T, z: T) -> Self {
            Vec3 { x, y, z }
        }
        pub fn len(&self) -> T
        where
            T: Float + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
        {
            self.len_sqr().sqrt()
        }
        pub fn len_sqr(&self) -> T
        where
            T: Copy,
        {
            self.x * self.x + self.y * self.y + self.z * self.z
        }
        //Returns this vector dotted into  other
        pub fn dot(&self, other: &Vec3<T>) -> T
        where
            T: Copy,
        {
            self.x * other.x + self.y * other.y + self.z * other.z
        }

        pub fn static_dot(v: &Vec3<T>, u: &Vec3<T>) -> T {
            v.x * u.x + v.y * u.y + v.z * u.z
        }
        //Static cross
        pub fn s_cross(v: &Vec3<T>, u: Vec3<T>) -> Vec3<T> {
            Vec3::new(
                v.y * u.z - v.z * u.y,
                v.y * u.x - v.x * u.z,
                v.x * u.y - v.y * u.x,
            )
        }
        pub fn normal(v: &Vec3<T>) -> Vec3<T>
        where
            T: Copy,
        {
            *v / v.len_sqr()
        }
        pub fn normalize(&mut self) {
            let v = Vec3::normal(self);
            self.x = v.x;
            self.y = v.y;
            self.z = v.z;
        }
    }

    impl<T> ops::Add<Vec3<T>> for Vec3<T>
    where
        T: std::ops::Add<T, Output = T> + Float,
    {
        type Output = Vec3<T>;

        fn add(self, _rhs: Vec3<T>) -> Vec3<T> {
            Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
        }
    }

    impl<T> ops::Sub<Vec3<T>> for Vec3<T>
    where
        T: std::ops::Sub<T, Output = T> + Float,
    {
        type Output = Vec3<T>;

        fn sub(self, _rhs: Vec3<T>) -> Vec3<T> {
            Vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
        }
    }
    impl<T> ops::AddAssign<Vec3<T>> for Vec3<T>
    where
        T: std::ops::Add<T, Output = T> + Copy,
    {
        fn add_assign(&mut self, other: Self) {
            *self = Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }
    impl<T> ops::SubAssign<Vec3<T>> for Vec3<T>
    where
        T: std::ops::Sub<T, Output = T> + Copy,
    {
        fn sub_assign(&mut self, other: Self) {
            *self = Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }

    impl<T> ops::DivAssign<T> for Vec3<T>
    where
        T: std::ops::Div<T, Output = T> + Copy,
    {
        fn div_assign(&mut self, rhs: T) {
            *self = Self {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            }
        }
    }
    impl<T> ops::MulAssign<T> for Vec3<T>
    where
        T: std::ops::Mul<T, Output = T> + Copy,
    {
        fn mul_assign(&mut self, rhs: T) {
            *self = Self {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            }
        }
    }
    impl<T> ops::Div<T> for Vec3<T>
    where
        T: std::ops::Div<T, Output = T> + Copy + Float,
    {
        type Output = Self;
        fn div(self, rhs: T) -> Self {
            Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
        }
    }
    impl<T> ops::Mul<T> for Vec3<T>
    where
        T: std::ops::Mul<T, Output = T> + Copy + Float,
    {
        type Output = Self;
        fn mul(self, rhs: T) -> Self {
            Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
        }
    }

    impl std::convert::From<Vec3<i32>> for Vec3<f32> {
        fn from(vi32: Vec3<i32>) -> Self {
            Vec3::new(vi32.x as f32, vi32.y as f32, vi32.z as f32)
        }
    }
    impl<T> std::fmt::Display for Vec3<T>
    where
        T: std::fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "<{},{},{}>", self.x, self.y, self.z)
        }
    }
}
#[cfg(test)]
mod test {
    use super::Vec3;
    #[test]
    fn basic() {
        let v = Vec3::new(5.0, 6.0, 6.0);
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let mut v3 = v - v1.into();
        println!("Length {}", v3.len());
        v3 += v;
        v3 /= 5 as f32;
        v1 *= 5.0;
        println!("{}", v3);
        println!("{}", v1);
    }
}
