use std::collections::HashMap;

#[allow(unused, dead_code)]
fn fib_xor(n: usize) -> usize {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n - 2 {
        a ^= b; // a = a ^ b
        b ^= a; // b = b ^ a ^ b => a
        a ^= b; // a ^ a ^ b => b

        b = a + b; // next val;
    }

    return b;
}

#[allow(unused, dead_code)]
fn fib_recurs(n: usize) -> usize {
    if n == 1 {
        return 0;
    }

    if n == 2 {
        return 1;
    }

    let a = fib_recurs(n - 1);
    let b = fib_recurs(n - 2);
    return a + b;
}

#[allow(unused, dead_code)]
fn fib_memo(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if memo.contains_key(&n) {
        return *memo.get(&n).unwrap();
    }
    if n == 1 {
        return 0;
    }

    if n == 2 {
        return 1;
    }
    let a = fib_memo(n - 1, memo);
    memo.insert(&n - 1, a);
    let b = fib_memo(n - 2, memo);
    memo.insert(&n - 2, b);
    return a + b;
}

// 0 1 1 2 3 5 8 13 21 34 55

const FIB_UPTO: usize = 30;
pub fn main() {
    let mut hmap = HashMap::new();
    // let xor = fib_xor(FIB_UPTO);
    // println!("xor: {}", xor);

    // let recurs = fib_recurs(FIB_UPTO);
    // println!("recurs: {}", recurs);

    let memo = fib_memo(FIB_UPTO, &mut hmap);
    println!("memo: {}", memo);
}
