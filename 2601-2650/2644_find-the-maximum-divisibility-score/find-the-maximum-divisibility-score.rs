
impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut max_score = 0;
        let mut max_divisor = std::i32::MAX;

        for &divisor in &divisors {
            let mut score = 0;
            for &num in &nums {
                if num % divisor == 0 {
                    score += 1;
                }
            }
            if score > max_score {
                max_score = score;
                max_divisor = divisor;
            } else if score == max_score {
                max_divisor = std::cmp::min(max_divisor, divisor);
            }
        }

        max_divisor

    }
}
