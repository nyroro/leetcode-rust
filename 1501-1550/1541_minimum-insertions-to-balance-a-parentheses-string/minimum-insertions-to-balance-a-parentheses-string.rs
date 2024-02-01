


impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut insertions = 0;
        let mut i = 0;
        let chars: Vec<char> = s.chars().collect();
        
        while i < chars.len() {
            if chars[i] == '(' {
                stack.push(i);
                i += 1;
            } else {
                if i + 1 < chars.len() && chars[i + 1] == ')' {
                    i += 2;
                } else {
                    insertions += 1;
                    i += 1;
                }
                if stack.len() > 0 {
                    stack.pop();
                } else {
                    insertions += 1;
                }
            }
        }
        
        insertions + (stack.len() as i32 * 2)
    }
}
