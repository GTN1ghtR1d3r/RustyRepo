pub struct MyClass{
    pub  a: f64,
  pub  b: f64,
}

pub trait BaseFunc {
   fn mult(&self)->f64;
   fn div(&self)->f64;
}

impl BaseFunc for MyClass{
    fn mult(&self)->f64{
        self.a*self.b
    }
    fn div(&self)->f64{
        self.a/self.b
    }
}