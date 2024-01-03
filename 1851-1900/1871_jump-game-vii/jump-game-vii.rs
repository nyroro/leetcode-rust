
use std::collections::VecDeque;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut queue = VecDeque::new();
        queue.push_back(0);
        
        for i in 1..n {
            if s[i] == '0' {
                while !queue.is_empty() && i - queue[0] > max_jump as usize {
                    queue.pop_front();
                }
                if !queue.is_empty() && i - queue[0] >= min_jump as usize {
                    queue.push_back(i);
                }
            }
        }
        
        if let Some(&last) = queue.back() {
            last == n - 1

        } else {
            false

        }
    }
}
