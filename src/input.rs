#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::matrix::Matrix;

pub fn get_input() -> (Matrix<f64>, Matrix<f64>) {
    let mut buffer = BufReader::new(File::open("inputfile").unwrap());
    let mut line = String::new();

    match buffer.read_line(&mut line) {
        Err(why) => panic!("{}", why),
        _ => {}
    }
    let mut nums = line.split_whitespace().take(2);
    let (r1, c1): (usize, usize) = (
        nums.next().unwrap().parse().unwrap(),
        nums.next().unwrap().parse().unwrap(),
    );

    line.clear();

    match buffer.read_line(&mut line) {
        Err(why) => panic!("{}", why),
        _ => {}
    }
    let mut nums = line.split_whitespace().take(2);
    let (r2, c2): (usize, usize) = (
        nums.next().unwrap().parse().unwrap(),
        nums.next().unwrap().parse().unwrap(),
    );

    line.clear();
    let mut index: usize = 0;
    let mut a_vals: Vec<Vec<f64>> = Vec::with_capacity(r1);

    while index < r1 {
        match buffer.read_line(&mut line) {
            Err(why) => panic!("{}", why),
            _ => {}
        }
        let num_vec: Vec<f64> = line
            .split_whitespace()
            .take(c1)
            .map(|num| num.parse().unwrap())
            .collect();
        a_vals.push(num_vec);
        index += 1;
        line.clear();
    }

    let mut index: usize = 0;
    let mut b_vals: Vec<Vec<f64>> = Vec::with_capacity(r1);

    while index < r2 {
        match buffer.read_line(&mut line) {
            Err(why) => panic!("{}", why),
            _ => {}
        }
        let num_vec: Vec<f64> = line
            .split_whitespace()
            .take(c2)
            .map(|num| num.parse().unwrap())
            .collect();
        b_vals.push(num_vec);
        index += 1;
        line.clear();
    }

    return (
        Matrix::<f64> {
            rows: r1,
            cols: c1,
            vals: a_vals,
        },
        Matrix::<f64> {
            rows: r2,
            cols: c2,
            vals: b_vals,
        },
    );
}
