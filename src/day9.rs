use itertools::Itertools;

use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day9 {
    numbers: Vec<Vec<i32>>,
}

impl FromInput for Day9 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut numbers = vec![];
        for l in lines {
            let mut values: Vec<i32> = vec![];
            for x in l.split_whitespace() {
                values.push(x.parse::<i32>().unwrap());
            }
            numbers.push(values);
        }
        Day9 { numbers }
    }
}

#[allow(dead_code)]
fn print_matrix(matrix: &Vec<Vec<i32>>) {
    let max = matrix
        .iter()
        .map(|vector| vector.iter().max().unwrap())
        .max()
        .unwrap();

    let max_len = max.to_string().len();

    for line in matrix {
        for v in line {
            let len = v.to_string().len();
            if len < max_len {
                for _ in 0..(max_len - len) {
                    print!(" ");
                }
            }
            print!("{} ", v);
        }
        println!();
    }
}

/* Build a matrix to solve */
fn build_matrix(dim: usize) -> Vec<Vec<i32>> {
    // init with 0
    let mut matrix = vec![];
    for _ in 0..dim {
        matrix.push(vec![0; dim]);
    }
    matrix
}

fn fill_forward(matrix: &mut [Vec<i32>], dim: usize) -> usize {
    let mut idx = 1_usize;

    loop {
        for i in idx..dim - 1 {
            matrix[idx][i] = matrix[idx - 1][i] - matrix[idx - 1][i - 1];
        }
        let zero_line = !matrix[idx].iter().map(|x| *x == 0).contains(&false);
        if zero_line {
            break;
        }
        idx += 1;
    }

    idx
}

fn solve_one(numbers: &Vec<i32>) -> i32 {
    // need columns for all numbers PLUS the one we're solving for
    let dim = numbers.len() + 1;

    let mut matrix = build_matrix(dim);

    // fill in first line
    for (i, n) in numbers.iter().enumerate() {
        matrix[0][i] = *n;
    }

    let idx = fill_forward(&mut matrix, dim);

    //print_matrix(&matrix);

    for i in (0..idx).rev() {
        matrix[i][dim - 1] = matrix[i + 1][dim - 1] + matrix[i][dim - 2];
    }

    //print_matrix(&matrix);

    matrix[0][dim - 1]
}

fn solve_two(numbers: &Vec<i32>) -> i32 {
    // need columns for all numbers PLUS the one we're solving for
    let dim = numbers.len() + 1;

    let mut matrix = build_matrix(dim);

    // fill in first line
    for (i, n) in numbers.iter().enumerate() {
        matrix[0][i + 1] = *n;
    }

    let idx = fill_forward(&mut matrix, dim);

    //print_matrix(&matrix);

    for i in (0..idx).rev() {
        matrix[i][i] = matrix[i][i + 1] - matrix[i + 1][i + 1];
    }

    //print_matrix(&matrix);

    matrix[0][0]
}

impl DaySolution for Day9 {
    fn part_one(&self) -> String {
        let mut sum = 0;
        for numbers in &self.numbers {
            sum += solve_one(numbers);
            //println!("----------------------------------------------------------------------------");
        }
        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0;
        for numbers in &self.numbers {
            sum += solve_two(numbers);
            //println!("----------------------------------------------------------------------------");
        }
        sum.to_string()
    }
}
