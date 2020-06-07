use std::fmt::Display;
use std::ops::{Add, Mul};

pub struct Matrix<T: Display> {
    pub rows: usize,
    pub cols: usize,
    pub vals: Vec<Vec<T>>,
}

impl<T: Display + Copy> Clone for Matrix<T> {
    fn clone(&self) -> Matrix<T> {
        let mut vals: Vec<Vec<T>> = Vec::with_capacity(self.rows);
        let mut index: usize = 0;
        for i in &self.vals {
            vals.push(Vec::with_capacity(self.cols));
            for j in i {
                vals[index].push(*j);
            }
            index += 1;
        }
        Matrix::<T> {
            rows: self.rows,
            cols: self.cols,
            vals,
        }
    }
}

impl<T> Add<Matrix<T>> for Matrix<T>
where
    T: Display + Copy + Add<Output = T> + Mul,
{
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

impl<T: Display + Copy> Matrix<T> {
    pub fn print(&self) {
        for i in &self.vals {
            for j in i {
                print!("{} ", j);
            }
            println!();
        }
        println!("===================");
    }
}

pub fn normal_multiplication<T: Display + Copy + Add<Output = T> + Mul<Output = T>>(
    a: &Matrix<T>,
    b: &Matrix<T>,
) -> Matrix<T> {
    if a.cols != b.rows {
        panic!("dimensions for multiplication don't match");
    }

    let mut vals: Vec<Vec<T>> = Vec::with_capacity(a.rows);

    for i in 0..a.rows {
        vals.push(Vec::with_capacity(b.cols));
        for j in 0..a.cols {
            vals[i].push(a.vals[i][0] * b.vals[0][j]);
            for k in 1..b.cols {
                vals[i][j] = vals[i][j] + (a.vals[i][k] * b.vals[k][j]);
            }
        }
    }

    Matrix::<T> {
        rows: a.rows,
        cols: b.cols,
        vals,
    }
}
