


impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![-1; 27]; s.len()];
        let idx = 0;
        let prev: char = '{';
        Solution::fun(s.as_bytes(), idx, prev, k, &mut dp)
    }

    fn fun(s: &[u8], idx: usize, prev: char, k: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
        if idx >= s.len() {
            return 0;
        }
        if dp[idx][prev as usize - 'a' as usize] != -1 {
            return dp[idx][prev as usize - 'a' as usize];
        }
        if idx == 0 || prev == '{' {
            dp[idx][prev as usize - 'a' as usize] = std::cmp::max(
                1 + Solution::fun(s, idx + 1, s[idx] as char, k, dp),
                Solution::fun(s, idx + 1, prev, k, dp),
            );
        } else if prev != '{' && (s[idx] as i32 - prev as i32).abs() <= k {
            dp[idx][prev as usize - 'a' as usize] = std::cmp::max(
                1 + Solution::fun(s, idx + 1, s[idx] as char, k, dp),
                Solution::fun(s, idx + 1, prev, k, dp),
            );
        } else if prev != '{' && (s[idx] as i32 - prev as i32).abs() > k {
            dp[idx][prev as usize - 'a' as usize] = Solution::fun(s, idx + 1, prev, k, dp);
        }
        dp[idx][prev as usize - 'a' as usize]
    }
}
