
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let start = "0000".to_string();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let deadends: HashSet<String> = deadends.into_iter().collect();
        
        if deadends.contains(&start) {
            return -1;
        }
        
        visited.insert(start.clone());
        queue.push_back((start, 0));
        
        while let Some((current, turns)) = queue.pop_front() {
            if current == target {
                return turns;
            }
            
            for i in 0..4 {
                let digit = current.chars().nth(i).unwrap().to_digit(10).unwrap();
                let next1 = current.chars().take(i).chain(std::iter::once(((digit + 1) % 10).to_string().chars().next().unwrap())).chain(current.chars().skip(i + 1)).collect::<String>();
                let next2 = current.chars().take(i).chain(std::iter::once(((digit + 9) % 10).to_string().chars().next().unwrap())).chain(current.chars().skip(i + 1)).collect::<String>();
                
                if !visited.contains(&next1) && !deadends.contains(&next1) {
                    visited.insert(next1.clone());
                    queue.push_back((next1, turns + 1));
                }
                
                if !visited.contains(&next2) && !deadends.contains(&next2) {
                    visited.insert(next2.clone());
                    queue.push_back((next2, turns + 1));
                }
            }
        }
        
        -1

    }
}
