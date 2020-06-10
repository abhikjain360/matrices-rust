#![allow(dead_code)]

/* for display traits of T */
use std::fmt::{Debug, Display};

/* for binary ops of Matrix */
use std::ops::{Add, Index, IndexMut, Mul, Sub};

/* for multithread/ concurrency */
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

// Struct for matrices
#[derive(Debug)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub vals: Vec<Vec<T>>,
}

/* clone implementation needed as vector can't copy */
impl<T> Clone for Matrix<T>
where
    T: Copy,
{
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

/* Indexing the matrices */
impl<T> Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Vec<T> {
        &self.vals[index]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Vec<T> {
        &mut self.vals[index]
    }
}

/* addition of matrices */
impl<T> Add<Matrix<T>> for Matrix<T>
where
    T: Copy + Add<Output = T>,
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
                vals[i].push(self[i][j] + other[i][j]);
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
    T: Copy + Sub<Output = T>,
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
                vals[i].push(self[i][j] - other[i][j]);
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
    T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + From<u8>,
{
    type Output = Matrix<T>;

    fn mul(mut self, mut other: Matrix<T>) -> Matrix<T> {
        let x: Matrix<T>;
        if self.rows > 700 && self.cols > 700 {
            x = multiplication_normal(&self, &other);
        } else {
            x = strassen_wrapper(&mut self, &mut other);
        }
        x
    }
}

/* Easy printing values */
impl<T> Matrix<T>
where
    T: Display + Copy,
{
    pub fn print(&self) {
        print!("rows: {} cols: {}\n", self.rows, self.cols);
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
impl<T> Matrix<T>
where
    T: Clone + From<u8> + Copy,
{
    pub fn fill_zeroes(&mut self, n: usize) {
        if n < self.cols || n < self.rows {
            panic!("can not add zeroes, sizes smaller than self");
        }

        let vec: Vec<T> = vec![T::from(0); n - self.cols];

        for i in 0..self.rows {
            self[i].extend(vec.iter().cloned());
        }

        let vec: Vec<T> = vec![T::from(0); n];

        for _ in self.rows..n {
            self.vals.push(vec.clone());
        }

        self.rows = n;
        self.cols = n;
    }
}

impl<T> Matrix<T>
where
    T: Copy,
{
    pub fn quad(&self, n: usize) -> Matrix<T> {
        if self.cols % 2 == 1 || self.rows % 2 == 1 {
            panic!("dimensions mismatch for quadrant division");
        }
        let (rows, cols) = (self.rows / 2, self.cols / 2);
        Matrix::<T> {
            rows,
            cols,
            vals: match n {
                2 => self.get_vec_part(0, rows, cols, self.cols),
                3 => self.get_vec_part(rows, self.rows, 0, cols),
                4 => self.get_vec_part(rows, self.rows, cols, self.cols),
                1 | _ => self.get_vec_part(0, rows, 0, cols),
            },
        }
    }

    pub fn get_vec_part(&self, r1: usize, r2: usize, c1: usize, c2: usize) -> Vec<Vec<T>> {
        let mut vec: Vec<Vec<T>> = Vec::with_capacity(r2);
        for i in 0..(r2 - r1) {
            vec.push(Vec::with_capacity(c2));
            for j in 0..(c2 - c1) {
                vec[i].push(self.vals[i + r1][j + c1]);
            }
        }
        vec
    }

    pub fn trim(&mut self, rows: usize, cols: usize) {
        self.rows = rows;
        self.cols = cols;
        self.vals = self.get_vec_part(0, rows, 0, cols);
    }
}

impl<T> Matrix<T>
where
    T: Eq,
{
    pub fn isequal(&self, other: &Matrix<T>) -> bool {
        let mut x: bool = true;
        if self.rows != other.rows || self.cols != other.cols {
            x = false
        }
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self[i][j] != other[i][j] {
                    x = false
                }
            }
        }
        x
    }
}

pub fn multiplication_normal<T>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    if a.cols != b.rows {
        panic!("dimensions for multiplication don't match");
    }

    let mut vals: Vec<Vec<T>> = Vec::with_capacity(a.rows);

    for i in 0..a.rows {
        vals.push(Vec::with_capacity(b.cols));
        for j in 0..b.cols {
            vals[i].push(a[i][0] * b[0][j]);
            for k in 1..a.cols {
                vals[i][j] = vals[i][j] + (a[i][k] * b[k][j]);
            }
        }
    }

    Matrix::<T> {
        rows: a.rows,
        cols: b.cols,
        vals,
    }
}

