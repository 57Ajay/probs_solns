pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let (mut lo, mut hi) = (0, people.len());
        let mut boats = 0;

        while lo < hi {
            hi -= 1;
            if lo < hi && people[lo] + people[hi] <= limit {
                lo += 1;
            }
            boats += 1;
        }
        boats
    }
}

pub fn main() {
    let people = vec![5, 1, 4, 2];
    let limit = 6;
    println!("{:?}", Solution::num_rescue_boats(people, limit));
}
