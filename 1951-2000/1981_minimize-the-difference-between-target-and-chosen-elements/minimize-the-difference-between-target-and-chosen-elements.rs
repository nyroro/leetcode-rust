
impl Solution {
    pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut dp = vec![vec![false; 70 * 70 + 1]; m];
        
        // 初始化第一行的dp数值

        for j in 0..n {
            dp[0][mat[0][j] as usize] = true;
        }
        
        // 更新dp数组

        for i in 1..m {
            for j in 0..n {
                for k in 0..=70 * 70 {
                    if dp[i - 1][k] {
                        let sum = k as i32 + mat[i][j];
                        if sum <= 70 * 70 {
                            dp[i][sum as usize] = true;
                        }
                    }
                }
            }
        }
        
        // 找到最接近target的和

        let mut ans = i32::MAX;
        for i in 0..=70 * 70 {
            if dp[m - 1][i] {
                ans = ans.min((target - i as i32).abs());
            }
        }
        
        ans

    }
}
