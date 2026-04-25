use std::io::{self, BufRead};

#[allow(dead_code)]
pub fn main() {
    greedy_sol();
}

#[allow(dead_code)]
fn non_greedy_sol() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let rc = n
        .trim()
        .split_whitespace()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    assert!(rc.len() == 2);

    let r = rc.get(0).unwrap();
    let c = rc.get(1).unwrap();

    let mut grid = vec![vec!['x'; *c]; *r];
    let mut final_grid = vec![vec!['x'; *c]; *r];

    for i in 0..*r {
        n.clear();
        io::stdin().read_line(&mut n).unwrap();
        let mut l = n.trim().chars().collect::<Vec<char>>();
        l.truncate(*c);
        grid[i] = l;
    }

    for i in 0..*r {
        for j in 0..*c {
            final_grid[i][j] = grid[i][j];
            let v = get_non_adj_key_(&mut final_grid, (i, j), *r, *c).unwrap();
            final_grid[i][j] = v;
        }
    }

    for i in 0..*r {
        for j in 0..*c {
            print!("{}", final_grid[i][j]);
        }
        println!();
    }
}
#[allow(dead_code)]
fn get_non_adj_key_<'a>(
    grid: &'a mut Vec<Vec<char>>,
    cp: (usize, usize),
    row: usize,
    col: usize,
) -> Option<char> {
    let mut adj_ks = Vec::<char>::new();
    adj_ks.push(grid[cp.0][cp.1]);

    let places_to_look = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for &(x, y) in &places_to_look {
        let nx = x + cp.0 as i32;
        let ny = y + cp.1 as i32;

        if nx >= 0 && nx < row as i32 && ny >= 0 && ny < col as i32 {
            adj_ks.push(grid[nx as usize][ny as usize]);
        }
    }

    let mut unq = Vec::<char>::new();

    for k in adj_ks {
        if !unq.contains(&k) {
            unq.push(k);
        }
    }

    // use std::time::{SystemTime, UNIX_EPOCH};

    let k = ['A', 'B', 'C', 'D']
        .iter()
        .filter(|v| !unq.contains(*v))
        .collect::<Vec<&char>>();

    // this is just so k can be random everytime
    // let mut seed = SystemTime::now()
    //     .duration_since(UNIX_EPOCH)
    //     .unwrap()
    //     .subsec_nanos() as usize;
    //
    // for i in (1..k.len()).rev() {
    //     seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    //     let j = seed % (i + 1);
    //     k.swap(i, j);
    // }

    Some(**k.get(0).unwrap())
}

#[allow(dead_code)]
fn greedy_sol() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let rc: Vec<usize> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let r = rc[0];
    let c = rc[1];

    let mut grid: Vec<Vec<char>> = Vec::with_capacity(r);
    for _ in 0..r {
        let line = lines.next().unwrap().unwrap();
        grid.push(line.chars().collect());
    }

    for i in 0..r {
        for j in 0..c {
            for &candidate in &['A', 'B', 'C', 'D'] {
                let mut fail = false;

                if grid[i][j] == candidate {
                    fail = true;
                }

                if j > 0 && grid[i][j - 1] == candidate {
                    fail = true;
                }

                if i > 0 && grid[i - 1][j] == candidate {
                    fail = true;
                }

                if !fail {
                    grid[i][j] = candidate;
                    break;
                }
            }
        }
    }

    for row in grid {
        let s: String = row.into_iter().collect();
        println!("{}", s);
    }
}
