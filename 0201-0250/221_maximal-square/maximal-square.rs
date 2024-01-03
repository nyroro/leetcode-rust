
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![0; n]; m];
        let mut max_side = 0;

        for i in 0..m {
            for j in 0..n {
                if i == 0 || j == 0 {
                    dp[i][j] = matrix[i][j] as i32 - '0' as i32;
                } else if matrix[i][j] == '1' {
                    dp[i][j] = (dp[i-1][j-1]).min(dp[i-1][j]).min(dp[i][j-1]) + 1;
                }
                max_side = max_side.max(dp[i][j]);
            }
        }

        max_side * max_side

    }
}
