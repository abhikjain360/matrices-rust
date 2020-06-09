#![allow(unused_variables)]
mod input;
mod matrix;
mod tests;

use matrix::Matrix;

fn main() {
    let (a, b): (Matrix<i32>, Matrix<i32>) = tests::create_test_matrices();
    //    a.print();
    //    b.print();
    let c = a.clone() * b.clone();
    //    c.print();
    //let c = matrix::multiplication_normal(&a.clone(), &b.clone());
    //   c.print();
}
