
use std::collections::HashSet;
use std::collections::VecDeque;

struct Point {
    x: i32,
    y: i32,
}

impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        let mut blocked_set: HashSet<(i32, i32)> = HashSet::new();
        for point in blocked {
            blocked_set.insert((point[0], point[1]));
        }

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut queue: VecDeque<Point> = VecDeque::new();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        queue.push_back(Point { x: source[0], y: source[1] });
        visited.insert((source[0], source[1]));

        while let Some(current) = queue.pop_front() {
            if current.x == target[0] && current.y == target[1] {
                return true;
            }

            for dir in &directions {
                let new_x = current.x + dir.0;
                let new_y = current.y + dir.1;
                if new_x >= 0 && new_x < 1000000 && new_y >= 0 && new_y < 1000000 && !blocked_set.contains(&(new_x, new_y)) && !visited.contains(&(new_x, new_y)) {
                    queue.push_back(Point { x: new_x, y: new_y });
                    visited.insert((new_x, new_y));
                }
            }
        }

        return false;
    }
}
