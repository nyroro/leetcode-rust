
impl Solution {
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let n = s.len();
        let mut dp = vec![vec![n as i32; k as usize + 1]; n + 1];
        dp[0][0] = 0;

        for i in 1..=n {
            for j in 1..=k as usize {
                for l in (1..=i).rev() {
                    let mut count = 0;
                    let mut left = l;
                    let mut right = i;
                    while left < right {
                        if s.chars().nth(left - 1) != s.chars().nth(right - 1) {
                            count += 1;
                        }
                        left += 1;
                        right -= 1;
                    }
                    dp[i][j] = dp[i][j].min(dp[l - 1][j - 1] + count);
                }
            }
        }

        dp[n][k as usize]
    }
}
