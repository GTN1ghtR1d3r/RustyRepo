pub mod example_file;
use example_file::BaseFunc;
use example_file::MyClass;
fn main(){
    let instance = MyClass::new( 17.0, 22.0); //this is like a psuedo constructor
    let result1 = instance.mult(); //method call
    let result2 = instance.div(); //method call
    println!("multiply results in {result1}");
    println!("div results in {result2}");
}