
use std::collections::BinaryHeap;
use std::cmp::Reverse;



impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        let m = grid[0].len();
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }
        let mut vis = vec![vec![0; m]; n];
        let mut bfs = BinaryHeap::new();
        bfs.push((Reverse(0), (0, 0)));

        while let Some((Reverse(tm), (x, y))) = bfs.pop() {
            if x == n - 1 && y == m - 1 {
                return tm as i64;
            }
            if vis[x][y] == 1 {
                continue;
            }
            vis[x][y] = 1;

            let move_directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
            for (dx, dy) in move_directions {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    let req = grid[nx][ny] as i64 - tm as i64;
                    let req = if req <= 1 { 1 } else if req % 2 == 0 { req + 1 } else { req };
                    if tm as i64 + req >= grid[nx][ny] as i64 && vis[nx][ny] == 0 {
                        bfs.push((Reverse(tm as i64 + req), (nx, ny)));
                    }
                }
            }
        }
        -1

    }
}
