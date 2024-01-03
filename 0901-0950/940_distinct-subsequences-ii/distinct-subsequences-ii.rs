
impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let modulo = 1_000_000_007;
        let mut last_occurrence = vec![-1; 26];
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;

        for (i, c) in s.chars().enumerate() {
            let index = (c as u8 - b'a') as usize;
            let prev_occurrence = last_occurrence[index];
            if prev_occurrence == -1 {
                dp[i + 1] = (dp[i] * 2) % modulo;
            } else {
                let mut diff = (dp[i] - dp[prev_occurrence as usize]) % modulo;
                if diff < 0 {
                    diff += modulo;
                }
                dp[i + 1] = (dp[i] + diff) % modulo;
            }
            last_occurrence[index] = i as i32;
        }

        let result = (dp[s.len()] - 1) % modulo;
        if result < 0 {
            result + modulo

        } else {
            result

        }
    }
}
