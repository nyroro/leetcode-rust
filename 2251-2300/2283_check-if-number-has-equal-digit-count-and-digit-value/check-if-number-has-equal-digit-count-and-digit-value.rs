
impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut count = vec![0; num.len()];

        for ch in num.chars() {
            let digit = ch.to_digit(10).unwrap() as usize;
            count[digit] += 1;
        }

        for (i, ch) in num.chars().enumerate() {
            let digit = ch.to_digit(10).unwrap() as usize;
            if count[i] != digit {
                return false;
            }
        }

        true

    }
}
