#[cfg(test)]
mod primitive_operations {
    use turtle_graphics::vector2::Vector2;

    #[test]
    fn length() {
        assert_eq!(Vector2::new(1.0, 0.0).length(), 1.0);
        assert_eq!(Vector2::new(0.0, 1.0).length(), 1.0);
        assert_eq!(Vector2::new(0.0, 0.0).length(), 0.0);
    }

    #[test]
    fn add() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(3.0, 4.0);
        assert_eq!(v1 + v2, Vector2::new(4.0, 6.0));
    }

    #[test]
    fn sub() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(3.0, 4.0);
        assert_eq!(v1 - v2, Vector2::new(-2.0, -2.0));
    }

    #[test]
    fn mul() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(3.0, 4.0);
        assert_eq!(v1 * v2, Vector2::new(3.0, 8.0));
    }

    #[test]
    fn mul_scalar() {
        let v1 = Vector2::new(1.0, 2.0);
        assert_eq!(v1 * 2.0, Vector2::new(2.0, 4.0));
    }

    #[test]
    fn div() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(3.0, 4.0);
        assert_eq!(v1 / v2, Vector2::new(1.0 / 3.0, 2.0 / 4.0));
    }

    #[test]
    fn div_scalar() {
        let v1 = Vector2::new(1.0, 2.0);
        assert_eq!(v1 / 2.0, Vector2::new(0.5, 1.0));
    }

    #[test]
    fn neg() {
        let v1 = Vector2::new(1.0, 2.0);
        assert_eq!(-v1, Vector2::new(-1.0, -2.0));
    }
}
