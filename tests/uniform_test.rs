extern crate tri_poly_moment;

use tri_poly_moment::Uniform;

#[test]
fn test_uniform_x() {
    // Test case 1
    let uniform_int = Uniform::new(10,2);
    assert_eq!(uniform_int.x(), 6.0);

    // Test case 2
    let uniform_float = Uniform::new(5.3,-2.0);
    assert_eq!(uniform_float.x(), 1.65);

    // Test case 3
    let uniform_int_float = Uniform::new(5,1.0);
    assert_eq!(uniform_int_float.x(), 3.0);
}