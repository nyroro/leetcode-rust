
impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack: Vec<usize> = Vec::new();
        let mut chars: Vec<char> = s.chars().collect();
        
        for i in 0..chars.len() {
            if chars[i] == '(' {
                stack.push(i);
            } else if chars[i] == ')' {
                let left = stack.pop().unwrap();
                let right = i;
                chars[left..=right].reverse();
            }
        }
        
        let result: String = chars.into_iter().filter(|&c| c != '(' && c != ')').collect();
        result

    }
}
