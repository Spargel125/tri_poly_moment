extern crate tri_poly_moment;

use tri_poly_moment::Uniform;

#[cfg(test)]
mod tests_uniform {
    use super::*;
    #[test]
    fn test_int() {
        // Test case 1
        let uniform = Uniform::new(3, 9);
        assert!(uniform.x() - 6.0 < 1e-12);
        assert!(uniform.xx() - 39.0 < 1e-12);
        assert!(uniform.c() - 0.045166412863648 < 1e-12);
        assert!(uniform.s() - (-0.013143705785961) < 1e-12);
        assert!(uniform.xc() - 0.560761429618663 < 1e-12);
        assert!(uniform.xs() - 0.916865557390441 < 1e-12);
        assert!(uniform.xxc() - 3.518188423893031 < 1e-12);
        assert!(uniform.xxs() - 11.936792649779795 < 1e-12);
        assert!(uniform.cc() - 0.480351177142802 < 1e-12);
        assert!(uniform.ss() - 0.916865557390441 < 1e-12);
        assert!(uniform.sc() - 0.012493899100262 < 1e-12);
        assert!(uniform.cs() - 0.012493899100262 < 1e-12);
    }
    #[test]
    fn test_float() {
        // Test case 2
        let uniform = Uniform::new(-2.1, 5.4);
        assert!(uniform.x() - 1.65 < 1e-12);
        assert!(uniform.xx() - 7.41 < 1e-12);
        assert!(uniform.c() - (0.012059317212385) < 1e-12);
        assert!(uniform.s() - (-0.151938530738999) < 1e-12);
        assert!(uniform.xc() - (-0.646150522962996) < 1e-12);
        assert!(uniform.xs() - (-0.303562644178352) < 1e-12);
        assert!(uniform.xxc() - (-1.889815931671436) < 1e-12);
        assert!(uniform.xxs() - (-4.056836457095673) < 1e-12);
        assert!(uniform.cc() - (0.438249599917331) < 1e-12);
        assert!(uniform.ss() - (0.561750400082669) < 1e-12);
        assert!(uniform.sc() - (-0.009864363829512) < 1e-12);
        assert!(uniform.cs() - (-0.009864363829512) < 1e-12);
    }
    #[test]
    fn test_int_float() {
        // Test case 3
        let uniform = Uniform::new(-13.3, 11);
        assert!(uniform.x() - (-1.15) < 1e-12);
        assert!(uniform.xx() - 50.53 < 1e-12);
        assert!(uniform.c() - 0.013597549150375 < 1e-12);
        assert!(uniform.s() - 0.030383682087062 < 1e-12);
        assert!(uniform.xc() - (-0.849526484937785) < 1e-12);
        assert!(uniform.xs() - (-0.422126218896357) < 1e-12);
        assert!(uniform.xxc() - (0.738959443649581) < 1e-12);
        assert!(uniform.xxs() - (3.685695659877433) < 1e-12);
        assert!(uniform.cc() - (0.510141908071887) < 1e-12);
        assert!(uniform.ss() - (0.489858091928113) < 1e-12);
        assert!(uniform.sc() - (0.011350961867270) < 1e-12);
        assert!(uniform.cs() - (0.011350961867270) < 1e-12);
    }
}
