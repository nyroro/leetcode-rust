
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![vec![-1; (k + 1) as usize]; n]; m];
        dp[0][0][0] = 0;
        let mut queue = VecDeque::new();
        queue.push_back((0, 0, 0, 0));

        while let Some((i, j, obstacles, steps)) = queue.pop_front() {
            if i == m - 1 && j == n - 1 {
                return steps;
            }

            for (di, dj) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;
                if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                    let ni = ni as usize;
                    let nj = nj as usize;
                    let next_obstacles = obstacles + grid[ni][nj];
                    if next_obstacles <= k && dp[ni][nj][next_obstacles as usize] == -1 {
                        dp[ni][nj][next_obstacles as usize] = steps + 1;
                        queue.push_back((ni, nj, next_obstacles, steps + 1));
                    }
                }
            }
        }

        -1

    }
}
