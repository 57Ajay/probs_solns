use std::io::{self, Write};

struct Spiral {
    row: usize,
    col: usize,
}

impl Spiral {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn get_val(&self) -> usize {
        let z = self.row.max(self.col);
        let z2 = z * z;
        if z % 2 == 0 {
            if self.row == z {
                z2 - (self.col - 1)
            } else {
                let prev_z2 = (z - 1) * (z - 1);
                prev_z2 + self.row
            }
        } else {
            if self.col == z {
                z2 - (self.row - 1)
            } else {
                let prev_z2 = (z - 1) * (z - 1);
                prev_z2 + self.col
            }
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<usize>().unwrap();

    let mut buffer = String::new();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for _ in 0..t {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();

        let mut nums = buffer.split_whitespace();
        let row = nums.next().unwrap().parse::<usize>().unwrap();
        let col = nums.next().unwrap().parse::<usize>().unwrap();

        let spiral = Spiral::new(row, col);
        writeln!(handle, "{}", spiral.get_val()).unwrap();
    }
}
