
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut stack: Vec<char> = Vec::new();
        let mut deletions = 0;
        
        for ch in s.chars() {
            if ch == 'a' {
                stack.push(ch);
            } else {
                if let Some(&top) = stack.last() {
                    if top == 'a' {
                        stack.pop();
                    } else {
                        deletions += 1;
                    }
                } else {
                    deletions += 1;
                }
            }
        }
        
        deletions + stack.len() as i32

    }
}
