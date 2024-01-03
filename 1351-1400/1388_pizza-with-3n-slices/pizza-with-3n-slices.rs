
impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        let n = slices.len();
        let m = n / 3;
        let mut dp = vec![vec![0; m + 1]; n + 1];
        
        let mut slices1 = slices.clone();
        slices1.pop();
        let mut slices2 = slices.clone();
        slices2.remove(0);
        
        let res1 = Self::helper(&slices1, m, &mut dp);
        let res2 = Self::helper(&slices2, m, &mut dp);
        
        res1.max(res2)
    }
    
    fn helper(slices: &Vec<i32>, m: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        let n = slices.len();
        for i in 1..=n {
            for j in 1..=m {
                if i == 1 {
                    dp[i][j] = slices[i - 1];
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i - 2][j - 1] + slices[i - 1]);
                }
            }
        }
        dp[n][m]
    }
}
