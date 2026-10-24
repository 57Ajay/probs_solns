pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        // i guess it is continutation of previous problem
        // which was to count number of 1 bits, so we can use
        // that as helper but here i have used this builtin function.
        if n == 0 {
            return vec![0];
        }
        let mut vec = Vec::with_capacity(n as usize + 1);

        for i in 0..=n {
            vec.push(i.count_ones() as i32);
        }

        vec
    }
}

pub fn main() {
    println!("{:?}", Solution::count_bits(4));
}
