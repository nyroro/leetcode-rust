
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; (n + 1) as usize]; (m + 1) as usize];
        for s in strs {
            let (zeros, ones) = count_zeros_ones(&s);
            for i in (zeros..=m as usize).rev() {
                for j in (ones..=n as usize).rev() {
                    dp[i][j] = dp[i][j].max(1 + dp[i - zeros][j - ones]);
                }
            }
        }
        dp[m as usize][n as usize]
    }
}

fn count_zeros_ones(s: &str) -> (usize, usize) {
    let (mut zeros, mut ones) = (0, 0);
    for c in s.chars() {
        if c == '0' {
            zeros += 1;
        } else if c == '1' {
            ones += 1;
        }
    }
    (zeros, ones)
}
