
use std::collections::VecDeque;



impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut q = VecDeque::new();
        q.push_back((0, 0, 0));
        let mut visited = vec![vec![false; n]; m];
        visited[0][0] = true;
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some((r, c, removed)) = q.pop_front() {
            if r == m - 1 && c == n - 1 {
                return removed;
            }
            for (dr, dc) in &dirs {
                let nr = (r as i32 + dr) as usize;
                let nc = (c as i32 + dc) as usize;
                if nr < m && nc < n && !visited[nr][nc] {
                    visited[nr][nc] = true;
                    let next_removed = if grid[nr][nc] == 1 { removed + 1 } else { removed };
                    if grid[nr][nc] == 0 {
                        q.push_front((nr, nc, next_removed));
                    } else {
                        q.push_back((nr, nc, next_removed));
                    }
                }
            }
        }
        -1

    }
}