pub fn multiplication_concurrency<T>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Send + Sync + From<u8> + 'static,
{
    if a.cols != b.rows {
        panic!("dimensions for multiplication don't match");
    }

    let vals = Arc::new(Mutex::new(Vec::with_capacity(a.rows)));
    let mut vec: Vec<JoinHandle<()>> = vec![];

    for i in 0..a.rows {
        let temp_vals = Arc::clone(&vals);
        let mut temp_vals = temp_vals.lock().unwrap();
        temp_vals.push(Vec::with_capacity(b.cols));
        drop(temp_vals);
        for j in 0..b.cols {
            let temp_vals = Arc::clone(&vals);
            let mut temp_vals = temp_vals.lock().unwrap();
            temp_vals[i].push(a[i][0] * b[0][j]);
            drop(temp_vals);

            let temp_a = a.vals[i].clone();
            let temp_b = b.get_vec_part(0, b.rows, j, j + 1);
            let cols = a.cols;
            let temp_i = i;
            let temp_vals = Arc::clone(&vals);
            vec.push(thread::spawn(move || {
                for k in 1..cols {
                    let mut temp_vals = temp_vals.lock().unwrap();
                    (*temp_vals)[temp_i][j] = temp_vals[temp_i][j] + temp_a[k] * temp_b[k][0];
                }
            }));
        }
    }

    for i in vec {
        i.join().unwrap();
    }

    let temp_vals = Arc::clone(&vals);
    let temp_vals = temp_vals.lock().unwrap();
    Matrix::<T> {
        rows: a.rows,
        cols: b.cols,
        vals: temp_vals.to_vec(),
    }
}

fn find_greatest_dim<T>(a: &Matrix<T>, b: &Matrix<T>) -> usize {
    let mut n: usize = a.rows;
    if n < a.cols {
        n = a.cols;
    }
    if n < b.cols {
        n = b.cols;
    }
    if n < b.rows {
        n = b.rows;
    }
    n
}

pub fn strassen_wrapper<T>(a: &mut Matrix<T>, b: &mut Matrix<T>) -> Matrix<T>
where
    T: Copy + Sub<Output = T> + Add<Output = T> + Mul<Output = T> + From<u8>,
{
    if a.cols != b.rows {
        panic!("dimensions for multiplication don't match");
    }

    let max_n = find_greatest_dim(&a, &b);
    let mut fill_n: usize = 1;

    while fill_n < max_n {
        fill_n <<= 1;
    }

    let (rows, cols) = (a.rows, b.cols);

    a.fill_zeroes(fill_n);
    b.fill_zeroes(fill_n);

    let mut c = strassen(&a, &b, fill_n);
    c.trim(rows, cols);
    c
}

pub fn strassen<T>(a: &Matrix<T>, b: &Matrix<T>, n: usize) -> Matrix<T>
where
    T: Copy + Sub<Output = T> + Add<Output = T> + Mul<Output = T> + From<u8>,
{
    let c: Matrix<T>;
    if n == 1 {
        c = Matrix::<T> {
            rows: 1,
            cols: 1,
            vals: vec![vec![a[0][0] * b[0][0]]],
        };
    } else {
        let p1 = a.quad(1) * (b.quad(2) - b.quad(4));
        let p2 = (a.quad(1) + a.quad(2)) * b.quad(4);
        let p3 = (a.quad(3) + a.quad(4)) * b.quad(1);
        let p4 = a.quad(4) * (b.quad(3) - b.quad(1));
        let p5 = (a.quad(1) + a.quad(4)) * (b.quad(1) + b.quad(4));
        let p6 = (a.quad(2) - a.quad(4)) * (b.quad(3) + b.quad(4));
        let p7 = (a.quad(1) - a.quad(3)) * (b.quad(1) + b.quad(2));

        c = combine_quad(
            &(p5.clone() + p4.clone() - p2.clone() + p6),
            &(p1.clone() + p2),
            &(p3.clone() + p4),
            &(p1 + p5 - p3 - p7),
        );
    }
    c
}

pub fn combine_quad<T>(a: &Matrix<T>, b: &Matrix<T>, c: &Matrix<T>, d: &Matrix<T>) -> Matrix<T>
where
    T: Clone + Copy,
{
    if a.rows != b.rows || a.cols != c.cols || b.cols != d.cols || c.rows != d.rows {
        panic!("incompatible quad dimensions for combining");
    }

    let mut vals: Vec<Vec<T>> = Vec::with_capacity(a.rows + c.rows);
    let (rows, cols) = (a.rows + c.rows, a.cols + b.cols);

    for i in 0..a.rows {
        vals.push(combine_vecs(a[i][..].to_vec(), b[i][..].to_vec()));
    }
    for i in 0..c.rows {
        vals.push(combine_vecs(c[i][..].to_vec(), d[i][..].to_vec()));
    }

    Matrix::<T> { rows, cols, vals }
}
pub fn combine_vecs<T>(a: Vec<T>, b: Vec<T>) -> Vec<T>
where
    T: Copy,
{
    let mut vec = a;
    for i in 0..b.len() {
        vec.push(b[i]);
    }
    vec
}
