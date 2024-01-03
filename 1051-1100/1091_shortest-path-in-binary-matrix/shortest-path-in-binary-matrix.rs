
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
        
        if grid[0][0] == 1 || grid[n-1][n-1] == 1 {
            return -1;
        }
        
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; n]; n];
        
        queue.push_back((0, 0, 1));
        visited[0][0] = true;
        
        while let Some((x, y, dist)) = queue.pop_front() {
            if x == n-1 && y == n-1 {
                return dist;
            }
            
            for (dx, dy) in &directions {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 && grid[nx as usize][ny as usize] == 0 && !visited[nx as usize][ny as usize] {
                    queue.push_back((nx as usize, ny as usize, dist + 1));
                    visited[nx as usize][ny as usize] = true;
                }
            }
        }
        
        -1

    }
}
