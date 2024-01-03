
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        let mut count = 0;

        for i in (0..n).rev() {
            for j in i..n {
                if s.chars().nth(i) == s.chars().nth(j) {
                    if j - i <= 2 || dp[i + 1][j - 1] {
                        dp[i][j] = true;
                        count += 1;
                    }
                }
            }
        }

        count

    }
}
