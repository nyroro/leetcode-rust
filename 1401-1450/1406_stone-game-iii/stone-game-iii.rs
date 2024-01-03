
impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut dp = vec![0; n + 1];
        
        for i in (0..n).rev() {
            let mut best = std::i32::MIN;
            let mut cur = 0;
            for k in 1..=3 {
                if i + k <= n {
                    cur += stone_value[i + k - 1];
                    best = best.max(cur - dp[i + k]);
                }
            }
            dp[i] = best;
        }
        
        if dp[0] > 0 {
            return "Alice".to_string();
        } else if dp[0] < 0 {
            return "Bob".to_string();
        } else {
            return "Tie".to_string();
        }
    }
}
