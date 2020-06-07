mod input;
mod matrix;

use matrix::Matrix;

fn main() {
    let (a, b): (Matrix<f64>, Matrix<f64>) = input::get_input();
    a.print();
    b.print();
    let c = a + b;
    c.print();
}
