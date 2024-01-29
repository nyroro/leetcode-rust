
use std::collections::{HashSet, VecDeque};



impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut seen = HashSet::new();
        let mut dq = VecDeque::new();
        dq.push_back((0, 1, 'r', 0));
        seen.insert((0, 1, 'r'));

        while let Some((i, j, pos, val)) = dq.pop_front() {
            if i == n - 1 && j == n - 1 && pos == 'r' {
                return val;
            }

            // Move right

            if j + 1 < n && grid[i][j + 1] == 0 && !seen.contains(&(i, j + 1, pos)) {
                if pos == 'r' || (pos == 'd' && i > 0 && grid[i - 1][j + 1] == 0) {
                    dq.push_back((i, j + 1, pos, val + 1));
                    seen.insert((i, j + 1, pos));
                }
            }

            // Move down

            if i + 1 < n && grid[i + 1][j] == 0 && !seen.contains(&(i + 1, j, pos)) {
                if pos == 'd' || (pos == 'r' && j > 0 && grid[i + 1][j - 1] == 0) {
                    dq.push_back((i + 1, j, pos, val + 1));
                    seen.insert((i + 1, j, pos));
                }
            }

            // Change direction to down if current is right

            if pos == 'r' && j > 0 && i + 1 < n && grid[i + 1][j] == 0 && grid[i + 1][j - 1] == 0 && !seen.contains(&(i + 1, j - 1, 'd')) {
                dq.push_back((i + 1, j - 1, 'd', val + 1));
                seen.insert((i + 1, j - 1, 'd'));
            }

            // Change direction to right if current is down

            if pos == 'd' && i > 0 && j + 1 < n && grid[i - 1][j + 1] == 0 && grid[i][j + 1] == 0 && !seen.contains(&(i - 1, j + 1, 'r')) {
                dq.push_back((i - 1, j + 1, 'r', val + 1));
                seen.insert((i - 1, j + 1, 'r'));
            }
        }

        -1

    }
}
