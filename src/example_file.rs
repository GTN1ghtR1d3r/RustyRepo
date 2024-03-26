pub struct MyClass{
      a: f64,
      b: f64,
}

pub trait BaseFunc { //trait defines like... a template? I guess for what you're going to implement in a class, you have to fill out the def for each of the traits.
    fn new(a:f64,b:f64)->Self; 
    fn mult(&self)->f64;
    fn div(&self)->f64;
}

impl BaseFunc for MyClass{
   fn new(a: f64, b: f64)->Self{
    return Self{a,b};
    }
   fn mult(&self)->f64{
        return self.a*self.b;
    }
   fn div(&self)->f64{
        return self.a/self.b;
    }
}