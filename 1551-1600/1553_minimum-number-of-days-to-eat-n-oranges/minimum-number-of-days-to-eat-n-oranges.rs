
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn min_days(n: i32) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(n);
        
        let mut days = 0;
        let mut visited = HashSet::new();
        
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                if let Some(curr) = queue.pop_front() {
                    if curr == 0 {
                        return days;
                    }
                    if !visited.contains(&(curr - 1)) {
                        queue.push_back(curr - 1);
                        visited.insert(curr - 1);
                    }
                    if curr % 2 == 0 && !visited.contains(&(curr - curr / 2)) {
                        queue.push_back(curr - curr / 2);
                        visited.insert(curr - curr / 2);
                    }
                    if curr % 3 == 0 && !visited.contains(&(curr - 2 * (curr / 3))) {
                        queue.push_back(curr - 2 * (curr / 3));
                        visited.insert(curr - 2 * (curr / 3));
                    }
                }
            }
            days += 1;
        }
        days

    }
}
