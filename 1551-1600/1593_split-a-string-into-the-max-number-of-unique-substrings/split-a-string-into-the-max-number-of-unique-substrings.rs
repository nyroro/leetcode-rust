
use std::collections::HashSet;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let mut max_count = 0;
        let mut visited = HashSet::new();
        Solution::backtrack(0, &s, &mut visited, &mut max_count);
        max_count

    }
    
    fn backtrack<'a>(index: usize, s: &'a str, visited: &mut HashSet<&'a str>, max_count: &mut i32) {
        if index == s.len() {
            *max_count = (*max_count).max(visited.len() as i32);
            return;
        }
        
        for i in index..s.len() {
            let substr = &s[index..=i];
            if !visited.contains(substr) {
                visited.insert(substr);
                Solution::backtrack(i + 1, s, visited, max_count);
                visited.remove(substr);
            }
        }
    }
}
