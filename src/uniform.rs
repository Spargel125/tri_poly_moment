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
    /// calc `E[x]`
    pub fn x(&self) -> f64 {
        let (l, u) = self.parse_argument();
        (u + l) / (2.0f64)
    }
    /// calc `E[x^2]`
    pub fn xx(&self) -> f64 {
        let (l, u) = self.parse_argument();
        (u * u + u * l + l * l) / 3.0f64
    }
    /// calc `E[cos(x)]`
    pub fn c(&self) -> f64 {
        let (l, u) = self.parse_argument();
        (u.sin() - l.sin()) / (u - l)
    }
    /// calc `E[sin(x)]`
    pub fn s(&self) -> f64 {
        let (l, u) = self.parse_argument();
        -(u.cos() - l.cos()) / (u - l)
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
