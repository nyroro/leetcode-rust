
use std::collections::VecDeque;

impl Solution {
    pub fn minimum_visited_cells(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        if n == 1 && m == 1 {
            return 1;
        }

        let mut par_r: Vec<Vec<i64>> = vec![vec![0; m + 1]; n];
        let mut par_c: Vec<Vec<i64>> = vec![vec![0; n + 1]; m];
        for i in 0..n {
            for j in 0..=m {
                par_r[i][j] = j as i64;
            }
        }
        for i in 0..m {
            for j in 0..=n {
                par_c[i][j] = j as i64;
            }
        }

        let mut cnt = 2;
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        q.push_back((0, 0));

        while !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz {
                let (x, y) = q.pop_front().unwrap();
                let mut j = Self::find(par_r[x][y + 1], &mut par_r[x]);
                while j < m as i64 && j <= grid[x][y] as i64 + y as i64 {
                    if x == n - 1 && j as usize == m - 1 {
                        return cnt;
                    }
                    q.push_back((x, j as usize));
                    par_r[x][j as usize] = j as i64 + 1;
                    j = Self::find(par_r[x][j as usize], &mut par_r[x]);
                }
                let mut i = Self::find(par_c[y][x + 1], &mut par_c[y]);
                while i < n as i64 && i <= grid[x][y] as i64 + x as i64 {
                    if i as usize == n - 1 && y == m - 1 {
                        return cnt;
                    }
                    q.push_back((i as usize, y));
                    par_c[y][i as usize] = i as i64 + 1;
                    i = Self::find(par_c[y][i as usize], &mut par_c[y]);
                }
            }
            cnt += 1;
        }
        -1

    }

    fn find(x: i64, par: &mut Vec<i64>) -> i64 {
        if x == par[x as usize] {
            return x;
        }
        par[x as usize] = Self::find(par[x as usize], par);
        par[x as usize]
    }
}
