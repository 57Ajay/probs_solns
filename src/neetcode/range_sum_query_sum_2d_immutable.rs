#![allow(unused)]

struct NumMatrix {
    data: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self { data: matrix }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut sum = 0;
        for r in row1..=row2 {
            for c in col1..=col2 {
                sum += self.data[r as usize][c as usize];
            }
        }
        sum
    }
}

pub fn main() {
    let matrix = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ];

    let num_matrix = NumMatrix::new(matrix);
    println!("{}", num_matrix.sum_region(2, 1, 4, 3));
    println!("{}", num_matrix.sum_region(1, 1, 2, 2));
}
