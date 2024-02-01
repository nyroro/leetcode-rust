
impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![0; n]; m];

        for col in (0..n).rev() {
            for row in 0..m {
                let mut max_moves = 0;
                for dr in [-1, 0, 1].iter() {
                    let new_row = row as i32 + dr;
                    if new_row < 0 || new_row >= m as i32 {
                        continue;
                    }
                    let new_col = col + 1;
                    if new_col < n {
                        if grid[new_row as usize][new_col] > grid[row][col] {
                            max_moves = max_moves.max(dp[new_row as usize][new_col]);
                        }
                    }
                }
                dp[row][col] = max_moves + 1;
            }
        }

        dp.iter().map(|row| *row.first().unwrap()).max().unwrap()
    }
}
