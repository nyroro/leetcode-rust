
impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut dp = vec![vec![0; 2]; n];
        dp[0][0] = arr[0];
        dp[0][1] = std::cmp::min(0, arr[0]);
        let mut result = arr[0];
        
        for i in 1..n {
            dp[i][0] = std::cmp::max(arr[i], dp[i-1][0] + arr[i]);
            dp[i][1] = std::cmp::max(dp[i-1][0], dp[i-1][1] + arr[i]);
            result = std::cmp::max(result, std::cmp::max(dp[i][0], dp[i][1]));
        }
        
        result

    }
}
