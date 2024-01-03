
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dp = vec![vec![0; cols]; rows];
        let mut max_area = 0;
        
        for i in 0..rows {
            for j in 0..cols {
                if matrix[i][j] == '1' {
                    if j == 0 {
                        dp[i][j] = 1;
                    } else {
                        dp[i][j] = dp[i][j-1] + 1;
                    }
                    
                    let mut width = dp[i][j];
                    for k in (0..=i).rev() {
                        width = width.min(dp[k][j]);
                        let height = i - k + 1;
                        max_area = max_area.max(width * height);
                    }
                }
            }
        }
        
        max_area as i32

    }
}
