use std::{
    io::{self, BufWriter, Read, Write},
    vec, write,
};

struct FenwickTree {
    tree: Vec<usize>,
    n: usize,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        let mut tree = vec![0; n + 1];
        for i in 1..=n {
            tree[i] = 1;
        }
        for i in 1..=n {
            let parent = i + (i & (!i + 1));
            if parent <= n {
                tree[parent] += tree[i];
            }
        }
        FenwickTree { tree, n }
    }

    fn add(&mut self, mut idx: usize, val: isize) {
        while idx <= self.n {
            if val < 0 {
                self.tree[idx] -= (-val) as usize;
            } else {
                self.tree[idx] += val as usize;
            }
            idx += idx & (!idx + 1);
        }
    }

    fn find_kth(&self, k: usize) -> usize {
        let mut idx = 0;
        let mut current_k = k;
        let mask = 1 << (usize::BITS - (self.n | 1).leading_zeros() - 1);

        let mut step = mask;
        while step > 0 {
            let next_idx = idx + step;
            if next_idx <= self.n && self.tree[next_idx] < current_k {
                idx = next_idx;
                current_k -= self.tree[idx];
            }
            step >>= 1;
        }
        idx + 1
    }
}

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_ascii_whitespace();

    let n: usize = match iter.next() {
        Some(val) => val.parse().unwrap(),
        None => return,
    };
    let k: usize = iter.next().unwrap().parse().unwrap();

    let mut bit = FenwickTree::new(n);
    let mut writer = BufWriter::new(io::stdout().lock());

    let mut current_pos = 0;
    for remaining in (1..=n).rev() {
        current_pos = (current_pos + k) % remaining;

        let real_idx = bit.find_kth(current_pos + 1);

        write!(writer, "{} ", real_idx).unwrap();

        bit.add(real_idx, -1);
    }

    writer.flush().unwrap();
}
