
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut cost = vec![vec![std::i32::MAX; n]; m];
        let mut pq = BinaryHeap::new();
        
        cost[0][0] = 0;
        pq.push(Reverse((0, 0, 0))); // (cost, x, y)
        
        while let Some(Reverse((c, x, y))) = pq.pop() {
            if x == m - 1 && y == n - 1 {
                return c;
            }
            for i in 1..=4 {
                let (nx, ny) = Self::next_position(x, y, i, &grid);
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let new_cost = if i == grid[x as usize][y as usize] {
                        c

                    } else {
                        c + 1

                    };
                    if new_cost < cost[nx as usize][ny as usize] {
                        cost[nx as usize][ny as usize] = new_cost;
                        pq.push(Reverse((new_cost, nx as usize, ny as usize)));
                    }
                }
            }
        }
        
        -1 // unreachable

    }
    
    fn next_position(x: usize, y: usize, direction: i32, grid: &Vec<Vec<i32>>) -> (i32, i32) {
        match direction {
            1 => (x as i32, y as i32 + 1),
            2 => (x as i32, y as i32 - 1),
            3 => (x as i32 + 1, y as i32),
            4 => (x as i32 - 1, y as i32),
            _ => unreachable!(),
        }
    }
}
