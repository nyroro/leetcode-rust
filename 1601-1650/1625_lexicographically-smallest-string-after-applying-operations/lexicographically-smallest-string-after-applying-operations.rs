
use std::collections::HashSet;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let mut visited = HashSet::new();
        let mut result = s.clone();

        fn rotate(s: &str, b: usize) -> String {
            let n = s.len();
            let mut res = vec!['0'; n];
            for i in 0..n {
                res[(i + b) % n] = s.chars().nth(i).unwrap();
            }
            res.into_iter().collect()
        }

        fn dfs(s: String, a: i32, b: i32, visited: &mut HashSet<String>) -> String {
            if visited.contains(&s) {
                return s;
            }
            visited.insert(s.clone());
            let mut result = s.clone();
            let s1: String = s.chars().enumerate().map(|(i, c)| if i % 2 == 1 { ((c as u8 - b'0' + a as u8) % 10 + b'0') as char } else { c }).collect();
            result = dfs(s1, a, b, visited);
            let s2 = rotate(&s, b as usize);
            result = std::cmp::min(result, dfs(s2, a, b, visited));
            result

        }

        dfs(result, a, b, &mut visited)
    }
}
