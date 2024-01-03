
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut left: i64 = 0;
        let mut right: i64 = (c as f64).sqrt() as i64; // Convert to f64 and then back to i64 to use sqrt


        while left <= right {
            let sum = left * left + right * right;
            if sum == c as i64 {
                return true;
            } else if sum < c as i64 {
                left += 1;
            } else {
                right -= 1;
            }
        }

        false

    }
}
