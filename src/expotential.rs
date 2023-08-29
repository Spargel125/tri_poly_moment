//! `strust Expotential`
pub struct Expotential<T> {
    lambda: T,
}

impl<T> Expotential<T>
where
    T: Into<f64> + Copy,
{
    /// define new `Expotential` distribution
    ///
    /// ## Arguments
    /// * `lambda` : parameter of expotensial distribution
    ///
    /// ## Example
    /// ```
    /// use tri_poly_moment::Expotential;
    /// let expotensial = Expotential::new(2);
    /// expotensial.x(); //if calc E[x]
    /// ```

    pub fn new(lambda: T) -> Self {
        Self { lambda: lambda }
    }
    /// `E[x]`
    pub fn x(&self) -> f64 {
        let lambda = self.lambda.into();
        1.0 / lambda
    }
    /// `E[x^2]`
    pub fn xx(&self) -> f64 {
        let lambda = self.lambda.into();
        2.0 / lambda.powf(2.0)
    }
    /// `E[cos(x)]`
    pub fn c(&self) -> f64 {
        let lambda = self.lambda.into();
        lambda.powf(2.0) / (lambda.powf(2.0) + 1.0)
    }
    /// `E[sin(x)]`
    pub fn s(&self) -> f64 {
        let lambda = self.lambda.into();
        lambda / (lambda.powf(2.0) + 1.0)
    }
    /// `E[x*cos(x)]`
    pub fn xc(&self) -> f64 {
        let lambda = self.lambda.into();
        (lambda.powf(3.0) - lambda) / (lambda.powf(2.0) + 1.0).powf(2.0)
    }
    /// `E[x*sin(x)]`
    pub fn xs(&self) -> f64 {
        let lambda = self.lambda.into();
        (2.0 * lambda.powf(2.0)) / (lambda.powf(2.0) + 1.0).powf(2.0)
    }
    /// `E[x^2*cos(x)]`
    pub fn xxc(&self) -> f64 {
        let lambda = self.lambda.into();
        (2.0 * lambda.powf(2.0)) * (lambda.powf(2.0) - 3.0) / (lambda.powf(2.0) + 1.0).powf(3.0)
    }
    /// `E[x^2*sin(x)]`
    pub fn xxs(&self) -> f64 {
        let lambda = self.lambda.into();
        (2.0 * lambda) * (3.0 * lambda.powf(2.0) - 1.0) / (lambda.powf(2.0) + 1.0).powf(3.0)
    }
    /// `E[cos(x)*sin(x)]`
    pub fn cs(&self) -> f64 {
        let lambda = self.lambda.into();
        lambda / (lambda.powf(2.0) + 4.0)
    }
    /// `E[sin(x)*cos(x)]`
    pub fn sc(&self) -> f64 {
        self.cs()
    }
    /// `E[cos(x)*cos(x)]`
    pub fn cc(&self) -> f64 {
        let lambda = self.lambda.into();
        0.5 * (lambda.powf(2.0) / (lambda.powf(2.0) + 4.0) + 1.0)
    }
    /// `E[sin(x)*sin(x)]`
    pub fn ss(&self) -> f64 {
        let lambda = self.lambda.into();
        0.5 * (1.0 - lambda.powf(2.0) / (lambda.powf(2.0) + 4.0))
    }
}
