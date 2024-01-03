
impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        for num in (n + 1)..=i32::MAX {
            let mut digit_count = std::collections::HashMap::new();
            let num_str = num.to_string();
            for ch in num_str.chars() {
                let digit = ch.to_digit(10).unwrap() as usize;
                *digit_count.entry(digit).or_insert(0) += 1;
            }
            let mut is_beautiful = true;
            for (digit, count) in digit_count {
                if digit != count {
                    is_beautiful = false;
                    break;
                }
            }
            if is_beautiful {
                return num;
            }
        }
        -1

    }
}
