
use std::collections::{VecDeque, BinaryHeap, HashSet};
use std::cmp::Reverse;



impl Solution {
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut queue = VecDeque::new();

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    queue.push_back((i, j));
                }
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 && grid[nx as usize][ny as usize] == 0 {
                    queue.push_back((nx as usize, ny as usize));
                    grid[nx as usize][ny as usize] = grid[x][y] + 1;
                }
            }
        }

        let mut max_safeness = BinaryHeap::new();
        max_safeness.push(Reverse((-grid[0][0], 0, 0)));
        let mut visited = HashSet::new();
        let mut result = i32::MAX;

        while let Some(Reverse((w, x, y))) = max_safeness.pop() {
            result = result.min(-w);
            if x == n - 1 && y == n - 1 {
                return result - 1;
            }
            for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 && !visited.contains(&(nx as usize, ny as usize)) {
                    max_safeness.push(Reverse((-grid[nx as usize][ny as usize], nx as usize, ny as usize)));
                    visited.insert((nx as usize, ny as usize));
                }
            }
        }

        unreachable!();
    }
}
