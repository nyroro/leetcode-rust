
use std::collections::HashSet;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let mut visited = HashSet::new();
        let mut result = s.clone();
        
        fn rotate(s: &str, b: i32) -> String {
            let b = b as usize;
            let n = s.len();
            let mut res = vec!['0'; n];
            for i in 0..n {
                res[(i + b) % n] = s.chars().nth(i).unwrap();
            }
            res.into_iter().collect()
        }
        
        fn dfs(s: &str, a: i32, b: i32, visited: &mut HashSet<String>, result: &mut String) {
            if visited.contains(s) {
                return;
            }
            visited.insert(s.to_string());
            if s < *result {
                *result = s.to_string();
            }
            let s1: String = s.chars().enumerate().map(|(i, c)| if i % 2 == 1 { ((c as u8 - b'0' + a as u8) % 10 + b'0') as char } else { c }).collect();
            dfs(&s1, a, b, visited, result);
            let s2 = rotate(s, b);
            dfs(&s2, a, b, visited, result);
        }
        
        dfs(&s, a, b, &mut visited, &mut result);
        result

    }
}
