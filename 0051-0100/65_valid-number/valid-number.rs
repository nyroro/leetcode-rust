
impl Solution {
    pub fn is_number(s: String) -> bool {
        let s = s.trim();

        if s.is_empty() {
            return false;
        }

        let mut valid = false;
        let mut has_digit = false;
        let mut has_dot = false;
        let mut has_e = false;

        for (i, c) in s.chars().enumerate() {
            match c {
                '0'..='9' => {
                    valid = true;
                    has_digit = true;
                }
                '+' | '-' => {
                    if i > 0 && s.chars().nth(i - 1) != Some('e') && s.chars().nth(i - 1) != Some('E') {
                        return false;
                    }
                }
                '.' => {
                    if has_dot || has_e {
                        return false;
                    }
                    has_dot = true;
                }
                'e' | 'E' => {
                    if has_e || !has_digit {
                        return false;
                    }
                    has_e = true;
                    valid = false;
                }
                _ => return false,
            }
        }

        valid

    }
}
