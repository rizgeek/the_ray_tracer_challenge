mod point_vector {
    pub struct PointVector {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32
    }

    impl PointVector {

        pub fn point(x: f32, y: f32, z: f32) -> PointVector {
            PointVector{x, y, z, w: 1.0}
        }

        pub fn vector(x: f32, y: f32, z: f32) -> PointVector {
            PointVector{x, y, z, w: 0.0}
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
    fn is_point() {
        let point: PointVector = PointVector::point(4.0, -4.0, 3.0);
        assert_eq!(point.is_point_or_vector(), 1, "is point");
    }

    #[test]
    fn is_vector() {
        let vector: PointVector = PointVector::vector(4.0, -4.0, 3.0);
        assert_eq!(vector.is_point_or_vector(), 0, "is vector");
    }

    #[test]
    fn point_must_have_w_1() {
        let point: PointVector = PointVector::point(4.0, -4.0, 3.0);
        assert_eq!(point.w, 1.0);
    }

    #[test]
    fn vector_must_have_w_0() {
        let vector: PointVector = PointVector::vector(4.0, -4.0, 3.0);
        assert_eq!(vector.w, 0.0);
    }

}