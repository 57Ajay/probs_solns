use std::io;

struct ChessBoard {
    board: Vec<Vec<u8>>,
}

impl ChessBoard {
    fn new() -> Self {
        Self {
            board: Vec::with_capacity(8),
        }
    }

    fn solve(
        row: usize,
        board: &Vec<Vec<u8>>,
        cols: &mut [bool; 8],
        diag1: &mut [bool; 15],
        diag2: &mut [bool; 15],
    ) -> i32 {
        if row == 8 {
            return 1;
        }

        let mut count = 0;

        for col in 0..8 {
            if board[row][col] == 1 {
                continue;
            }

            let d1 = row + col;
            let d2 = row as isize - col as isize + 7;

            if cols[col] || diag1[d1] || diag2[d2 as usize] {
                continue;
            }

            cols[col] = true;
            diag1[d1] = true;
            diag2[d2 as usize] = true;

            count += Self::solve(row + 1, board, cols, diag1, diag2);

            cols[col] = false;
            diag1[d1] = false;
            diag2[d2 as usize] = false;
        }

        count
    }
}

#[allow(dead_code)]
pub fn main() {
    let mut buf = String::new();
    let mut chess_board = ChessBoard::new();
    for _ in 0..8 {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let row = buf
            .trim()
            .chars()
            .map(|x| (x != '.') as u8)
            .collect::<Vec<u8>>();
        assert_eq!(row.len(), 8);
        chess_board.board.push(row);
    }
    assert_eq!(chess_board.board.len(), 8);

    let mut cols = [false; 8];
    let mut diag1 = [false; 15];
    let mut diag2 = [false; 15];

    let ways = ChessBoard::solve(0, &chess_board.board, &mut cols, &mut diag1, &mut diag2);
    println!("{}", ways);
}
