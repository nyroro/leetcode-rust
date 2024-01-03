
impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let n = target.len();
        let mut dp = vec![std::i32::MAX; 1 << n];
        dp[0] = 0;

        for i in 0..(1 << n) {
            if dp[i] == std::i32::MAX {
                continue;
            }
            for sticker in &stickers {
                let mut next = i;
                for ch in sticker.chars() {
                    for (j, target_ch) in target.chars().enumerate() {
                        if ch == target_ch && ((next >> j) & 1) == 0 {
                            next |= 1 << j;
                            break;
                        }
                    }
                }
                dp[next] = dp[next].min(dp[i] + 1);
            }
        }

        if dp[(1 << n) - 1] == std::i32::MAX {
            return -1;
        }
        return dp[(1 << n) - 1];
    }
}
