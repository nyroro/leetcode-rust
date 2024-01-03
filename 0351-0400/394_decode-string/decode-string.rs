
impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<(usize, String)> = Vec::new();
        let mut repeat_count = 0;
        let mut decoded_string = String::new();

        for c in s.chars() {
            if c.is_digit(10) {
                repeat_count = repeat_count * 10 + c.to_digit(10).unwrap() as usize;
            } else if c == '[' {
                stack.push((repeat_count, decoded_string));
                repeat_count = 0;
                decoded_string = String::new();
            } else if c == ']' {
                let (count, prev_string) = stack.pop().unwrap();
                decoded_string = prev_string + decoded_string.repeat(count).as_str();
            } else {
                decoded_string.push(c);
            }
        }

        decoded_string

    }
}
