extern crate tri_poly_moment;

use tri_poly_moment::Expotential;

#[cfg(test)]
mod tests_expotential {
    use super::*;
    #[test]
    fn test_int() {
        // Test case 1
        let arg = Expotential::new(2);
        assert!((arg.x() - 0.5).abs() < 1e-12);
        assert!((arg.xx() - 0.5).abs() < 1e-12);
        assert!((arg.c() - (0.8)).abs() < 1e-12);
        assert!((arg.s() - (0.4)).abs() < 1e-12);
        assert!((arg.xc() - (0.24)).abs() < 1e-12);
        assert!((arg.xs() - (0.32)).abs() < 1e-12);
        assert!((arg.xxc() - (0.064)).abs() < 1e-12);
        assert!((arg.xxs() - (0.352)).abs() < 1e-12);
        assert!((arg.cc() - (0.750)).abs() < 1e-12);
        assert!((arg.ss() - (0.25)).abs() < 1e-12);
        assert!((arg.sc() - (0.25)).abs() < 1e-12);
        assert!((arg.cs() - (0.25)).abs() < 1e-12);
    }
    #[test]
    fn test_float() {
        // Test case 2
        let arg = Expotential::new(2.3);
        assert!((arg.x() - (0.434782608695652)).abs() < 1e-12);
        assert!((arg.xx() - (0.378071833648393)).abs() < 1e-12);
        assert!((arg.c() - (0.841017488076312)).abs() < 1e-12);
        assert!((arg.s() - (0.365659777424483)).abs() < 1e-12);
        assert!((arg.xc() - (0.249392757575681)).abs() < 1e-12);
        assert!((arg.xs() - (0.267414145652245)).abs() < 1e-12);
        assert!((arg.xxc() - (0.097357455253361)).abs() < 1e-12);
        assert!((arg.xxs() - (0.274863368068631)).abs() < 1e-12);
        assert!((arg.cc() - (0.784714747039828)).abs() < 1e-12);
        assert!((arg.ss() - (0.215285252960172)).abs() < 1e-12);
        assert!((arg.sc() - (0.247578040904198)).abs() < 1e-12);
        assert!((arg.cs() - (0.247578040904198)).abs() < 1e-12);
    }
}
