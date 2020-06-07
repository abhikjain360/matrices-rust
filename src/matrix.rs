use std::fmt::Display;
use std::ops::Add;

pub struct Matrix<T: Display + Copy> {
    pub rows: usize,
    pub cols: usize,
    pub vals: Vec<Vec<T>>,
}

impl<T: Display + Copy> Matrix<T> {
    pub fn print(&self) {
        for i in &self.vals {
            for j in i {
                print!("{} ", j);
            }
            println!();
        }
    }
}

impl<T: Display + Copy + Add<Output = T>> Add<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, other: Matrix<T>) -> Matrix<T> {
        if self.cols != other.cols || self.rows != other.rows {
            panic!("add: matrix dimensions not equal");
        }
        let mut vals: Vec<Vec<T>> = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            vals.push(Vec::with_capacity(self.cols));
            for j in 0..self.cols {
                vals[i].push(self.vals[i][j] + other.vals[i][j]);
            }
        }

        Matrix::<T> {
            rows: self.rows,
            cols: self.cols,
            vals,
        }
    }
}
