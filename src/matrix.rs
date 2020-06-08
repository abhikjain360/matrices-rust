/* for display traits of T */
use std::fmt::Display;

/* for binary ops of Matrix */
use std::ops::{Add, Mul, Sub};

// Struct for matrices
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub vals: Vec<Vec<T>>,
}

/* clone implementation needed as vector can't copy */
impl<T: Copy> Clone for Matrix<T> {
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

/* addition of matrices */
impl<T> Add<Matrix<T>> for Matrix<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
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

/* subtraction of matrices */
impl<T> Sub<Matrix<T>> for Matrix<T>
where
    T: Copy + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<T>;
    fn sub(self, other: Matrix<T>) -> Matrix<T> {
        if self.cols != other.cols || self.rows != other.rows {
            panic!("add: matrix dimensions not equal");
        }
        let mut vals: Vec<Vec<T>> = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            vals.push(Vec::with_capacity(self.cols));
            for j in 0..self.cols {
                vals[i].push(self.vals[i][j] - other.vals[i][j]);
            }
        }

        Matrix::<T> {
            rows: self.rows,
            cols: self.cols,
            vals,
        }
    }
}

/* multiplication of matrices */
impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<T>;

    fn mul(self, other: Matrix<T>) -> Matrix<T> {
        multiplication_normal(&self, &other)
    }
}

/* Easy printing values */
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

/* method needed for strassen */
impl<T: Clone + From<u32>> Matrix<T> {
    pub fn fill_zeroes(&mut self, n: usize) {
        if n < self.cols || n < self.rows {
            panic!("can not add zeroes, sizes smaller than self");
        }

        let vec: Vec<T> = vec![T::from(0); n - self.cols];

        for i in 0..self.rows {
            self.vals[i].extend(vec.iter().cloned());
        }

        let vec: Vec<T> = vec![T::from(0); n];

        for _ in self.rows..n {
            self.vals.push(vec.clone());
        }
    }
}

fn multiplication_normal<T: Copy + Add<Output = T> + Mul<Output = T>>(
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
