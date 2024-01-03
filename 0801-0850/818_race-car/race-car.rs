
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn racecar(target: i32) -> i32 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((0, 1)); // (position, speed)
        visited.insert((0, 1));
        let mut steps = 0;

        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let (pos, speed) = queue.pop_front().unwrap();
                if pos == target {
                    return steps;
                }
                let new_pos1 = pos + speed;
                let new_speed1 = speed * 2;
                if !visited.contains(&(new_pos1, new_speed1)) {
                    queue.push_back((new_pos1, new_speed1));
                    visited.insert((new_pos1, new_speed1));
                }
                let new_speed2 = if speed > 0 { -1 } else { 1 };
                if !visited.contains(&(pos, new_speed2)) {
                    queue.push_back((pos, new_speed2));
                    visited.insert((pos, new_speed2));
                }
            }
            steps += 1;
        }

        -1

    }
}
