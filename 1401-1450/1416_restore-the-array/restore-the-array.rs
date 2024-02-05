
impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let mod_val = 1_000_000_007;
        let s_chars: Vec<char> = s.chars().collect();
        let n = s_chars.len();
        
        // Create a vector to store the number of possible arrays for each index

        let mut dp: Vec<i64> = vec![0; n + 1];
        dp[n] = 1;

        // Iterate through the string from right to left

        for i in (0..n).rev() {
            if s_chars[i] == '0' {
                continue;
            }
            let mut num: i64 = 0;
            for j in i..(n.min(i + 32)) {
                num = num * 10 + (s_chars[j] as i64 - '0' as i64);
                if num > k as i64 {
                    break;
                }
                dp[i] = (dp[i] + dp[j + 1]) % mod_val;
            }
        }
        
        dp[0] as i32

    }
}
