
impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut captures = 0;
        let mut rook_row = 0;
        let mut rook_col = 0;

        // Find the position of the rook

        for i in 0..8 {
            for j in 0..8 {
                if board[i][j] == 'R' {
                    rook_row = i;
                    rook_col = j;
                    break;
                }
            }
        }

        // Move up

        let mut row = rook_row;
        while row > 0 {
            row -= 1;
            if board[row][rook_col] == 'p' {
                captures += 1;
                break;
            } else if board[row][rook_col] == 'B' {
                break;
            }
        }

        // Move down

        row = rook_row;
        while row < 7 {
            row += 1;
            if board[row][rook_col] == 'p' {
                captures += 1;
                break;
            } else if board[row][rook_col] == 'B' {
                break;
            }
        }

        // Move left

        let mut col = rook_col;
        while col > 0 {
            col -= 1;
            if board[rook_row][col] == 'p' {
                captures += 1;
                break;
            } else if board[rook_row][col] == 'B' {
                break;
            }
        }

        // Move right

        col = rook_col;
        while col < 7 {
            col += 1;
            if board[rook_row][col] == 'p' {
                captures += 1;
                break;
            } else if board[rook_row][col] == 'B' {
                break;
            }
        }

        captures

    }
}
