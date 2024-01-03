
use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut queue = VecDeque::new();
        let mut fresh_count = 0;
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 2 {
                    queue.push_back((i, j, 0));
                } else if grid[i][j] == 1 {
                    fresh_count += 1;
                }
            }
        }
        
        let mut minutes = 0;
        while let Some((i, j, min)) = queue.pop_front() {
            minutes = minutes.max(min);
            for &(dx, dy) in &directions {
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32 && grid[x as usize][y as usize] == 1 {
                    grid[x as usize][y as usize] = 2;
                    fresh_count -= 1;
                    queue.push_back((x as usize, y as usize, min + 1));
                }
            }
        }
        
        if fresh_count == 0 {
            minutes

        } else {
            -1

        }
    }
}
