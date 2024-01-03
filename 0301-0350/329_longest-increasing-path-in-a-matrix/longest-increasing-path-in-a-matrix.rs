
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![0; n]; m];
        
        fn dfs(matrix: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
            if dp[i][j] != 0 {
                return dp[i][j];
            }
            let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            let mut max_len = 1;
            for (dx, dy) in dirs {
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if x >= 0 && x < matrix.len() as i32 && y >= 0 && y < matrix[0].len() as i32 && matrix[x as usize][y as usize] > matrix[i][j] {
                    max_len = max_len.max(1 + dfs(matrix, dp, x as usize, y as usize));
                }
            }
            dp[i][j] = max_len;
            max_len

        }
        
        let mut result = 0;
        for i in 0..m {
            for j in 0..n {
                result = result.max(dfs(&matrix, &mut dp, i, j));
            }
        }
        result

    }
}
