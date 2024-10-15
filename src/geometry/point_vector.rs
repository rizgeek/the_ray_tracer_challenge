mod point_vector {
    pub struct PointVector {
        x: f32,
        y: f32,
        z: f32,
        w: f32
    }

    impl PointVector {
        pub fn new(x: f32, y: f32, z: f32, w: f32) -> PointVector {
            PointVector {x, y, z, w}
        }

        pub fn point(x: f32, y: f32, z: f32) -> PointVector {
            Self::new(x, y, z, 1.0)
        }

        pub fn vector(x: f32, y: f32, z: f32) -> PointVector {
            Self::new(x,y,z, 0.0)
        }

        pub fn is_point_or_vector(&self) -> u8 {
            match self.w {
                w if w >= 1.0 => 1,
                _ => 0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::point_vector::*;

    #[test]
    fn is_point_tuple() {
        let tuple: PointVector = PointVector::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(tuple.is_point_or_vector(), 1, "is point");
    }

    #[test]
    fn is_vector_tuple() {
        let tuple: PointVector = PointVector::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(tuple.is_point_or_vector(), 0, "is vector");
    }

    #[test]
    fn is_point_direct() {
        let point: PointVector = PointVector::point(4.0, -4.0, 3.0);
        assert_eq!(point.is_point_or_vector(), 1, "is point");
    }

    #[test]
    fn is_vector_direct() {
        let vector: PointVector = PointVector::vector(4.0, -4.0, 3.0);
        assert_eq!(vector.is_point_or_vector(), 0, "is vector");
    }
}