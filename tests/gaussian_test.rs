extern crate tri_poly_moment;

use tri_poly_moment::Gaussian;

#[cfg(test)]
mod tests_gaussian {
    use super::*;
    #[test]
    fn test_int() {
        // Test case 1
        let arg = Gaussian::new(3, 9);
        assert!((arg.x() - 3.0).abs() < 1e-12);
        assert!((arg.xx() - 18.0).abs() < 1e-12);
        assert!((arg.c() - (-0.010997823217620)).abs() < 1e-12);
        assert!((arg.s() - (0.001567701681014)).abs() < 1e-12);
        assert!((arg.xc() - (-0.047102784781985)).abs() < 1e-12);
        assert!((arg.xs() - (-0.094277303915540)).abs() < 1e-12);
        assert!((arg.xxc() - (0.608206971935328)).abs() < 1e-12);
        assert!((arg.xxs() - (-0.692647659655360)).abs() < 1e-12);
        assert!((arg.cc() - (0.500000007311687)).abs() < 1e-12);
        assert!((arg.ss() - (0.499999992688313)).abs() < 1e-12);
        assert!((arg.sc() - (-2.127746188964214e-09)).abs() < 1e-12);
        assert!((arg.cs() - (-2.127746188964214e-09)).abs() < 1e-12);
    }
    #[test]
    fn test_float() {
        // Test case 2
        let arg = Gaussian::new(-2.1, 0.4 * 0.4);
        assert!((arg.x() - (-2.1)).abs() < 1e-12);
        assert!((arg.xx() - (4.57)).abs() < 1e-12);
        assert!((arg.c() - (-0.466031691565746)).abs() < 1e-12);
        assert!((arg.s() - (-0.796842676707630)).abs() < 1e-12);
        assert!((arg.xc() - (1.106161380561287)).abs() < 1e-12);
        assert!((arg.xs() - (1.598804550435504)).abs() < 1e-12);
        assert!((arg.xxc() - (-2.653312697898903)).abs() < 1e-12);
        assert!((arg.xxs() - (-3.307998563297974)).abs() < 1e-12);
        assert!((arg.cc() - (0.321998788334247)).abs() < 1e-12);
        assert!((arg.ss() - (0.678001211665753)).abs() < 1e-12);
        assert!((arg.sc() - (0.316446953937443)).abs() < 1e-12);
        assert!((arg.cs() - (0.316446953937443)).abs() < 1e-12);
    }
    #[test]
    fn test_int_float() {
        // Test case 3
        let arg = Gaussian::new(2.5, 2 * 2);
        assert!((arg.x() - (2.5)).abs() < 1e-12);
        assert!((arg.xx() - (10.25)).abs() < 1e-12);
        assert!((arg.c() - (-0.108422998123248)).abs() < 1e-12);
        assert!((arg.s() - (0.080994397131532)).abs() < 1e-12);
        assert!((arg.xc() - (-0.595035083834248)).abs() < 1e-12);
        assert!((arg.xs() - (-0.231205999664163)).abs() < 1e-12);
        assert!((arg.xxc() - (-0.996455703421959)).abs() < 1e-12);
        assert!((arg.xxs() - (-2.634177745971273)).abs() < 1e-12);
        assert!((arg.cc() - (0.500047579031086)).abs() < 1e-12);
        assert!((arg.ss() - (0.499952420968914)).abs() < 1e-12);
        assert!((arg.sc() - (-1.608416285690032e-04)).abs() < 1e-12);
        assert!((arg.cs() - (-1.608416285690032e-04)).abs() < 1e-12);
    }
}
