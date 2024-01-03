


impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut stack: Vec<(char, i64)> = Vec::new();
        let mut size: i64 = 0;

        for c in s.chars() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap() as i64;
                size *= digit;
            } else {
                size += 1;
            }
            stack.push((c, size));
        }

        let mut k = k as i64;
        while let Some((c, count)) = stack.pop() {
            k %= count;
            if k == 0 && c.is_alphabetic() {
                return c.to_string();
            }
        }

        String::new()
    }
}
