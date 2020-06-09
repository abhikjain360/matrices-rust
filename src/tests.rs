use rand::distributions::{Distribution, Uniform};

use crate::matrix::Matrix;

pub fn create_test_matrices() -> (Matrix<i32>, Matrix<i32>) {
    let mut rng = rand::thread_rng();
    //    let gen = Uniform::from(1..8);
    //    let (r1, c1, c2): (usize, usize, usize) = (
    //        gen.sample(&mut rng),
    //        gen.sample(&mut rng),
    //        gen.sample(&mut rng),
    //    );
    let (r1, c1, c2): (usize, usize, usize) = (1024, 1024, 1024);
    let gen = Uniform::from(1..1024);

    let mut v1: Vec<Vec<i32>> = Vec::with_capacity(r1);
    for i in 0..r1 {
        v1.push(Vec::with_capacity(c1));
        for _ in 0..c1 {
            let x = gen.sample(&mut rng);
            v1[i].push(x);
        }
    }

    let mut v2: Vec<Vec<i32>> = Vec::with_capacity(c1);
    for i in 0..c1 {
        v2.push(Vec::with_capacity(c1));
        for _ in 0..c2 {
            let x = gen.sample(&mut rng);
            v2[i].push(x);
        }
    }

    (
        Matrix::<i32> {
            rows: r1,
            cols: c1,
            vals: v1.clone(),
        },
        Matrix::<i32> {
            rows: c1,
            cols: c2,
            vals: v2.clone(),
        },
    )
}
