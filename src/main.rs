pub mod example_file;
use example_file::BaseFunc;
use example_file::MyClass;
use ndarray::prelude::*;
fn main(){
    let instance = MyClass::new( 17.0, 22.0); //this is like a psuedo constructor
    let result1 = instance.mult(); //method call
    let result2 = instance.div(); //method call
    println!("multiply results in {result1}");
    println!("div results in {result2}");

    let arr1 = Array::zeros((3, 2, 4).f());
    arr1.mapv(f64::sin);
    
    // Specify just the element type and infer the dimensionality:
    let arr2 = Array::<f64, _>::zeros((3, 2, 4).f());
    let arr3: Array<f64, _> = Array::zeros((3, 2, 4).f());
    
    // Specify both the element type and dimensionality:
    let arr4 = Array3::<f64>::zeros((3, 2, 4).f());
    let arr5: Array3<f64> = Array::zeros((3, 2, 4).f());
    let arr6 = Array::<f64, Ix3>::zeros((3, 2, 4).f());
    let arr7: Array<f64, Ix3> = Array::zeros((3, 2, 4).f());
}