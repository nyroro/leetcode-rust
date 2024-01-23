
use std::collections::VecDeque;

impl Solution {
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        let max_val = 1_000_000_000;
        let m = grid.len();
        let n = grid[0].len();
        let mut f = vec![vec![-1; n]; m];
        let mut p = vec![vec![-1; n]; m];
        let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();
        let directions = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    q.push_back((i, j, 0));
                }
            }
        }

        while let Some((x, y, s)) = q.pop_front() {
            f[x][y] = s;
            for (dx, dy) in &directions {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx < m && ny < n && grid[nx][ny] != 2 {
                    if f[nx][ny] == -1 || s + 1 < f[nx][ny] {
                        q.push_back((nx, ny, s + 1));
                    }
                }
            }
        }

        q.push_back((0, 0, 0));

        while let Some((x, y, s)) = q.pop_front() {
            p[x][y] = s;
            for (dx, dy) in &directions {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx < m && ny < n && grid[nx][ny] != 2 {
                    if p[nx][ny] == -1 && (s + 1 < f[nx][ny] || (s + 1 == f[nx][ny] && nx == m - 1 && ny == n - 1) || f[nx][ny] == -1) {
                        q.push_back((nx, ny, s + 1));
                    }
                }
            }
        }

        if p[m - 1][n - 1] < 0 {
            return -1;
        }
        if f[m - 1][n - 1] < 0 {
            return max_val;
        }
        if p[m - 1][n - 1] == f[m - 1][n - 1] {
            return 0;
        }
        let ret = f[m - 1][n - 1] - p[m - 1][n - 1];
        if m > 1 && n > 1 {
            let p1 = p[m - 1][n - 2];
            let p2 = p[m - 2][n - 1];
            let d1 = f[m - 1][n - 2] - p1;
            let d2 = f[m - 2][n - 1] - p2;
            if p1 >= 0 && p2 >= 0 && (d1 > ret || d2 > ret) {
                return ret;
            }
        }
        ret - 1

    }
}
