
impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        let (row, col) = (destination[0] as usize, destination[1] as usize);
        let mut dp = vec![vec![0; col + 1]; row + 1];
        for i in (0..=row).rev() {
            for j in (0..=col).rev() {
                if i == row && j == col {
                    dp[i][j] = 1;
                } else if i == row {
                    dp[i][j] = dp[i][j + 1];
                } else if j == col {
                    dp[i][j] = dp[i + 1][j];
                } else {
                    dp[i][j] = dp[i + 1][j] + dp[i][j + 1];
                }
            }
        }
        
        let mut result = String::new();
        let mut k = k;
        let (mut i, mut j) = (0, 0);
        while i < row && j < col {
            if dp[i + 1][j] >= k {
                result.push('H');
                j += 1;
            } else {
                result.push('V');
                k -= dp[i + 1][j];
                i += 1;
            }
        }
        while i < row {
            result.push('V');
            i += 1;
        }
        while j < col {
            result.push('H');
            j += 1;
        }
        
        result

    }
}
