pub mod my_class;
use crate::my_class::BaseFunc;
fn main(){
    let instance = my_class::MyClass{a:17.0, b:22.0};
    let result1 = instance.mult();
    let result2 = instance.div();
    println!("multiply results in {result1}");
    println!("div results in {result2}");
}