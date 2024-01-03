
impl Solution {
    pub fn max_product(s: String) -> i32 {
        let n = s.len();
        let s = s.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![0; n]; 1 << n];
        
        // 计算回文子序列的长度

        for mask in 0..1 << n {
            let mut l = 0;
            let mut r = n - 1;
            while l < r {
                if (mask >> l) & 1 == 0 {
                    l += 1;
                } else if (mask >> r) & 1 == 0 {
                    r -= 1;
                } else if s[l] != s[r] {
                    break;
                } else {
                    l += 1;
                    r -= 1;
                }
            }
            if l >= r {
                dp[mask][0] = (mask.count_ones() as i32).max(dp[mask][0]);
            }
        }
        
        // 遍历所有可能的子序列

        let mut result = 0;
        for mask1 in 0..1 << n {
            for mask2 in 0..1 << n {
                if mask1 & mask2 == 0 {
                    result = result.max(dp[mask1][0] * dp[mask2][0]);
                }
            }
        }
        
        result

    }
}
