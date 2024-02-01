
use std::collections::VecDeque;
use std::cmp;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut queue = VecDeque::new();
        let mut dist = vec![vec![-1; n]; n];
        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        
        // Find all land cells and add them to the queue

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    queue.push_back((i as i32, j as i32));
                    dist[i][j] = 0;
                }
            }
        }
        
        // Perform BFS

        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in &dirs {
                let new_x = x + dx;
                let new_y = y + dy;
                if new_x >= 0 && new_x < n as i32 && new_y >= 0 && new_y < n as i32 && dist[new_x as usize][new_y as usize] == -1 {
                    dist[new_x as usize][new_y as usize] = dist[x as usize][y as usize] + 1;
                    queue.push_back((new_x, new_y));
                }
            }
        }
        
        // Find the maximum distance

        let mut max_dist = -1;
        for i in 0..n {
            for j in 0..n {
                if dist[i][j] > 0 {
                    max_dist = cmp::max(max_dist, dist[i][j]);
                }
            }
        }
        
        max_dist

    }
}
