
use std::collections::HashSet;



impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        let mut blocked_set: HashSet<(i32, i32)> = HashSet::new();
        for point in blocked {
            blocked_set.insert((point[0], point[1]));
        }

        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((source[0], source[1]));

        if !Solution::dfs(source[0], source[1], target[0], target[1], &mut visited, &blocked_set) {
            return false;
        }

        visited.clear();
        visited.insert((target[0], target[1]));

        return Solution::dfs(target[0], target[1], source[0], source[1], &mut visited, &blocked_set);
    }

    fn dfs(x: i32, y: i32, target_x: i32, target_y: i32, visited: &mut HashSet<(i32, i32)>, blocked: &HashSet<(i32, i32)>) -> bool {
        if x == target_x && y == target_y {
            return true;
        }

        if visited.len() > 200000 {
            return true;
        }

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for dir in &directions {
            let new_x = x + dir.0;
            let new_y = y + dir.1;
            if new_x >= 0 && new_x < 1000000 && new_y >= 0 && new_y < 1000000 && !blocked.contains(&(new_x, new_y)) && visited.insert((new_x, new_y)) {
                if Solution::dfs(new_x, new_y, target_x, target_y, visited, blocked) {
                    return true;
                }
            }
        }

        return false;
    }
}
