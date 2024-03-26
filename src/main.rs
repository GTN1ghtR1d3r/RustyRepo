pub mod example_file;
use example_file::BaseFunc;
use example_file::MyClass;
fn main(){
    let instance = MyClass::new( 17.0, 22.0);
    let result1 = instance.mult();
    let result2 = instance.div();
    println!("multiply results in {result1}");
    println!("div results in {result2}");
}