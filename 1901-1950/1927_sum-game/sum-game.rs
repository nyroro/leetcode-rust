
impl Solution {
    pub fn sum_game(num: String) -> bool {
        let n = num.len();
        let half = n / 2;
        let mut left_sum = 0;
        let mut right_sum = 0;
        let mut left_count = 0;
        let mut right_count = 0;

        for (i, c) in num.chars().enumerate() {
            if i < half {
                if c == '?' {
                    left_count += 1;
                } else {
                    left_sum += c.to_digit(10).unwrap();
                }
            } else {
                if c == '?' {
                    right_count += 1;
                } else {
                    right_sum += c.to_digit(10).unwrap();
                }
            }
        }

        return (left_count + right_count) % 2 == 1 || left_sum - right_sum != (right_count - left_count) as i32 * 9 / 2;
    }
}
