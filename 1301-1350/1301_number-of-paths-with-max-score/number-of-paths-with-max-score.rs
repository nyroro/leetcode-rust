
impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let n = board.len();
        let mut dp: Vec<Vec<(i32, i32)>> = vec![vec![(std::i32::MIN, 0); n]; n];
        let dirs: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (1, 1)];

        dp[n-1][n-1] = (0, 1);

        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if board[i].chars().nth(j).unwrap() == 'X' || (i == n-1 && j == n-1) {
                    continue;
                }

                for &(dx, dy) in &dirs {
                    let x = (i as i32 + dx) as usize;
                    let y = (j as i32 + dy) as usize;

                    if x < n && y < n && dp[x][y].0 != std::i32::MIN {
                        let score = if board[i].chars().nth(j).unwrap() == 'E' {
                            0

                        } else {
                            board[i].chars().nth(j).unwrap().to_digit(10).unwrap() as i32

                        };

                        if dp[i][j].0 < dp[x][y].0 + score {
                            dp[i][j] = (dp[x][y].0 + score, dp[x][y].1);
                        } else if dp[i][j].0 == dp[x][y].0 + score {
                            dp[i][j].1 = (dp[i][j].1 + dp[x][y].1) % 1_000_000_007;
                        }
                    }
                }
            }
        }

        if dp[0][0].0 == std::i32::MIN {
            vec![0, 0]
        } else {
            vec![dp[0][0].0, dp[0][0].1]
        }
    }
}
