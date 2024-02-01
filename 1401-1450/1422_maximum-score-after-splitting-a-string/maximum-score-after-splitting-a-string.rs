
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut max_score = 0;
        let n = s.len();

        for i in 1..n {
            let left = &s[..i];
            let right = &s[i..];

            let left_zeros = left.chars().filter(|&c| c == '0').count();
            let right_ones = right.chars().filter(|&c| c == '1').count();

            let score = left_zeros + right_ones;
            max_score = max_score.max(score);
        }

        max_score

    }
}
