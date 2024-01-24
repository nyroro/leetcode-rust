
impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let mut stack: Vec<usize> = Vec::new();
        let mut k: Vec<usize> = Vec::new();
        let s_chars: Vec<char> = s.chars().collect();
        let locked_chars: Vec<char> = locked.chars().collect();

        if s_chars.len() % 2 != 0 {
            return false;
        }

        for i in 0..s_chars.len() {
            if locked_chars[i] == '0' {
                k.push(i);
            } else {
                if s_chars[i] == '(' {
                    stack.push(i);
                } else {
                    if let Some(_) = stack.pop() {
                        // do nothing

                    } else if let Some(_) = k.pop() {
                        // do nothing

                    } else {
                        return false;
                    }
                }
            }
        }

        for &t in stack.iter().rev() {
            if let Some(&k_idx) = k.last() {
                if k_idx > t {
                    k.pop();
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        return true;
    }
}
