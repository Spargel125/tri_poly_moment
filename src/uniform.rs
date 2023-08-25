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
    pub fn new(l: T, u: U) -> Self {
        Self { l: l, u: u }
    }
    /// calc E[x]
    pub fn x(&self) -> f64 {
        let u = self.u.into();
        let l = self.l.into();
        (u + l) / (2.0f64)
    }
    /// calc E[x^2]
    pub fn xx(&self) -> f64 {
        let u = self.u.into();
        let l = self.l.into();
        (u * u + u * l + l * l) / 3.0f64
    }
    /// calc E[cos(x)]
    pub fn c(&self) -> f64 {
        let u = self.u.into();
        let l = self.l.into();
        (u.sin() - l.sin()) / (u - l)
    }
    // calc E[sin(x)]
    pub fn s(&self) -> f64 {
        let u = self.u.into();
        let l = self.l.into();
        -(u.cos() - l.cos()) / (u - l)
    }
}
