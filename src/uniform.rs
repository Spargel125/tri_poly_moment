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

    pub fn x(&self) -> f64 {
        let u = self.u.into();
        let l = self.l.into();
        (u + l) / (2.0f64)
    }

    pub fn xx(&self) -> f64 {
        let u = self.u.into();
        let l = self.l.into();
        (u*u + u*l + l*l) / 3.0f64
    }
}