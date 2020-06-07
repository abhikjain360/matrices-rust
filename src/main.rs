mod input;
mod matrix;

use matrix::Matrix;

fn main() {
    let (a, b): (Matrix<f64>, Matrix<f64>) = input::get_input();
    a.print();
    b.print();
    let mut c = a.clone() + b.clone();
    c.print();
    c = matrix::normal_multiplication(&a, &b);
    c.print();
    a.print();
}
