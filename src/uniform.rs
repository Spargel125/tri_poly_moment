//! `strust Uniform`
pub struct Uniform<T, U> {
    l: T,
    u: U,
}

impl<T, U> Uniform<T, U>
where
    T: Into<f64> + Copy,
    U: Into<f64> + Copy,
{
    /// define new `Uniform` distribution
    ///
    /// ## Arguments
    /// * `l` : lower of uniform distribution
    /// * `u` : upper of uniform distribution
    ///
    /// ## Example
    /// ```
    /// use tri_poly_moment::Uniform;
    /// let uniform = Uniform::new(-13.3, 11);
    /// uniform.x(); //if calc E[x]
    /// ```

    fn parse_argument(&self) -> (f64, f64) {
        (self.l.into(), self.u.into())
    }

    pub fn new(l: T, u: U) -> Self {
        Self { l: l, u: u }
    }
    /// `E[x]`
    pub fn x(&self) -> f64 {
        let (l, u) = self.parse_argument();
        (u + l) / (2.0f64)
    }
    /// `E[x^2]`
    pub fn xx(&self) -> f64 {
        let (l, u) = self.parse_argument();
        (u * u + u * l + l * l) / 3.0f64
    }
    /// `E[cos(x)]`
    pub fn c(&self) -> f64 {
        let (l, u) = self.parse_argument();
        (u.sin() - l.sin()) / (u - l)
    }
    /// `E[sin(x)]`
    pub fn s(&self) -> f64 {
        let (l, u) = self.parse_argument();
        -(u.cos() - l.cos()) / (u - l)
    }
    /// `E[x*cos(x)]`
    pub fn xc(&self) -> f64 {
        let (l, u) = self.parse_argument();
        -(-u * u.sin() + l * l.sin() - u.cos() + l.cos()) / (u - l)
    }
    /// `E[x*sin(x)]`
    pub fn xs(&self) -> f64 {
        let (l, u) = self.parse_argument();
        -(u * u.cos() - l * l.cos() - u.sin() + l.sin()) / (u - l)
    }
    /// `E[x^2 cos(x)]`
    pub fn xxc(&self) -> f64 {
        let (l, u) = self.parse_argument();
        -((-u * u + 2.0) * u.sin() - 2.0 * u * u.cos()
            + (l * l - 2.0) * l.sin()
            + 2.0 * l * l.cos())
            / (u - l)
    }
    /// `E[x^2 sin(x)]`
    pub fn xxs(&self) -> f64 {
        let (l, u) = self.parse_argument();
        ((-u * u + 2.0) * u.cos() + 2.0 * u * u.sin() + (l * l - 2.0) * l.cos() - 2.0 * l * l.sin())
            / (u - l)
    }
    /// `E[cos(x)*sin(x)]`
    pub fn cs(&self) -> f64 {
        let (l, u) = self.parse_argument();
        -0.5 * ((2.0 * u).cos() - (2.0 * l).cos()) / (2.0 * u - 2.0 * l)
    }
    /// `E[sin(x)*cos(x)]`
    pub fn sc(&self) -> f64 {
        self.cs()
    }
    /// `E[cos(x)*cos(x)]`
    pub fn cc(&self) -> f64 {
        let (l, u) = self.parse_argument();
        0.5 * (((2.0 * u).sin() - (2.0 * l).sin()) / (2.0 * u - 2.0 * l) + 1.0)
    }
    /// `E[sin(x)*sin(x)]`
    pub fn ss(&self) -> f64 {
        let (l, u) = self.parse_argument();
        0.5 * (1.0 - ((2.0 * u).sin() - (2.0 * l).sin()) / (2.0 * u - 2.0 * l))
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;
    #[test]
    fn parse_test() {
        let arg = Uniform::new(-2, 5.5);
        let (l, u) = arg.parse_argument();
        assert!(l == -2.0 && u == 5.5);
    }
}
