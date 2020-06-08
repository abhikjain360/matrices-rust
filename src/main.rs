mod input;
mod matrix;

use matrix::Matrix;

fn main() {
    let (a, b): (Matrix<f64>, Matrix<f64>) = input::get_input();
    let c = a.clone() * b.clone();
    c.print();
    let c = matrix::multiplication_normal(&a, &b);
    c.print();
}
