mod input;
mod matrix;

use matrix::Matrix;

fn main() {
    let (mut a, b): (Matrix<f64>, Matrix<f64>) = input::get_input();
    a.fill_zeroes(4);
    a.print();
}
