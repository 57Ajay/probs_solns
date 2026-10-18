pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut boxes = [0u16; 9];

        for r in 0..9 {
            for c in 0..9 {
                let ch = board[r][c];

                if ch == '.' {
                    continue;
                }

                let digit = (ch as u8 - b'1') as usize;
                let bit = 1u16 << digit;

                let box_idx = (r / 3) * 3 + (c / 3);

                if rows[r] & bit != 0 || cols[c] & bit != 0 || boxes[box_idx] & bit != 0 {
                    return false;
                }

                rows[r] |= bit;
                cols[c] |= bit;
                boxes[box_idx] |= bit;
            }
        }

        true
    }
}

pub fn main() {
    let board = vec![
        vec!['1', '2', '.', '.', '3', '.', '.', '.', '.'],
        vec!['4', '.', '.', '5', '.', '.', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '.', '3'],
        vec!['5', '.', '.', '.', '6', '.', '.', '.', '4'],
        vec!['.', '.', '.', '8', '.', '3', '.', '.', '5'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '.', '.', '.', '.', '.', '2', '.', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '8'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    println!("{:?}", Solution::is_valid_sudoku(board));
}
