//! `strust Gaussian`
pub struct Gaussian<T, U> {
    mu: T,
    sigma2: U,
}

impl<T, U> Gaussian<T, U>
where
    T: Into<f64> + Copy,
    U: Into<f64> + Copy,
{
    /// define new `Gaussian` distribution
    ///
    /// ## Arguments
    /// * `mu` : mean of gaussian distribution
    /// * `sigma2` :  variance of gaussian distribution (sigma2 = sigma^2)
    ///
    /// ## Example
    /// ```
    /// use tri_poly_moment::Gaussian;
    /// let gaussian = Gaussian::new(3, 4);
    /// gaussian.x(); //if calc E[x]
    /// ```

    fn parse_argument(&self) -> (f64, f64) {
        (self.mu.into(), self.sigma2.into())
    }
    pub fn new(mu: T, sigma2: U) -> Self {
        Self {
            mu: mu,
            sigma2: sigma2,
        }
    }
    ///`E[x]`
    pub fn x(&self) -> f64 {
        let (mu, _sigma2) = self.parse_argument();
        mu
    }
    /// `E[x^2]`
    pub fn xx(&self) -> f64 {
        let (mu, sigma2) = self.parse_argument();
        sigma2 + mu * mu
    }
    /// `E[cos(x)]`
    pub fn c(&self) -> f64 {
        let (mu, sigma2) = self.parse_argument();
        mu.cos() * (-0.5 * sigma2).exp()
    }
    /// `E[sin(x)]`
    pub fn s(&self) -> f64 {
        let (mu, sigma2) = self.parse_argument();
        mu.sin() * (-0.5 * sigma2).exp()
    }
    /// `E[x*cos(x)]`
    pub fn xc(&self) -> f64 {
        let (mu, sigma2) = self.parse_argument();
        (mu * mu.cos() - sigma2 * mu.sin()) * (-0.5 * sigma2).exp()
    }
    /// `E[x*sin(x)]`
    pub fn xs(&self) -> f64 {
        let (mu, sigma2) = self.parse_argument();
        (mu * mu.sin() + sigma2 * mu.cos()) * (-0.5 * sigma2).exp()
    }
    /// `E[x^2*cos(x)]`
    pub fn xxc(&self) -> f64 {
        let (mu, sigma2) = self.parse_argument();
        ((sigma2 + mu.powf(2.0) - sigma2.powf(2.0)) * mu.cos() - 2.0 * mu * sigma2 * mu.sin())
            * (-0.5 * sigma2).exp()
    }
    /// `E[x^2*sin(x)]`
    pub fn xxs(&self) -> f64 {
        let (mu, sigma2) = self.parse_argument();
        ((sigma2 + mu.powf(2.0) - sigma2.powf(2.0)) * mu.sin() + 2.0 * mu * sigma2 * mu.cos())
            * (-0.5 * sigma2).exp()
    }
    /// `E[cos(x)*sin(x)]`
    pub fn cs(&self) -> f64 {
        let (mu, sigma2) = self.parse_argument();
        0.5 * (2.0 * mu).sin() * (-0.5 * 4.0 * sigma2).exp()
    }
    /// `E[sin(x)*cos(x)]`
    pub fn sc(&self) -> f64 {
        self.cs()
    }
    /// `E[cos^2(x)]`
    pub fn cc(&self) -> f64 {
        let (mu, sigma2) = self.parse_argument();
        0.5 * ((2.0 * mu).cos() * (-0.5 * 4.0 * sigma2).exp() + 1.0)
    }
    /// `E[sin^2(x)]`
    pub fn ss(&self) -> f64 {
        let (mu, sigma2) = self.parse_argument();
        0.5 * (1.0 - (2.0 * mu).cos() * (-0.5 * 4.0 * sigma2).exp())
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;
    #[test]
    fn parse_test() {
        let arg = Gaussian::new(-2, 5.5);
        let (l, u) = arg.parse_argument();
        assert!(l == -2.0 && u == 5.5);
    }
}
