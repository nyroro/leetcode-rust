
impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
        
        // Helper function to check if the string is valid

        fn is_valid(s: &String) -> bool {
            let mut count = 0;
            for ch in s.chars() {
                if ch == '(' {
                    count += 1;
                } else if ch == ')' {
                    count -= 1;
                    if count < 0 {
                        return false;
                    }
                }
            }
            count == 0

        }
        
        // Depth-first search function

        fn dfs(s: &String, visited: &mut std::collections::HashSet<String>, result: &mut Vec<String>, start: usize, last_i: usize, last_j: usize, open: char, close: char) {
            let mut count = 0;
            for i in (start..s.len()).step_by(1) {
                if s.chars().nth(i).unwrap() == open {
                    count += 1;
                } else if s.chars().nth(i).unwrap() == close {
                    count -= 1;
                }
                if count >= 0 {
                    continue;
                }
                for j in (last_j..=i).step_by(1) {
                    if s.chars().nth(j).unwrap() == close && (j == last_j || s.chars().nth(j - 1).unwrap() != close) {
                        let new_str = s[..j].to_string() + &s[j + 1..];
                        if !visited.contains(&new_str) {
                            visited.insert(new_str.clone());
                            dfs(&new_str, visited, result, i, i, j, open, close);
                        }
                    }
                }
                return;
            }
            let reversed = s.chars().rev().collect::<String>();
            if open == '(' {
                dfs(&reversed, visited, result, 0, 0, 0, close, open);
            } else {
                result.push(reversed);
            }
        }
        
        dfs(&s, &mut visited, &mut result, 0, 0, 0, '(', ')');
        result

    }
}
