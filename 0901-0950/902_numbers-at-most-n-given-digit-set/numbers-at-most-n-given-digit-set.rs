
impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let n_str = n.to_string();
        let n_len = n_str.len();
        let mut dp = vec![0; n_len + 1];
        dp[n_len] = 1;

        for i in (0..n_len).rev() {
            let digit = n_str.chars().nth(i).unwrap();
            for d in &digits {
                if d.chars().next().unwrap() < digit {
                    dp[i] += digits.len().pow((n_len - i - 1) as u32);
                } else if d.chars().next().unwrap() == digit {
                    dp[i] += dp[i + 1];
                }
            }
        }

        for i in 1..n_len {
            dp[0] += digits.len().pow(i as u32);
        }

        dp[0] as i32

    }
}
