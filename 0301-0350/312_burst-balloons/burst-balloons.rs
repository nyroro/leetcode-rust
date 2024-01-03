
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0; n+2]; n+2];
        let mut new_nums = vec![1];
        new_nums.extend(nums);
        new_nums.push(1);

        for len in 1..=n {
            for i in 1..=n-len+1 {
                let j = i + len - 1;
                for k in i..=j {
                    let score = new_nums[i-1] * new_nums[k] * new_nums[j+1];
                    dp[i][j] = dp[i][j].max(score + dp[i][k-1] + dp[k+1][j]);
                }
            }
        }

        dp[1][n]
    }
}
