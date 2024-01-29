


impl Solution {
    pub fn is_possible_to_cut_path(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        // number of paths from (0, 0) to (i, j)
        let mut dp1 = vec![vec![0; n + 1]; m + 1];
        dp1[1][1] = 1;
        for i in 1..=m {
            for j in 1..=n {
                if grid[i-1][j-1] == 1 {
                    dp1[i][j] += dp1[i-1][j] + dp1[i][j-1];
                }
            }
        }

        // number of paths from (i, j) to (m-1, n-1)
        let mut dp2 = vec![vec![0; n + 1]; m + 1];
        dp2[m-1][n-1] = 1;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if grid[i][j] == 1 {
                    dp2[i][j] += dp2[i+1][j] + dp2[i][j+1];
                }
            }
        }

        // number of paths from (0, 0) to (m-1, n-1)
        let target = dp1[m][n];

        for i in 0..m {
            for j in 0..n {
                if (i != 0 || j != 0) && (i != m-1 || j != n-1) {
                    if dp1[i+1][j+1] * dp2[i][j] == target {
                        return true;
                    }
                }
            }
        }

        false

    }
}
