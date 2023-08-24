pub struct Uniform<T,U> {
    u:T,
    l:U,
}

impl<T,U> Uniform<T,U>
where 
    T:Into<f64> + Copy,
    U:Into<f64> + Copy,
{
    pub fn new(u:T,l:U) -> Self{
        Self { u: u, l: l }
    }

    pub fn x(&self) -> f64{
        (self.u.into()+self.l.into())/(2.0f64)
    }
}

