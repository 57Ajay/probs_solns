use std::collections::VecDeque;
use std::io;

#[allow(dead_code)]
pub fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: usize = input.trim().parse().unwrap();
    assert!(n >= 4);

    let mut grid: Vec<Vec<i32>> = vec![vec![-1; n]; n];

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    grid[0][0] = 0;
    queue.push_back((0, 0));

    // MOVES
    // There are maximum 8 possible moves for a Knight (dy, dx) and minimum 2.
    let moves: [(i32, i32); 8] = [
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
    ];

    // BFS ALGORITHM
    while let Some((y, x)) = queue.pop_front() {
        let current_dist = grid[y][x];

        // Iterate through all 8 possible knight moves
        for &(dy, dx) in &moves {
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;

            if ny >= 0 && ny < n as i32 && nx >= 0 && nx < n as i32 {
                let ny_u = ny as usize;
                let nx_u = nx as usize;

                if grid[ny_u][nx_u] == -1 {
                    grid[ny_u][nx_u] = current_dist + 1;
                    queue.push_back((ny_u, nx_u));
                }
            }
        }
    }

    for row in grid {
        for (i, &val) in row.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", val);
        }
        println!();
    }
}
