
use std::convert::TryInto;



impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut rows = vec![vec![0; n + 1]; m];
        let mut cols = vec![vec![0; n]; m + 1];
        let mut diag = vec![vec![0; n + 1]; m + 1];
        let mut anti = vec![vec![0; n + 1]; m + 1];

        for i in 0..m {
            for j in 0..n {
                rows[i][j + 1] = grid[i][j] + rows[i][j];
                cols[i + 1][j] = grid[i][j] + cols[i][j];
                diag[i + 1][j + 1] = grid[i][j] + diag[i][j];
                anti[i + 1][j] = grid[i][j] + anti[i][j + 1];
            }
        }

        let mut ans = 1;
        for i in 0..m {
            for j in 0..n {
                for k in 1..=std::cmp::min(i, j) {
                    let ii = i - k;
                    let jj = j - k;
                    let val = diag[i + 1][j + 1] - diag[ii][jj];
                    let mut match_val = val == anti[i + 1][jj] - anti[ii][j + 1];
                    for r in ii..=i {
                        match_val &= val == rows[r][j + 1] - rows[r][jj];
                    }
                    for c in jj..=j {
                        match_val &= val == cols[i + 1][c] - cols[ii][c];
                    }
                    if match_val {
                        ans = std::cmp::max(ans, k + 1);
                    }
                }
            }
        }
        ans.try_into().unwrap()
    }
}
