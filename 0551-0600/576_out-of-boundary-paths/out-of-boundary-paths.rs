
impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let modulo = 1000000007;
        let mut dp = vec![vec![vec![0; n as usize]; m as usize]; max_move as usize + 1];
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        
        for k in 1..=max_move {
            for i in 0..m {
                for j in 0..n {
                    for dir in &directions {
                        let ni = i + dir.0;
                        let nj = j + dir.1;
                        if ni < 0 || ni >= m || nj < 0 || nj >= n {
                            dp[k as usize][i as usize][j as usize] += 1;
                        } else {
                            dp[k as usize][i as usize][j as usize] = (dp[k as usize][i as usize][j as usize] + dp[(k - 1) as usize][ni as usize][nj as usize]) % modulo;
                        }
                    }
                }
            }
        }
        
        dp[max_move as usize][start_row as usize][start_column as usize]
    }
}
