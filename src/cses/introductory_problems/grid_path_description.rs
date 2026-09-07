#![allow(unused)]
use std::{
    io::{self, BufRead},
    println,
};

const DR: [isize; 4] = [-1, 1, 0, 0];
const DC: [isize; 4] = [0, 0, -1, 1];

fn dfs(r: usize, c: usize, step: usize, visited: &mut [[bool; 9]; 9], path: &[u8; 48]) -> u32 {
    if r == 7 && c == 1 {
        return if step == 48 { 1 } else { 0 };
    }

    if step == 48 {
        return 0;
    }

    if visited[r - 1][c] && visited[r + 1][c] && !visited[r][c - 1] && !visited[r][c + 1] {
        return 0;
    }
    if visited[r][c - 1] && visited[r][c + 1] && !visited[r - 1][c] && !visited[r + 1][c] {
        return 0;
    }

    visited[r][c] = true;
    let mut total = 0;

    let dir_code = path[step];
    if dir_code < 4 {
        let nr = (r as isize + DR[dir_code as usize]) as usize;
        let nc = (c as isize + DC[dir_code as usize]) as usize;
        if !visited[nr][nc] {
            total += dfs(nr, nc, step + 1, visited, path);
        }
    } else {
        for dir in 0..4 {
            let nr = (r as isize + DR[dir]) as usize;
            let nc = (c as isize + DC[dir]) as usize;
            if !visited[nr][nc] {
                total += dfs(nr, nc, step + 1, visited, path);
            }
        }
    }

    visited[r][c] = false;
    total
}

pub fn solve(input_str: &str) -> u32 {
    let mut path = [4u8; 48];
    for (i, &b) in input_str.trim().as_bytes().iter().enumerate().take(48) {
        path[i] = match b {
            b'U' => 0,
            b'D' => 1,
            b'L' => 2,
            b'R' => 3,
            _ => 4,
        };
    }

    let mut visited = [[false; 9]; 9];
    for i in 0..9 {
        visited[0][i] = true;
        visited[8][i] = true;
        visited[i][0] = true;
        visited[i][8] = true;
    }

    dfs(1, 1, 0, &mut visited, &path)
}

pub fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    if let Some(Ok(line)) = input.next() {
        println!("{}", solve(&line));
    }
}
