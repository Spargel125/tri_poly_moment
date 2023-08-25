extern crate tri_poly_moment;

use tri_poly_moment::Uniform;

#[cfg(test)]
mod tests_uniform {
    use super::*;
    #[test]
    fn test_int() {
        // Test case 1
        let uniform_int = Uniform::new(3, 9);
        assert!(uniform_int.x() - 6.0 < 1e-12);
        assert!(uniform_int.xx() - 39.0 < 1e-12);
    }
    #[test]
    fn test_float() {
        // Test case 2
        let uniform_float = Uniform::new(-2.1, 5.4);
        assert!(uniform_float.x() - 1.65 < 1e-12);
        assert!(uniform_float.xx() - 7.41 < 1e-12);
    }
    #[test]
    fn test_int_float() {
        // Test case 3
        let uniform_int_float = Uniform::new(-13.3, 11);
        assert!(uniform_int_float.x() - (-1.15) < 1e-12);
        assert!(uniform_int_float.xx() - 50.53 < 1e-12);
    }
}
