
impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut dp = vec![false; n];
        dp[0] = true;
        
        for i in 1..n {
            if s[i] == '0' {
                for j in (i - max_jump.max(0) as usize)..=(i - min_jump as usize) {
                    if j < n && dp[j] {
                        dp[i] = true;
                        break;
                    }
                }
            }
        }
        
        dp[n - 1]
    }
}
