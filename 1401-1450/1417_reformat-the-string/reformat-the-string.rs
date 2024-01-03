
impl Solution {
    pub fn reformat(s: String) -> String {
        let mut letters = String::new();
        let mut digits = String::new();

        for ch in s.chars() {
            if ch.is_alphabetic() {
                letters.push(ch);
            } else {
                digits.push(ch);
            }
        }

        let diff = letters.len() as i32 - digits.len() as i32;
        if diff.abs() > 1 {
            return String::new();
        }

        let mut result = String::new();
        let mut larger = &letters;
        let mut smaller = &digits;

        if diff < 0 {
            larger = &digits;
            smaller = &letters;
        }

        for i in 0..larger.len() {
            result.push(larger.chars().nth(i).unwrap());
            if i < smaller.len() {
                result.push(smaller.chars().nth(i).unwrap());
            }
        }

        result

    }
}
