
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let rows = heights.len();
        let cols = heights[0].len();
        let mut dist = vec![vec![std::i32::MAX; cols]; rows];
        let mut pq = BinaryHeap::new();
        
        dist[0][0] = 0;
        pq.push(Reverse((0, 0, 0)));
        
        while let Some(Reverse((effort, row, col))) = pq.pop() {
            if row == rows - 1 && col == cols - 1 {
                return effort;
            }
            
            if effort > dist[row][col] {
                continue;
            }
            
            for (dr, dc) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nr = row as i32 + dr;
                let nc = col as i32 + dc;
                
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    let new_effort = (heights[nr][nc] - heights[row][col]).abs().max(effort);
                    
                    if new_effort < dist[nr][nc] {
                        dist[nr][nc] = new_effort;
                        pq.push(Reverse((new_effort, nr, nc)));
                    }
                }
            }
        }
        
        0

    }
}
