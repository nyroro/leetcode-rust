
use std::collections::{HashSet, VecDeque};

struct State {
    pos: i32,
    steps: i32,
    forward: bool,
}

impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        let mut visited = HashSet::new();
        let mut forbidden_set: HashSet<i32> = forbidden.into_iter().collect();
        let mut queue = VecDeque::new();
        queue.push_back(State { pos: 0, steps: 0, forward: true });
        visited.insert((0, true));
        visited.insert((0, false));

        while let Some(State { pos, steps, forward }) = queue.pop_front() {
            if pos == x {
                return steps;
            }

            if pos + a <= 6000 && !forbidden_set.contains(&(pos + a)) && visited.insert((pos + a, true)) {
                queue.push_back(State { pos: pos + a, steps: steps + 1, forward: true });
            }

            if forward && pos - b > 0 && !forbidden_set.contains(&(pos - b)) && visited.insert((pos - b, false)) {
                queue.push_back(State { pos: pos - b, steps: steps + 1, forward: false });
            }
        }

        -1

    }
}
